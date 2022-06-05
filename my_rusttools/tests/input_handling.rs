#![allow(unused_comparisons)]
use std::ops::RangeBounds;
use my_rusttools::{StdinExtended, ParseStdinExtended};

#[test]
#[ignore = "input testing"]
fn until_parsed_test() {     
    let num: usize = ParseStdinExtended::new()
        .read_line_until_parsed(
            ||println!("Please enter a positive number,"),
            |err|eprintln!("invalid input: {err}")
        );
     
    assert!((..).contains(&num));
}

#[test]
#[ignore = "input testing"]
fn float_until_parsed_test() {
    let num: f64 = ParseStdinExtended::new()
        .read_line_until_parsed(
            ||println!("Please enter a positive number,"),
            |err|eprintln!("invalid input: {err}")
        );
 
    assert!((..).contains(&num));   
}

#[test]
#[ignore = "input testing"]
fn yes_no_map() {
    let uinp = ParseStdinExtended::new()
    .read_line_until_mapped(
        |x|match x.to_lowercase().trim() {
                "y" | "yes" => Some(true),
                _ => None,
        },
        ||println!("Please enter y(es) to continue.")
    );

    assert!(uinp);
}

#[test]
#[ignore = "input testing"]
fn lines_test() {
    let lines = StdinExtended::new()
        .read_lines(1..4, |x|println!("Please enter up to 3 values.\nCurrent count: {x}"))
        .expect("input error")
        .lines()
        .count();

    assert!((1..4).contains(&lines));
}