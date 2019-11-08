use std::io::Write;
use std::str::FromStr;

use serde::{Serialize, Deserialize};
use futures::executor::block_on;


#[derive(Serialize, Deserialize, Debug)]
struct Message {
   contents: String,
}


struct Absolute;


async fn process<T:FromStr>(y:i64, x:i64) -> Option<(T, T)> {
    let mut z = T::from_str("test");
    let product = y * x;
    let msg = Box::<Message>::new(Message {contents: "test".to_string()});
    Box::<Absolute>::new(Absolute);
    println!("In Async {}", product);
    async {
        println!("Am i getting the hang of this.")
    }.await;
    return None
}


fn main() {
    Absolute;
    let mut numbers: Vec<u64> = Vec::new();
    println!("Hello, world!");
    let mut d = u64::from_str("2");
    match d.as_mut(){
        Ok(v) => numbers.push(*v),
        Err(e) => writeln!(std::io::stderr(), "test {:?}", e).unwrap(),
    }
    numbers.push(d.expect("something went wrong"));
    println!("{:?}", numbers);
    let entry_point = async move {
        process::<i32>(6, 2).await;
    };
    block_on(entry_point);

    println!("{}", "test");
    let m:[i16;4];
    std::process::exit(0);
}


#[test]
fn test_me() {
    assert_eq!(process::<i32>(1, 2), None);
    assert_eq!(process::<i32>(5, 6), None);
}
