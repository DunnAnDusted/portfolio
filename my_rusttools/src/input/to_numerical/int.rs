use std::{
    str::FromStr,
    ops::RangeBounds,
    num::ParseIntError
};
use super::super::{
    basic_input,
    to_numerical::error::*,
};

pub trait TakeintInput<T, U, F, E>
where
    T: Copy,
    T: PartialOrd,
    T: FromStr<Err = ParseIntError>,
    U: RangeBounds<T>,
    F: FnMut(), 
    E: FnMut(InputInvalidError), {
        fn take_int(&self) -> Result<T>;

        fn take_int_until_parsed(&self, mut notif: F, mut err_notif: E) -> T {
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

        fn take_int_include_range(&self, range: &U) -> Result<T>{
            Err(match self.take_int() {
                Ok(int) if range.contains(&int) => return Ok(int),
                Ok(_) => InputInvalidError{kind: InputErrorKind::OutsideValidRange},
                Err(err) => err,
            })
        }

        fn take_int_exclude_range(&self, range: &U) -> Result<T> {
            Err(match self.take_int() {
                Ok(int) if range.contains(&int) => InputInvalidError{kind: InputErrorKind::InInvalidRange},
                Ok(int) => return Ok(int),
                Err(err) => err,
            })
        }

        fn take_int_include_range_until_valid(&self, range: &U, mut notif: F, mut err_notif: E) -> T {
            loop {
                notif();
                
                match self.take_int_include_range(range) {
                    Ok(int) => break int,
                    Err(err) => err_notif(err),
                }
            }
        }

        fn take_int_exclude_range_until_valid(&self, range: &U, mut notif: F, mut err_notif: E) -> T {
            loop {
                notif();
                
                match self.take_int_exclude_range(range) {
                    Ok(int) => break int,
                    Err(err) => err_notif(err),
                }
            }
        }
    }

impl<T, U, F, E, V> TakeintInput<T, U, F, E> for V
where
    V: basic_input::TakeBasicInput,
    T: Copy,
    T: PartialOrd,
    T: FromStr<Err = ParseIntError>,
    U: RangeBounds<T>,
    F: FnMut(), 
    E: FnMut(InputInvalidError), {
        fn take_int(&self) -> Result<T> {
            Ok(self.take_string_input()
                .trim()
                .parse()?)
        }
    }
