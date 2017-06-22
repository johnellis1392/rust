#![deny(warnings)]

extern crate lib;
// use lib::matrix::{Matrix};
// use lib::seq::{seq_result};
// use std::io::{self, BufRead, Read, BufReader, Write, Cursor};
use std::io::{self, BufRead};



// Types for representing data in the problem
#[derive(PartialEq, Eq, Debug)]
pub enum Elem {
    Player,
    Princess,
    Null
}

impl Elem {
    pub fn from_char(c: char) -> Option<Elem> {
        match c {
            'p' => Some(Elem::Princess),
            'm' => Some(Elem::Player),
            '-' => Some(Elem::Null),
            _ => None,
        }
    }
}


// fn read_line<R>(mut input: &R, mut line: &String) -> Result<String, String>
//         where R: Read {
//     Ok(String::new())
// }


/**
 * Read n lines from input
 */
// fn read_lines<R>(mut input: BufReader<R>, n: i32) -> Result<Vec<String>, String>
// fn read_lines<R: Read>(input: BufReader<R>, n: i32) -> Result<Vec<String>, String> {
//     let mut result = vec![];
//     for line in input.lines().take(n as usize) {
//         result.push(line);
//     }
//     seq_result(result).map_err(|_| "An error occurred while processing input".to_owned())
//
//     // let result = vec![];
//     // (0..n).fold(Ok(result), |res, _| {
//     //     let mut line = String::new();
//     //     match input.read_line(&mut line) {
//     //         Ok(_) => res.map(|mut acc| {
//     //             acc.push(line.trim().to_owned());
//     //             acc
//     //         }),
//     //         Err(_) => Err("An error occurred while reading from input".to_string()),
//     //     }
//     // })
// }
//
// fn with_prompt<F, A>(prompt: &'static str, mut callback: F) -> A where F: FnMut() -> A {
//     println!("{string}", string=prompt);
//     callback()
// }
//
// fn convert_to_num(mut vector: Vec<String>) -> Result<i32, String> {
//     let err_string = "An error occurred while reading input".to_owned();
//     if vector.len() != 1 {
//         Err(err_string)
//     } else {
//         match vector.pop() {
//             Some(value) => value
//                 .parse::<i32>()
//                 .map_err(|_| err_string),
//             None => Err(err_string),
//         }
//     }
// }

//
// fn save_princess<R, W>(mut reader: R, mut writer: W)
//         where R: Read, W: Write {
//
//     // let result = with_prompt("Enter number of lines: ", || read_lines(reader, 1))
//     //     .and_then(convert_to_num)
//     //     .and_then(|n| with_prompt("Enter line: ", || read_lines(reader, n)))
//     //     .map(|v| v.into_iter().map(String::into_bytes).collect())
//     //     .and_then(Matrix::from_vec);
//
//     // let reader = BufReader::new(reader);
//     let result = read_lines(BufReader::new(reader), 1)
//         .and_then(convert_to_num)
//         .and_then(|n| read_lines(BufReader::new(reader), n))
//         .map(|v| v.into_iter().map(String::into_bytes).collect())
//         .and_then(Matrix::from_vec);
//
//     // writer.
//     println!("Result: {:?}", result);
// }


fn main() {
    // let args = std::env::args();
    // let stdin = io::stdin();
    // let input = stdin.lock();
    // let stdout = io::stdout();
    // save_princess(input, stdout);
    let stdin = io::stdin();
    println!("Print something: ");
    let input = stdin.lock();
    let i1 = input.lines().take(1).next();
    println!("Entered this: {:?}", i1);
    let input = stdin.lock();
    println!("Print something else: ");
    let i2 = input.lines().take(1).next();
    println!("Entered this: {:?}", i2);
}



#[cfg(test)]
mod test {
    use super::{Elem};

    #[cfg(test)]
    mod elem {
        use super::{Elem};

        #[test]
        fn test_from_char_should_return_correct_element_type() {
            assert_eq!(Elem::from_char('p'), Some(Elem::Princess));
            assert_eq!(Elem::from_char('m'), Some(Elem::Player));
            assert_eq!(Elem::from_char('-'), Some(Elem::Null));
            assert_eq!(Elem::from_char('s'), None);
        }

    }


    #[cfg(test)]
    mod save_princess {
        use super::*;

        #[test]
        fn save_princess_should_find_optimal_path() {
            unimplemented!()
        }
    }

    // #[cfg(test)]
    // mod input {
    // }

}
