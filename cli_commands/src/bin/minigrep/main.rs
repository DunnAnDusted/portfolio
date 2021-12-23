mod lib;

use std::{
    env,
    process
};

fn main() {
    let config = lib::Config::new(env::args())
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(1); 
        });

    if let Err(err) = lib::run(config) {
        eprintln!("File reading error: {}", err);
    }
}
