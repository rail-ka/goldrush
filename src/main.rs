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

// use actix::{dev::StreamHandler, Actor};
// use actix_web::{
//     middleware::Logger,
//     web::{get, post, put, Json},
//     {web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder},
// };
use actix_web::client::{Client, ClientBuilder, Connector};
use env_logger::Env;
use rustc_hash::FxHashMap;
use std::{cell::RefCell, rc::Rc};
use std::io::{stdout, Write};
use crate::models::*;
// use bincode::Options;
use std::time::Duration;
use crate::requests::{build_health_check, build_balance, build_licenses, build_licenses_set, build_explore, build_dig, build_cash};

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//     env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
//     info!("main start!");
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//
//     let client = Client::new();
//
//     let uri = "http://httpbin.org/ip".parse()?;
//
//     let mut resp = client.get(uri).await?;
//
//     println!("Response: {}", resp.status());
//
//     while let Some(chunk) = resp.body_mut().data().await {
//         stdout().write_all(&chunk?).await?;
//     }
//
//     Ok(())
// }

pub struct WebClient {
    pub client: Client
}

pub struct Store {
    pub cash: u32,
    pub free_licences: u32,
    pub licences: LicenseList,
    pub treasures: TreasureList,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("main start!");

    let mut cb = ClientBuilder::new();
    let mut connector = Connector::new()
        .conn_keep_alive(Duration::from_secs(70))
        .conn_lifetime(Duration::from_secs((60*9)+58))
        .finish(); // TODO: нужно не здать ответа слишком долго, так как намеренно будут тянуть с ответом

    let mut client = cb
        .disable_redirects()
        .connector(connector)
        .finish();

    let mut web_client = WebClient { client };

    let builder_health_check = build_health_check(&web_client);
    let builder_balance = build_balance(&web_client);
    let builder_licenses = build_licenses(&web_client);
    let builder_licenses_set = build_licenses_set(&web_client);
    let builder_explore = build_explore(&web_client);
    let builder_dig = build_dig(&web_client);
    let builder_cash = build_cash(&web_client);


    /*
    сначала делаем explore с координатами x,y (какими?)

    */

    Ok(())
}
