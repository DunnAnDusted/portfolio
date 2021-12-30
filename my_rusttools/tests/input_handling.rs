#![allow(unused_comparisons)]
use my_rusttools::input::*;
use std::io;

#[test]
#[ignore]
fn until_parsed_test() {     
    let num: usize = io::stdin()
        .take_input_until_parsed(
            ||println!("Please enter a positive number:"),
            |err|println!("invalid input: {}", err)
        );
     
    assert!(num >= 0);
}

#[test]
#[ignore]
fn until_valid_include_test() {     
    let num: usize = io::stdin()
        .take_input_until_parsed_and_validated(
            |x|(..=100).contains::<usize>(&x),
            ||println!("Please enter a positive number up to 100:"),
            |err|println!("invalid input: {}", err)
        );
     
    assert!(num <= 100);
}

#[test]
#[ignore]
fn until_valid_exclude_test() {     
    let num: usize = io::stdin()
        .take_input_until_parsed_and_validated(
            |x|!(..=10).contains::<usize>(&x),
            ||println!("Please enter a number greater than 10:"),
            |err|println!("invalid input: {}", err)
        );
     
    assert!(num > 10);
}

#[test]
#[ignore]
fn float_until_parsed_test() {
    let num: f64 = io::stdin()
        .take_input_until_parsed(
            ||println!("Please enter a positive number:"),
            |err|println!("invalid input: {}", err)
        );
 
    assert!(num >= f64::MIN);   
}

#[test]
#[ignore]
fn yes_no() {
    assert!([true, false].contains(&io::stdin()
        .take_input_until_mapped(
            |mut x| {
                x.make_ascii_lowercase();
                let trimmed = x.trim();

                if ["yes", "y"].contains(&trimmed) {
                    Some(true)
                } else if ["no", "n"].contains(&trimmed) {
                    Some(false)
                } else {
                    None
                }
            },
            ||println!("Please enter 'y(es)' or 'n(o)':"))
        ));
}