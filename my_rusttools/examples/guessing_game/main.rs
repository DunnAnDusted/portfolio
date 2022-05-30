use std::{io, cmp::Ordering};
use rand::Rng;
use my_rusttools::StdinExtended;

fn main() -> io::Result<()> {
    let secret: u8 = rand::thread_rng().gen_range(1..=100);
    let cli_inp = StdinExtended::new();
    println!("Guess the number!");

    loop {
        let guess: u8 = loop {
            println!("Please enter a number from 1 to 100:");
            
            let guess = cli_inp.read_line_new_string()?
                .trim()
                .parse();

            print!("\x1B[2J\x1B[1;1H");

            match guess {
                Ok(x @ 1..=100) => break x,
                Ok(x) => println!("invalid input: guess {x}, doesn't fall within range 1..100"),
                Err(err) => println!("invalid input: {err}")
            }
        };

        println!("Your guess: {guess}");

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                return Ok(());
            }
        }
    }
}