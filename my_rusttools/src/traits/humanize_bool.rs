//! A small convinience implementation,
//! with methods for conveting booleans into
//! string referances.

/// A interface, intended to provide convenience methods,
/// for conveting boolean values into related string terms.
pub trait HumanizeBooleans {
    /// Converts a boolean to a string of
    /// `Yes` or `No`.
    /// 
    /// # Examples
    /// ```
    /// use my_rusttools::traits::HumanizeBooleans;
    /// 
    /// assert_eq!("Yes", true.yes_no());
    /// assert_eq!("No", false.yes_no());
    /// ```
    fn yes_no(self) -> &'static str;

    /// Converts a boolean to a
    /// `y` of `n` character.
    /// 
    /// # Examples
    /// ```
    /// use my_rusttools::traits::HumanizeBooleans;
    /// 
    /// assert_eq!('y', true.yes_no_short());
    /// assert_eq!('n', false.yes_no_short());
    /// ```
    fn yes_no_short(self) -> char;

    /// Converts a boolean to a string
    /// of `On` or `Off`.
    /// 
    /// # Examples
    /// ```
    /// use my_rusttools::traits::HumanizeBooleans;
    /// 
    /// assert_eq!("On", true.on_off());
    /// assert_eq!("Off", false.on_off());
    /// ```
    fn on_off(self) -> &'static str;
}

impl HumanizeBooleans for bool {
    #[inline]
    fn yes_no(self) -> &'static str {
        if self {
            "Yes"
        } else {
            "No"
        }
    }

    #[inline]
    fn yes_no_short(self) -> char {
        if self {
            'y'
        } else {
            'n'
        }
    }

    #[inline]
    fn on_off(self) -> &'static str {
        if self {
            "On"
        } else {
            "Off"
        }
    }
}