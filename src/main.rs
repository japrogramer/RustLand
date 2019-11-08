use std::io::Write;
use std::str::FromStr;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct Message {
   contents: String,
}


fn process<T:FromStr>(y:i64, x:i64) -> Option<(T, T)> {
    let mut z = T::from_str("test");
    let msg = Message {contents: "test".to_string()};
    return None
}


fn main() {
    let mut numbers: Vec<u64> = Vec::new();
    println!("Hello, world!");
    let mut d = u64::from_str("2");
    match d.as_mut(){
        Ok(v) => numbers.push(*v),
        Err(e) => writeln!(std::io::stderr(), "test {:?}", e).unwrap(),
    }
    numbers.push(d.expect("something went wrong"));
    println!("{:?}", numbers);
    std::process::exit(0);
}


#[test]
fn test_me() {
    assert_eq!(process::<i32>(1, 2), None);
    assert_eq!(process::<i32>(5, 6), None);
}
