#![deny(warnings)]

extern crate lib;
use lib::{Matrix};
use std::io::{self};



/**
 * Read n lines from stdin
 */
fn read_lines(stdin: io::Stdin, n: u32) -> Result<Vec<String>, String> {
    let result = vec![];
    (0..n).fold(Ok(result), |res, _| {
        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(_) => res.map(|mut acc| {
                acc.push(line);
                acc
            }),
            Err(_) => Err("An error occurred while reading from stdin".to_string()),
        }
    })

    // for i in 0..n {
    //     let mut input = String::new();
    //     match stdin.read_line(&mut input) {
    //         Ok(_) => Ok(input),
    //         Err(_) => Err(format!("Failed to read from stdin")),
    //     };
    // }
    //
    // result
}

fn with_prompt<F, A>(prompt: &'static str, callback: F) -> A where F: Fn() -> A {
    println!("{string}", string=prompt);
    callback()
}

fn convert_to_num(mut vector: Vec<String>) -> Result<u32, String> {
    let err_string = "An error occurred while reading stdin".to_owned();
    if vector.len() != 1 {
        Err(err_string)
    } else {
        match vector.pop() {
            Some(value) => value.parse::<u32>()
                .map_err(|_| err_string),
            None => Err(err_string),
        }
    }
}

fn main() {
    // let args = std::env::args();
    // Matrix::validate_lengths(vec![]);
    let result = with_prompt("", || read_lines(io::stdin(), 1))
        .and_then(convert_to_num)
        .and_then(|n| with_prompt("Enter line: ", || read_lines(io::stdin(), n)))
        .and_then(Matrix::from_vec);

    // .map(|vector| vector.map(String::into_bytes))
    // .and_then(|vector| Matrix::new)

    // match with_prompt("Enter text: ", || read_lines(io::stdin(), 3)) {
    //     Ok(s) => {
    //         let mut input = String::new();
    //         for i in s.into_iter() {
    //             input.push_str(i.trim());
    //         }
    //         println!("Read string: {string}", string=input)
    //     },
    //     Err(e) => println!("Error: {error}", error=e),
    // }
}
