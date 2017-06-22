#![deny(warnings)]


#[allow(dead_code)]
pub fn seq_option<A>(v: Vec<Option<A>>) -> Option<Vec<A>> {
    v.into_iter()
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


#[allow(dead_code)]
pub fn seq_result<A, B>(v: Vec<Result<A, B>>) -> Result<Vec<A>, B> {
    v.into_iter()
        .fold(Ok(vec![]), |res, i| {
            match i {
                Err(e) => Err(e),
                Ok(value) => res.map(|mut acc| {
                    acc.push(value);
                    acc
                })
            }
        })
}


#[cfg(test)]
mod test {
    use super::{seq_option, seq_result};


    #[cfg(test)]
    mod seq_option {
        use super::{seq_option};

        #[test]
        fn test_seq_should_return_option_of_array() {
            let input: Vec<Option<i32>> = vec![Some(1),Some(2),Some(3)];
            let output: Option<Vec<i32>> = seq_option(input);
            let expected_output: Option<Vec<i32>> = Some(vec![1,2,3]);
            assert!(output.is_some());
            assert_eq!(output, expected_output);
        }

        #[test]
        fn test_seq_returns_none_on_invalid_array() {
            let input: Vec<Option<i32>> = vec![Some(1),None,Some(3)];
            let output: Option<Vec<i32>> = seq_option(input);
            let expected_output: Option<Vec<i32>> = None;
            assert!(output.is_none());
            assert_eq!(output, expected_output);
        }

    }



    #[cfg(test)]
    mod seq_result {
        use super::{seq_result};

        #[test]
        fn test_seq_should_return_ok_of_array() {
            let input: Vec<Result<i32, String>> = vec![Ok(1), Ok(2), Ok(3)];
            let output: Result<Vec<i32>, String> = seq_result(input);
            let expected_output: Result<Vec<i32>, String> = Ok(vec![1, 2, 3]);
            assert!(output.is_ok());
            assert_eq!(output, expected_output);
        }

        #[test]
        fn test_eq_should_return_err_on_invalid_array() {
            let input: Vec<Result<i32, String>> = vec![Ok(1), Err("Something went wrong".to_owned()), Ok(3)];
            let output: Result<Vec<i32>, String> = seq_result(input);
            let expected_result: Result<Vec<i32>, String> = Err("Something went wrong".to_owned());
            assert!(output.is_err());
            assert_eq!(output, expected_result);
        }

    }

}
