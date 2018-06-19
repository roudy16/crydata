use chrono::prelude::*;
use chrono::{Utc, DateTime};
use std::io::Write;
use std::fmt;

#[derive(Debug)]
pub enum Coin {
    Bitcoin,
    BitcoinCash,
    Ether,
    Litecoin,
}

#[derive(Debug)]
pub enum Exchange {
    GDAX,
}

pub struct ExchangeInteractor {
    pub host: String,
}

impl ExchangeInteractor {
    pub fn fetch_history(&self, coin: Coin, start: DateTime<Utc>, end: DateTime<Utc>) {
        println!("{:?}\n", coin);
    }
}

impl fmt::Display for Coin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            &Coin::Bitcoin => write!(f, "Bitcoin"),
            &Coin::BitcoinCash => write!(f, "BitcoinCash"),
            &Coin::Ether => write!(f, "Ether"),
            &Coin::Litecoin => write!(f, "Litecoin"),
        }
    }
}
