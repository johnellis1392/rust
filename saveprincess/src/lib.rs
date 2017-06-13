#![deny(warnings)]


pub fn seq<A>(a: Vec<Option<A>>) -> Option<Vec<A>> {
    let mut result: Vec<A> = vec![];
    for i in a {
        match i {
            None => {
                return None
            },
            Some(value) => {
                result.push(value)
            }
        }
    }
    return Some(result)
}


pub fn seq2<A>(a: Vec<Option<A>>) -> Option<Vec<A>> {
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
    use super::{seq, seq2};

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

    #[test]
    fn test_seq2_should_return_option_of_array() {
        let input: Vec<Option<i32>> = vec![Some(1),Some(2),Some(3)];
        let output: Option<Vec<i32>> = seq2(input);
        let expected_output: Option<Vec<i32>> = Some(vec![1,2,3]);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_seq2_returns_none_on_invalid_array() {
        let input: Vec<Option<i32>> = vec![Some(1),None,Some(3)];
        let output: Option<Vec<i32>> = seq2(input);
        let expected_output: Option<Vec<i32>> = None;
        assert_eq!(output, expected_output);
    }
}
