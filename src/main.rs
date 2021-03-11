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
use hyper::client::HttpConnector;
use hyper::Client;
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

#[derive(PartialEq, Clone, Default, Serialize, Deserialize, Debug)]
struct Metric {
    min: u64,
    max: u64,
    sum: u64,
    count: u64,
}

fn create_client() -> Client<HttpConnector> {
    let mut http_connector = HttpConnector::new();
    // let mut connector = TimeoutConnector::new(http_connector);
    http_connector.set_keepalive(Some(Duration::from_secs(10)));
    // http_connector.set_connect_timeout(Some(Duration::from_millis(10_000)));
    http_connector.set_connect_timeout(Some(Duration::from_millis(10)));
    http_connector.set_reuse_address(true);
    http_connector.set_nodelay(true);

    let client = hyper::Client::builder()
        .pool_idle_timeout(Duration::from_secs(10))
        .build(http_connector);
    client
}

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
        .unwrap();
    client
}

#[derive(PartialEq, Clone, Default, Serialize, Deserialize, Debug)]
struct ExploreArgs {
    pub size_x: u64, // размер поля в точках
    pub size_y: u64, // размер поля в точках
    pub cpus: u64,
    pub cpu: u64,
    pub area_divisor: u64,
    pub start_x: u64,
    pub timeout_millis: u64,
}

/*
запускает все на одном cpu
*/
async fn run_explore(args: ExploreArgs) -> Result<Metric, u16> {
    let ExploreArgs {
        size_x,
        size_y,
        cpus,
        cpu,
        area_divisor,
        start_x,
        timeout_millis,
    } = args;

    let y_iter_count = 3500u64 / area_divisor / cpus / size_y;

    let start = Instant::now();
    // min, max, sum, count
    let mut time_explore: (u64, u64, u64, u64) = (100_000, 0, 0, 0);

    let fx_builder: FxHashMapBuilder = FxHashMapBuilder::default();

    let mut errors: FxHashMap<u16, u32> = FxHashMap::with_capacity_and_hasher(100, fx_builder);

    let client = client_builder(timeout_millis);

    println!("cpu: {} size_x: {} size_y: {} cpus: {} area_divisor: {}", cpu, size_x, size_y, cpus, area_divisor);

    let mut is_area_error = false;

    let mut areas_without_gold = 0;
    let mut areas_count = 0;

    let mut amount_metric = Metric {
        min: u64::MAX,
        max: 0,
        sum: 0,
        count: 0,
    };

    let start_y = cpu * y_iter_count * size_y;

    let mut y_iter = 0;

    while y_iter < y_iter_count {
        let pos_y = start_y + (y_iter * size_y);
        let mut pos_x = start_x;

        while pos_x < 3500 {
            let area: Area = Area {
                pos_x,
                pos_y,
                size_x,
                size_y,
            };

            pos_x += size_x;

            let time = Instant::now();

            // let res = explore2(&client, area).await;
            let res = explore3(&client, &area).await;

            areas_count += 1;

            let duration = time.elapsed();
            let millis = duration.as_micros() as u64;
            let min = &mut time_explore.0;
            let max = &mut time_explore.1;
            let sum = &mut time_explore.2;
            let count = &mut time_explore.3;
            *count = *count + 1;
            *sum = *sum + millis;
            if millis > *max {
                *max = millis;
            }
            if millis < *min {
                *min = millis;
            }

            match res {
                Ok(report) => {
                    let amount = report.amount as u64;
                    let ret_area = report.area;
                    if ret_area != area {
                        is_area_error = true;
                    }

                    if amount > 0 {

                        let min = &mut amount_metric.min;
                        let max = &mut amount_metric.max;
                        let sum = &mut amount_metric.sum;
                        let count = &mut amount_metric.count;
                        *count = *count + 1;
                        *sum = *sum + amount;
                        if amount > *max {
                            *max = amount;
                        }
                        if amount < *min {
                            *min = amount;
                        }

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

    println!("cpu: {} end: {}", cpu, end.as_millis());
    println!("cpu: {} areas_count: {} areas_without_gold: {}", cpu, areas_count, areas_without_gold);

    if time_explore.3 > 0 {
        println!("cpu: {} explore min: {} max: {} avg: {}", cpu, time_explore.0, time_explore.1, (time_explore.2 / time_explore.3));
    }

    if amount_metric.count > 0 {
        println!("cpu: {} amounts min: {} max: {} avg: {}", cpu, amount_metric.min, amount_metric.max, (amount_metric.sum / amount_metric.count));
    }

    for (error_code, count) in errors {
        println!("cpu: {} error_code: {}, count: {}", cpu, error_code, count);
    }
    if is_area_error {
        println!("cpu: {} area equal error",  cpu);
    }
    Ok(amount_metric)
}

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
        .build()?;

    rt.block_on(async {
        let start = Instant::now();
        loop {
            let client = client_builder(7);
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

    // 1, 2, 4, 5, 7, 10, 14, 20, 25, 28, 35, 50, 70, 100, 125, 140,  175,  250,  350,  500,  700

    // let cpus = 2;
    // let size_x = 35;
    // // if cpus = 2 and size_x = 50, area_divisor one of 5, 7
    // // if cpus = 2 and size_x = 35, area_divisor one of 2, 5, 10
    // // (3500 / cpus / size_x) should be divided on area_divisor
    // let area_divisor = 2;
    // let f1 = run_explore(size_x, cpus, 0, area_divisor);
    // let f2 = run_explore(size_x, cpus, 0, area_divisor);

    rt.block_on(async {
        // area_divisor: 1, 2, 4, 5, 10, 20
        // run_explore(35, 1, 1, 0, 20).await;
        // run_explore(ExploreArgs {
        //     size_x: 35,
        //     size_y: 1,
        //     cpus: 1,
        //     cpu: 0,
        //     area_divisor: 20,
        //     start_x: 0,
        //     timeout_millis: 7
        // }).await;

        let args7 = ExploreArgs {
            size_x: 3500,
            size_y: 500,
            cpus: 7,
            cpu: 0,
            area_divisor: 1,
            start_x: 0,
            timeout_millis: 60_000,
        };
        let a0 = run_explore(args7.clone()).await;
        let a1 = run_explore(ExploreArgs { cpu: 1, ..args7 }).await;
        let a2 = run_explore(ExploreArgs { cpu: 2, ..args7 }).await;
        let a3 = run_explore(ExploreArgs { cpu: 3, ..args7 }).await;
        let a4 = run_explore(ExploreArgs { cpu: 4, ..args7 }).await;
        let a5 = run_explore(ExploreArgs { cpu: 5, ..args7 }).await;
        let a6 = run_explore(ExploreArgs { cpu: 6, ..args7 }).await;

        println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?}", a0, a1, a2, a3, a4, a5, a6)

        // let f1 = tokio::spawn(f1);
        // let f2 = tokio::spawn(f2);
        // tokio::join!(f1, f2);
    });
        // .map_err(|e| {
        //     eprintln!("{}", e);
        // });

    Ok(())
    // system.run()
}
