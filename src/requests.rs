use crate::models::{Balance, LicenseList, Wallet, License, Area, Report, Dig, TreasureList, Treasure};
use std::error::Error;
use crate::WebClient;
use awc::{FrozenClientRequest, Client};
use actix_web::ResponseError;

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

lazy_static! {
    static ref URL1: String = build_url("health-check");
}
// /health-check
// request 0
// Returns 200 if service works okay.
pub const HEALTH_CHECK: u16 = 0;
pub fn build_health_check(client: &WebClient) -> FrozenClientRequest {
    client.client.get(&*URL1).freeze().unwrap()
}
pub async fn health_check(req: &FrozenClientRequest) -> u16 {
    match req.send().await {
        Ok(res) => {
            res.status().as_u16()
        }
        Err(e) => {
            e.status_code().as_u16()
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
pub fn build_balance(client: &WebClient) -> FrozenClientRequest {
    client.client.get(&*URL2).freeze().unwrap()
}
pub async fn balance(req: &FrozenClientRequest) -> Result<Balance, u16> {
    match req.send().await {
        Ok(mut res) => {
            res.json().await.map_err(|e| {
                e.status_code().as_u16()
            })
        }
        Err(e) => {
            Err(e.status_code().as_u16())
        }
    }
}

// /licenses
// request 2
// Returns a list of issued licenses.
pub const GET_LICENSES: u16 = 2;
lazy_static! {
    static ref URL3: String = build_url("licenses");
}
pub fn build_licenses(client: &WebClient) -> FrozenClientRequest {
    client.client.get(&*URL3).freeze().unwrap()
}
pub async fn licenses(req: &FrozenClientRequest) -> Result<LicenseList, u16> {
    match req.send().await {
        Ok(mut res) => {
            res.json().await.map_err(|e| {
                e.status_code().as_u16()
            })
        }
        Err(e) => {
            Err(e.status_code().as_u16())
        }
    }
}

// post /licenses
// request 3
// пустой массив для получения бесплатной лицензии
// errors: 409.1002: no more active licenses allowed
pub const PULL_LICENSES: u16 = 3;
pub fn build_pull_licenses(client: &WebClient) -> FrozenClientRequest {
    client.client.post(&*URL3).freeze().unwrap()
}
pub async fn pull_licenses(req: &FrozenClientRequest, body: Wallet) -> Result<License, u16> {
    match req.send_json(&body).await {
        Ok(mut res) => {
            res.json().await.map_err(|e| {
                e.status_code().as_u16()
            })
        }
        Err(e) => {
            Err(e.status_code().as_u16())
        }
    }
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
pub fn build_explore(client: Client) -> FrozenClientRequest {
    client.post(&*URL5).freeze().unwrap()
}
pub async fn explore(req: &FrozenClientRequest, body: Area) -> Result<Report, u16> {
    match req.send_json(&body).await {
        Ok(mut res) => {
            res.json().await.map_err(|e| {
                e.status_code().as_u16()
            })
        }
        Err(e) => {
            Err(e.status_code().as_u16())
        }
    }
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
pub fn build_dig(client: &WebClient) -> FrozenClientRequest {
    client.client.post(&*URL6).freeze().unwrap()
}
pub async fn dig(req: &FrozenClientRequest, body: Dig) -> Result<TreasureList, u16> {
    match req.send_json(&body).await {
        Ok(mut res) => {
            res.json().await.map_err(|e| {
                e.status_code().as_u16()
            })
        }
        Err(e) => {
            Err(e.status_code().as_u16())
        }
    }
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
pub fn build_cash(client: &WebClient) -> FrozenClientRequest {
    client.client.post(&*URL7).freeze().unwrap()
}
pub async fn cash(req: &FrozenClientRequest, body: Treasure) -> Result<Wallet, u16> {
    match req.send_json(&body).await {
        Ok(mut res) => {
            res.json().await.map_err(|e| {
                e.status_code().as_u16()
            })
        }
        Err(e) => {
            Err(e.status_code().as_u16())
        }
    }
}
