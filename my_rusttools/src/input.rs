//! Custom input handling tools.
use std::{
    io::{self, Read},
    ops::{Bound::*, RangeBounds, Deref, DerefMut, ControlFlow},
    os::unix::prelude::AsRawFd,
    str::FromStr, process
};

/// A newtype wrapper of [`std::io::Stdin`],
/// to extend it with custom methods.
/// 
/// # Examples
/// 
/// ```
/// use my_rusttools::StdinExtended;
///  
/// let uinp = StdinExtended::new();
/// println!("{:?}", uinp.read_line_new_string());
/// ```
#[derive(Debug)]
pub struct StdinExtended(pub io::Stdin);

impl StdinExtended {
    /// Constructs a new extended version of the handle
    /// to the standard input of the current process.
    /// 
    /// # Examples
    /// 
    /// Using implicit synchronization:
    /// ```
    /// use std::io;
    /// use my_rusttools::StdinExtended;
    /// 
    /// fn main() -> io::Result<()> {
    ///     let uinp = StdinExtended::new();
    ///     println!("{}", uinp.read_line_new_string()?);
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// Using explicit syncronization:
    /// ```
    /// use std::io::{self, BufRead};
    /// use my_rusttools::StdinExtended;
    /// 
    /// fn main() -> io::Result<()> {
    ///     let mut buffer = String::new();
    ///     let uinp = StdinExtended::new();
    ///     let mut handle = uinp.lock();
    /// 
    ///     println!("{}", handle.read_line(&mut buffer)?);
    ///     Ok(())
    /// }
    /// ```
    pub fn new() -> Self {
        Self(io::stdin())
    }

    /// Locks the handle this type wraps and reads a line of input,
    /// appending it to a new buffer.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::io;
    /// use my_rusttools::StdinExtended;
    /// 
    /// fn main() -> io::Result<()> {
    ///     let uinp = StdinExtended::new();
    ///     println!("{}", uinp.read_line_new_string()?);
    ///     Ok(())
    /// }
    /// ```
    pub fn read_line_new_string(&self) -> io::Result<String> {
        let mut ret = String::new();

        self.read_line(&mut ret).map(|_|ret)
    }

    /// Repeatedly locks the handle this type warps,
    /// reading a number of lines within the range specified,
    /// to a new buffer.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::io;
    /// use my_rusttools::StdinExtended;
    /// use std::ops::ControlFlow;
    /// 
    /// fn main() -> io::Result<()> {
    ///     let uinp = StdinExtended::new();
    ///     let input = uinp.read_lines(1..=3,
    ///         |curr|println!("Please enter between 1 and 3 lines.\nCurrent count: {}", curr.lines().count()),
    ///         |err, curr|{
    ///             eprintln!("input error: {}\nerror at {}", err, curr);
    ///             ControlFlow::Break(())
    ///         }
    ///     )?;
    /// 
    ///     println!("{}", input);
    ///     Ok(())
    /// }
    /// ```
    pub fn read_lines<U: RangeBounds<usize>, F, EF>(&self, bounds: U, mut notif: F, mut err_notif: EF) -> io::Result<String> where
    F: FnMut(&str),
    EF: FnMut(&io::Error, &str) -> ControlFlow<()> {
        let mut ret = String::new();
        let mut line_count = 0;

        let start = *match bounds.start_bound() {
            Included(start) => start,
            Excluded(start) => start,
            Unbounded => &0,
        };

        let end = match bounds.end_bound() {
            Included(end) => *end,
            Excluded(end) => end -1,
            Unbounded => usize::MAX,
        };

        loop {
            if line_count >= end || line_count == usize::MAX {
                break Ok(ret);
            }

            notif(ret.as_str());

            if let Err(err) = self.read_line(&mut ret) {
                if let ControlFlow::Break(()) = err_notif(&err, ret.as_str()) {
                    break Err(err);
                }
            }

            let new_line_count = ret.trim().lines().filter(|x|!x.is_empty()).count();

            if new_line_count - line_count < 1 && new_line_count.checked_sub(start).is_some() {
                break Ok(ret);
            } else {
                ret = ret.lines()
                    .filter(|x|!x.is_empty())
                    .fold(String::new(), |acc, x|acc + x + "\n");
            }

            line_count = new_line_count;
        }
    }
}

impl Deref for StdinExtended {
    type Target = io::Stdin;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StdinExtended {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Read for StdinExtended {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl AsRawFd for StdinExtended {
    fn as_raw_fd(&self) -> std::os::unix::prelude::RawFd {
        self.0.as_raw_fd()
    }
}

impl Default for StdinExtended {
    fn default() -> Self {
        Self::new()
    }
}

/// A newtype wrapper for [`StdinExtended`],
/// to extend it with parsing behaviour,
/// with the assumption a process should exit upon an IO error.
#[derive(Debug)]
pub struct ParseStdinExtended(pub StdinExtended);

impl ParseStdinExtended {
    /// Constructs a new parsing enabled version of `StdinExtended`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use my_rusttools::ParseStdinExtended;
    /// 
    /// let uinp = ParseStdinExtended::new();
    /// println!("{:?}", uinp.read_line_parse::<usize>>());
    /// ```
    pub fn new() -> ParseStdinExtended {
        ParseStdinExtended(StdinExtended::new())
    }

    /// Locks the handle of this type,
    /// attempting to parse the line of input it reads.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use my_rusttools::ParseStdinExtended;
    /// 
    /// let uinp = ParseStdinExtended::new();
    /// 
    /// match uinp.read_line_parse() {
    ///     Ok(num @ 0usize..=10) => println!("{num} is a pretty small number..."),
    ///     Ok(num) => println!("{num} that isn't so small!"),
    ///     Err(_) => eprintln!("That's not a number..."),
    /// }
    /// ```
    pub fn read_line_parse<T: FromStr>(&self) -> Result<T, T::Err> {
        self.read_line_new_string()
            .map_or_else(
                |err|{
                    eprintln!("input error: {}", err);
                    process::exit(1);
            }, 
            |x|x.trim().parse()
        )
    }

    /// Repeatedly locks the handle of this type,
    /// until the line of input it reads is parsed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use my_rusttools::ParseStdinExtended;
    /// 
    /// let uinp: usize = ParseStdinExtended::new()
    ///     .read_line_until_parsed(
    ///         ||println!("Please input a positive number!"),
    ///         |err|eprintln!("invalid input: {err}")
    ///     );
    /// 
    /// match uinp {
    ///     0..=10 => println!("{uinp} is a pretty small number"),
    ///     _ => println!("{uinp} isn't so small!"),
    /// }
    /// ```
    pub fn read_line_until_parsed<T, F, E>(&self, mut notif: F, mut err_notif: E) -> T where
    T: FromStr,
    F: FnMut(),
    E: FnMut(T::Err), {
        loop {
            notif();

            match self.read_line_parse() {
                Ok(parsed) => return parsed,
                Err(err) => err_notif(err),
            }
        }
    }

    /// Repeatedly locks the handle of this type,
    /// until the return value from the passed closure of a [`Some`] enum.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use my_rusttools::ParseStdinExtended;
    /// 
    /// let uinp = ParseStdinExtended::new()
    ///     .read_line_until_mapped(
    ///         |x|match x.to_lowercase().trim() {
    ///                 "y" | "yes" => Some(true),
    ///                 "n" | "no" => Some(false),
    ///                 _ => None,
    ///         },
    ///         ||println!("Please enter y(es)/n(o),")
    ///     );
    /// 
    /// println!("{uinp}");
    /// ```
    pub fn read_line_until_mapped<T, F, G>(&self, mut f: F, mut notif: G) -> T where
    F: FnMut(String) -> Option<T>,
    G: FnMut(), {
        loop {
            notif();

            let uinp = self.read_line_new_string()
                .map_or_else(|err|{
                        eprintln!("input error: {}", err);
                        process::exit(1);
                    }, &mut f);

            if let Some(ret) = uinp {
                return ret;
            }
        }
    }
}

impl Deref for ParseStdinExtended {
    type Target = StdinExtended;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ParseStdinExtended {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Read for ParseStdinExtended {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl AsRawFd for ParseStdinExtended {
    fn as_raw_fd(&self) -> std::os::unix::prelude::RawFd {
        self.0.as_raw_fd()
    }
}

impl Default for ParseStdinExtended {
    fn default() -> Self {
        Self::new()
    }
}