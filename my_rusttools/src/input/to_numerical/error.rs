use std::{
    fmt,
    num::ParseIntError,
    num::{ParseFloatError, IntErrorKind},
    result,
};

/// An error which can be returned
/// when taking numerical input from a buffer.
/// 
/// This error is used as the error type
/// used for numerical input handling interfaces,
/// in the [`my_rusttools::input`] module.
/// 
/// [`my_rusttools::input`]: super::super
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumInputError {
    pub(super) kind: NumInputErrorKind
}

/// An enum to indicate the various types of errors
/// that can invalidate numerical input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumInputErrorKind {
    /// Value parsed falls outside the valid range,
    /// specified in the function parameter call.
    OutsideValidRange,
    /// Value parsed falls inside the invalid range,
    /// specified in the function parameter call.
    InInvalidRange,
    /// Value being parsed is empty.
    ///
    /// Among other causes, this variant will be constructed when parsing an empty string.
    Empty,
    /// Contains an invalid digit in its context.
    ///
    /// Among other causes, this variant will be constructed when parsing a string that
    /// contains a non-ASCII char.
    ///
    /// This variant is also constructed when a `+` or `-` is misplaced within a string
    /// either on its own or in the middle of a number.
    InvalidDigit,
    /// Integer is too large to store in target integer type.
    PosOverflow,
    /// Integer is too small to store in target integer type.
    NegOverflow,
    /// Value was Zero
    ///
    /// This variant will be emitted when the parsing string has a value of zero, which
    /// would be illegal for non-zero types.
    Zero,
}

impl NumInputError {
    /// Outputs the detailed cause of why the input was invalidated.
    pub fn kind(&self) -> &NumInputErrorKind {
        &self.kind
    }
        
}
        
impl fmt::Display for NumInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            NumInputErrorKind::OutsideValidRange => "parsed input invalidated, not found within included range",
            NumInputErrorKind::InInvalidRange => "parsed input invalidated, found within excluded range",
            NumInputErrorKind::Empty => "cannot parse numerical value from empty string",
            NumInputErrorKind::InvalidDigit => "invalid digit found in string",
            NumInputErrorKind::PosOverflow => "number too large to fit in target type",
            NumInputErrorKind::NegOverflow => "number too small to fit in target type",
            NumInputErrorKind::Zero => "number would be zero for non-zero type",
        }.fmt(f)
    }
}

impl From<ParseIntError> for NumInputError {
    fn from(err: ParseIntError) -> NumInputError {
        Self { 
            kind: match err.kind() {
                IntErrorKind::Empty => NumInputErrorKind::Empty,
                IntErrorKind::InvalidDigit => NumInputErrorKind::InvalidDigit,
                IntErrorKind::PosOverflow => NumInputErrorKind::PosOverflow,
                IntErrorKind::NegOverflow => NumInputErrorKind::NegOverflow,
                IntErrorKind::Zero => NumInputErrorKind::Zero,
                &_ => panic!("unaccounted for error: {}", err),
            }
        }
    }
}

impl From<ParseFloatError> for NumInputError {
    fn from(err: ParseFloatError) -> NumInputError {
        Self {
            kind: match err.to_string().as_str() {
                "cannot parse float from empty string" => NumInputErrorKind::Empty,
                "invalid float literal" => NumInputErrorKind::InvalidDigit,
                &_ => panic!("unaccounted for error: {}", err),
            }
        }
    }
}

/// A custom shaddowing of `std::result::Result`, 
/// enforcing the error type of return values.
pub type Result<T> = result::Result<T, NumInputError>;