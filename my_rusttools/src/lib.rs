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
/// # Example
/// 
/// ```
/// use my_rusttools::pigify;
/// 
/// let pigified = pigify("Example");
/// assert_eq!("Example-hay", pigified.as_str());
/// ```
pub fn pigify(convert: &str) -> String {
    static VOWELS: &[char] = &['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'];

    convert.trim()
        .split_word_bounds()
        .fold(String::new(), |acc, x| {
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
        })
}