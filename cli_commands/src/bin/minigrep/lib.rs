use std::{
    fs,
    io,
};
use regex;

/// Config struct for searching for lines in a string,
/// containing the specified query.
/// 
/// # Examples
/// ```
/// let config = lib::Config::new(env::args().skip(1)) // Attempts to construct a new minigrep config struct, based on the command arguments minus the first file path argument.
///     .unwrap_or_else(|err| {
///         eprintln!("usage: minigrep <Text: RegEx> <Text: File Path>\n\narguments cannot be parsed: {}", err);
///         process::exit(1); // Prints usage and error, then exits the process, if a `Config` struct can't be constructed.
///     });
/// ```
#[derive(Debug, Clone)]
pub struct Config {
    query: regex::Regex,
    path: String,
}

impl Config {
    /// Attempts to create a new `Config` struct,
    /// with a query based on the arguments passed.
    /// 
    /// # Errors
    /// 
    /// Will return `Err` if the command had no arguments,
    /// or did not provide a valid Regular Expression.
    /// 
    /// # Examples
    /// ```
    /// let config = lib::Config::new(env::args().skip(1)) // Attempts to construct a new minigrep config struct, based on the command arguments minus the first file path argument.
    ///     .unwrap_or_else(|err| {
    ///         eprintln!("usage: minigrep <Text: RegEx> <Text: File Path>\n\narguments cannot be parsed: {}", err);
    ///         process::exit(1); // Prints usage and error, then exits the process, if a `Config` struct can't be constructed.
    ///     });
    /// ```
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Self, String> {
            match args.next() {
                    Some(query) => {
                        match regex::Regex::new(&query) {
                            Ok(query) => {
                                Ok(Config {
                                    query: query,
                                    path: args.collect(),
                                })
                            }
                            Err(err) => Err(err.to_string())
                        }
                    }
                    None => Err(String::from("expected a query and file path."))
            }.map_err(|err|format!("invalid arguments. {}", err))
    }

    /// Searches for lines matching the specified query
    /// in the passed string.
    /// 
    /// # Examples
    /// ```
    /// let content: String = fs::read_to_string(&config.path)?;
    ///
    /// for item in config.search(&content) {
    ///        println!("{}", item);
    /// }
    /// ```
    fn search<'a>(&'a self, contents: &'a str) -> impl Iterator<Item = &'a str>{
        contents.lines()
            .filter(|line|self.query.is_match(line))
    }

    /// Gets a referance to the query a `Config` was created with.
    /// 
    /// # Examples
    /// ```
    /// let args = ["\\AExample\\z", ""];
    /// let config = Config::new(args.iter().map(|x|x.to_string()))
    ///     .unwrap();
    /// 
    /// assert!(config.query().is_match("Example"));
    /// ```
    pub fn query(&self) -> &regex::Regex {
        &self.query
    }

    /// Returns a string slice refering to the file path
    /// a `Config` was created with.
    /// 
    /// # Examples
    /// ```
    /// let args = ["\\A\\z", "Example"];
    /// let config = Config::new(args.iter().map(|x|x.to_string()))
    ///     .unwrap();
    /// 
    /// assert_eq!("Example", config.path());
    /// ```
    pub fn path(&self) -> &str {
        &self.path
    }
}

pub fn run(config: Config) -> io::Result<()> {
    let content: String = fs::read_to_string(config.path())?;

    for item in config.search(&content) {
        println!("{}", item);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex;

    #[test]
    fn parsing_test() {
        regex::Regex::new("\\A\\z").unwrap();
    }
}