pub mod factories;
mod gcacher;
mod input;
pub mod traits;

pub use gcacher::GCacher;
pub use input::*;

use unicode_segmentation::UnicodeSegmentation;
use reitertools::ReItertools;

/// Roughly translates the provided string
/// into Pig Latin!
/// 
/// # Example
/// 
/// ```
/// use my_rusttools::pigify;
/// 
/// let pigified = pigify("Example");
/// assert_eq!("Example-hay", pigified.as_str());
/// ```
pub fn pigify(convert: &str) -> String {
    static VOWELS: &str = "aAeEiIoOuU";

    convert.trim()
        .split_word_bounds()
        .fold(String::new(), |acc, x| {
            // Guard for cases where the item isn't a word.
            if !x.contains(char::is_alphabetic) {
                return acc + x;
            }

            let mut curr_graphs = x.graphemes(true); // Splits the item into it's graphemes.
            
            let (header_graph, ay_graph) = match curr_graphs.next() {
                None => panic!("invalid `&str`: {x}"),
                Some(x) if x.contains(|y|VOWELS.contains(y)) => (x, "h"), // Checks if the first grapheme contains a vowel.
                Some(x) => ("", x), // Returns an empty string for the leading value if the item is a consonant.
            };
            
            // Reformats the values as a new string, trimming leading cases,
            // before being appended to the builder string and returning it.
            acc + format!("{}{}-{}ay", header_graph, curr_graphs.as_str(), ay_graph).trim_start_matches('-')
        })
}

/// Tests whether a string is a palindrome.
/// 
/// If the string is empty, `is_palindrome()` will return `true`.
/// 
/// `is_palindrome()` normalises the case of the string,
/// meaning strings with equivilent characters of the differing cases,
/// will register as a palindrome.
/// 
/// # Examples
/// ```
/// # use my_rusttools::is_palindrome;
/// #
/// assert!(is_palindrome("Wow"));
/// assert!(!is_palindrome("Wow, Rust!"));
/// ```
pub fn is_palindrome(check: &str) -> bool {
    check.chars()
        .map(|x|x.to_lowercase())
        .next_with(|iter|Some((iter.next()?, iter.next_back()?)))
        .all(|(x, y)|x.eq(y))
}