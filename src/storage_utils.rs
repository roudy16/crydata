use chrono::prelude::*;
use chrono::{Duration, Datelike};
use csv::{Writer};

use std::error::Error;
use std::path::Path;
use std::fs::{File, OpenOptions, DirBuilder};
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

pub fn make_history_file_name(coin: &Coin, year: i32, month: u32) -> String {
    let year_month_str = Utc.ymd(year, month, 1).format("%Y%m").to_string();
    return format!("{}_{}", coin.to_string(), year_month_str);
}

pub fn collect_history_to_dir(coin: &Coin, product_str: &str, start: &DateTime<Utc>,
                              end: &DateTime<Utc>, dir_path: &Path) -> Result<bool, String> {
    const REQUESTS_PER_SEC: f64 = 3.0;
    const SECS_PER_REQUEST: f64 = 1.0 / REQUESTS_PER_SEC;
    let three_hundred_minutes = Duration::minutes(300);

    DirBuilder::new()
        .recursive(true)
        .create(dir_path).unwrap();

    let mut client = PublicClient::new();

    let (_, mut cur_end) = calc_month_boundary_dates(start);
    let mut cur_start: DateTime<Utc> = *start;


    loop {
        let file_name= format!("{}.csv", make_history_file_name(&coin, start.year(), start.month()));
        let file_path_str = format!("{}/{}", dir_path.to_str().unwrap(), file_name);
        let file_path = Path::new(&file_path_str);
        let mut f = OpenOptions::new().write(true).create(true).open(file_path).unwrap();
        let mut csv_writer = Writer::from_writer(f);

        let mut candles: Vec<Candle> = Vec::new();

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

            let mut chunk_of_candles = match client.get_historic_rates(product_str, inner_start, inner_end, 60) {
                Ok(val) => Some(val),
                Err(e) => None,
            };

            if chunk_of_candles.is_some() {
                candles.append(&mut chunk_of_candles.unwrap());
            }

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

        csv_writer.write_record(&["time", "low", "high", "open", "close", "volume"]).unwrap();

        for c in &candles {
            csv_writer.write_record(
                &[
                    c.time.to_string(),
                    c.low.to_string(),
                    c.high.to_string(),
                    c.open.to_string(),
                    c.close.to_string(),
                    c.volume.to_string()]).unwrap();
        }

        csv_writer.flush().unwrap();

        if cur_end == *end {
            break;
        }

        let (mut temp_start, mut temp_end) = calc_month_boundary_dates(&cur_end);
        cur_start = temp_start;
        cur_end = temp_end;
    }


    return Ok(true);
}
