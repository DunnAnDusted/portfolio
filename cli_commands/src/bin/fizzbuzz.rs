use std::{
    fmt::Write,
    env,
    process,
};

fn main() {
    let iterations: usize = match env::args()
        .skip(1)
        .collect::<String>()
        .trim()
        .parse() {
            Ok(num) => num,
            Err(err) => {
                eprintln!("usage: fizzbuzz <Num: Whole number>\n{}", err);
                process::exit(1);
            }
        };

    println!("{}", fizzbuzz(iterations));
}

fn fizzbuzz(iterations: usize) -> String {
    let mut ret = String::new();

    for i in 1..iterations {
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
            break;
        }
    }

    ret
}