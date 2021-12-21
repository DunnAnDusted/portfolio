use std::{
    fmt,
    num::ParseIntError,
    num::{ParseFloatError, IntErrorKind},
    result,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputInvalidError {
    pub(super) kind: InputErrorKind
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputErrorKind {
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

/// Outputs the detailed cause of why the buffer input was invalidated.
impl InputInvalidError {
    pub fn kind(&self) -> &InputErrorKind {
        &self.kind
    }
        
}
        
impl fmt::Display for InputInvalidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            InputErrorKind::OutsideValidRange => "parsed input invalidated, not found within included range",
            InputErrorKind::InInvalidRange => "parsed input invalidated, found within excluded range",
            InputErrorKind::Empty => "cannot parse numerical value from empty string",
            InputErrorKind::InvalidDigit => "invalid digit found in string",
            InputErrorKind::PosOverflow => "number too large to fit in target type",
            InputErrorKind::NegOverflow => "number too small to fit in target type",
            InputErrorKind::Zero => "number would be zero for non-zero type",
        }.fmt(f)
    }
}

impl From<ParseIntError> for InputInvalidError {
    fn from(err: ParseIntError) -> InputInvalidError {
        Self { 
            kind: match err.kind() {
                IntErrorKind::Empty => InputErrorKind::Empty,
                IntErrorKind::InvalidDigit => InputErrorKind::InvalidDigit,
                IntErrorKind::PosOverflow => InputErrorKind::PosOverflow,
                IntErrorKind::NegOverflow => InputErrorKind::NegOverflow,
                IntErrorKind::Zero => InputErrorKind::Zero,
                &_ => panic!("unaccounted for error: {}", err),
            }
        }
    }
}

impl From<ParseFloatError> for InputInvalidError {
    fn from(err: ParseFloatError) -> InputInvalidError {
        Self {
            kind: match err.to_string().as_str() {
                "cannot parse float from empty string" => InputErrorKind::Empty,
                "invalid float literal" => InputErrorKind::InvalidDigit,
                &_ => panic!("unaccounted for error: {}", err),
            }
        }
    }
}

/// A custom shaddowing of `std::result::Result`, 
/// enforcing the error type of return values.
pub type Result<T> = result::Result<T, InputInvalidError>;