use std::{
    fmt::Write,
    env,
    process,
};

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

    println!("{}", fizzbuzz(iterations)); // Runs the fizzbuzz process, then prints the result.
}

fn fizzbuzz(iterations: usize) -> String {
    let mut ret = String::new();

    for i in 1..=iterations {
        let mut push = String::new();

        if i%3 == 0 {
            push.push_str("Fizz");
        }
        if i%5 == 0 {
            push.push_str("Buzz");
        }
        if push.is_empty() {
            push = i.to_string();
        }

        if let Err(_) = write!(ret, "{}\n", push) {
            break; // Attempts to buffer append the iteration string, to the return string, and breaks the loop so the process will return early if there's an issue.
        }
    }

    ret
}