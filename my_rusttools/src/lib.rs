pub mod factories;
mod gcacher;
mod input;
pub mod traits;

pub use gcacher::GCacher;
pub use input::StdinExtended;

use unicode_segmentation::UnicodeSegmentation;

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