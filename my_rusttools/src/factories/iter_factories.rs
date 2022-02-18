use std::ops::RangeBounds;

/// Creates an iterator which returns all the primes,
/// less than or equal to `upper_bound`.
/// 
/// # Examples
/// ```
/// # use my_rusttools::factories::sieve_primes;
/// #
/// let primes = sieve_primes(10);
/// 
/// assert!(primes.eq(vec![2, 3, 5, 7]));
/// ```
pub fn sieve_primes(upper_bound: usize) -> impl Iterator<Item = usize> {
    match upper_bound {
        0 | 1 => Vec::new(), // Escapes the 0 and 1 cases early.
        x => (3usize..).step_by(2) // Only steps over odd values, even value inherrantly not being prime.
            .take_while(|i|i * i <= x)
            .fold(vec![true; x + 1],|mut acc, i|{
                acc.iter_mut()
                    .skip(i * i) // The given index may be a prime number, so is skipped.
                    .step_by(i) // Steps through the multiple of the index.
                    .for_each(|x|*x = false); // Marking each multiple as not prime.
                acc // Returns `Vec` of prime markers.
            }),
    }.into_iter()
        .enumerate()
        .filter_map(|x|match x {
            (0 | 1, _) => None, // Discards indexes `0` and `1`, due to being ruled against being primes.
            (2, _) => Some(2), // Explicitly includes the index `2`, due to being the only even prime.
            (x, _) if x % 2 == 0 => None,
            (x, y) if y => Some(x), // Includes indexes which are still marked as primes.
            _ => None,
        })
}

/// Creates an iterator which returns values
/// from the specified range, in the specified steps.
/// 
/// # Panics
/// 
/// Will panic if the given step is `0`.
/// 
/// # Examples
/// ```
/// # use my_rusttools::factories::range_with_step;
/// #
/// let range = range_with_step(0..=10, 2);
/// let nums = vec![0, 2, 4, 6, 8, 10];
///
/// assert!(range.eq(nums));
/// ```
pub fn range_with_step<T, U>(range: U, step: usize) -> impl Iterator<Item = U::Item>
where
    T: ?Sized,
    U: RangeBounds<T> + Iterator {
        range.step_by(step)
    }

/// Creates an iterator which returns
/// the fizzbuzz sequence.
/// 
/// # Overflow Behaviour
/// 
/// The function does not guard against overflows,
/// overflow in the [`Iterator`] implementation (when the contained
/// data type reaches its numerical limit) is allowed to panic, wrap, or
/// saturate. This behavior is defined by the implementation of the [`Step`]
/// trait. For primitive integers, this follows the normal rules, and respects
/// the overflow checks profile (panic in debug, wrap in release),
/// so iterating more than [`usize::MAX`] elements,
/// either produces the wrong result, or panics.
/// If debug assertions are enabled, a panic is guaranteed.
/// 
/// Note also that overflow happens earlier than you might assume: the overflow happens
/// in the call to `next` that yields the maximum value, as the range must be
/// set to a state to yield the next value.
/// 
/// [`Step`]: std::iter::Step
/// 
/// # Examples
/// ```
/// # use my_rusttools::factories::fizzbuzz;
/// #
/// assert_eq!(Some("FizzBuzz".to_string()), fizzbuzz().nth(14));
/// ```
pub fn fizzbuzz() -> impl Iterator<Item = String> {
    // Sets up cycling iterators, with `Fizz` and `Buzz` values at the correct intervals,
    // then zips them into a single iterator.
    let fizzy = ["", "", "Fizz"].into_iter().cycle();
    let buzzy = ["", "", "", "", "Buzz"].into_iter().cycle();
    let fizzbuzz = fizzy.zip(buzzy);

    // Zips the cycling sequence into a `RangeFrom`,
    // due to needing to begin indexing at `1`.
    (1usize..).zip(fizzbuzz)
        .map(|(i, x)|
            match x {
                ("", "") => i.to_string(), // Matches for values where the index isn't devisible by `3` or `5`.
                (x, y) => x.to_owned() + y
            }
        )
}