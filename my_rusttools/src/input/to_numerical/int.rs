use std::{
    str::FromStr,
    ops::RangeBounds,
    num::ParseIntError
};
use super::super::{
    basic_input,
    to_numerical::error::*,
};

/// An interface for handling integer input.
/// 
/// # Notifications
/// 
/// Due to the variety of context these functions may be used in,
/// functions enforcing repeating behaviour,
/// such as those that loop until a valid integer is parsed,
/// provide an arguments for closures to specify notification behaviours for these loops.
pub trait TakeIntInput<T>
where
    T: Copy,
    T: PartialOrd,
    T: FromStr<Err = ParseIntError>, {
        /// Takes input from the specified buffer,
        /// attempting to parse it as an integer value.
        /// 
        /// # Examples
        /// ```
        /// use std::io;
        /// use my_rusttools::input::TakeIntInput;
        /// 
        /// println!("Please enter a number:");
        /// 
        /// match io::stdin().take_int() {
        ///     Ok(num) => println!("entered numer: {}", num),
        ///     Err(err) => println!("error: {}", err),
        /// }
        /// ```
        fn take_int(&self) -> Result<T>;

        /// Takes input from a specified buffer,
        /// attempting to parse it as an integer,
        /// repeatedly until a valid value is parsed.
        /// 
        /// # Notifications
        /// 
        /// `notif` allows for a process that will be executed
        /// on each repetition of the loop in the function,
        /// to allow feedback to the user, for example.
        /// 
        /// `err_notif` is similar, but specifies how to handle input errors.
        /// 
        /// # Examples
        /// ```
        /// use std::io;
        /// use my_rusttools::input::TakeIntInput;
        /// 
        /// let num: usize = io::stdin()
        ///     .take_int_until_parsed(||println!("Please enter a positive number:"), |err|println!("invalid input: {}", err));
        /// 
        /// assert!(num >= 0);
        /// ```
        fn take_int_until_parsed<F, E>(&self, mut notif: F, mut err_notif: E) -> T 
        where
            F: FnMut(), 
            E: FnMut(NumInputError), {
                loop {
                    notif();
            
                    match self.take_int() { 
                        Ok(int) => break int,
                        Err(err) => {
                            err_notif(err);
                        },
                    }
                }
            }

        /// Takes an integer input,
        /// validating it falls within the specified range.
        /// 
        /// # Examples
        /// ```
        /// use std::io;
        /// use my_rusttools::input::TakeIntInput;
        /// 
        /// println!("Please enter a number from 0 to 100:");
        /// 
        /// match io::stdin().take_int_include_range(&(0..=100)) {
        ///     Ok(num) => println!("entered numer: {}", num),
        ///     Err(err) => println!("error: {}", err),
        /// }
        /// ```
        fn take_int_include_range<U: RangeBounds<T>>(&self, range: &U) -> Result<T>{
            Err(match self.take_int() {
                Ok(int) if range.contains(&int) => return Ok(int),
                Ok(_) => NumInputError{kind: NumInputErrorKind::OutsideValidRange},
                Err(err) => err,
            })
        }

        /// Takes an integer input,
        /// validating it falls outside the specified range.
        /// 
        /// # Examples
        /// ```
        /// use std::io;
        /// use my_rusttools::input::TakeIntInput;
        /// 
        /// println!("Please enter a number greater than 10:");
        /// 
        /// match io::stdin().take_int_exclude_range(&(..=10)) {
        ///     Ok(num) => println!("entered numer: {}", num),
        ///     Err(err) => println!("error: {}", err),
        /// }
        /// ```
        fn take_int_exclude_range<U: RangeBounds<T>>(&self, range: &U) -> Result<T> {
            Err(match self.take_int() {
                Ok(int) if range.contains(&int) => NumInputError{kind: NumInputErrorKind::InInvalidRange},
                Ok(int) => return Ok(int),
                Err(err) => err,
            })
        }

        /// Takes an integer input,
        /// until a valid value is parsed,
        /// and falls within the specified range.
        /// 
        /// # Notifications
        /// 
        /// `notif` allows for a process that will be executed
        /// on each repetition of the loop in the function,
        /// to allow feedback to the user, for example.
        /// 
        /// `err_notif` is similar, but specifies how to handle input errors.
        /// 
        /// # Examples
        /// ```
        /// use std::io;
        /// use my_rusttools::input::TakeIntInput;
        /// 
        /// let num: usize = io::stdin()
        ///     .take_int_include_range_until_valid(&(..=100), ||println!("Please enter a positive number up to 100:"), |err|println!("invalid input: {}", err));
        /// 
        /// assert!(num <= 100);
        /// ```
        fn take_int_include_range_until_valid<U: RangeBounds<T>, F, E>(&self, range: &U, mut notif: F, mut err_notif: E) -> T 
        where
            F: FnMut(), 
            E: FnMut(NumInputError), {
                loop {
                    notif();
                    
                    match self.take_int_include_range(range) {
                        Ok(int) => break int,
                        Err(err) => err_notif(err),
                    }
                }
            }

        /// Takes an integer input,
        /// until a valid value is parsed,
        /// and falls outside the specified range.
        /// 
        /// # Notifications
        /// 
        /// `notif` allows for a process that will be executed
        /// on each repetition of the loop in the function,
        /// to allow feedback to the user, for example.
        /// 
        /// `err_notif` is similar, but specifies how to handle input errors.
        /// 
        /// # Examples
        /// ```
        /// use std::io;
        /// use my_rusttools::input::TakeIntInput;
        /// 
        /// let num: usize = io::stdin()
        ///     .take_int_exclude_range_until_valid(&(..=10), ||println!("Please enter a number greater than 10:"), |err|println!("invalid input: {}", err));
        /// 
        /// assert!(num > 10);
        /// ```
        fn take_int_exclude_range_until_valid<U: RangeBounds<T>, F, E>(&self, range: &U, mut notif: F, mut err_notif: E) -> T 
        where
            F: FnMut(), 
            E: FnMut(NumInputError), {
                loop {
                    notif();
                    
                    match self.take_int_exclude_range(range) {
                        Ok(int) => break int,
                        Err(err) => err_notif(err),
                    }
                }
            }
    }

impl<T, U> TakeIntInput<T> for U
where
    U: basic_input::TakeBasicInput,
    T: Copy,
    T: PartialOrd,
    T: FromStr<Err = ParseIntError>, {
        fn take_int(&self) -> Result<T> {
            Ok(self.take_string_input()
                .trim()
                .parse()?)
        }
    }
