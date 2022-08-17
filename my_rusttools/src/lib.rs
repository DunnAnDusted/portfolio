pub mod factories;
mod gcacher;
mod input;
pub mod traits;

pub use gcacher::GCacher;
pub use input::*;

use unicode_segmentation::UnicodeSegmentation;

/// Roughly translates the provided `&str`
/// into Pig Latin!
/// 
/// # Panics
/// 
/// May panic if `convert` contains a byte sequence
/// which would fail to produce a valid grapheme cluster.
/// 
/// # Example
/// 
/// ```
/// use my_rusttools::pigify;
/// 
/// let pigified = pigify("Example");
/// assert_eq!(pigified, "Example-hay");
/// ```
pub fn pigify(convert: &str) -> String {
    use std::borrow::Cow;

    const VOWELS: &[char] = &['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'];

    convert.trim()
        .split_word_bounds()
        // Empty strings make no sense, but could cause errors...
        .filter(|x| !x.is_empty())
        // Checks whether an item should be processed (contains Latin characters).
        .map(|x| x.starts_with(|y| matches!(y, 'a'..='z' | 'A'..='Z'))
            .then(|| {
                let mut curr_graphs = x.graphemes(true);

                // TODO: `None` varient means something funky happened with the string,
                // and I'm not sure how I'd deal with that currently,
                // other than leaving it to panic...
                let head = curr_graphs.next()
                    .unwrap_or_else(|| panic!("invalid `&str`: {x}"));

                // Removes the first grapheme from the word,
                // to be `ay_head` if it doesn't contain a vowel.
                let (ret, ay_head) = head.contains(VOWELS)
                    .then_some((head, "h"))
                    .unwrap_or_else(|| (curr_graphs.as_str(), head));

                let mut ret = ret.to_owned();

                // Preceeding hyphen looks weird,
                // so should only be pushed if it isn't the first character.
                if !ret.is_empty() {
                    ret.push('-');
                }

                // Returns a `Cow`, due to only conditionally allocating memory.
                Cow::Owned(ret + ay_head + "ay")
            })
            .unwrap_or_else(|| Cow::Borrowed(x))
        )
        .collect()
}
