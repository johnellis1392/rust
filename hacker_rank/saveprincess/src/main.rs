#![deny(warnings)]

extern crate lib;
use lib::{seq};


fn main() {
    // let args = std::env::args();
    let a: Vec<Option<i32>> = vec![Some(1), Some(2), Some(3)];
    let c = seq(a);

    match c {
        Some(_) => println!("Success!"),
        None => println!("Error!"),
    }
}
