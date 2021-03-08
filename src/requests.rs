use crate::models::{Balance, LicenseList, Wallet, License, Area, Report, Dig, TreasureList, Treasure};
use std::error::Error;
use crate::WebClient;
use awc::{FrozenClientRequest};

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
// const HOST: &str = &std::env::var("ADDRESS").unwrap();
// const HOST: &'static str = env!("ADDRESS");
fn build_url(path: &str) -> String {
    format!("http://{}:8000/{}", &*HOST, path)
}

lazy_static! {
    static ref URL1: String = build_url("health-check");
}
// /health-check
// Returns 200 if service works okay.
// const URL1: &str = &build_url("health-check");
pub fn build_health_check(client: &WebClient) -> FrozenClientRequest {
    client.client.get(&*URL1).freeze().unwrap()
}
pub async fn health_check(req: &FrozenClientRequest) -> Result<u16, Box<dyn Error>> {
    let status = req.send().await?.status().as_u16();
    Ok(status)
}

// /balance
// Returns a current balance.
// const URL2: &str = &build_url("balance");
lazy_static! {
    static ref URL2: String = build_url("balance");
}
pub fn build_balance(client: &WebClient) -> FrozenClientRequest {
    client.client.get(&*URL2).freeze().unwrap()
}
pub async fn balance(req: &FrozenClientRequest) -> Result<Balance, Box<dyn Error>> {
    let balance: Balance = req.send().await?.json().await?;
    // let balance: Balance = res.deserialize()?;
    Ok(balance)
}

// /licenses
// Returns a list of issued licenses.
// const URL3: &str = &build_url("licenses");
lazy_static! {
    static ref URL3: String = build_url("licenses");
}
pub fn build_licenses(client: &WebClient) -> FrozenClientRequest {
    client.client.get(&*URL3).freeze().unwrap()
}
pub async fn licenses(req: &FrozenClientRequest) -> Result<LicenseList, Box<dyn Error>> {
    let licenses: LicenseList = req.send().await?.json().await?;
    Ok(licenses)
}

// post /licenses
// пустой массив для получения бесплатной лицензии
// errors: 409.1002: no more active licenses allowed
pub fn build_licenses_set(client: &WebClient) -> FrozenClientRequest {
    client.client.post(&*URL3).freeze().unwrap()
}
pub async fn licenses_set(req: &FrozenClientRequest, body: Wallet) -> Result<License, Box<dyn Error>> {
    let license: License = req.send_json(&body).await?.json().await?;
    Ok(license)
}

// post /explore
// Returns amount of treasures in the provided area at full depth.
// args: Area to be explored
// return: Report about found treasures.
// errors: 422.1000: wrong coordinates
// const URL5: &str = &build_url("explore");
lazy_static! {
    static ref URL5: String = build_url("explore");
}
pub fn build_explore(client: &WebClient) -> FrozenClientRequest {
    client.client.post(&*URL5).freeze().unwrap()
}
pub async fn explore(req: &FrozenClientRequest, body: Area) -> Result<Report, Box<dyn Error>> {
    let report: Report = req.send_json(&body).await?.json().await?;
    Ok(report)
}

// post /dig
// Dig at given point and depth, returns found treasures.
// args: License, place and depth to dig.
// return: List of treasures found.
// errors: 422.1000: wrong coordinates
// 422.1001: wrong depth
// const URL6: &str = &build_url("dig");
lazy_static! {
    static ref URL6: String = build_url("dig");
}
pub fn build_dig(client: &WebClient) -> FrozenClientRequest {
    client.client.post(&*URL6).freeze().unwrap()
}
pub async fn dig(req: &FrozenClientRequest, body: Dig) -> Result<TreasureList, Box<dyn Error>> {
    let list: TreasureList = req.send_json(&body).await?.json().await?;
    Ok(list)
}

// post /cash
// Exchange provided treasure for money.
// args: Treasure for exchange.
// return: Payment for treasure.
// errors: 409.1003: treasure is not digged
// const URL7: &str = &build_url("cash");
lazy_static! {
    static ref URL7: String = build_url("cash");
}
pub fn build_cash(client: &WebClient) -> FrozenClientRequest {
    client.client.post(&*URL7).freeze().unwrap()
}
pub async fn cash(req: &FrozenClientRequest, body: Treasure) -> Result<Wallet, Box<dyn Error>> {
    let wallet: Wallet = req.send_json(&body).await?.json().await?;
    Ok(wallet)
}
