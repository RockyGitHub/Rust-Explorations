use std::io::Write;




fn main() {
    let datetime = chrono::offset::Utc::now();
    let mut buf = [0; 500];
    let n = write!(buf.as_mut_slice(), "{}", datetime).unwrap();
    let datetimestr = std::str::from_utf8(&buf[..n]).unwrap();
    println!("Hello, world!");

}
