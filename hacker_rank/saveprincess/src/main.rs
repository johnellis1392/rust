#![deny(warnings)]

extern crate lib;
use lib::matrix::{Matrix};
use lib::seq::{seq_result};
use std::io::{self, BufRead, Read, BufReader, BufWriter, Write};
use std::io::Result as IOResult;
// use std::io::Error as IOError;
// use std::io::{self, BufRead};



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
// fn read_lines<R>(mut input: BufReader<R>, n: i32) -> Result<Vec<String>, String>
fn read_lines<R: Read>(input: BufReader<R>, n: i32) -> Result<Vec<String>, String> {
    let mut result = vec![];
    for line in input.lines().take(n as usize) {
        let _line = line.map(|s| s.trim().to_owned());
        result.push(_line);
    }
    seq_result(result).map_err(|_| "An error occurred while processing input".to_owned())
}

// fn with_prompt<F, A>(prompt: &'static str, mut callback: F) -> A where F: FnMut() -> A {
//     println!("{string}", string=prompt);
//     callback()
// }

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


fn save_princess<R, W>(reader: &Fn() -> BufReader<R>, writer: &Fn() -> BufWriter<W>) -> IOResult<usize>
        where   R: Read,
                W: Write {

    let result = read_lines(reader(), 1)
        .and_then(convert_to_num)
        .and_then(|n| read_lines(reader(), n))
        .map(|v| v.into_iter().map(String::into_bytes).collect())
        .and_then(Matrix::from_vec);

    writer().write(format!("Result: {:?}", result).as_bytes())
}


fn main() {
    // let args = std::env::args();
    let stdin = io::stdin();
    let stdout = io::stdout();
    let input = &|| BufReader::new(stdin.lock());
    let output = &|| BufWriter::new(stdout.lock());
    match save_princess(input, output) {
        Ok(_) => println!(""),
        Err(_) => println!("An error occurred while executing the solution"),
    };
}



#[cfg(test)]
mod test {
    use super::{Elem, save_princess};


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
        use super::{save_princess};
        use std::io::{BufReader, BufWriter, Cursor};

        #[test]
        fn save_princess_should_find_optimal_path() {
            let input = Cursor::new("3\n1 2 3\n4 5 6\n7 8 9".to_owned().into_bytes());
            let output = Cursor::new(Vec::new());

            let input_stream: &Fn() -> BufReader<Cursor<Vec<u8>>> = &|| BufReader::new(input.clone());
            let output_stream: &Fn() -> BufWriter<Cursor<Vec<u8>>> = &|| BufWriter::new(output.clone());

            let result = save_princess(input_stream, output_stream);
            assert!(result.is_ok());
            assert_eq!(output.into_inner(), "[[1, 2, 3], [4, 5, 6], [7, 8, 9]]".as_bytes());
        }

    }

}
