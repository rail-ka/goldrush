// use serde::{Serialize, Deserialize};
// use actix::prelude::*;
use std::{
    // cell::RefMut,
    // slice::Iter,
    string::String,
    vec::Vec,
    // fmt::{Display, Formatter},
    // collections::BTreeMap,
};
use std::cmp::Ordering;
use crossbeam::queue::ArrayQueue;
use std::sync::Arc;
use crate::requests::pull_licenses3;
use reqwest::Client;
use rustc_hash::FxHashMap;
use crate::{ErrMapArc, BalanceArc, err_add};
// use num_traits::{FromPrimitive};

// #[derive(PartialEq, Clone, Copy, Serialize, Deserialize, Debug)]
// #[derive(FromPrimitive)]
// pub enum FieldType {
//     // конкретный ответ из конкретных значений, классика
//     Select,
//     // несколько ответов из конкретных значений
//     Several,
//     // диапазон значений (с шагом), например от 1000 до 2000 (с шагом в 500)
//     Range,
//     // указывается некоторая величина (с двумя цифрами после запятой или целое число): 35, 192.22, 100000
//     Number,
//     // указывается конкретное строковое значение
//     // email, phone, regex
//     Text,
//     // указывается конкретное время (в зависимости от точности: секунды, минуты, часы, дни, недели, месяцы, годы)
//     // например: в 12 часов 31 декабря 2017 года
//     Date,
//     Time,
//     // указывается время в количестве (в зависимости от точности: секунды, минуты, часы, дни, недели, месяцы, годы)
//     // например: двое суток и 6 часов = 54 часа
//     TimeCount,
//     // указывается аккаунт в приложении
//     Users,
//     // Geolocation
// }
//
// #[derive(PartialEq, Clone, Default, Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct SelectField {
//     pub options: Vec<String>,
//     pub use_custom_option: bool,
//     pub required: bool,
//     pub default_value: Option<String>,
// }

#[derive(PartialEq, Clone, Default, Serialize, Deserialize, Debug)]
pub struct Error {
    pub code: u64,
    pub message: String,
}

#[derive(PartialEq, Clone, Default, Serialize, Deserialize, Debug)]
pub struct Balance {
    pub balance: u64,
    pub wallet: Wallet,
}

// max items 1000
// uniqueItems: true
pub type Wallet = Vec<u64>;

// min 0
pub type Amount = u64;

#[derive(PartialEq, Eq, Clone, Copy, Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub id: u64,
    pub dig_allowed: u64, // min 0
    pub dig_used: Amount,
}

impl License {
    pub fn dig_count(&self) -> u64 {
        self.dig_allowed - self.dig_used
    }
}

impl PartialOrd for License {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let s = self.dig_count();
        let o = other.dig_count();
        s.partial_cmp(&o)
    }
}

impl Ord for License {
    fn cmp(&self, other: &Self) -> Ordering {
        let s = self.dig_allowed - self.dig_used;
        let o = other.dig_allowed - other.dig_used;
        s.cmp(&o)
    }
}

pub type LicenseList = Vec<License>;

pub type LicenseSet = Arc<ArrayQueue<License>>;

pub fn push_back(set: &LicenseSet, license: License) {
    if let Err(l) = set.push(license) {
        eprintln!("license queue is full! {}", license.id);
    }
}

pub async fn pull(set: &LicenseSet, client: &Client, balance: &BalanceArc, errors: &ErrMapArc, sum: usize) -> License {
    let get_license = || async {
        loop {
            let mut balance = balance.lock().await;
            let wallet: Vec<u64> = {
                if balance.balance == 0 {
                    vec![]
                } else {
                    let mut coins: Vec<u64> = Vec::with_capacity(sum);
                    for _ in 0..sum {
                        let coin = balance.wallet.pop().unwrap_or_else(|| {
                            eprintln!("wallet empty but balance != 0");
                            0
                        });
                        coins.push(coin);
                        balance.balance -= 1;
                    }
                    coins
                }
            };
            match pull_licenses3(client, &wallet, false, false).await {
                Ok(license) => {
                    return license;
                }
                Err(e) => {
                    // TODO: if 402 get licenses
                    err_add(&errors, e).await; // TODO: можно не ждать, spawn current thread
                }
            }
        }
    };
    match set.pop() {
        Some(license) => {
            if license.dig_count() == 0 {
                // TODO: request
                let l: License = get_license().await;
                l
            } else {
                // ...
                license
            }
        }
        None => {
            // TODO: request
            let l: License = get_license().await;
            l
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
// #[derive(Message)]
// #[rtype(result = "usize")]
pub struct Area {
    pub pos_x: u64, // min 0
    pub pos_y: u64, // min 0
    pub size_x: u64, // min 1
    pub size_y: u64, // min 1
}

#[derive(PartialEq, Eq, Clone, Copy, Default, Serialize, Deserialize, Debug)]
pub struct Report {
    pub area: Area,
    pub amount: Amount,
}

impl PartialOrd for Report {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.amount.partial_cmp(&other.amount)
    }
}

impl Ord for Report {
    fn cmp(&self, other: &Self) -> Ordering {
        self.amount.cmp(&other.amount)
    }
}

#[derive(PartialEq, Clone, Copy, Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Dig {
    #[serde(rename = "licenseID")]
    pub license_id: u64,
    pub pos_x: u64, // min 0
    pub pos_y: u64, // min 0
    pub depth: u64, // min 1 max 100
}

pub type Treasure = String;

pub type TreasureList = Vec<Treasure>;
