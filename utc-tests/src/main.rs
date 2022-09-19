use std::thread;

use chrono::{TimeZone, Offset};

fn main() {
    let time = chrono::offset::Utc::now();
    println!("{}", time);
    thread::sleep(std::time::Duration::from_secs(1));
    println!("{}", time.signed_duration_since(chrono::offset::Utc::now()));
}
