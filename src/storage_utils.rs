use chrono::prelude::*;
use chrono::{Duration, Datelike};

use std::error::Error;
use std::path::Path;
use std::io;
use std::thread;
use exchange_interactor::Coin;

use gdax_client::{PublicClient};
use gdax_client::public::{Candle};

pub fn calc_month_boundary_dates(&dt: &DateTime<Utc>) -> (DateTime<Utc>, DateTime<Utc>) {
    let cur_year = dt.year();
    let cur_month = dt.month();

    let next_month = match cur_month {
        m if m == 12 => 1,
        m if m < 12 => m + 1,
        _ => 0,
    };

    let next_year = match cur_month {
        m if m == 12 => cur_year + 1,
        _ => cur_year,
    };

    let month_start_date = Utc.ymd(cur_year, cur_month, 1).and_hms(0, 0, 0);
    let next_month_date = Utc.ymd(next_year, next_month, 1).and_hms(0, 0, 0);

    return (month_start_date, next_month_date);
}

pub fn make_history_file_name(coin: Coin, year: i32, month: u32) -> String {
    let year_month_str = Utc.ymd(year, month, 1).format("%Y%m").to_string();
    return format!("{}_{}", coin.to_string(), year_month_str);
}

pub fn collect_history_to_dir(coin: Coin, start: &DateTime<Utc>,
                              end: &DateTime<Utc>, dir_path: &Path) -> Result<bool, String> {
    const REQUESTS_PER_SEC: f64 = 3.0;
    const SECS_PER_REQUEST: f64 = 1.0 / REQUESTS_PER_SEC;
    let three_hundred_minutes = Duration::minutes(300);

    let mut client = PublicClient::new();

    let (_, mut cur_end) = calc_month_boundary_dates(start);
    let mut cur_start: DateTime<Utc> = *start;

    let mut candles: Vec<Candle> = Vec::new();

    loop {
        if cur_end > *end {
            cur_end = *end;
        }

        // TODO Create a file to write data to

        let mut inner_start = cur_start;

        loop {
            let mut inner_end = inner_start + three_hundred_minutes;

            if inner_end > cur_end {
                inner_end = cur_end;
            }

            let req_start_time = Utc::now();

            let mut chunk_of_candles = client.get_historic_rates("BTC-USD", inner_start, inner_end, 60).unwrap();
            candles.append(&mut chunk_of_candles);

            let req_end_time = Utc::now();

            let req_wait_until = req_start_time + Duration::milliseconds((SECS_PER_REQUEST * 2000.0).ceil() as i64);
            if req_wait_until > req_end_time {
                let wait_amount: Duration = req_wait_until - req_end_time;
                thread::sleep(wait_amount.to_std().unwrap());
            }

            if inner_end == cur_end {
                break;
            }

            inner_start = inner_end;
        }

        if cur_end == *end {
            break;
        }

        let (mut temp_start, mut temp_end) = calc_month_boundary_dates(&cur_end);
        cur_start = temp_start;
        cur_end = temp_end;
    }

    println!("Num candles collected: {}", candles.len());

    return Ok(true);
}
