#![allow(dead_code)]
// #[macro_use]
// extern crate num_derive;
#[macro_use]
extern crate serde_derive;
// #[macro_use]
extern crate serde_json;
// #[macro_use]
// extern crate log;
#[macro_use]
extern crate lazy_static;
// use hyper::Client;

mod models;
mod requests;
// mod explore;
// mod floor;

// use actix::{dev::StreamHandler, Actor};
// use actix_web::{
//     middleware::Logger,
//     web::{get, post, put, Json},
//     {web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder},
// };
// use env_logger::Env;
use rustc_hash::{FxHashMap, FxHasher};
// use tokio::runtime::Runtime;
// use std::{cell::RefCell, rc::Rc};
// use std::io::{stdout, Write};
use crate::models::*;
// use bincode::Options;
use std::time::{Instant};
use crate::requests::*;
// use std::collections::{HashMap, HashSet};
use std::hash::BuildHasherDefault;
use tokio::time::Duration;
// use hyper::client::HttpConnector;
// use hyper::Client;
use std::collections::{BinaryHeap, binary_heap::PeekMut};
use std::sync::Arc;
// use parking_lot::{RwLock};
use tokio::sync::{RwLock, Mutex};
use crossbeam::queue::ArrayQueue;
// use hyper_timeout::TimeoutConnector;
// use bitvec::prelude::*;
// use chrono::{Utc, DateTime};
// use crate::floor::Floor;
// use actix::Actor;
// use actix_rt::{Arbiter, System};
// use awc::{Client, ClientBuilder, Connector};

pub type FxHashMapBuilder = BuildHasherDefault<FxHasher>;

// #[derive(Clone)]
// pub struct WebClient {
//     pub client: Client
// }

#[derive(PartialEq, Clone, Default, Serialize, Deserialize, Debug)]
pub struct Store {
    pub cash: u64,
    pub free_licences: u64,
    pub licences: LicenseList,
    pub treasures: TreasureList,
}

// метрики сундуков для каждого этажа
#[derive(PartialEq, Clone, Default, Serialize, Deserialize, Debug)]
pub struct TreasuresMetrics {
    // минимальное количество денег в сундуке на этаже
    min_cash: u64,
    // максимальное количество денег в сундуке на этаже
    max_cash: u64,
    // среднее количество денег в сундуке на этаже
    avg_cash: u64,
    // количество сундуков
    treasures_count: u64,
    // количество денег на этаже
    cash_count: u64,
}

#[derive(PartialEq, Clone, Default, Serialize, Deserialize, Debug)]
pub struct LicencesMetrics {
    // min_amount: u32,
}

// #[derive(PartialEq, Clone, Default, Serialize, Deserialize, Debug)]
// pub struct FloorStore {
//     pub cash: u32,
//     // min_cash, max_cash, avg_cash, count
//     pub treasures_metrics: TreasuresMetrics,
//     // TODO: нужно записывать параметры полученных лицензий для того чтобы определить лучшие параметры для запроса на получение лицензий
//     pub licences_metrics: LicencesMetrics,
//     // key - (error_code, request_id), value - count
//     pub errors: FxHashMap<(u16, u16), u32>,
//     // min, max, sum, count
//     pub time_explore: (u64, u64, u64, u64),
//     // min, max, sum, count
//     pub time_pull_licences: (u64, u64, u64, u64),
// }

const SIDE_SIZE: usize = 3500;
// 12_250_000
const POINT_COUNT: usize = SIDE_SIZE * SIDE_SIZE;
// const ONE_ITER_COUNT: usize = 490;
// 25_000 (8*3125)
// const ITERATIONS: usize = POINT_COUNT / ONE_ITER_COUNT;
const FLOOR_COUNT: usize = 10;

const VARIANTS: [usize; 5] = [5, 10, 50, 100, 500];

// pub type BitFields = BitArr!(for 512, in Msb0, u8);

// fn create_client() -> Client {
//     let cb = ClientBuilder::new();
//     let connector = Connector::new()
//         .conn_keep_alive(Duration::from_secs(70))
//         // TODO: нужно не ждать ответа слишком долго, так как намеренно будут тянуть с ответом
//         // .conn_lifetime(Duration::from_secs((60*9)+58))
//         .conn_lifetime(Duration::from_secs(5))
//         .finish();
//
//     let client = cb
//         .disable_redirects()
//         .connector(connector)
//         .finish();
//     client
// }

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Debug)]
struct Metric {
    min: u64,
    max: u64,
    sum: u64,
    count: u64,
}

impl Default for Metric {
    fn default() -> Self {
        Metric {
            min: u64::MAX,
            max: 0,
            sum: 0,
            count: 0,
        }
    }
}

impl Metric {
    pub fn update(&mut self, value: u64) {
        self.count += 1;
        self.sum += value;
        if value > self.max {
            self.max = value;
        }
        if value < self.min {
            self.min = value;
        }
    }

    pub fn avg(&self) -> f64 {
        if self.count > 0 {
            (self.sum as f64) / (self.count as f64)
        } else {
            0f64
        }
    }

    pub fn print(&self, item: &str) {
        // let percentile25 = 0;
        // let median = 0;
        // let percentile75 = 0;
        println!(
            // "{} min: {} max: {} avg: {} 25: {} 50: {}, 75: {}",
            "{} min: {} max: {} avg: {}",
            item,
            self.min,
            self.max,
            self.avg(),
            // percentile25,
            // median,
            // percentile75,
        );
    }
}

// fn create_client() -> Client<HttpConnector> {
//     let mut http_connector = HttpConnector::new();
//     // let mut connector = TimeoutConnector::new(http_connector);
//     http_connector.set_keepalive(Some(Duration::from_secs(10)));
//     // http_connector.set_connect_timeout(Some(Duration::from_millis(10_000)));
//     http_connector.set_connect_timeout(Some(Duration::from_millis(10)));
//     http_connector.set_reuse_address(true);
//     http_connector.set_nodelay(true);
//
//     let client = hyper::Client::builder()
//         .pool_idle_timeout(Duration::from_secs(10))
//         .build(http_connector);
//     client
// }

fn client_builder(timeout_millis: u64) -> reqwest::Client {
    // let mut headers = reqwest::header::HeaderMap::new();
    // headers.insert("content-type", reqwest::header::HeaderValue::from_static("application/json"));

    let client = reqwest::ClientBuilder::new()
        // .default_headers(headers)
        // .connect_timeout(Duration::from_millis(3))
        .timeout(Duration::from_millis(timeout_millis)) // TODO: for explore large areas нужно устанавливать большие лимиты
        .pool_idle_timeout(Duration::from_secs(2))
        .tcp_nodelay(true)
        .build()
        .expect("client builder error");
    client
}

#[derive(PartialEq, Clone, Default, Serialize, Deserialize, Debug)]
struct ExploreArgs {
    pub step_x: u64, // размер поля в точках, то есть шаг координаты х
    pub step_y: u64, // размер поля в точках, то есть шаг координаты у
    pub size_x: u64,
    pub size_y: u64,
    pub cpus: u64, // количество воркеров
    pub cpu: u64, // номер текущего воркера
    pub area_divisor: u64, // на сколько нужно поделить поле (size_y / area_divisor), делится координата у
    pub start_x: u64, // с какой точки координаты х начать
    pub start_y: u64, // с какой точки координаты у начать
    pub timeout_millis: u64, // максимальное время для запроса в миллисекундах
}

/*
запускает все на одном cpu
*/
async fn run_explore(args: ExploreArgs, is_log: bool, level: u16) -> Result<BinaryHeap<Report>, u16> {
    let ExploreArgs {
        step_x,
        step_y,
        size_x,
        size_y,
        cpus,
        cpu,
        area_divisor,
        start_x,
        start_y,
        timeout_millis,
    } = args;

    let y_iter_count = size_y / area_divisor / cpus / step_y;

    let start = Instant::now();
    // min, max, sum, count
    let mut time_explore: Metric = Metric::default();

    let fx_builder: FxHashMapBuilder = FxHashMapBuilder::default();

    let mut errors: FxHashMap<u16, u32> = FxHashMap::with_capacity_and_hasher(100, fx_builder.clone());

    let client = client_builder(timeout_millis);

    if is_log {
        println!("cpu: {} step_x: {} step_y: {} cpus: {} area_divisor: {}", cpu, step_x, step_y, cpus, area_divisor);
        println!("size_x: {} size_y: {} areas_count: {} ", size_x, size_y, (size_x / step_x));
    }


    let mut is_area_error = false;

    let mut areas_without_gold = 0;

    let mut amount_map: FxHashMap<u64, u64> = FxHashMap::with_capacity_and_hasher(14000, fx_builder.clone());

    let mut amount_metric = Metric::default();

    let mut extremes_count = 0;

    let start_y = start_y + (cpu * y_iter_count * step_y);

    let mut y_iter = 0;

    let cap =  if level > 0 { (((size_x * size_y) / area_divisor) / step_x) / step_y } else { 60 }; // 17_500 for level 0 and 5 for level 1

    let mut reports: BinaryHeap<Report> = BinaryHeap::with_capacity(cap as usize);

    'y_iter: while y_iter < y_iter_count {
        let pos_y = start_y + (y_iter * step_y);
        let mut pos_x = start_x;

        while pos_x < (start_x + size_x) {
            let area: Area = Area {
                pos_x,
                pos_y,
                size_x: step_x,
                size_y: step_y,
            };

            pos_x += step_x;

            let time = Instant::now();

            // let res = explore2(&client, area).await;
            let res = explore3(&client, &area).await;

            let duration = time.elapsed();
            let millis = duration.as_micros() as u64;
            time_explore.update(millis);

            match res {
                Ok(report) => {
                    // если нашли большое количество экстремумов, то не продолжать поиски,
                    // переходить на следующее поле.

                    let amount = report.amount as u64;
                    let ret_area = report.area;
                    if ret_area != area {
                        is_area_error = true;
                    }

                    if amount > 0 {
                        match amount_map.get_mut(&amount) {
                            Some(v) => {
                                *v += 1;
                            }
                            None => {
                                amount_map.insert(amount, 1);
                            }
                        }

                        if level == 0 {
                            // экстремумы - те, которые в 5 раз больше минимального значения
                            // FIXME: const
                            if amount > (amount_metric.min * 5) {
                                extremes_count += 1;
                                // TODO: стоит туда отправлять только экстремумы?
                                reports.push(report);

                                // FIXME: const
                                if extremes_count > 50 {
                                    break 'y_iter;
                                }
                            }
                        } else {
                            reports.push(report);
                        }

                        amount_metric.update(amount);

                        // if store.licences.len() == 0 {
                        //     let wallet = vec![];
                        //     let licence = pull_licenses2(&client, wallet).await?;
                        //     store.licences.push(licence);
                        // }
                        // match store.licences.pop() {
                        //     Some(license) => {}
                        //     None => {}
                        // }
                    } else {
                        areas_without_gold += 1;
                    }
                }
                Err(code) => {
                    match errors.get_mut(&code) {
                        Some(count) => {
                            *count += 1;
                        }
                        None => {
                            errors.insert(code, 1);
                        }
                    }
                }
            }
        }
        y_iter += 1;
    }

    let end = start.elapsed();

    if is_log {
        println!("end: {}", end.as_millis());

        if level < 2 {
            println!("areas_without_gold: {}", areas_without_gold);
        }

        if time_explore.count > 0 {
            println!("explore min: {} max: {} avg: {}", time_explore.min, time_explore.max, time_explore.avg());
        }

        // array_extremes_count - число полей с большим числом сундуков
        if amount_metric.count > 0 && level != 2 {
            println!("area_amounts_min: {} area_max: {} area_amounts_avg: {} array_extremes_count (> min*5): {}", amount_metric.min, amount_metric.max, amount_metric.avg(), extremes_count);

            for (amount_in_area, areas_count) in amount_map.iter() {
                println!("amount_in_area: {} areas_count: {}", amount_in_area, areas_count);
            }
        }
        if level == 2 {
            println!("exists in point: {}", amount_metric.sum);
        }

        for (error_code, count) in errors {
            println!("error_code: {}, count: {}", error_code, count);
        }
        if is_area_error {
            println!("area equal error");
        }
    }

    Ok(reports)
}
/*
async fn get_license(licenses: &mut Arc<Mutex<BinaryHeap<License>>>, balance: &mut Balance, client: &reqwest::Client, iter: usize) -> Result<bool, u16> {
    let mut licenses = licenses.lock().await;
    println!("licenses len: {}", licenses.len());
    if licenses.is_empty() { // TODO
        if balance.balance == 0 {
            println!("iter: {}", iter);
            let license = pull_licenses3(&client, &Vec::new(), false, false).await;
            license.map(|license| {
                licenses.push(license);
            });
            Err(1)
        } else {
            // FIXME: const value 1 for license amount
            let mut coins: Vec<u64> = Vec::with_capacity(iter);
            for _ in 0..iter {
                let coin = balance.wallet.pop().unwrap_or(0);
                coins.push(coin);
                balance.balance -= 1;
            }
            let is_log = if iter < 3 { println!("iter: {}", iter); true } else { false };
            let license = pull_licenses3(&client, &coins, is_log, is_log).await;
            license.map(|license| {
                licenses.push(license);
                // TODO: push is clone???
            });
            Err(1)
        }
    } else {
        let license = licenses.peek_mut().expect("licenses is empty");
        if license.dig_count() == 0 {
            PeekMut::<'_, models::License>::pop(license);
            // license.pop();
            // get_license(licenses, balance, client).await
            // Err(1)
        } else {
            // Ok((PeekMut::<'_, models::License>::pop(license), false))
            // license.pop()
            // Ok((license, false))
        }
        Ok(true)
    }
}
*/
pub type ErrMapArc = Arc<RwLock<FxHashMap<u16, u32>>>;

pub async fn err_add(map: &ErrMapArc, err: u16) {
    let mut w = map.write().await;
    match w.get_mut(&err) {
        Some(count) => {
            *count += 1;
        }
        None => {
            w.insert(err, 1);
        }
    }
}

pub type BalanceArc = Arc<Mutex<Balance>>;

// #[actix_rt::main]
// -> std::io::Result<()>
// #[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let system = System::new();
    // env_logger::Builder::from_env(Env::default().default_filter_or("info"))
    //     .format(|buf, record| { writeln!(buf, "{}", re) })
    //     .init();
    // info!("main start!");
    // let time: DateTime<Utc> = Utc::now();
    // println!("main start");

    // let store = Store {
    //     cash: 0,
    //     free_licences: 0,
    //     licences: Vec::with_capacity(100),
    //     treasures: Vec::with_capacity(100),
    //     // errors: FxHashMap::with_capacity_and_hasher(10, fx_builder),
    //     // time_explore: (0, 0, 0, 0)
    // };

    // let bitfields = bitarr![Msb0, u8; 0; 512];

    /*
    сначала делаем explore с координатами x,y (какими?)
    потом копаем?
    нужно ввести некий показатель ценности этажа (соотношение ценности кладов к стоимости их добычи)
    необходимо так же записывать время на api запрос на каждом этаже: минимальное время, максимальное время, среднее арифметическое время
    */

    // let cpus = num_cpus::get();
    // println!("cpu count: {}", cpus);
    // let cpus = if cpus > 4 { 4 } else { cpus };

    // let rt = Runtime::new()?;
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(4)
        .build()?;

    rt.block_on(async {
        let start = Instant::now();
        loop {
            let client = client_builder(10);
            if health_check3(&client).await == 200 {
                break;
            }
            if start.elapsed().as_secs() > 1 {
                println!("too many times wait ready server");
                break;
            }
        }
        let end = start.elapsed();
        println!("server ready: {}", end.as_millis());
    });

    // for 3500:
    // 1, 2, 4, 5, 7, 10, 14, 20, 25, 28, 35, 50, 70, 100, 125, 140,  175,  250,  350,  500,  700
    // for 35:
    // 1, 5, 7

    // let cpus = 2;
    // let size_x = 35;
    // // if cpus = 2 and size_x = 50, area_divisor one of 5, 7
    // // if cpus = 2 and size_x = 35, area_divisor one of 2, 5, 10
    // // (3500 / cpus / size_x) should be divided on area_divisor
    // let area_divisor = 2;
    // let f1 = run_explore(size_x, cpus, 0, area_divisor);
    // let f2 = run_explore(size_x, cpus, 0, area_divisor);

    // for var in std::env::vars() {
    //     println!("key: {} value: {}", var.0, var.1);
    // }

    // TODO: какая структура данных должна быть у лицензии?
    // Arc+Mutex + ??? Vec? FxHashMap? Queue?
    // Можно брать лицензию из очереди, работать с ней и ложить обратно в очередь. Таким образом очередь будет идти по циклу (по кругу)...

    let fx_builder: FxHashMapBuilder = FxHashMapBuilder::default();

    let license_set: LicenseSet = Arc::new(ArrayQueue::new(10));
    let license_errors: ErrMapArc = Arc::new(RwLock::new(FxHashMap::with_capacity_and_hasher(100, fx_builder.clone())));

    let balance: Balance = Balance {
        balance: 0,
        wallet: Vec::with_capacity(500),
    };
    let balance = Arc::new(Mutex::new(balance));

    let mut license_set2 = license_set.clone();
    let le1 = license_errors.clone();

    rt.spawn(async move {
        // println!("licenses get started");
        let client = reqwest::ClientBuilder::new()
            // .connect_timeout(Duration::from_millis(3))
            .timeout(Duration::from_millis(150))
            // .pool_idle_timeout(Duration::from_secs(2))
            // .tcp_nodelay(true)
            .build()
            .expect("client builder error");

        // get balance
        let balance = balance3(&client).await;
        println!("balance: {:?}", balance);

        // get licenses
        if let Ok(l) = licenses3(&client).await {
            println!("licenses: {:?}", l);
        }

        while license_set2.len() < 3 {
            match pull_licenses3(&client, &vec![], false, false).await {
                Ok(license) => {
                    license_set2.push(license);
                    // println!("congrats! pull license successful");
                }
                Err(e) => {
                    err_add(&le1, e);
                }
            }
        }
    });

    let mut license_set2 = license_set.clone();
    let balance2 = balance.clone();
    let le1 = license_errors.clone();

    rt.spawn(async move {
        let client = client_builder(100);
        let mut interval = tokio::time::interval(Duration::from_secs(5));
        interval.tick().await;

        loop {
            interval.tick().await;
            if license_set2.len() < 5 {
                let license = get_license(&client, &balance2, &le1, 1).await;
                license_set2.push(license);
                let license = get_license(&client, &balance2, &le1, 1).await;
                license_set2.push(license);
                let license = get_license(&client, &balance2, &le1, 1).await;
                license_set2.push(license);

            }
        }

    });

    let (cash_sender, mut cash_receiver) = tokio::sync::mpsc::channel::<Treasure>(1000);

    let balance2 = balance.clone();

    rt.spawn(async move {
        let mut cash_time = Metric::default();
        let mut cash_count = Metric::default();
        let mut cash_errors: FxHashMap<u16, u32> = FxHashMap::with_capacity_and_hasher(100, fx_builder.clone());
        let client = client_builder(100);

        while let Some(t) = cash_receiver.recv().await {
            let time = Instant::now();
            match cash3(&client, &t).await {
                Ok(mut wallet) => {
                    let millis = time.elapsed().as_millis() as u64;
                    let count = wallet.len();
                    cash_time.update(millis);
                    // TODO
                    let mut balance = balance2.lock().await;
                    balance.balance += count as u64;
                    balance.wallet.append(&mut wallet);
                    cash_count.update(count as u64);
                }
                Err(e) => {
                    match cash_errors.get_mut(&e) {
                        Some(count) => {
                            *count += 1;
                        }
                        None => {
                            cash_errors.insert(e, 1);
                        }
                    }
                }
            }
        }
    });

    let (explore35_sender, mut explore35_receiver) = tokio::sync::mpsc::channel::<Treasure>(1000);

    let res = rt.block_on(async {
        println!("explore started");
        // area_divisor: 1, 2, 4, 5, 10, 20
        // FIXME: const
        let args = ExploreArgs {
            step_x: 35,
            step_y: 1,
            size_x: 3500,
            size_y: 3500,
            cpus: 1,
            cpu: 0,
            area_divisor: 20,
            start_x: 0,
            start_y: 0,
            timeout_millis: 10,
        };

        run_explore(ExploreArgs { cpu: 0, ..args }, false, 0).await

        // for i in 0..1  {
        //     run_explore(ExploreArgs { cpu: i, ..args }).await;
        // }

        // let f1 = tokio::spawn(f1);
        // let f2 = tokio::spawn(f2);
        // tokio::join!(f1, f2);
    });

    rt.block_on(async {

    });

    if let Ok(mut res) = res {
        let balance = balance.clone();
        let mut license_set2 = license_set.clone();
        let le1 = license_errors.clone();

        let tx = cash_sender.clone();

        rt.block_on(async {
            let client = client_builder(100);
            // let empty_wallet: Wallet = vec![];

            // let mut license_time = Arc::new(RwLock::new(Metric::default()));
            let license_time = Arc::new(tokio::sync::RwLock::new(Metric::default()));
            let dig_time = Arc::new(RwLock::new(Metric::default()));
            let cash_time = Arc::new(RwLock::new(Metric::default()));

            let license_count = Arc::new(RwLock::new(Metric::default()));
            let cash_count = Arc::new(RwLock::new(Metric::default()));

            let fx_builder: FxHashMapBuilder = FxHashMapBuilder::default();

            let dig_errors: ErrMapArc = Arc::new(RwLock::new(FxHashMap::with_capacity_and_hasher(100, fx_builder.clone())));
            let cash_errors: ErrMapArc = Arc::new(RwLock::new(FxHashMap::with_capacity_and_hasher(100, fx_builder.clone())));

            let lt1 = license_time.clone();
            let dt1 = dig_time.clone();
            let ct1 = cash_time.clone();
            let lc1 = license_count.clone();
            let cc1 = cash_count.clone();
            let de1 = dig_errors.clone();
            let ce1 = cash_errors.clone();

            tokio::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(60));
                let mut interval_iter = 0;
                interval.tick().await;
                loop {
                    interval.tick().await;
                    println!("interval_iter: {}", interval_iter);
                    let ltr = lt1.read().await;
                    // let ltr = lt1.read();
                    println!(
                        "license_time min: {} max: {} sum: {} count: {} avg: {}",
                        ltr.min,
                        ltr.max,
                        ltr.sum,
                        ltr.count,
                        ltr.avg(),
                    );

                    let dtr = dt1.read().await;
                    println!(
                        "dig_time min: {} max: {} sum: {} count: {} avg: {}",
                        dtr.min,
                        dtr.max,
                        dtr.sum,
                        dtr.count,
                        dtr.avg(),
                    );

                    let ctr = ct1.read().await;
                    println!(
                        "cash_time min: {} max: {} sum: {} count: {} avg: {}",
                        ctr.min,
                        ctr.max,
                        ctr.sum,
                        ctr.count,
                        ctr.avg(),
                    );

                    let lcr = lc1.read().await;
                    println!(
                        "license_count min: {} max: {} sum: {} count: {} avg: {}",
                        lcr.min,
                        lcr.max,
                        lcr.sum,
                        lcr.count,
                        lcr.avg(),
                    );

                    let ccr = cc1.read().await;
                    println!(
                        "cash_count min: {} max: {} sum: {} count: {} avg: {}",
                        ccr.min,
                        ccr.max,
                        ccr.sum,
                        ccr.count,
                        ccr.avg(),
                    );

                    let ler = le1.read().await;
                    for (error_code, count) in ler.iter() {
                        println!("license_errors code: {}, count: {}", error_code, count);
                    }

                    let der = de1.read().await;
                    for (error_code, count) in der.iter() {
                        println!("dig_errors code: {}, count: {}", error_code, count);
                    }

                    let cer = ce1.read().await;
                    for (error_code, count) in cer.iter() {
                        println!("cash_errors code: {}, count: {}", error_code, count);
                    }

                    interval_iter += 1;
                }
            });

            while let Some(report) = res.pop() {
                // FIXME: const
                let args = ExploreArgs {
                    step_x: 7,
                    step_y: 1,
                    size_x: 35,
                    size_y: 1,
                    cpus: 1,
                    cpu: 0,
                    area_divisor: 1,
                    start_x: report.area.pos_x,
                    start_y: report.area.pos_y,
                    timeout_millis: 10,
                };

                /*
                берем 35 точек, делаем поиск по 7 точек. возвращается приоритетная очередь по площадям
                */
                let res = run_explore(args, false, 1).await;
                if let Ok(mut bh) = res {
                    while let Some(report) = bh.pop() {
                        // TODO: стоит ли делать explore? может быстрее сразу копать?
                        // FIXME: const
                        let args = ExploreArgs {
                            step_x: 1,
                            step_y: 1,
                            size_x: 7,
                            size_y: 1,
                            cpus: 1,
                            cpu: 0,
                            area_divisor: 1,
                            start_x: report.area.pos_x,
                            start_y: report.area.pos_y,
                            timeout_millis: 10,
                        };

                        let res = run_explore(args, false, 2).await;
                        if let Ok(res) = res {
                            // тут уже либо есть либо нет
                            let h = res.peek();
                            if let Some(report) = h {
                                if report.amount > 0 {
                                    let x = report.area.pos_x;
                                    let y = report.area.pos_y;

                                    let le1 = license_errors.clone();

                                    let mut found_count = 0;
                                    let mut depth = 1;

                                    loop {
                                        if (depth > 10) || (found_count == report.amount) {
                                            break;
                                        }

                                        let time = Instant::now();

                                        // FIXME: sum = 1
                                        let mut license = license_pull(&license_set2, &client, &balance, &le1, 1).await;

                                        // TODO: тут будут считаться и локальные запросы, без запроса на сервер!!!
                                        let millis = time.elapsed().as_millis() as u64;
                                        let mut ltw = license_time.write().await; // TODO: lt1?
                                        ltw.update(millis);
                                        let mut lcw = license_count.write().await; // TODO: lc1?
                                        lcw.update(license.dig_allowed);

                                        let dig: Dig = Dig {
                                            license_id: license.id,
                                            pos_x: x,
                                            pos_y: y,
                                            depth, // TODO: ???
                                        };

                                        let time = Instant::now();
                                        let res = dig3(&client, &dig).await;

                                        depth += 1;

                                        license.dig_used += 1;

                                        if license.dig_count() != 0 {
                                            // println!("{:?}", license);
                                            license_set2.push(license).expect("license is full");
                                        }

                                        // if let Ok(l) = licenses3(&client).await {
                                        //     println!("{:?}", l);
                                        // }

                                        let millis = time.elapsed().as_millis() as u64;
                                        let mut dtw = dig_time.write().await; // TODO: dt1?
                                        dtw.update(millis);
                                        match res {
                                            Ok(res) => {
                                                let len = res.len();
                                                found_count += len as u64;

                                                for t in res {
                                                    tx.send(t).await.map_err(|e| {
                                                        eprintln!("{:?}", e);
                                                    });
                                                    // let time = Instant::now();
                                                    /*match cash3(&client, &t).await {
                                                        Ok(mut wallet) => {
                                                            let millis = time.elapsed().as_millis() as u64;
                                                            let count = wallet.len();
                                                            let mut ctw = cash_time.write().await;
                                                            ctw.update(millis);
                                                            // TODO
                                                            let mut balance = balance.lock().await;
                                                            balance.balance += count as u64;
                                                            balance.wallet.append(&mut wallet);
                                                            let mut ccw = cash_count.write().await;

                                                            ccw.update(count as u64);
                                                        }
                                                        Err(e) => {
                                                            let mut w = cash_errors.write().await;
                                                            match w.get_mut(&e) {
                                                                Some(count) => {
                                                                    *count += 1;
                                                                }
                                                                None => {
                                                                    w.insert(e, 1);
                                                                }
                                                            }
                                                        }
                                                    }*/
                                                }
                                            }
                                            Err(e) => {
                                                let mut w = dig_errors.write().await;
                                                match w.get_mut(&e) {
                                                    Some(count) => {
                                                        *count += 1;
                                                    }
                                                    None => {
                                                        w.insert(e, 1);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
    };

    Ok(())
}
