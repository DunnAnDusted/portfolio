use std::{
    cmp::Ordering,
    process
};
use rand::Rng;
use my_rusttools::input::StdinExtended;

fn main() {
    let secret: u8 = rand::thread_rng().gen_range(1..=100);
    let cli_inp = StdinExtended::new();
    println!("Guess the number!");

    loop {
        let guess: u8 = loop {
            println!("Please enter a number from 1 to 100,");

            match cli_inp.read_line_new_string()
                .map_or_else(|err|{
                    eprintln!("input error: {}", err);
                    process::exit(1);
                },
                |x|x.trim().parse()
                ) {
                    Ok(guess) => break guess,
                    Err(err) => eprintln!("invalid input: {}", err)
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