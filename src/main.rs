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
use tokio::runtime::Runtime;
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
    pub cash: u32,
    pub free_licences: u32,
    pub licences: LicenseList,
    pub treasures: TreasureList,
}

// метрики сундуков для каждого этажа
#[derive(PartialEq, Clone, Default, Serialize, Deserialize, Debug)]
pub struct TreasuresMetrics {
    // минимальное количество денег в сундуке на этаже
    min_cash: u32,
    // максимальное количество денег в сундуке на этаже
    max_cash: u32,
    // среднее количество денег в сундуке на этаже
    avg_cash: u32,
    // количество сундуков
    treasures_count: u32,
    // количество денег на этаже
    cash_count: u32,
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

struct Metric {
    min: u64,
    max: u64,
    sum: u64,
    count: u64,
}

fn create_client() -> Client<HttpConnector> {
    let mut  http_connector = HttpConnector::new();
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

/*
запускает все на одном cpu
*/
async fn run_explore(size_x: usize, conn: usize, cpus: usize, cpu: usize) -> Result<(), u16> {
    let size_x = size_x as u32;
    let y_count = 3500 / conn;
    let conn_del_cpus = conn / cpus;

    let start = Instant::now();
    // min, max, sum, count
    let mut time_explore: (u64, u64, u64, u64) = (100_000, 0, 0, 0);

    let fx_builder: FxHashMapBuilder = FxHashMapBuilder::default();

    let mut errors: FxHashMap<u16, u32> = FxHashMap::with_capacity_and_hasher(100, fx_builder);

    let client = create_client();

    let from_inclusive = cpu * conn_del_cpus;
    let to_exclusive = (cpu * conn_del_cpus) + conn_del_cpus;

    // println!("from_inclusive: {} to_exclusive: {} size_x: {} conn: {}", from_inclusive, to_exclusive, size_x, conn);
    println!("size_x: {}", size_x);

    let mut is_area_error = false;

    let mut areas_without_gold = 0;
    let mut areas_count = 0;

    let mut amount_metric = Metric {
        min: u64::MAX,
        max: 0,
        sum: 0,
        count: 0,
    };

    // let mut store = Store {
    //     cash: 0,
    //     free_licences: 0,
    //     licences: Vec::with_capacity(100),
    //     treasures: Vec::with_capacity(100),
    //     // errors: FxHashMap::with_capacity_and_hasher(10, fx_builder),
    //     // time_explore: (0, 0, 0, 0)
    // };

    for i in from_inclusive..to_exclusive {
        let start_y = i * y_count;

        let mut y_iter = 0;

        let time = start.elapsed();

        if time.as_secs() > (60 * 9) {
            println!("timeout min: {}, max: {}, sum: {}, count: {}, avg: {}", time_explore.0, time_explore.1, time_explore.2, time_explore.3, (time_explore.2 / time_explore.3));
        }

        while y_iter < y_count {
            let pos_y = (start_y + y_iter) as u32;
            let mut pos_x = 0;

            while pos_x < 3500 {
                let area: Area = Area {
                    pos_x,
                    pos_y,
                    size_x,
                    size_y: 1,
                };

                pos_x += size_x;

                let time = Instant::now();

                let res = explore2(&client, area).await;

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
                        let amount = report.amount;
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
                            *sum = *sum + millis;
                            if millis > *max {
                                *max = millis;
                            }
                            if millis < *min {
                                *min = millis;
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
    }

    let end = start.elapsed();

    println!("end: {}", end.as_millis());
    println!("areas_count: {} areas_without_gold: {}", areas_count, areas_without_gold);
    println!("explore min: {} max: {} avg: {}", time_explore.0, time_explore.1, (time_explore.2 / time_explore.3));

    println!("amounts min: {} max: {} avg: {}", amount_metric.min, amount_metric.max, (amount_metric.sum / amount_metric.count));

    for (error_code, count) in errors {
        println!("error_code: {}, count: {}", error_code, count);
    }
    if is_area_error {
        println!("area equal error");
    }
    Ok(())
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
    println!("main start");

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

    // let v5_2 = (5, 2); // 10
    // let v5_4: (usize, usize) = (5, 4); // 20
    // let v5_20: (usize, usize) = (5, 20); // 100
    // let v5_200: (usize, usize) = (5, 100); // 500
    // let v5_2000: (usize, usize) = (5, 700); // 3_500

    // let v10_2 = (10, 2); // 20
    // let v10_4: (usize, usize) = (10, 4); // 40
    // let v10_20: (usize, usize) = (10, 20); // 200
    // let v10_100: (usize, usize) = (10, 100); // 1000
    // let v10_700: (usize, usize) = (10, 700); // 7k

    // let v50_2 = (50, 2); // 100
    // let v50_4: (usize, usize) = (50, 4); // 200
    // let v50_20: (usize, usize) = (50, 20); // 1000
    // let v50_100: (usize, usize) = (50, 100); // 5k
    // let v50_700: (usize, usize) = (50, 700); // 35k

    // let v100_2 = (100, 2); // 200
    // let v100_4: (usize, usize) = (100, 4); // 400
    // let v100_20: (usize, usize) = (100, 20); // 2000
    // let v100_100: (usize, usize) = (100, 100); // 10k
    // let v100_700: (usize, usize) = (100, 700); // 70k

    // let v500_2 = (500, 2); // 1000
    // let v500_4: (usize, usize) = (500, 4); // 2000
    // let v500_20: (usize, usize) = (500, 20); // 10k
    // let v500_100: (usize, usize) = (500, 100); // 50k
    // let v500_700: (usize, usize) = (500, 700); // 350k

    let v50_4: (usize, usize) = (50, 2);
    let v70_4: (usize, usize) = (70, 2);
    let v100_4: (usize, usize) = (100, 2);
    let v125_4: (usize, usize) = (125, 2);
    // let v140_4: (usize, usize) = (140, 4);
    // let v175_4: (usize, usize) = (175, 4);
    // let v250_4: (usize, usize) = (250, 4);
    // let v350_4: (usize, usize) = (350, 4);
    // let v500_4: (usize, usize) = (500, 4);
    // let v700_4: (usize, usize) = (700, 4);
    // let v875_4: (usize, usize) = (875, 4);

    // let cpus = num_cpus::get();
    // println!("cpu count: {}", cpus);
    // let cpus = if cpus > 4 { 4 } else { cpus };

    let cpus = 2;

    // let rt = Runtime::new()?;
    let basic_rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    basic_rt.block_on(async {
        let start = Instant::now();
        loop {
            let client = create_client();
            if health_check2(&client).await == 200 {
                break;
            }
        }
        let end = start.elapsed();
        println!("server ready: {}", end.as_millis());
    });

    let (size_x, conn) = v50_4;
    let f = run_explore(size_x, conn, cpus, 0);
    basic_rt.block_on(f);

    let (size_x, conn) = v70_4;
    let f = run_explore(size_x, conn, cpus, 0);
    basic_rt.block_on(f);

    let (size_x, conn) = v100_4;
    let f = run_explore(size_x, conn, cpus, 0);
    basic_rt.block_on(f);

    let (size_x, conn) = v125_4;
    let f = run_explore(size_x, conn, cpus, 0);
    basic_rt.block_on(f);

    Ok(())
    // system.run()
}
