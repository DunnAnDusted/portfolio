use std::{
    str::FromStr,
    ops::RangeBounds,
    num::ParseFloatError
};
use super::super::{
    basic_input,
    to_numerical::error::*,
};

/// An interface for handling float input.
/// 
/// # Notifications
/// 
/// Due to the variety of context these functions may be used in,
/// functions enforcing repeating behaviour,
/// such as those that loop until a valid integer is parsed,
/// provide an arguments for closures to specify notification behaviours for these loops.
pub trait TakefloatInput<T>
where
    T: Copy,
    T: PartialOrd,
    T: FromStr<Err = ParseFloatError>, {
        /// Takes input from the specified buffer,
        /// attempting to parse it as a floating point value.
        /// 
        /// # Examples
        /// ```
        /// use std::io;
        /// use my_rusttools::input::TakeFloatInput;
        /// 
        /// println!("Please enter a decimal number:");
        /// 
        /// match io::stdin().take_int() {
        ///     Ok(num) => println!("entered numer: {}", num),
        ///     Err(err) => println!("error: {}", err),
        /// }
        /// ```
        fn take_float(&self) -> Result<T>;

        /// Takes input from a specified buffer,
        /// attempting to parse it as an floating point value,
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
        /// use my_rusttools::input::TakeFloatInput;
        /// 
        /// let num: f64 = io::stdin()
        ///     .take_float_until_parsed(||println!("Please enter a floating point number:"), |err|println!("invalid input: {}", err));
        /// 
        /// assert!(num >= f64::MIN);
        /// ```
        fn take_float_until_parsed<F, E>(&self, mut notif: F, mut err_notif: E) -> T 
        where 
            F: FnMut(), 
            E: FnMut(NumInputError), {
                loop {
                    notif();
            
                    match self.take_float() { 
                        Ok(float) => break float,
                        Err(err) => {
                            err_notif(err);
                        },
                    }
                }
            }

        /// Takes an integer floating point input,
        /// validating it falls within the specified range.
        /// 
        /// # Examples
        /// ```
        /// use std::io;
        /// use my_rusttools::input::TakeFloatInput;
        /// 
        /// println!("Please enter a floating point number from 0 to 100:");
        /// 
        /// match io::stdin().take_float_include_range(&(0..=100)) {
        ///     Ok(num) => println!("entered numer: {}", num),
        ///     Err(err) => println!("error: {}", err),
        /// }
        /// ```
        fn take_float_include_range<U: RangeBounds<T>>(&self, range: &U) -> Result<T>{
            Err(match self.take_float() {
                Ok(float) if range.contains(&float) => return Ok(float),
                Ok(_) => NumInputError{kind: NumInputErrorKind::OutsideValidRange},
                Err(err) => err,
            })
        }

        /// Takes an floating point input,
        /// validating it falls outside the specified range.
        /// 
        /// # Examples
        /// ```
        /// use std::io;
        /// use my_rusttools::input::TakeFloatInput;
        /// 
        /// println!("Please enter a number from 0 to 100:");
        /// 
        /// match io::stdin().take_float_exclude_range(&(0..=100)) {
        ///     Ok(num) => println!("entered numer: {}", num),
        ///     Err(err) => println!("error: {}", err),
        /// }
        /// ```
        fn take_float_exclude_range<U: RangeBounds<T>>(&self, range: &U) -> Result<T> {
            Err(match self.take_float() {
                Ok(float) if range.contains(&float) => NumInputError{kind: NumInputErrorKind::InInvalidRange},
                Ok(float) => return Ok(float),
                Err(err) => err,
            })
        }

        /// Takes an floating point input,
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
        /// use my_rusttools::input::TakeFloatInput;
        /// 
        /// let num: f64 = io::stdin()
        ///     .take_float_include_range_until_valid(&(0..=100), ||println!("Please enter a floating point number from 0 to 100:"), |err|println!("invalid input: {}", err));
        /// 
        /// assert!(num <= 100 && num > 0);
        /// ```
        fn take_float_include_range_until_valid<U: RangeBounds<T>, F, E>(&self, range: &U, mut notif: F, mut err_notif: E) -> T 
        where
            F: FnMut(), 
            E: FnMut(NumInputError), {
                loop {
                    notif();
                    
                    match self.take_float_include_range(range) {
                        Ok(float) => break float,
                        Err(err) => err_notif(err),
                    }
                }
            }

        /// Takes an floating point input,
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
        /// use my_rusttools::input::TakeFloatInput;
        /// 
        /// let num: f64 = io::stdin()
        ///     .take_float_exclude_range_until_valid(&(0..=10), ||println!("Please enter a floating point number less than 0, or greater than 10:"), |err|println!("invalid input: {}", err));
        /// 
        /// assert!(num > 10 || num < 0);
        /// ```
        fn take_float_exclude_range_until_valid<U: RangeBounds<T>, F, E>(&self, range: &U, mut notif: F, mut err_notif: E) -> T 
        where
            F: FnMut(), 
            E: FnMut(NumInputError), {
                loop {
                    notif();
                    
                    match self.take_float_exclude_range(range) {
                        Ok(float) => break float,
                        Err(err) => err_notif(err),
                    }
                }
            }
    }

impl<T, U> TakefloatInput<T> for U
where
    U: basic_input::TakeBasicInput,
    T: Copy,
    T: PartialOrd,
    T: FromStr<Err = ParseFloatError>, {
        fn take_float(&self) -> Result<T> {
            Ok(self.take_string_input()
                .trim()
                .parse()?)
        }
    }
