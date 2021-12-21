//! An interface to map processed user input,
//! and attempt to match it against a boolean case,
//! and its associated error type.
use std::fmt::{
    self,
    Display
};
use super::basic_input::TakeBasicInput;

/// An interface to process and map user input
/// against boolean cases.
pub trait TakeBoolInput<F> 
where
    F: Fn(String) -> String, {
        /// Processes a `String` input, before attempting to map the resulting value,
        /// to two `&str` arrays, designated to be `true` (`t`) or `false` (`f`).
        /// If the processed input does not match against either set,
        /// the `Result` will contain a [`ToBoolError`].
        /// 
        /// # Examples
        /// ```
        /// use std::io;
        /// use my_rusttools::input::TakeBoolInput;
        /// 
        /// match io::stdin()
        ///     .input_to_bool(&["y", "yes"], &["n", "no"], |x|x.to_lowercase()) {
        ///         Ok(case) => println!("User input: {}", case),
        ///         Err(err) => println!("Invalid input: {}", err),
        ///     }
        /// ```
        fn input_to_bool(&self, t: &[&str], f: &[&str], process: F) -> Result<bool, ToBoolError>;

        /// Takes and processes a `String` input, before attempting to map the resulting value,
        /// to two `&str` arrays, designated to be `true` (`t`) or `false` (`f`),
        /// until the processed value can be mapped against one.
        /// 
        /// If `message` is defined as `Some`,
        /// its value will printed for the user with every iteraction.
        /// 
        /// # Examples
        /// ```
        /// use std::io;
        /// use my_rusttools::input::TakeBoolInput;
        /// 
        /// assert!([true, false].contains(&io::stdin()
        ///     .input_to_bool_until_valid(&["y", "yes"], &["n", "no"], Some("Input y(es)/n(o)"), |x|x.to_lowercase())));
        /// ```
        fn input_to_bool_until_valid(&self, t: &[&str], f: &[&str], message: Option<&str>, process: F) -> bool;

        /// Processes a `String` input, before attempting to map the resulting value,
        /// to an array parameter, designated to be `true` (`t`).
        /// If the processed input does not match against the set,
        /// it is assumed to be mapped to `false`.
        /// 
        /// # Examples
        /// ```
        /// use std::io;
        /// use my_rusttools::input::TakeBoolInput;
        /// 
        /// assert!([true, false].contains(&io::stdin()
        ///     .input_to_bool_assume_false(&["y", "yes"], |x|x.to_lowercase())));
        /// ```
        fn input_to_bool_assume_false(&self, t: &[&str], process: F) -> bool;

        /// Processes a `String` input, before attempting to map the resulting value,
        /// to an array parameter, designated to be `false` (`f`).
        /// If the processed input does not match against the set,
        /// it is assumed to be mapped to `true`.
        /// 
        /// # Examples
        /// ```
        /// use std::io;
        /// use my_rusttools::input::TakeBoolInput;
        /// 
        /// assert!([true, false].contains(&io::stdin()
        ///     .input_to_bool_assume_true(&["n", "no"], |x|x.to_lowercase())));
        /// ```
        fn input_to_bool_assume_true(&self, f: &[&str], process: F) -> bool;
    }

impl<T, F> TakeBoolInput<F> for T
where
    F: Fn(String) -> String,
    T: TakeBasicInput, {
        fn input_to_bool(&self, t: &[&str], f: &[&str], process: F) -> Result<bool, ToBoolError> {
            let uinp = process(self.take_string_input());

            Ok(if t.contains(&uinp.trim()) {
                true
            } else if f.contains(&uinp.trim()) {
                false
            } else {
                return Err(ToBoolError);
            })
        }

        fn input_to_bool_until_valid(&self, t: &[&str], f: &[&str], message: Option<&str>, process: F) -> bool {
            loop {
                if let Some(message) = message {
                    print!("{}", message);
                }

                if let Ok(res) = self.input_to_bool(t, f, &process) {
                    break res;
                }
            }
        }
        
        fn input_to_bool_assume_false(&self, t: &[&str], process: F) -> bool {
            let uinp = process(self.take_string_input());

            t.contains(&uinp.as_str())
        }

        fn input_to_bool_assume_true(&self, f: &[&str], process: F) -> bool {
            let uinp = process(self.take_string_input());

            !f.contains(&uinp.as_str())
        }
    }

/// An error which can be returned
/// when attempting to match a converted input against
/// specified collections.
/// 
/// # Potential causes
/// 
/// `ToBoolError` is primarily thrown,
/// due to the converted input being unable
/// to be matched against a value in either array parameter.
/// 
/// # Examples
/// ```
/// use my_rusttools::input::ToBoolError;
/// 
/// assert_eq!("input did not map to a value in either parameter array".to_string(), ToBoolError.to_string());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToBoolError;
    
impl Display for ToBoolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "input did not map to a value in either parameter array".fmt(f)
    }
}