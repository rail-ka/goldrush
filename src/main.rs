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
// use std::{cell::RefCell, rc::Rc};
// use std::io::{stdout, Write};
use crate::models::*;
// use bincode::Options;
use std::time::{Instant};
use crate::requests::*;
// use std::collections::{HashMap, HashSet};
use std::hash::BuildHasherDefault;
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


/*
запускает все на одном cpu
*/
async fn run_explore(size_x: usize, conn: usize, cpus: usize) {
    let size_x = size_x as u32;
    let y_count = 3500 / conn;
    let cpu = 0;
    let conn_del_cpus = conn / cpus;

    let start = Instant::now();
    // min, max, sum, count
    let mut time_explore: (u64, u64, u64, u64) = (0, 0, 0, 0);

    let fx_builder: FxHashMapBuilder = FxHashMapBuilder::default();

    let mut errors: FxHashMap<u16, u32> = FxHashMap::with_capacity_and_hasher(100, fx_builder);

    for i in (cpu * conn_del_cpus)..((cpu * conn_del_cpus) + conn_del_cpus) {
        let start_y = i * y_count;

        let start = Instant::now();

        let mut y_iter = 0;

        println!("i: {}, start_y: {}, y_iter: {}, y_count: {}", i, start_y, y_iter, y_count);
        while y_iter < y_count {
            let pos_y = (start_y + y_iter) as u32;
            let mut pos_x = 0;

            // println!("pos_y: {}, pos_x: {}", pos_y, pos_x);

            while pos_x < 3500 {
                let area: Area = Area {
                    pos_x,
                    pos_y,
                    size_x,
                    size_y: 1,
                };

                // println!("req start, pos_x: {}", pos_x);

                pos_x += size_x;

                let client = hyper::Client::new();

                let res = explore2(&client, area).await;

                let duration = start.elapsed();
                let millis = duration.as_millis() as u64;
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
                        // println!("{:?}", report);
                    }
                    Err(code) => {
                        // println!("req end with code: {}", code);
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
    println!("min: {}, max: {}, sum: {}, count: {}, avg: {}", time_explore.0, time_explore.1, time_explore.2, time_explore.3, (time_explore.2 / time_explore.3));
    for (error, count) in errors {
        println!("erorr: {}, count: {}", error, count);
    }
}

// #[actix_rt::main]
// -> std::io::Result<()>
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // let system = System::new();
    // env_logger::Builder::from_env(Env::default().default_filter_or("info"))
    //     .format(|buf, record| { writeln!(buf, "{}", re) })
    //     .init();
    // info!("main start!");
    // let time: DateTime<Utc> = Utc::now();
    println!("main start");

    let store = Store {
        cash: 0,
        free_licences: 0,
        licences: Vec::with_capacity(100),
        treasures: Vec::with_capacity(100),
        // errors: FxHashMap::with_capacity_and_hasher(10, fx_builder),
        // time_explore: (0, 0, 0, 0)
    };

    // let bitfields = bitarr![Msb0, u8; 0; 512];

    /*
    сначала делаем explore с координатами x,y (какими?)
    потом копаем?
    нужно ввести некий показатель ценности этажа (соотношение ценности кладов к стоимости их добычи)
    необходимо так же записывать время на api запрос на каждом этаже: минимальное время, максимальное время, среднее арифметическое время
    */

    // let v5_2 = (5, 2); // 10
    let v5_4: (usize, usize) = (5, 4); // 20
    let v5_20: (usize, usize) = (5, 20); // 100
    let v5_200: (usize, usize) = (5, 100); // 500
    let v5_2000: (usize, usize) = (5, 700); // 3_500

    // let v10_2 = (10, 2); // 20
    let v10_4: (usize, usize) = (10, 4); // 40
    let v10_20: (usize, usize) = (10, 20); // 200
    let v10_200: (usize, usize) = (10, 100); // 1000
    let v10_2000: (usize, usize) = (10, 700); // 7k

    // let v50_2 = (50, 2); // 100
    let v50_4: (usize, usize) = (50, 4); // 200
    let v50_20: (usize, usize) = (50, 20); // 1000
    let v50_200: (usize, usize) = (50, 100); // 5k
    let v50_2000: (usize, usize) = (50, 700); // 35k

    // let v100_2 = (100, 2); // 200
    let v100_4: (usize, usize) = (100, 4); // 400
    let v100_20: (usize, usize) = (100, 20); // 2000
    let v100_200: (usize, usize) = (100, 100); // 10k
    let v100_2000: (usize, usize) = (100, 700); // 70k

    // let v500_2 = (500, 2); // 1000
    let v500_4: (usize, usize) = (500, 4); // 2000
    let v500_20: (usize, usize) = (500, 20); // 10k
    let v500_200: (usize, usize) = (500, 100); // 50k
    let v500_2000: (usize, usize) = (500, 700); // 350k

    let cpus = num_cpus::get();
    println!("cpu count: {}", cpus);
    let cpus = if cpus > 4 { 4 } else { cpus };

    let (size_x, conn) = v500_20;

    run_explore(size_x, conn, cpus).await;

    Ok(())
    // system.run()
}
