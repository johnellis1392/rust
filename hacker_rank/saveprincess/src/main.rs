#![deny(warnings)]

extern crate lib;
use lib::matrix::{Matrix};
use std::io::{self, BufRead, Write, Cursor};



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



/**
 * Read n lines from input
 */
fn read_lines<R>(mut input: R, n: i32) -> Result<Vec<String>, String>
        where R: BufRead {
    let result = vec![];
    (0..n).fold(Ok(result), |res, _| {
        let mut line = String::new();
        match input.read_line(&mut line) {
            Ok(_) => res.map(|mut acc| {
                acc.push(line.trim().to_owned());
                acc
            }),
            Err(_) => Err("An error occurred while reading from input".to_string()),
        }
    })
}

fn with_prompt<F, A>(prompt: &'static str, mut callback: F) -> A where F: FnMut() -> A {
    println!("{string}", string=prompt);
    callback()
}

fn convert_to_num(mut vector: Vec<String>) -> Result<i32, String> {
    let err_string = "An error occurred while reading input".to_owned();
    if vector.len() != 1 {
        Err(err_string)
    } else {
        match vector.pop() {
            Some(value) => value
                .parse::<i32>()
                .map_err(|_| err_string),
            None => Err(err_string),
        }
    }
}


fn save_princess<R, W>(mut reader: R, mut writer: W)
        where R: BufRead, W: Write {

    // let result = with_prompt("Enter number of lines: ", || read_lines(reader, 1))
    //     .and_then(convert_to_num)
    //     .and_then(|n| with_prompt("Enter line: ", || read_lines(reader, n)))
    //     .map(|v| v.into_iter().map(String::into_bytes).collect())
    //     .and_then(Matrix::from_vec);
    let result = read_lines(reader, 1)
        .and_then(convert_to_num)
        .and_then(|n| read_lines(reader, n))
        .map(|v| v.into_iter().map(String::into_bytes).collect())
        .and_then(Matrix::from_vec);

    // writer.
    println!("Result: {:?}", result);
}


fn main() {
    // let args = std::env::args();
    let stdin = io::stdin();
    let input = stdin.lock();
    let stdout = io::stdout();
    save_princess(input, stdout);
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

    // #[cfg(test)]
    // mod input {
    // }

}
