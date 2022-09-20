use std::{thread, str::FromStr};

use chrono::{ Utc, DateTime, Duration};

// fn main() {
//     let time = chrono::offset::Utc::now();
//     println!("{}", time);
//     thread::sleep(std::time::Duration::from_secs(1));
//     println!("{}", time.signed_duration_since(chrono::offset::Utc::now()));

//     let db = sled::open("/tmp/blrg").unwrap();
    
//     //db.open_tree(time.to_string()).unwrap();

//     let tree_names = db.tree_names();
//     let mut date_times = std::vec::Vec::<DateTime<Utc>>::with_capacity(tree_names.len());
//     for name in tree_names {
//         let name_str = std::str::from_utf8(name.as_ref()).unwrap();
//         println!("name: \t[{}]", name_str);
//         if let Ok(date_time) = DateTime::<Utc>::from_str(name_str) {
//             //println!("datetime: [{}]", date_time);
//             date_times.push(date_time);
//         }
//     }
//     let oldest = date_times.iter().min().unwrap();

//     println!("oldest: \t{}", oldest);

//     let configuration_setting = 1; //hours
//     let now = Utc::now();
//     // let min_time = now - Duration::hours(configuration_setting);
//     // println!("min time: {}", min_time);
//     // for time in date_times {
//     //     if time < min_time {
//     //         println!("removing");
//     //         db.drop_tree(time.to_string()).unwrap();
//     //     }
//     //     else {
//     //         println!("not removing");
//     //     }
//     // }

//     let newest = *date_times.iter().max().unwrap();
//     let retention = chrono::Duration::hours(configuration_setting);
//     if newest < (now - retention) {
//         println!("hi");
//     }
    
// }

fn main() {
    let db = sled::open("/tmp/blrg2").unwrap();

    let tree = db.open_tree("blah").unwrap();
    let tree2 = db.open_tree("blah2").unwrap();

    tree.insert([0], "hi").unwrap();
    tree2.insert([1], "asdef").unwrap();

    println!("len: {}", tree.len());
    println!("len: {}", tree2.len());
    println!("len: {}", db.len());
    println!("len: {}", db.tree_names().len());
    let names = db.tree_names();
    for name in names {
        println!("{}", std::str::from_utf8( name.as_ref()).unwrap());
    }
}