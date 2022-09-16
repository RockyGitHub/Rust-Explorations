use std::{thread, time};

fn main() {
    println!("Hello, world!");
    loop {
        thread::sleep(time::Duration::from_secs(10));
    }
}
