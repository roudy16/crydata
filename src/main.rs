extern crate chrono;
extern crate crydataget;

use chrono::{Utc, DateTime};
use crydataget::exchange_interactor::{Coin, ExchangeInteractor};
use crydataget::storage_utils::*;

use std::io;

fn main() {
    let e = ExchangeInteractor {
        host: String::from("http://"),
    };

    let today = Utc::now();

    e.fetch_history(Coin::Bitcoin, today, today);

    let boundaries = calc_month_boundary_dates(&today);
    println!("{:?}", boundaries);

    let file_name_test = make_history_file_name(Coin::Bitcoin, 2017, 4);
    println!("{}", file_name_test);
}
