use crate::models::{Balance, LicenseList, Wallet, License, Area, Report, Dig, TreasureList, Treasure};

/*
First number is HTTP Status code, second is value of "code" field in returned JSON object, text description may or may not match "message" field in returned JSON object.

errors:
422.1000: wrong coordinates
422.1001: wrong depth
409.1002: no more active licenses allowed
409.1003: treasure is not digged
*/

// /health-check
pub fn health_check() {}

// /balance
pub fn balance() -> Balance {}

// /licenses
pub fn licenses() -> LicenseList {}

// post /licenses
// пустой массив для получения бесплатной лицензии
// errors: 409.1002: no more active licenses allowed
pub fn licenses_set(body: Wallet) -> License {}

// post /explore
// Returns amount of treasures in the provided area at full depth.
// args: Area to be explored
// return: Report about found treasures.
// errors: 422.1000: wrong coordinates
pub fn explore(body: Area) -> Report {}

// post /dig
// Dig at given point and depth, returns found treasures.
// args: License, place and depth to dig.
// return: List of treasures found.
// errors: 422.1000: wrong coordinates
// 422.1001: wrong depth
pub fn dig(body: Dig) -> TreasureList {}

// post /cash
// Exchange provided treasure for money.
// args: Treasure for exchange.
// return: Payment for treasure.
// errors: 409.1003: treasure is not digged
pub fn cash(body: Treasure) -> Wallet {}
