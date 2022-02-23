use std::cmp::Ordering;
use rand::Rng;
use my_rusttools::ParseStdinExtended;

fn main() {
    let secret: u8 = rand::thread_rng().gen_range(1..=100);
    let cli_inp = ParseStdinExtended::new();
    println!("Guess the number!");

    loop {
        let guess: u8 = loop {
            if let x @ 0..=100 = cli_inp.read_line_until_parsed(
                ||println!("Please enter a number from 1 to 100,"),
                |err|eprintln!("invalid input: {err}")
            ) {
                break x;
            }
        };

        println!("Your guess: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}