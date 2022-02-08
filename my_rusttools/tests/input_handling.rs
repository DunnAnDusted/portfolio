#![allow(unused_comparisons)]
use std::ops::ControlFlow;
use my_rusttools::input::*;

#[test]
#[ignore = "input testing"]
fn until_parsed_test() {     
    let uinp = StdinExtended::new();
    let num: usize = loop {
        println!("Please enter a positive whole number,");

        match uinp.read_line_new_string()
            .map_or_else(|err|panic!("input error: {}", err), |x|x.trim().parse()) {
                Ok(num) => break num,
                Err(err) => eprintln!("invalid input: {}", err),
            }
    };
     
    assert!(num >= 0);
}

#[test]
#[ignore = "input testing"]
fn until_valid_include_test() { 
    let uinp = StdinExtended::new();    
    let num: usize = loop {
        println!("Please enter a positive whole number, up to 100,");

        match uinp.read_line_new_string()
            .map_or_else(|err|panic!("input error: {}", err), |x|x.trim().parse()) {
                Ok(num) if (..100).contains(&num) => break num,
                Ok(num) => eprintln!("invalid input: {} greater than 100", num),
                Err(err) => eprintln!("invalid input: {}", err)
            }
    };
     
    assert!(num <= 100);
}

#[test]
#[ignore = "input testing"]
fn until_valid_exclude_test() {     
    let uinp = StdinExtended::new();
    let num: usize = loop {
        println!("Please enter a number greater than 10,");

        match uinp.read_line_new_string()
            .map_or_else(|err|panic!("input error: {}", err), |x|x.trim().parse()) {
                Ok(num) if !(..=10).contains(&num) => break num,
                Ok(num) => eprintln!("invalid input: {} less than/equal to 10", num),
                Err(err) => eprintln!("invalid input: {}", err),
            }
    };
     
    assert!(num > 10);
}

#[test]
#[ignore = "input testing"]
fn float_until_parsed_test() {
    let uinp = StdinExtended::new();
    let num: f64 = loop {
        println!("Please enter a decimal number,");

        match uinp.read_line_new_string()
            .map_or_else(|err|panic!("input error: {}", err), |x|x.trim().parse()) {
                Ok(num) => break num,
                Err(err) => eprintln!("invalid input: {}", err),
            }
    };
 
    assert!(num >= f64::MIN);   
}

#[test]
#[ignore = "input testing"]
fn yes_no() {
    let uinp = StdinExtended::new();
    let uinp = loop {
        println!("Please enter \"y(es)\" or \"n(o)\"");

        let ret = uinp.read_line_new_string()
            .map_or_else(|err|panic!("input error: {}", err), |mut x|{
                x.make_ascii_lowercase();

                match x.trim() {
                    "y" | "yes" => Ok(true),
                    "n" | "no" => Ok(false),
                    other => {
                        eprintln!("invalid input: {}", other);
                        Err(())
                    },
                }
            });

        if let Ok(ret) = ret {
            break ret;
        }
    };

    assert!([true, false].contains(&uinp));
}

#[test]
#[ignore = "input testing"]
fn lines_test() {
    let lines = StdinExtended::new().read_lines(1..=3, 
        |x|println!("Please enter up to 3 values.\nCurrent count: {}", x.lines().count()), 
        |_, _|ControlFlow::Break(())
    ).expect("input error")
        .lines()
        .count();

    assert!((1..4).contains(&lines));
}