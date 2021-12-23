use std::{
    fs,
    io,
};
use regex;

#[derive(Debug, Clone)]
pub struct Config {
    query: regex::Regex,
    path: String,
}

impl Config {
    pub fn new<T>(mut args: T) -> Result<Config, String> 
    where 
        T: Iterator<Item = String> {
            args.next();
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
                    None => Err(String::from("Expected a query and file path."))
            }.map_err(|err|format!("Invalid arguments. {}\nUsage: <Query: RegEx> <File Path: Text>", err))
    }

    fn search<'a>(&self, contents: &'a str) -> Vec<&'a str> {
        contents.lines()
            .filter(|line|self.query.is_match(line))
            .collect()
    }

    pub fn get_query(&self) -> &regex::Regex {
        &self.query
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }
}

pub fn run(config: Config) -> io::Result<()> {
    let content: String = fs::read_to_string(&config.path)?;

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
        regex::Regex::new("").unwrap();
    }
}