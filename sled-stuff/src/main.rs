fn main() {
    println!("Hello, world!");


    let db = sled::open("mydb").unwrap();

    db.insert(&[1], &[2]).unwrap();
    
    drop(db);
}
