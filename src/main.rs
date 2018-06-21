extern crate chrono;
extern crate crydataget;

use chrono::{Utc, DateTime, TimeZone};
use crydataget::exchange_interactor::{Coin, ExchangeInteractor};
use crydataget::storage_utils::*;

use std::io;
use std::path::Path;

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

    let dir = Path::new("./data_dump/");
    let start = chrono::Utc.ymd(2017, 6, 10).and_hms(0, 0, 0);
    let end = chrono::Utc.ymd(2017, 6, 11).and_hms(12, 0, 0);

    collect_history_to_dir(Coin::Bitcoin, &start, &end, &dir);
}
