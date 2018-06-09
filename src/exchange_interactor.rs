use chrono::prelude::*;
use chrono::{Utc, Date};

#[derive(Debug)]
pub enum Coin {
    Bitcoin,
    BitcoinCash,
    Ether,
    Litecoin,
    Doge,
}

#[derive(Debug)]
pub enum Exchange {
    GDAX,
}

pub struct ExchangeInteractor {
    pub host: String,
}

impl ExchangeInteractor {
    pub fn fetch_history(&self, coin: Coin, start: Date<Utc>, end: Date<Utc>) {
        println!("{:?}", coin);
    }
}
