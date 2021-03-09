#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate serde_derive;
// #[macro_use]
extern crate serde_json;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
// use hyper::Client;

mod models;
mod requests;
mod explore;
mod floor;

// use actix::{dev::StreamHandler, Actor};
// use actix_web::{
//     middleware::Logger,
//     web::{get, post, put, Json},
//     {web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder},
// };
use actix_web::client::{Client, ClientBuilder, Connector};
use env_logger::Env;
use rustc_hash::{FxHashMap, FxHasher};
use std::{cell::RefCell, rc::Rc};
use std::io::{stdout, Write};
use crate::models::*;
// use bincode::Options;
use std::time::{Duration, Instant, SystemTime};
use crate::requests::*;
use std::collections::{HashMap, HashSet};
use std::hash::BuildHasherDefault;
use bitvec::prelude::*;
use chrono::{Utc, DateTime};
use crate::floor::Floor;
use actix::Actor;
use actix_rt::{Arbiter, System};

pub type FxHashMapBuilder = BuildHasherDefault<FxHasher>;

#[derive(Clone)]
pub struct WebClient {
    pub client: Client
}

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

#[derive(PartialEq, Clone, Default, Serialize, Deserialize, Debug)]
pub struct FloorStore {
    pub cash: u32,
    // min_cash, max_cash, avg_cash, count
    pub treasures_metrics: TreasuresMetrics,
    // TODO: нужно записывать параметры полученных лицензий для того чтобы определить лучшие параметры для запроса на получение лицензий
    pub licences_metrics: LicencesMetrics,
    // key - (error_code, request_id), value - count
    pub errors: FxHashMap<(u16, u16), u32>,
    // min, max, sum, count
    pub time_explore: (u64, u64, u64, u64),
    // min, max, sum, count
    pub time_pull_licences: (u64, u64, u64, u64),
}

const SIDE_SIZE: usize = 3500;
// 12_250_000
const POINT_COUNT: usize = SIDE_SIZE * SIDE_SIZE;
// const ONE_ITER_COUNT: usize = 490;
// 25_000 (8*3125)
// const ITERATIONS: usize = POINT_COUNT / ONE_ITER_COUNT;
const FLOOR_COUNT: usize = 10;

const VARIANTS: [usize; 5] = [5, 10, 50, 100, 500];

pub type BitFields = BitArr!(for 512, in Msb0, u8);

fn create_client() -> Client {
    let cb = ClientBuilder::new();
    let connector = Connector::new()
        .conn_keep_alive(Duration::from_secs(70))
        // TODO: нужно не ждать ответа слишком долго, так как намеренно будут тянуть с ответом
        // .conn_lifetime(Duration::from_secs((60*9)+58))
        .conn_lifetime(Duration::from_secs(5))
        .finish();

    let client = cb
        .disable_redirects()
        .connector(connector)
        .finish();
    client
}

// #[actix_rt::main]
fn main() -> std::io::Result<()> {
    let system = System::new();
    // env_logger::Builder::from_env(Env::default().default_filter_or("info"))
    //     .format(|buf, record| { writeln!(buf, "{}", re) })
    //     .init();
    // info!("main start!");
    let time: DateTime<Utc> = Utc::now();
    println!("main start at: {}", time);

    let fx_builder: FxHashMapBuilder = FxHashMapBuilder::default();

    let mut store = Store {
        cash: 0,
        free_licences: 0,
        licences: Vec::with_capacity(100),
        treasures: Vec::with_capacity(100),
        // errors: FxHashMap::with_capacity_and_hasher(10, fx_builder),
        // time_explore: (0, 0, 0, 0)
    };

    // let mut interval = tokio::time::interval(Duration::from_secs(60));
    //
    // for i in 0..FLOOR_COUNT {
    //     interval.tick().await;
    //     let floor_addr = Floor { floor: i }.start();
    //
    // }

    // let builder_health_check = build_health_check(&web_client);
    // let builder_balance = build_balance(&web_client);
    // let builder_licenses = build_licenses(&web_client);
    // let builder_licenses_set = build_pull_licenses(&web_client);
    // let builder_explore = build_explore(&web_client);
    // let builder_dig = build_dig(&web_client);
    // let builder_cash = build_cash(&web_client);

    /*
    сначала делаем explore с координатами x,y (какими?)
    потом копаем?
    нужно ввести некий показатель ценности этажа (соотношение ценности кладов к стоимости их добычи)
    необходимо так же записывать время на api запрос на каждом этаже: минимальное время, максимальное время, среднее арифметическое время
    */

    // let connection_count: [usize; 4] = [2, 20, 200, 2000];

    // let v5_2 = (5, 2); // 10
    let v5_4 = (5, 4); // 20
    let v5_20 = (5, 20); // 100
    let v5_200 = (5, 100); // 500
    let v5_2000 = (5, 700); // 3_500

    // let v10_2 = (10, 2); // 20
    let v10_4 = (10, 4); // 40
    let v10_20 = (10, 20); // 200
    let v10_200 = (10, 100); // 1000
    let v10_2000 = (10, 700); // 7k

    // let v50_2 = (50, 2); // 100
    let v50_4 = (50, 4); // 200
    let v50_20 = (50, 20); // 1000
    let v50_200 = (50, 100); // 5k
    let v50_2000 = (50, 700); // 35k

    // let v100_2 = (100, 2); // 200
    let v100_4 = (100, 4); // 400
    let v100_20 = (100, 20); // 2000
    let v100_200 = (100, 100); // 10k
    let v100_2000 = (100, 700); // 70k

    // let v500_2 = (500, 2); // 1000
    let v500_4 = (500, 4); // 2000
    let v500_20 = (500, 20); // 10k
    let v500_200 = (500, 100); // 50k
    let v500_2000 = (500, 700); // 350k

    let cpus = num_cpus::get();
    println!("cpu count: {}", cpus);
    let cpus = if cpus > 4 { 4 } else { cpus };

    let (size_x, conn) = v500_20;

    for cpu in 0..cpus { // 4 cpu
        let arbiter = Arbiter::new();
        let conn_del_cpus = conn / cpus; // 5
        let execution = async move {
            let client = create_client();
            // let web_client = WebClient { client };
            let builder_explore = build_explore(client);

            // let explorer_actor = explore::Explorer {
            //     builder_explore
            // }.start();

            let y_count = 3500 / conn;

            for i in (cpu * conn_del_cpus)..((cpu * conn_del_cpus) + conn_del_cpus) {
                // 0..4, 5..9, 10..14, 15..19

                let start_y = i * y_count;
                // println!("{} {} {}", i, y_count, start_y);
                // let mut stop_y = (i+1) * y_count;

                let start = Instant::now();

                let mut y_iter = 0;

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
                        // if pos_y > 3400 { println!("{}, {}", cpu, pos_y); }
                        pos_x += size_x;

                        // explorer_actor.send(area);

                        // builder_explore.send_json(&area).await;

                        // let res = explore(&builder_explore, area).await;
                        // println!("{:?}", res);
                    }
                    y_iter += 1;
                }
            }
        };
        arbiter.spawn(execution);
    }

    {


        // for i in 0..conn {
        //     let arbiter = arbiters.get(arbiter_index).unwrap();
        //     if (arbiter_index + 1) == cpus {
        //         arbiter_index = 0;
        //     } else {
        //         arbiter_index += 1;
        //     }
        //
        //     let mut start_y = 0;
        //     let mut stop_y = 3500;
        //
        //     if conn == 2 {
        //         start_y = i * y_count; // 1750
        //         stop_y = (i+1) * y_count;
        //     } else if conn == 20 {
        //         start_y = i * y_count; // 175
        //         stop_y = (i+1) * y_count;
        //     } else if conn == 100 {
        //         start_y = i * y_count; // 35
        //         stop_y = (i+1) * y_count;
        //     } else { // 700
        //         start_y = i * y_count; // 5
        //         stop_y = (i+1) * y_count;
        //     }
        //
        //     let execution = async move {
        //         let start_y = start_y.clone();
        //         let y_count = y_count.clone();
        //         let size_x = size_x.clone();
        //
        //
        //         let start = Instant::now();
        //
        //         let mut pos_y = start_y;
        //
        //         let mut y_iter = 0;
        //
        //         while y_iter < y_count {
        //             let mut pos_x = 0;
        //             while pos_x < 3500 {
        //                 let area: Area = Area {
        //                     pos_x,
        //                     pos_y,
        //                     size_x,
        //                     size_y: 1,
        //                 };
        //                 // println!("{}, {}, {}", pos_x, pos_y, size_x);
        //                 pos_x += size_x;
        //
        //                 let res = explore(&builder_explore.clone(), area).await;
        //                 println!("{:?}", res);
        //             }
        //             y_iter += 1;
        //         }
        //     };
        //
        //     arbiter.spawn(execution);
        // }
    }

    // let mut y = 0;
    // while y < 7 {
    //     let mut x = 0;
    //     let start_y = y * size * iter_count;
    //
    //     while x < 7 {
    //         let start_x = x * size * iter_count;
    //         // 49 вариантов
    //
    //         x += 1;
    //     }
    //     y += 1;
    // }

    // 1 variant, 5 size
    // {
    //     let size: u32 = 5;
    //     let iter_count = 100;
    //
    //     // variant for 2 connections
    //     {
    //     }
    //
    //     // variant for 20 connections
    //     {
    //         let start = Instant::now();
    //         let pos_x: u32 = 0;
    //         let pos_y: u32 = 0;
    //         let area: Area = Area {
    //             pos_x,
    //             pos_y,
    //             size_x: size,
    //             size_y: size,
    //         };
    //         for i in 0..20 {
    //
    //         }
    //     }
    //
    //     // variant for 200 connections
    //     {
    //         let start = Instant::now();
    //         let pos_x: u32 = 0;
    //         let pos_y: u32 = 0;
    //         let area: Area = Area {
    //             pos_x,
    //             pos_y,
    //             size_x: size,
    //             size_y: size,
    //         };
    //         for i in 0..200 {
    //
    //         }
    //     }
    //
    //     // variant for 2000 connections
    //     {
    //         let start = Instant::now();
    //         let pos_x: u32 = 0;
    //         let pos_y: u32 = 0;
    //         let area: Area = Area {
    //             pos_x,
    //             pos_y,
    //             size_x: size,
    //             size_y: size,
    //         };
    //         for i in 0..2000 {
    //
    //         }
    //     }
    //
    // }

    // 2 variant, 10 size
    {
        // let pos_x: u32 = 500;
        // let pos_y: u32 = 500;
        // let size: u32 = 10;
        // let iter_count = 50;
    }

    // 3 variant, 50 size
    {
        // let pos_x: u32 = 1000;
        // let pos_y: u32 = 1000;
        // let size: u32 = 50;
        // let iter_count = 10;
    }

    // 4 variant, 100 size
    {
        // let pos_x: u32 = 1500;
        // let pos_y: u32 = 1500;
        // let size: u32 = 100;
        // let iter_count = 5;
    }

    // 5 variant, 500 size
    {
        // let pos_x: u32 = 2000;
        // let pos_y: u32 = 2000;
        // let size: u32 = 500;
        // let iter_count = 1;
    }

    // for floor in 0..FLOOR_COUNT {
    //     let mut floor_store = FloorStore {
    //         cash: 0,
    //         treasures_metrics: TreasuresMetrics::default(),
    //         licences_metrics: LicencesMetrics::default(),
    //         errors: FxHashMap::with_capacity_and_hasher(10, fx_builder.clone()),
    //         time_explore: (0, 0, 0, 0),
    //         time_pull_licences: (0, 0, 0, 0),
    //     };
    //     let mut pos_x: u32 = 0;
    //     let mut pos_y: u32 = 0;
    //     let mut n: usize = 0; // текущий номер обхода
    //     let mut size: u32 = 22;
    //
    //     let bitfields = bitarr![Msb0, u8; 0; 512];
    //
    //     if (size * n) <= 3500 {
    //
    //     } else {
    //         let last_size = 3500 - (size * n);
    //
    //     }
    //
    //     let start = Instant::now();
    //     let area: Area = Area {
    //         pos_x,
    //         pos_y,
    //         size_x: size,
    //         size_y: size,
    //     };
    //
    //     let res = explore(&builder_explore, area).await;
    //
    //     let duration = start.elapsed();
    //     let millis = duration.as_millis() as u64;
    //     let min = &mut floor_store.time_explore.0;
    //     let max = &mut floor_store.time_explore.1;
    //     let sum = &mut floor_store.time_explore.2;
    //     let count = &mut floor_store.time_explore.3;
    //     *count = *count + 1;
    //     *sum = *sum + millis;
    //     if millis > *max {
    //         *max = millis;
    //     }
    //     if millis < *min {
    //         *min = millis;
    //     }
    //
    //     match res {
    //         Ok(res) => {
    //             let amount = res.amount;
    //             // &mut floor_store.treasures
    //         }
    //         Err(e) => {
    //             match floor_store.errors.get_mut(&(e, EXPLORE)) {
    //                 Some(count) => {
    //                     *count += 1;
    //                 }
    //                 None => {
    //                     floor_store.errors.insert((e, EXPLORE), 1);
    //                 }
    //             }
    //
    //         }
    //     }
    // }

    // Ok(())
    system.run()
}
