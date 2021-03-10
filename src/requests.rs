use crate::models::{Balance, LicenseList, Wallet, License, Area, Report, Dig, TreasureList, Treasure};
// use std::error::Error;
// use crate::Client;
use awc::{FrozenClientRequest, Client};
use actix_http::ResponseError;
use hyper::{body::{to_bytes}, Client as HyperClient, Uri, Request, Method, Body};
use hyper::client::HttpConnector;
use serde::{Serialize};
// use std::borrow::Borrow;
use serde::de::DeserializeOwned;
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

lazy_static! {
    static ref URL1: String = build_url("health-check");
    static ref URL11: Uri = URL1.parse::<Uri>().unwrap();
}
// /health-check
// request 0
// Returns 200 if service works okay.
pub const HEALTH_CHECK: u16 = 0;
pub fn build_health_check(client: &Client) -> FrozenClientRequest {
    client.get(&*URL1).freeze().unwrap()
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
// pub fn build_health_check(client: &Client) -> FrozenClientRequest {
//     client.get(&*URL1).freeze().unwrap()
// }
pub async fn health_check2(client: &HyperClient<HttpConnector>) -> u16 {
    let uri: Uri = URL1.parse::<Uri>().unwrap();
    match client.get(uri).await {
        Ok(res) => {
            res.status().as_u16()
        }
        Err(_) => {
            1000
        }
    }
}

// /balance
// request 1
// Returns a current balance.
pub const BALANCE: u16 = 1;
lazy_static! {
    static ref URL2: String = build_url("balance");
    static ref URL22: Uri = URL2.parse::<Uri>().unwrap();
}
pub fn build_balance(client: &Client) -> FrozenClientRequest {
    client.get(&*URL2).freeze().unwrap()
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
pub async fn balance2(req: &HyperClient<HttpConnector>) -> Result<Balance, u16> {
    match req.get((&*URL22).clone()).await {
        Ok(res) => {
            let body = res.into_body();
            let body = to_bytes(body).await.map_err(|_| 1001u16)?;
            let balance: Balance = serde_json::from_slice(&body).map_err(|_| 1002u16)?;
            Ok(balance)
        }
        Err(_) => {
            Err(1000u16)
        }
    }
}

// /licenses
// request 2
// Returns a list of issued licenses.
pub const GET_LICENSES: u16 = 2;
lazy_static! {
    static ref URL3: String = build_url("licenses");
    static ref URL33: Uri = URL3.parse::<Uri>().unwrap();
}
pub fn build_licenses(client: &Client) -> FrozenClientRequest {
    client.get(&*URL3).freeze().unwrap()
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
pub async fn licenses2(req: &HyperClient<HttpConnector>) -> Result<LicenseList, u16> {
    match req.get((&*URL33).clone()).await {
        Ok(res) => {
            let body = res.into_body();
            let body = to_bytes(body).await.map_err(|_| 1001u16)?;
            let license: LicenseList = serde_json::from_slice(&body).map_err(|_| 1002u16)?;
            Ok(license)
        }
        Err(_) => {
            Err(1000)
        }
    }
}

async fn make_post<Req, Res>(req: &HyperClient<HttpConnector>, body: &Req, uri: &Uri) -> Result<Res, u16>
    where Req: Serialize,
    Res: DeserializeOwned {
    let json = serde_json::to_string(body).map_err(|_| 1003u16)?;
    let request = Request::builder()
        .method(Method::POST)
        .uri(uri.clone())
        .header("content-type", "application/json")
        .body(Body::from(json))
        .map_err(|_| 1004u16)?;
    let res = req.request(request).await.map_err(|_| 1000u16)?;
    let body = res.into_body();
    let bytes = to_bytes(body).await.map_err(|_| 1001u16)?;
    let r: &[u8] = bytes.as_ref();
    let parsed: Res = serde_json::from_slice(r).map_err(|_| 1002u16)?;
    Ok(parsed)
}

// post /licenses
// request 3
// пустой массив для получения бесплатной лицензии
// errors: 409.1002: no more active licenses allowed
pub const PULL_LICENSES: u16 = 3;
pub fn build_pull_licenses(client: &Client) -> FrozenClientRequest {
    client.post(&*URL3).freeze().unwrap()
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
pub async fn pull_licenses2(req: &HyperClient<HttpConnector>, body: Wallet) -> Result<License, u16> {
    let json = serde_json::to_string(&body).map_err(|_| 1003u16)?;
    let request = Request::builder()
        .method(Method::POST)
        .uri((&*URL33).clone())
        .header("content-type", "application/json")
        .body(Body::from(json))
        .map_err(|_| 1004u16)?;
    match req.request(request).await {
        Ok(res) => {
            let body = res.into_body();
            let body = to_bytes(body).await.map_err(|_| 1001u16)?;
            let license: License = serde_json::from_slice(&body).map_err(|_| 1002u16)?;
            Ok(license)
        }
        Err(_) => {
            Err(1000)
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
    static ref URL55: Uri = URL5.parse::<Uri>().unwrap();
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
pub async fn explore2(req: &HyperClient<HttpConnector>, body: Area) -> Result<Report, u16> {
    make_post(req, &body, &*URL55).await
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
    static ref URL66: Uri = URL6.parse::<Uri>().unwrap();
}
pub fn build_dig(client: &Client) -> FrozenClientRequest {
    client.post(&*URL6).freeze().unwrap()
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
pub async fn dig2(req: &HyperClient<HttpConnector>, body: Dig) -> Result<TreasureList, u16> {
    make_post(req, &body, &*URL66).await
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
    static ref URL77: Uri = URL7.parse::<Uri>().unwrap();
}
pub fn build_cash(client: &Client) -> FrozenClientRequest {
    client.post(&*URL7).freeze().unwrap()
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
pub async fn cash2(req: &HyperClient<HttpConnector>, body: Treasure) -> Result<Wallet, u16> {
    make_post(req, &body, &*URL77).await
}
