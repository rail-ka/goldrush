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

mod models;
mod requests;

use actix::{dev::StreamHandler, Actor};
use actix_web::{
    middleware::Logger,
    web::{get, post, put, Json},
    {web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder},
};
use actix_web_actors::ws;
use env_logger::Env;
use rustc_hash::FxHashMap;
use std::{cell::RefCell, rc::Rc};

fn main() {
    println!("Hello, world!");
}
