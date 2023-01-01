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

    convert.trim()
        .split_word_bounds()
        // Checks whether an item should be processed (contains Latin characters).
        .map(|x| {
            x.starts_with(|y| matches!(y, 'a'..='z' | 'A'..='Z'))
                .then(|| {
                    const VOWELS: &[char] = &['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'];

                    let mut graphemes = x.graphemes(true);

                    let head = graphemes.next()
                        .expect(
                            "this can only be caused due to an empty string, which shouldn't be possible, \
                            because empty strings don't start with *anything*"
                        );

                    // If the first grapheme is a vowel, it should remain the head of the word.
                    // If it's instead a consenant, it should be moved to the back of the string.
                    let (ret, ay_head) = head.contains(VOWELS)
                        .then_some((head, "h"))
                        .unwrap_or(("", head));
                    let mut ret = ret.to_owned() + graphemes.as_str();

                    // Only push hyphen to returned string, if the string isn't empty,
                    // because preceeding hyphen doesn't look right...
                    if !ret.is_empty() {
                        ret.push('-');
                    }

                    ret + ay_head + "ay"
                })
                .map_or(Cow::Borrowed(x), Cow::Owned)
        })
        .collect()
}
