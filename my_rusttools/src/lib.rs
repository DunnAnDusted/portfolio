pub mod factories;
mod gcacher;
mod input;
pub mod traits;

pub use gcacher::GCacher;
pub use input::StdinExtended;

use unicode_segmentation::UnicodeSegmentation;
use reitertools::ReItertools;

/// Roughly translates the provided string
/// into Pig Latin!
/// 
/// # Example
/// ```
/// use my_rusttools::pigify;
/// 
/// let pigified = pigify("Example");
/// assert_eq!("Example-hay", pigified.as_str());
/// ```
pub fn pigify(convert: &str) -> String {
    const VOWELS: &str = "aAeEiIoOuU";
    let mut ret: String = String::new();

    for curr in convert.trim().split_word_bounds() {
        if curr.contains(char::is_alphabetic) {
            let mut curr_graphs: unicode_segmentation::Graphemes = curr.graphemes(true);
            let ay_char: &str = if curr.starts_with(|x| VOWELS.contains(x)) {
                "h"
            } 
            else {
                curr_graphs.next()
                    .expect("invalid grapheme length")
            };
            ret.push_str(format!("{}-{}ay", curr_graphs.as_str(), ay_char)
                .trim_start_matches('-'));
        }
        else {
            ret.push_str(curr);
        }
    }

    ret
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