//! Custom input handling tools.
use std::{
    io,
    ops::{Bound::*, RangeBounds},
    str::FromStr,
    iter::{Map, Enumerate},
    vec,
};

/// An interface for taking basic input as strings.
pub trait TakeInputBasic<T> {
    /// Takes a line of input as a `String`.
    ///
    /// # Examples
    /// 
    /// ```
    /// use std::io;
    /// use my_rusttools::input::TakeInputBasic;
    ///
    /// let a = io::stdin().take_string_input();
    /// println!("User input: {}", a);
    /// ```
    fn take_input(&self) -> String;

    /// Takes a line of input as a `String`,
    /// mapping it to `T` by applying the passed function.
    ///
    /// # Examples
    /// 
    /// ```
    /// use std::io;
    /// use my_rusttools::input::TakeInputBasic;
    ///
    /// match io::stdin().take_string_input_then_map(|mut x| {
    ///     x.make_ascii_lowercase();
    ///     let trimmed = x.trim();
    ///
    ///     if ["y", "yes"].contains(&trimmed) {
    ///         Ok(true)
    ///     } else if ["n", "no"].contains(&trimmed) {
    ///         Ok(false)
    ///     } else {
    ///         Err("input must be \"y(es)\" or \"n(o)\"")
    ///     }
    /// }) {
    ///     Ok(case) => println!("User's input was: {}", case),
    ///     Err(err) => eprintln!("invalid input: {}", err),
    /// }
    /// ```
    fn take_input_then_map<F>(&self, f: F) -> T
    where
        F: FnOnce(String) -> T, {
            f(self.take_input())
        }

    /// Takes a line of input as a `String`,
    /// mapping it to [`Option<T>`] by applying the passed function,
    /// until a [`Some`] varient is returned.
    /// 
    /// # Closures
    /// 
    /// The signiture of this method features two closures,
    /// one of which warrents explanation as to its usage:
    /// 
    /// 
    /// ## `notif`
    /// 
    /// `notif` will be run at the beginning of each loop,
    /// and is intended to allow the developer to push a notification
    /// on the intended usage of the program.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::io;
    /// use my_rusttools::input::TakeInputBasic;
    /// 
    /// let uinp: usize = io::stdin()
    ///     .take_string_input_until_mapped(
    ///         |x|x.trim()
    ///             .parse()
    ///             .ok(), 
    ///         ||println!("Please enter a posive whole number:")
    /// );
    /// 
    /// println!("User input: {}", uinp);
    /// ```
    /// 
    /// [`Option<T>`]: std::option::Option
    /// [`Some`]: std::option::Option::Some
    fn take_input_until_mapped<F, G>(&self, mut f: F, mut notif: G) -> T
    where
        F: FnMut(String) -> Option<T>,
        G: FnMut(), {
            loop {
                notif();

                if let Some(ret) = f(self.take_input()) {
                    break ret;
                }
            }
        }

    /// Takes lines of input as a `String`,
    /// between the bounds specified,
    /// or until index limits are reached.
    ///
    /// # Limits
    ///
    /// Regardless of the number of iterations taking input,
    /// the method should immediately return if the input `String`
    /// reaches the maximum size.
    /// 
    /// # Closures
    /// 
    /// The signiture of this method features two closures,
    /// which warrents explantation:
    /// 
    /// ## `notif`
    /// 
    /// `notif` will run at the beginning of each loop,
    /// and is intended to allow the developer to push a notification
    /// on the intended usage of the program.
    /// 
    /// In the case of this closure,
    /// that also includes the index of the current iteration,
    /// to allow behaviour to be differentiated by line,
    /// as well as a string slice of the collective contents of the input thus far.
    /// 
    /// ## `err_notif`
    /// 
    /// `err_notif` will run if there is an error reading input,
    /// and is intended to allow the developer to push a notification on the misinput,
    /// as well as specify whether the function will proceed following this error.
    /// 
    /// Similarly to `notif`, `error_notif` is also provided with values
    /// indicating the current state of the function,
    /// allowing them to be incorperated into the behaviour of the closure.
    ///
    /// # Examples
    /// 
    /// ```
    /// use std::io;
    /// use my_rusttools::input::TakeInputBasic;
    ///
    /// let a = io::stdin().take_lines_input(
    ///     |x, _|println!("Please input between 3 and 10 lines.\n{} number remaining.", 3 - x);,
    ///     |err, x, _|{println!("input error: {}", err); x >= 3}, 
    ///     3..10
    /// );
    /// assert!(a.lines().count() > 2);
    /// ```
    fn take_lines_input<U: RangeBounds<usize>, F, EF>(&self, notif: F, err_notif: EF, bounds: U) -> String
    where
        F: FnMut(usize, &str),
        EF: FnMut(io::Error, usize, &str) -> bool;

    /// Takes lines of input as a `String`,
    /// between the bounds specified,
    /// or until index limits are reached.
    /// 
    /// # Limits
    /// 
    /// Regardless of the number of iterations taking input,
    /// the method should immediately return if the input `String`
    /// reaches the maximum size.
    /// 
    /// # Closures
    /// 
    /// The signitiure of this method, features three closures,
    /// two of which warrent explantation:
    /// 
    /// ## `notif`
    /// 
    /// `notif` will run at the beginning of each loop,
    /// and is intended to allow the developer to push a notification
    /// on the intended usage of the program.
    /// 
    /// In the case of this closure,
    /// that also includes the index of the current iteration,
    /// to allow behaviour to be differentiated by line,
    /// as well as a string slice of the collective contents of the input thus far.
    /// 
    /// ## `err_notif`
    /// 
    /// `err_notif` will run if there is an error reading input,
    /// and is intended to allow the developer to push a notification on the misinput,
    /// as well as specify whether the function will proceed following this error.
    /// 
    /// Similarly to `notif`, `error_notif` is also provided with values
    /// indicating the current state of the function,
    /// allowing them to be incorperated into the behaviour of the closure.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::io;
    /// use my_rusttools::input::TakeInputBasic;
    /// 
    /// let a = io::stdin().take_lines_input(
    ///     |x, _|println!("Please input between 3 and 10 numbers.\n{} numbers remaining.", 3 - x);,
    ///     |err, x, _|{println!("input error: {}", err); x >= 3}, 
    ///     |(_, x)|x.trim().parse::<isize>(),
    ///     3..10
    /// );
    /// assert!(a.count() > 2);
    /// ```
    fn take_lines_input_then_map<E, U: RangeBounds<usize>, F, G, EF>(&self, notif: G, err_notif: EF, f: F, bounds: U) -> Map<Enumerate<vec::IntoIter<String>>, F>
    where
        F: FnMut((usize, String)) -> T,
        G: FnMut(usize, &str), 
        EF: FnMut(io::Error, usize, &str) -> bool, {
            self.take_lines_input(notif, err_notif, bounds)
                .lines()
                .map(|x|x.to_string())
                .collect::<Vec<String>>()
                .into_iter()
                .enumerate()
                .map(f)
        }
}

impl<T> TakeInputBasic<T> for io::Stdin {
    fn take_input(&self) -> String {
        let mut ret = String::new();

        self.read_line(&mut ret).expect("failed to read input");

        ret
    }

    fn take_lines_input<U: RangeBounds<usize>, F, EF>(&self, mut notif: F, mut err_notif: EF, bounds: U) -> String 
    where
        F: FnMut(usize, &str),
        EF: FnMut(io::Error, usize, &str) -> bool, {
        let mut ret = String::new();
        let mut iterations = 0;
        let start = *match bounds.start_bound() {
            Included(start) => start,
            Excluded(start) => start,
            Unbounded => &0,
        };
        let end = match bounds.end_bound() {
            Included(end) => *end,
            Excluded(end) => end - 1,
            Unbounded => usize::MAX,
        };

        loop {
            if iterations >= end || iterations >= usize::MAX {
                break;
            }

            notif(iterations, ret.as_str());

            let size = match self.read_line(&mut ret) {
                Ok(size) => size,
                Err(err) => {
                    if err_notif(err, iterations, ret.as_str()) {
                        break;
                    }

                    0
                },
            }.checked_sub(2)
                .unwrap_or_default();

            if size > 1 {
                iterations += 1
            } else {
                ret.truncate(ret.len() - 2);
                if iterations >= start {
                    break;
                }
            }
        }

        ret
    }
}

pub trait TakeInputParse<T, E> 
where
    T: FromStr<Err = E>, {
        /// Takes input from a buffer,
        /// parsing it into another type.
        /// 
        /// # Errors
        /// 
        /// Will return [`Err`] if it's not possible to parse the input into the desired type.
        /// 
        /// # Examples
        /// 
        /// ```
        /// use std::io;
        /// use my_rusttools::input::TakeInputParse;
        /// 
        /// match io::stdin().take_input_parse::<usize>() {
        ///     Ok(inp) => println!("User input: {}", inp),
        ///     Err(err) => eprintln!("invalid input: {}", err),
        /// }
        /// ```
        fn take_input_then_parse(&self) -> Result<T, E>;

        /// Takes input from a buffer,
        /// parsing it into another type
        /// until it's parsed sucessfully.
        /// 
        /// # Closures
        /// 
        /// The signiture for this method features two closures,
        /// warrenting explanation as to their purpose:
        /// 
        /// ## `notif`
        /// 
        /// `notif` will be run at the beginning of every loop,
        /// and is intended to allow the developer to push a notification
        /// on the expected usage of the program.
        /// 
        /// ## `err_notif`
        /// 
        /// `err_notif` will be run if there is an error parsing the input,
        /// and is intended to allow the developer to push a notification
        /// on the misinput, and the expected usage of the program.
        /// 
        /// # Examples
        /// 
        /// ```
        /// use std::io;
        /// use my_rusttools::input::TakeInputParse;
        /// 
        /// let uinp: usize = io::stdin().take_input_until_parsed(
        ///     ||println!("Please enter a positive whole number:"),
        ///     |err|println!("invalid input: {}", err)
        /// );
        /// 
        /// println!("User input: {}", uinp);
        /// ```
        fn take_input_until_parsed<F, EF>(&self, mut notif: F, mut err_notif: EF) -> T
        where
            F: FnMut(),
            EF: FnMut(E), {
                loop {
                    notif();

                    match self.take_input_then_parse() {
                        Ok(ret) => break ret,
                        Err(err) => err_notif(err),
                    }
                }
            }

        /// Takes input from a buffer,
        /// parsing into another type
        /// until it's parsed sucessfully,
        /// and the resulting value is validated by the passed function.
        /// 
        /// # Closures
        /// 
        /// The signiture for this method features three closures,
        /// warrenting explanation as to their purpose:
        /// 
        /// ## `validate`
        /// 
        /// `validate` is applied to the resulting value from parsing the buffer input,
        /// only breaking from reading from the buffer where the predicate returns `true`.
        /// In the case of an invalid input, this sould also be used to notiy users of why it was invalidated.
        /// 
        /// ## `notif`
        /// 
        /// `notif` will be run at the beginning of every loop,
        /// and is intended to allow the developer to push a notification
        /// on the expected usage of the program.
        /// 
        /// ## `err_notif`
        /// 
        /// `err_notif` will be run if there is an error parsing the input,
        /// and is intended to allow the developer to push a notification
        /// on the misinput, and the expected usage of the program.
        /// 
        /// # Examples
        /// 
        /// ```
        /// use std::io;
        /// use my_rusttools::input::TakeInputParse;
        /// 
        /// let proceed: bool = io::stdin().take_input_until_parsed_and_valid(
        ///     |x|x,
        ///     ||println!("Enter 'true'."),
        ///     |err|println!("invalid input: {}", err)
        /// );
        /// 
        /// assert!(proceed);
        /// ```
        fn take_input_until_parsed_and_validated<F, EF, V>(&self, mut validate: V, mut notif: F, mut err_notif: EF) -> T
        where
            F: FnMut(),
            EF: FnMut(E),
            V: FnMut(&T) -> bool, {
                loop {
                    let ret = self.take_input_until_parsed(&mut notif, &mut err_notif);

                    if validate(&ret) {
                        break ret;
                    }
                }
            }
    }

impl<T, E, V> TakeInputParse<T, E> for V
where
    T: FromStr<Err = E>,
    V: TakeInputBasic<T>, {
        fn take_input_then_parse(&self) -> Result<T, E> {
            self.take_input()
                .trim()
                .parse()
        }
    }

#[cfg(test)]
mod input_tests {
    use super::*;
    use std::io;

    #[test]
    fn mapping_test() {
        match io::stdin().take_input_then_map(|mut x| {
            x.make_ascii_lowercase();
            let trimmed = x.trim();

            Ok(if ["y", "yes"].contains(&trimmed) {
                true
            } else if ["n", "no"].contains(&trimmed) {
                false
            } else {
                return Err("input must be \"y(es)\" or \"n(o)\"");
            })
        }) {
            Ok(case) => println!("User's input was: {}", case),
            Err(err) => println!("invalid input: {}", err),
        };
    }

    #[test]
    fn mapping_test_two() {
        let uinp: usize = io::stdin().take_input_until_mapped(|x|
            x.trim()
            .parse()
            .ok(), ||println!("Please enter a posive whole number:"));

        println!("User input: {}", uinp);
    }
}
