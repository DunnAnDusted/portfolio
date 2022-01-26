use std::{
    env,
    process,
};
use my_rusttools::factories::fizzbuzz;

fn main() {
    let iterations: usize = env::args()
        .skip(1)
        .collect::<String>() // Takes the command arguments following the binary path, and collects it as a String.
        .trim()
        .parse() // Trims the string, and attempts to parse it as a `usize`.
        .unwrap_or_else(|err|{
            eprintln!("usage: fizzbuzz <Num: Whole number>\n\narguments cannot be parsed: {}", err);
            process::exit(1); // Prints usage and error, then exits the process, if the value can't be parsed.
        });

    fizzbuzz()
        .take(iterations)
        .for_each(|x|println!("{}", x));
}