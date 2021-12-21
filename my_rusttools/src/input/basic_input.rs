//! An interface for taking basic input as strings.
use std::{
    io::Stdin,
    ops::{
        RangeBounds,
        Bound::*,
    },
};

/// An interface for taking basic input as strings.
pub trait TakeBasicInput {
    /// Takes a line of input as a `String`.
    /// 
    /// # Examples
    /// ```
    /// use std::io;
    /// use my_rusttools::input::TakeBasicInput;
    /// 
    /// let a = io::stdin().take_string_input();
    /// println!("User input: {}", a);
    /// ```
    fn take_string_input(&self) -> String;

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
    /// # Examples
    /// ```
    /// use std::io;
    /// use my_rusttools::input::TakeBasicInput;
    /// 
    /// let a = io::stdin().take_lines_input(3..10);
    /// assert!(a.lines().count() > 2);
    /// ```
    fn take_lines_input<T: RangeBounds<usize>>(&self, bounds: T) -> String;
}

impl TakeBasicInput for Stdin {
    fn take_string_input(&self) -> String {
        let mut ret = String::new();

        self.read_line(&mut ret)
            .expect("failed to read input");

        ret
    }

    fn take_lines_input<T: RangeBounds<usize>>(&self, bounds: T) -> String {
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
            if iterations >= end || iterations >= usize::MAX || ret.len() >= usize::MAX {
                break;
            }

            let size = self.read_line(&mut ret)
                .expect("failed to read input");

            if size > 2 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    #[ignore]
    fn take_lines() {
        let a = io::stdin().take_lines_input(3..10);
        assert!(a.lines().count() > 2);
    }
}