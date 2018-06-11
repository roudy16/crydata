extern crate chrono;
extern crate crydataget;

use chrono::{Utc, Date};
use crydataget::exchange_interactor::{Coin, ExchangeInteractor};

fn main() {
    let e = ExchangeInteractor {
        host: String::from("http://"),
    };

    let today = Utc::now().date();

    e.fetch_history(Coin::Bitcoin, today, today);
}
