#![deny(warnings)]



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


#[derive(PartialEq, Eq, Debug)]
pub struct Matrix<A> {
    width: usize,
    height: usize,
    elements: Vec<Vec<A>>,
}

impl <A> Matrix<A> {

    pub fn new(width: usize, height: usize, elements: Vec<A>) -> Result<Matrix<A>, String> {
        if width * height != elements.len() {
            Err(format!("Invalid size measurements given to Matrix constructor; Expected '{size}', got '{invalid_size}'", size=width * height, invalid_size=elements.len()))
        } else {
            match transpose(elements, width, height) {
                Err(s) => Err(s),
                Ok(res) => Ok(Matrix {
                    width: width,
                    height: height,
                    elements: res,
                })
            }
        }
    }

    fn validate_lengths(vector: Vec<Vec<A>>) -> Result<Vec<Vec<A>>, String> {
        let mut lengths: Vec<usize> = vector.iter().map(|i| i.len()).collect();
        match lengths.pop() {
            Some(first) => {
                match lengths.iter().all(|&i| i == first) {
                    true => Ok(vector),
                    false => Err("Invalid element vector supplied to Matrix".to_owned()),
                }
            },
            None => Ok(vector),
        }
    }

    pub fn from_vec(vector: Vec<Vec<A>>) -> Result<Matrix<A>, String> {
        Matrix::validate_lengths(vector)
            .map(|elements| Matrix {
                width: elements.len(),
                height: elements.first().map(Vec::len).unwrap_or(0),
                elements: elements,
            })
    }

}


/**
 * Partial function application macro
 */
// macro_rules! {
//     ($expression:expr) => (
//         || $expression
//     )
// };


/**
 * Trait representing the break function.
 * Based on haskell's break:
 * break :: (a -> Bool) -> [a] -> ([a], [a])
 */
pub trait Break<A> where Self: Sized {
    fn break_at(self, i: usize) -> Result<(Self, Self), String>
        where A: Clone;
    fn break_when<F>(self, pred: F) -> (Self, Self)
        where A: Clone, F: Fn(&A) -> bool;
}


impl <A> Break<A> for Vec<A> {

    fn break_at(self, i:usize) -> Result<(Vec<A>, Vec<A>), String> where A: Clone {
        if i > self.len() {
            Err(format!("Invalid size given to function Vec<A>.break: {size}", size=i))
        } else {
            let length = self.len();
            let v1 = self[0..i].to_vec();
            let v2 = self[i..length].to_vec();
            Ok((v1, v2))
        }
    }


    // Alternate method
//    fn break_at(self, i:usize) -> Result<(Vec<A>, Vec<A>), String> where A: Clone {
//        if i > self.len() {
//            Err(format!("Invalid size given to function Vec<A>.break: {size}", size=i))
//        } else {
//            let length = self.len();
//            let v1 = self.iter().cloned().take(i).collect();
//            let v2 = self.into_iter().skip(i).take(length - i).collect();
//            Ok((v1, v2))
//        }
//    }

    fn break_when<F>(self, pred: F) -> (Vec<A>, Vec<A>)
            where A: Clone, F: Fn(&A) -> bool {
        let v1: Vec<A> = self.iter().cloned().take_while(|i| pred(i)).collect();
        let v2: Vec<A> = self.into_iter().skip_while(|i| pred(i)).collect();
        (v1, v2)
    }

}


// Trait representing a pure-functional pop operation
pub trait PPop<A> where Self: Sized {
    fn ppop(self) -> Option<(A, Self)>;
}

impl <A> PPop<A> for Vec<A> {
    fn ppop(mut self) -> Option<(A, Self)> {
        self.pop().map(|i| (i, self))
    }
}



/**
 * Transform a vector into a 2-dimensional vector with the specified size.
 */
pub fn transpose<A>(mut elements: Vec<A>, width: usize, height: usize) -> Result<Vec<Vec<A>>, String> {
    if width * height != elements.len() {
        Err(format!(
            "Invalid size measurements given to Vec<A>.transpose; Expected '{size}', got '{invalid_size}'",
            size=width * height,
            invalid_size=elements.len())
        )
    } else {
        let mut rows: Vec<Vec<A>> = Vec::new();
        for _ in 0..width {
            let column: Vec<A> = elements.drain(0..height).collect();
            rows.push(column);
        }
        Ok(rows)
    }
}


/**
 * Turn a list of non-deterministic values into a result of a list.
 * Based on haskell's sequence function:
 * sequence :: (Monad m, Traversable t) => t (m a) -> m (t a)
 */
pub fn seq<A>(a: Vec<Option<A>>) -> Option<Vec<A>> {
    a.into_iter()
        .fold(Some(vec![]), |res, i| {
            match i {
                None => None,
                Some(value) => res.map(|mut acc| {
                    acc.push(value);
                    acc
                })
            }
        })
}


#[cfg(test)]
mod tests {
    use super::{seq, transpose, Elem, Matrix, Break, PPop};

    // Seq
    #[test]
    fn test_seq_should_return_option_of_array() {
        let input: Vec<Option<i32>> = vec![Some(1),Some(2),Some(3)];
        let output: Option<Vec<i32>> = seq(input);
        let expected_output: Option<Vec<i32>> = Some(vec![1,2,3]);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_seq_returns_none_on_invalid_array() {
        let input: Vec<Option<i32>> = vec![Some(1),None,Some(3)];
        let output: Option<Vec<i32>> = seq(input);
        let expected_output: Option<Vec<i32>> = None;
        assert_eq!(output, expected_output);
    }



    // Transpose
    #[test]
    fn test_transpose_should_resize_vector() {
        let input: Vec<i32> = vec![1,2,3,4,5,6,7,8,9];
        let output = transpose(input, 3, 3);
        let expected_output = Ok(vec![
            vec![1,2,3],
            vec![4,5,6],
            vec![7,8,9],
        ]);
        assert_eq!(output, expected_output);

        let input: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];
        let output = transpose(input, 2, 5);
        let expected_output = Ok(vec![
            vec![1,2,3,4,5],
            vec![6,7,8,9,10],
        ]);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_transpose_should_fail_if_invalid_bounds_given() {
        let input: Vec<i32> = vec![1,2,3,4,5,6,7,8,9];
        let output = transpose(input, 3, 4);
        assert!(output.is_err());
    }



    #[cfg(test)]
    mod partial {

        // #[test]
        // fn test_partial_expands_function() {
        //     let f = partial!(_ * 2);
        //     let expected_result = 4;
        //     let result = f(2);
        //     assert_eq!(result, expected_result);
        // }

    }


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
    mod matrix {
        use super::{Matrix};

        #[test]
        fn test_matrix_should_create_new_matrix() {
            let elements: Vec<i32> = vec![0,1,2,3,4,5,6,7,8];
            let expected_elements = vec![
                vec![0,1,2],
                vec![3,4,5],
                vec![6,7,8],
            ];

            let width: usize = 3;
            let height: usize = width;
            let matrix: Result<Matrix<i32>, String> = Matrix::new(width, height, elements);

            assert!(matrix.is_ok());
            let result_matrix = matrix.unwrap();
            assert_eq!(result_matrix.width, width);
            assert_eq!(result_matrix.height, height);
            assert_eq!(result_matrix.elements, expected_elements);
        }

        #[test]
        fn test_matrix_should_fail_if_invalid_values_given() {
            let elements: Vec<i32> = vec![0,1,2,3,4,5,6,7,8,9]; // Length doesn't match params
            let matrix = Matrix::new(3, 3, elements);
            assert!(matrix.is_err());
        }

        // #[test]
        // fn test_validate_lengths_should_return_vector_on_success() {
        //     let input: Vec<Vec<u32>> = vec![
        //         vec![1,2,3],
        //         vec![4,5,6],
        //         vec![7,8,9],
        //     ];
        //     let output: Result<Matrix<u32>, String> = Matrix::validate_lengths(input);
        //     assert_eq!(output, input);
        // }
        //
        // #[test]
        // fn test_validate_lengths_should_return_error_on_failure() {
        //     let input: Vec<Vec<u32>> = vec![
        //         vec![1,2,3],
        //         vec![4],
        //         vec![5,6],
        //     ];
        //     let output: Result<Matrix<u32>, String> = Matrix::validate_lengths(input);
        //     assert!(output.is_err());
        // }
        //
        // #[test]
        // fn test_from_vec_should_create_matrix() {
        //     let input: Vec<Vec<u32>> = vec![
        //         vec![1,2,3],
        //         vec![4,5,6],
        //         vec![7,8,9],
        //     ];
        //     let output: Result<Matrix<u32>, String> = Matrix::from_vec(input);
        //     let expected_output: Matrix<u32> = Ok(Matrix {
        //         width: 3usize,
        //         height: 3usize,
        //         elements: vec![
        //             vec![1,2,3],
        //             vec![4,5,6],
        //             vec![7,8,9],
        //         ]
        //     });
        //     assert_eq!(output, expected_output);
        // }
        //
        // #[test]
        // fn test_from_vec_should_return_error_on_mismatched_lengths() {
        //     let input: Vec<Vec<u32>> = vec![
        //         vec![1,2,3],
        //         vec![4],
        //         vec![5,6],
        //     ];
        //     let output: Result<Matrix<u32>, String> = Matrix::from_vec(input);
        //     assert!(output.is_err());
        // }
        //
    }


    #[cfg(test)]
    mod break_at {
        use super::{Break};

        #[test]
        fn test_break_at_splits_vector_in_half() {
            let elements = vec![1,2,3,4,5,6,7];
            assert_eq!(elements.break_at(3), Ok((vec![1,2,3], vec![4,5,6,7])));

            let elements = vec![1,2];
            assert_eq!(elements.break_at(1), Ok((vec![1], vec![2])));

            let elements = vec![1,2,3,4,5,6];
            assert_eq!(elements.break_at(6), Ok((vec![1,2,3,4,5,6], vec![])));
        }

        #[test]
        fn test_break_at_returns_error_on_invalid_values() {
            let elements = vec![1,2,3];
            assert!(elements.break_at(4).is_err());
        }


        #[test]
        fn test_break_when_splits_vector_in_half() {
            let elements = vec![1,2,3,4,5,6,7];
            assert_eq!(elements.break_when(|&i| i <= 3), (vec![1,2,3], vec![4,5,6,7]));

            let elements = vec![1,2];
            assert_eq!(elements.break_when(|&i| i <= 1), (vec![1], vec![2]));

            let elements = vec![1,2,3,4,5,6];
            assert_eq!(elements.break_when(|&i| i < 10), (vec![1,2,3,4,5,6], vec![]));
        }

    }


    #[cfg(test)]
    mod ppop {
        use super::{PPop};

        #[test]
        fn it_should_return_proper_result() {
            let input = vec![1,2,3];
            let output = input.ppop();
            assert!(output.is_some());
            assert_eq!(output, Some((3, vec![1,2])));
        }

        #[test]
        fn it_should_return_an_error_for_empty_vectors() {
            let input: Vec<u32> = vec![];
            let output = input.ppop();
            assert!(output.is_none());
        }

    }
}
