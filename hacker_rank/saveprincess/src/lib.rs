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
    use super::{seq, transpose, Elem, Matrix};

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

    }

}

