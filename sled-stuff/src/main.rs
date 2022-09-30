use std::{thread, time::Duration};
use chrono::Utc;

fn main() {
    println!("Hello, world!");

    let config = sled::Config::new().temporary(true);
    let db = config.open().unwrap();

    db.insert(Utc::now().to_string(), "hi").unwrap();
    thread::sleep(Duration::from_millis(200));
    db.insert(Utc::now().to_string(), "hello").unwrap();
    thread::sleep(Duration::from_millis(200));
    db.insert(Utc::now().to_string(), "sup").unwrap();
    thread::sleep(Duration::from_millis(200));
    db.insert(Utc::now().to_string(), "asdf").unwrap();
    thread::sleep(Duration::from_millis(200));
    db.insert(Utc::now().to_string(), "howdy").unwrap();

    for thing in db.iter() {
        println!(
            "{}",
            String::from_utf8(thing.unwrap().1.as_ref().to_vec()).unwrap()
        );
    }

    for thing in db.iter().flatten() {
        println!("{}", String::from_utf8(thing.1.as_ref().to_vec()).unwrap());
    }

    drop(db);
}
