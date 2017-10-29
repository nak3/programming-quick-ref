extern crate chrono;

use chrono::prelude::*;

fn main() {
    let now: DateTime<Local> = Local::now();
    let deadline: DateTime<Local> = Local.ymd(2017, 12, 31).and_hms_milli(0, 0, 0, 0);

    println!("{}", deadline.signed_duration_since(now).num_days());
}
