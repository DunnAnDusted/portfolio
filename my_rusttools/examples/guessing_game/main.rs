use std::{
    cmp::Ordering,
    io,
};
use rand::Rng;
use my_rusttools::input::TakeInputParse;

fn main() {
    let secret: u8 = rand::thread_rng().gen_range(1..=100);
    let cli_inp = io::stdin();
    println!("Guess the number!");

    loop {
        let guess: u8 = cli_inp.take_input_until_parsed_and_validated(
                |x|(1..=100).contains::<u8>(&x),
                ||println!("Please input a guess from 1 to 100:"),
                |err|println!("invalid input: {}", err)
            );
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