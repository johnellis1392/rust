#![deny(warnings)]

use vec::{transpose};


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



#[cfg(test)]
mod test {
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



    #[test]
    fn test_validate_lengths_should_return_vector_on_success() {
        let input: Vec<Vec<u32>> = vec![
            vec![1,2,3],
            vec![4,5,6],
            vec![7,8,9],
        ];
        let output: Result<Vec<Vec<u32>>, String> = Matrix::validate_lengths(input.clone());
        assert!(output.is_ok());
        assert_eq!(output.unwrap(), input);
    }

    #[test]
    fn test_validate_lengths_should_return_error_on_failure() {
        let input: Vec<Vec<u32>> = vec![
            vec![1,2,3],
            vec![4],
            vec![5,6],
        ];
        let output: Result<Vec<Vec<u32>>, String> = Matrix::validate_lengths(input);
        assert!(output.is_err());
    }



    #[test]
    fn test_from_vec_should_create_matrix() {
        let input: Vec<Vec<u32>> = vec![
            vec![1,2,3],
            vec![4,5,6],
            vec![7,8,9],
        ];
        let output: Result<Matrix<u32>, String> = Matrix::from_vec(input);
        let expected_output: Matrix<u32> = Matrix {
            width: 3usize,
            height: 3usize,
            elements: vec![
                vec![1,2,3],
                vec![4,5,6],
                vec![7,8,9],
            ]
        };
        assert!(output.is_ok());
        assert_eq!(output.unwrap(), expected_output);
    }

    #[test]
    fn test_from_vec_should_return_error_on_mismatched_lengths() {
        let input: Vec<Vec<u32>> = vec![
            vec![1,2,3],
            vec![4],
            vec![5,6],
        ];
        let output: Result<Matrix<u32>, String> = Matrix::from_vec(input);
        assert!(output.is_err());
    }

}
