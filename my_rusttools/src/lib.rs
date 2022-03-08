pub mod factories;
mod gcacher;
mod input;
pub mod traits;

pub use gcacher::GCacher;
pub use input::*;

use unicode_segmentation::UnicodeSegmentation;

/// Roughly translates the provided string
/// into Pig Latin!
/// 
/// # Errors
/// 
/// Will return an `Err` varient, if the expected size of the resulting string exceeds `usize::MAX`.
/// Note: This calculation is somewhat rough, meaning the actual resulting length may have been less than calculated,
/// but the process is more to allow pre-allocation of required memory,
/// rather than handling cases where string capacity would be exceeded.
/// As such, the error case is more of a formality of handling the calculation results.
/// 
/// # Example
/// 
/// ```
/// use my_rusttools::pigify;
/// 
/// let pigified = pigify("Example");
/// assert_eq!(Ok("Example-hay".to_string()), pigified);
/// ```
pub fn pigify(convert: &str) -> Result<String, &'static str> {
    static VOWELS: &[char] = &['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'];

    // Calculates rough length for the resulting string.
    let cap = convert.unicode_words()
        .count()
        .checked_mul(4)
        .and_then(|x|x.checked_add(convert.len()))
        .ok_or("to long to convert")?;

    let mut ret = convert.trim()
        .split_word_bounds()
        .fold(String::with_capacity(cap), |acc, x| {
            // Guard for cases where the item isn't a word.
            if !x.contains(char::is_alphabetic) {
                return acc + x;
            }

            let mut curr_graphs = x.graphemes(true); // Splits the item into it's graphemes.
            
            let (header_graph, ay_graph) = curr_graphs.next()
                .map(|x|if x.contains(VOWELS) {(x, "h")} else {("", x)})
                .expect(format!("invalid `&str`: {x}").as_str());

            // Reformats the values as a new string, trimming leading cases,
            // before being appended to the builder string and returning it.
            acc + format!("{}{}-{}ay", header_graph, curr_graphs.as_str(), ay_graph).trim_start_matches('-')
        });

        ret.shrink_to_fit();
        Ok(ret)
}