//! Custom input handling tools.
use std::{
    io::{self, Read, BufRead},
    ops::{Bound::*, RangeBounds, Deref, DerefMut},
    os::unix::prelude::AsRawFd,
    str::FromStr, 
    process
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
    /// # Exit Behaviour
    /// 
    /// If the line count has exceeded the lower bound, and the user enters an empty line,
    /// the method will return the string.
    /// 
    /// The user can continue entering lines after this, until the previous condition is met,
    /// or the specified upper bound is reached,
    /// after which the method will always return the string.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::io;
    /// use my_rusttools::StdinExtended;
    /// use std::ops::ControlFlow;
    /// 
    /// fn main() -> io::Result<()> {
    ///     StdinExtended::new()
    ///         .read_lines(1..=3, |x|println!("Please enter between 1 and 3 lines.\nCurrent count: {x}"))
    ///         .map(|x|println!("input:\n\n{x}"))
    /// }
    /// ```
    pub fn read_lines<U: RangeBounds<usize>, F>(&self, bounds: U, mut notif: F) -> io::Result<String> where
    F: FnMut(usize), {
        notif(0); // Runs the initial notification, because it normally only runs as part of the iteration process.

        let upper = match bounds.end_bound() {
            Included(&end) => end,
            Excluded(&end) => end.checked_sub(1).unwrap_or_default(),
            Unbounded => usize::MAX,
        };

        self.lock()
            .lines()
            // `Scan` struct used to keep track of the number of lines read, propagating the count forward.
            .scan(0, |lines, io_result|{
                let deref_io_result = io_result.as_deref()
                    .map(str::trim);

                match deref_io_result {
                    Ok(z) if !z.is_empty() => *lines += 1,
                    _ => (),
                }

                match deref_io_result {
                    Ok("") if bounds.contains(lines) => None,
                    _ => Some(io_result.map(|y|(*lines, y))),
                }
            })
            .filter_map(|io_result|{
                match io_result.as_ref() {
                    Ok((y, _)) if y < &upper => notif(*y),
                    _ => (),
                }

                let io_result = io_result.map(|x|x.1);

                match io_result.as_deref().map(str::trim) {
                    Ok("") => None,
                    _ => Some(io_result),
                }
            })
            .take(upper)
            .collect::<Result<Vec<_>, _>>()
            .map(|x|x.join("\n"))
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