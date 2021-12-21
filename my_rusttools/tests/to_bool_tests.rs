use my_rusttools::input::{
    TakeBoolInput,
    ToBoolError,
};
use std::io;    

#[test]
#[ignore]
fn yes_no() {
    assert!([true, false].contains(&io::stdin()
        .input_to_bool_until_valid(&["y", "yes"], &["n", "no"], Some("Input y(es)/n(no"), |x|x.to_lowercase())));
}

#[test]
fn err_test() {
    assert_eq!("input did not map to a value in either parameter array".to_string(), ToBoolError.to_string());
}