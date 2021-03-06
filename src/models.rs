use serde::{Serialize, Deserialize};
use std::{
    cell::RefMut,
    slice::Iter,
    string::String,
    vec::Vec,
    fmt::{Display, Formatter},
    collections::BTreeMap,
};
use num_traits::{FromPrimitive};

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
    code: u32,
    message: String,
}

#[derive(PartialEq, Clone, Default, Serialize, Deserialize, Debug)]
pub struct Balance {
    balance: u32,
    wallet: Wallet,
}

// max items 1000
// uniqueItems: true
pub type Wallet = Vec<u32>;

// min 0
pub type Amount = u32;

#[derive(PartialEq, Clone, Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct License {
    id: u32,
    dig_allowed: u32, // min 0
    dig_used: Amount,
}

pub type LicenseList = Vec<License>;

#[derive(PartialEq, Clone, Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Area {
    pos_x: u32, // min 0
    pos_y: u32, // min 0
    size_x: u32, // min 1
    size_y: u32, // min 1
}

#[derive(PartialEq, Clone, Default, Serialize, Deserialize, Debug)]
pub struct Report {
    area: Area,
    amount: Amount,
}

#[derive(PartialEq, Clone, Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Dig {
    #[serde(rename = "licenseID")]
    license_id: u32,
    pos_x: u32, // min 0
    pos_y: u32, // min 0
    depth: u32, // min 1 max 100
}

pub type Treasure = String;

pub type TreasureList = Vec<Treasure>;