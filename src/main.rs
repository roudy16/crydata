extern crate chrono;
extern crate crydataget;

use chrono::{Utc, DateTime, TimeZone};
use crydataget::exchange_interactor::{Coin, ExchangeInteractor};
use crydataget::storage_utils::*;

use std::io;
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let start_str = format!("{}T00:00:00Z", &args[1]);
    let end_str = format!("{}T00:00:00Z", &args[2]);

    println!("start: {}, end: {}", start_str, end_str);

    let start: DateTime<Utc> = start_str.parse::<DateTime<Utc>>().unwrap();
    let end: DateTime<Utc> = end_str.parse::<DateTime<Utc>>().unwrap();

    let dir = Path::new("./data_dump/");

    let coins_to_collect = [
        (Coin::Bitcoin, "BTC-USD"),
        (Coin::BitcoinCash, "BHC-USD"),
        (Coin::Ether, "ETH-USD"),
        (Coin::Litecoin, "LTC-USD"),
    ];

    for coin_prod_pair in &coins_to_collect {
        collect_history_to_dir(&coin_prod_pair.0, &coin_prod_pair.1, &start, &end, &dir);
    }
}
