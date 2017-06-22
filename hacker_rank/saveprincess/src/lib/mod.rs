#![deny(warnings)]

pub mod vec;
pub mod matrix;
pub mod seq;


/**
 * Partial function application macro
 */
// macro_rules! {
//     ($expression:expr) => (
//         || $expression
//     )
// };



#[cfg(test)]
mod tests {
    // use super::{partial};

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

}
