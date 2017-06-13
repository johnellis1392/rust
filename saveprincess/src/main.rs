#![deny(warnings)]

extern crate lib;
use lib::{seq, seq2};

fn main() {
    let a: Vec<Option<i32>> = vec![Some(1), Some(2), Some(3)];
    let c = seq(a);

    match c {
        Some(_) => println!("Success!"),
        None => println!("Error!"),
    }

    let a: Vec<Option<i32>> = vec![Some(1), Some(2), Some(3)];
    let c = seq2(a);

    match c {
        Some(_) => println!("Success!"),
        None => println!("Error!"),
    }
}

