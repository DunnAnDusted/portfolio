#![allow(unused_comparisons)]
use my_rusttools::input::*;
use std::io;

#[test]
#[ignore]
fn until_parsed_test() {     
    let num: usize = io::stdin()
        .take_int_until_parsed(||println!("Please enter a positive number:"), |err|println!("invalid input: {}", err));
     
    assert!(num >= 0);
}

#[test]
#[ignore]
fn range_test() {
    println!("Please enter a number greater than 10:");
         
    match io::stdin().take_int_include_range(&(..=10)) {
        Ok(num) => println!("entered numer: {}", num),
        Err(err) => println!("error: {}", err),
    }
}

#[test]
#[ignore]
fn until_valid_include_test() {     
    let num: usize = io::stdin()
        .take_int_include_range_until_valid(&(..=100), ||println!("Please enter a positive number up to 100:"), |err|println!("invalid input: {}", err));
     
    assert!(num <= 100);
}

#[test]
#[ignore]
fn until_valid_exclude_test() {     
    let num: usize = io::stdin()
        .take_int_exclude_range_until_valid(&(..=10), ||println!("Please enter a number greater than 10:"), |err|println!("invalid input: {}", err));
     
    assert!(num > 10);
}

#[test]
#[ignore]
fn float_until_parsed_test() {
    let num: f64 = io::stdin()
        .take_float_until_parsed(||println!("Please enter a positive number:"), |err|println!("invalid input: {}", err));
 
    assert!(num >= f64::MIN);   
}