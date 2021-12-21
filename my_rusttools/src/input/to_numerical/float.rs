use std::{
    str::FromStr,
    ops::RangeBounds,
    num::ParseFloatError
};
use super::super::{
    basic_input,
    to_numerical::error::*,
};

pub trait TakefloatInput<T, U, F, E>
where
    T: Copy,
    T: PartialOrd,
    T: FromStr<Err = ParseFloatError>,
    U: RangeBounds<T>,
    F: FnMut(), 
    E: FnMut(InputInvalidError), {
        fn take_float(&self) -> Result<T>;

        fn take_float_until_parsed(&self, mut notif: F, mut err_notif: E) -> T {
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

        fn take_float_include_range(&self, range: &U) -> Result<T>{
            Err(match self.take_float() {
                Ok(float) if range.contains(&float) => return Ok(float),
                Ok(_) => InputInvalidError{kind: InputErrorKind::OutsideValidRange},
                Err(err) => err,
            })
        }

        fn take_float_exclude_range(&self, range: &U) -> Result<T> {
            Err(match self.take_float() {
                Ok(float) if range.contains(&float) => InputInvalidError{kind: InputErrorKind::InInvalidRange},
                Ok(float) => return Ok(float),
                Err(err) => err,
            })
        }

        fn take_float_include_range_until_valid(&self, range: &U, mut notif: F, mut err_notif: E) -> T {
            loop {
                notif();
                
                match self.take_float_include_range(range) {
                    Ok(float) => break float,
                    Err(err) => err_notif(err),
                }
            }
        }

        fn take_float_exclude_range_until_valid(&self, range: &U, mut notif: F, mut err_notif: E) -> T {
            loop {
                notif();
                
                match self.take_float_exclude_range(range) {
                    Ok(float) => break float,
                    Err(err) => err_notif(err),
                }
            }
        }
    }

impl<T, U, F, E, V> TakefloatInput<T, U, F, E> for V
where
    V: basic_input::TakeBasicInput,
    T: Copy,
    T: PartialOrd,
    T: FromStr<Err = ParseFloatError>,
    U: RangeBounds<T>,
    F: FnMut(), 
    E: FnMut(InputInvalidError), {
        fn take_float(&self) -> Result<T> {
            Ok(self.take_string_input()
                .trim()
                .parse()?)
        }
    }
