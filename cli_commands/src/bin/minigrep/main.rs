mod lib;

use std::{
    env,
    process
};

fn main() {
    let config = lib::Config::new(env::args().skip(1)) // Attempts to construct a new minigrep config struct, based on the command arguments minus the first file path argument.
        .unwrap_or_else(|err| {
            eprintln!("usage: minigrep <Text: RegEx> <Text: File Path>\n\narguments cannot be parsed: {}", err);
            process::exit(1); // Prints usage and error, then exits the process, if a `Config` struct can't be constructed.
        });

    if let Err(err) = lib::run(config) {
        eprintln!("file reading error: {}", err); // Runs the main process of the command, and prints and error if the specified file can't be found.
    }
}
