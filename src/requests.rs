use crate::models::{Balance, LicenseList, Wallet, License, Area, Report, Dig, TreasureList, Treasure};
// use std::error::Error;
// use crate::Client;
// use awc::{FrozenClientRequest, Client};
// use actix_http::ResponseError;
use serde::{Serialize};
// use std::borrow::Borrow;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::time::Duration;
// use actix_web::ResponseError;

/*
First number is HTTP Status code, second is value of "code" field in returned JSON object, text description may or may not match "message" field in returned JSON object.

errors:
422.1000: wrong coordinates
422.1001: wrong depth
409.1002: no more active licenses allowed
409.1003: treasure is not digged
*/
lazy_static! {
    static ref HOST: String = std::env::var("ADDRESS").unwrap();
}
fn build_url(path: &str) -> String {
    format!("http://{}:8000/{}", &*HOST, path)
}

async fn make_request<Req, Res>(client: &reqwest::Client, body: &Req, uri: &str, is_log: bool, is_log_res: bool) -> Result<Res, u16>
    where Req: Serialize,
          Res: DeserializeOwned + Debug {

    let req = client.post(uri)
        .json(body)
        .timeout(Duration::from_millis(150))
        .build()
        .expect("request can't build");

    if is_log {
        if let Some(body) = req.body() {
            let body = body.as_bytes().unwrap();
            let body = String::from_utf8_lossy(body);
            println!("req path: {:?} body: {:?}", req.url().path(), body);
        }
    }

    let res = client.execute(req)
        // .send()
        .await
        .map_err(|e| e.status().map(|e| e.as_u16()).unwrap_or(1000))?;

    let status = res.status().as_u16();

    if status != 200 {
        if is_log_res {
            let body = res.bytes().await.map(|b| {
                String::from_utf8(b.clone().to_vec())
            });
            match body {
                Ok(body) => {
                    println!("res: {:?}", body);
                }
                Err(e) => {
                    println!("res e: {:?}", e);
                }
            }
        }
        return Err(status)
    }

    let body: Res = res.json().await.map_err(|e| e.status().map(|e| e.as_u16()).unwrap_or(1200))?;

    if is_log_res {
        println!("res: {:?}", body);
    }

    Ok(body)
}

lazy_static! {
    static ref URL1: String = build_url("health-check");
}
// /health-check
// request 0
// Returns 200 if service works okay.
pub const HEALTH_CHECK: u16 = 0;
// pub fn build_health_check(client: &Client) -> FrozenClientRequest {
//     client.get(&*URL1).freeze().unwrap()
// }
// pub async fn health_check(req: &FrozenClientRequest) -> u16 {
//     match req.send().await {
//         Ok(res) => {
//             res.status().as_u16()
//         }
//         Err(e) => {
//             e.status_code().as_u16()
//         }
//     }
// }
pub async fn health_check3(client: &reqwest::Client) -> u16 {
    match client
        .get(&*URL1)
        .send()
        .await {
        Ok(res) => {
            res.status().as_u16()
        }
        Err(e) => {
            match e.status() {
                Some (e) => e.as_u16(),
                None => 1000u16
            }
        }
    }
}

// /balance
// request 1
// Returns a current balance.
pub const BALANCE: u16 = 1;
lazy_static! {
    static ref URL2: String = build_url("balance");
}
// pub fn build_balance(client: &Client) -> FrozenClientRequest {
//     client.get(&*URL2).freeze().unwrap()
// }
// pub async fn balance(req: &FrozenClientRequest) -> Result<Balance, u16> {
//     match req.send().await {
//         Ok(mut res) => {
//             res.json().await.map_err(|e| {
//                 e.status_code().as_u16()
//             })
//         }
//         Err(e) => {
//             Err(e.status_code().as_u16())
//         }
//     }
// }
pub async fn balance3(client: &reqwest::Client) -> Result<Balance, u16> {
    let res = client
        .get(&*URL2)
        .send()
        .await
        .map_err(|_| 1000u16)?;

    let status = res.status().as_u16();

    if status != 200 {
        return Err(status)
    }

    let balance: Balance = res.json().await.map_err(|_| 1002u16)?;
    Ok(balance)
}

// /licenses
// request 2
// Returns a list of issued licenses.
pub const GET_LICENSES: u16 = 2;
lazy_static! {
    static ref URL3: String = build_url("licenses");
}
// pub fn build_licenses(client: &Client) -> FrozenClientRequest {
//     client.get(&*URL3).freeze().unwrap()
// }
// pub async fn licenses(req: &FrozenClientRequest) -> Result<LicenseList, u16> {
//     match req.send().await {
//         Ok(mut res) => {
//             res.json().await.map_err(|e| {
//                 e.status_code().as_u16()
//             })
//         }
//         Err(e) => {
//             Err(e.status_code().as_u16())
//         }
//     }
// }
pub async fn licenses3(client: &reqwest::Client) -> Result<LicenseList, u16> {
    let req = client
        .get(&*URL3)
        .timeout(Duration::from_millis(150))
        .header("content-type", "application/json")
        .build().unwrap();

    // if let Some(Ok(b)) = req.body().map(|b| b.as_bytes().map(|b| { String::from_utf8(b.to_vec()) })).flatten() {
    //     println!("req: {}", b);
    // }

    // for x in req.headers() {
    //     if let Ok(value) = String::from_utf8(x.1.as_bytes().to_vec()) {
    //         println!("req header: {} value: {}", x.0, value);
    //     }
    // }

    // let method = req.method().as_str();
    // let url = req.url().to_string();
    // let d = Duration::from_millis(1);
    // let t = req.timeout().unwrap_or(&d);
    // println!("req method: {} url: {} timeout: {}", method, url, t.as_millis());

    let res = client.execute(req)
        .await
        .map_err(|_| 1000u16)?;

    // for x in res.headers() {
    //     if let Ok(value) = String::from_utf8(x.1.as_bytes().to_vec()) {
    //         println!("res header: {} value: {}", x.0, value);
    //     }
    // }

    let status = res.status().as_u16();

    if status != 200 {
        // let body = res.bytes().await.map(|b| {
        //     String::from_utf8(b.clone().to_vec())
        // });

        // println!("{:?}", body);
        return Err(status)
    }

    let license: LicenseList = res.json().await.map_err(|_| 1002u16)?;
    Ok(license)
}

// post /licenses
// request 3
// пустой массив для получения бесплатной лицензии
// errors: 409.1002: no more active licenses allowed
pub const PULL_LICENSES: u16 = 3;
// pub fn build_pull_licenses(client: &Client) -> FrozenClientRequest {
//     client.post(&*URL3).freeze().unwrap()
// }
// pub async fn pull_licenses(req: &FrozenClientRequest, body: Wallet) -> Result<License, u16> {
//     match req.send_json(&body).await {
//         Ok(mut res) => {
//             res.json().await.map_err(|e| {
//                 e.status_code().as_u16()
//             })
//         }
//         Err(e) => {
//             Err(e.status_code().as_u16())
//         }
//     }
// }
pub async fn pull_licenses3(client: &reqwest::Client, body: &Wallet, is_log: bool, is_log_res: bool) -> Result<License, u16> {
    make_request(client, body, &URL3, is_log, is_log_res).await
}

// post /explore
// request 4
// Returns amount of treasures in the provided area at full depth.
// args: Area to be explored
// return: Report about found treasures.
// errors: 422.1000: wrong coordinates
pub const EXPLORE: u16 = 4;
lazy_static! {
    pub static ref URL5: String = build_url("explore");
}
// pub fn build_explore(client: Client) -> FrozenClientRequest {
//     client.post(&*URL5).freeze().unwrap()
// }
// pub async fn explore(req: &FrozenClientRequest, body: Area) -> Result<Report, u16> {
//     match req.send_json(&body).await {
//         Ok(mut res) => {
//             res.json().await.map_err(|e| {
//                 e.status_code().as_u16()
//             })
//         }
//         Err(e) => {
//             Err(e.status_code().as_u16())
//         }
//     }
// }
pub async fn explore3(client: &reqwest::Client, body: &Area) -> Result<Report, u16> {
    make_request(client, body, &URL5, false, false).await
}

// post /dig
// request 5
// Dig at given point and depth, returns found treasures.
// args: License, place and depth to dig.
// return: List of treasures found.
// errors: 422.1000: wrong coordinates
// 422.1001: wrong depth
pub const DIG: u16 = 5;
lazy_static! {
    static ref URL6: String = build_url("dig");
}
// pub fn build_dig(client: &Client) -> FrozenClientRequest {
//     client.post(&*URL6).freeze().unwrap()
// }
// pub async fn dig(req: &FrozenClientRequest, body: Dig) -> Result<TreasureList, u16> {
//     match req.send_json(&body).await {
//         Ok(mut res) => {
//             res.json().await.map_err(|e| {
//                 e.status_code().as_u16()
//             })
//         }
//         Err(e) => {
//             Err(e.status_code().as_u16())
//         }
//     }
// }
pub async fn dig3(client: &reqwest::Client, body: &Dig) -> Result<TreasureList, u16> {
    make_request(client, body, &URL6, false, false).await
}

// post /cash
// request 6
// Exchange provided treasure for money.
// args: Treasure for exchange.
// return: Payment for treasure.
// errors: 409.1003: treasure is not digged
pub const CASH: u16 = 6;
lazy_static! {
    static ref URL7: String = build_url("cash");
}
// pub fn build_cash(client: &Client) -> FrozenClientRequest {
//     client.post(&*URL7).freeze().unwrap()
// }
// pub async fn cash(req: &FrozenClientRequest, body: Treasure) -> Result<Wallet, u16> {
//     match req.send_json(&body).await {
//         Ok(mut res) => {
//             res.json().await.map_err(|e| {
//                 e.status_code().as_u16()
//             })
//         }
//         Err(e) => {
//             Err(e.status_code().as_u16())
//         }
//     }
// }
pub async fn cash3(client: &reqwest::Client, body: &Treasure) -> Result<Wallet, u16> {
    make_request(client, body, &URL7, false, false).await
}
