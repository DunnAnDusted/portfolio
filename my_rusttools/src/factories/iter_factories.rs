use std::ops::RangeBounds;

/// Creates an iterator which returns all the primes,
/// less than or equal to `upper_bound`.
/// 
/// # Examples
/// ```
/// # use my_rusttools::factories::sieve_primes;
/// #
/// let mut primes = sieve_primes(10);
/// 
/// assert!(primes.eq(vec![2, 3, 5, 7]));
/// ```
pub fn sieve_primes(upper_bound: usize) -> impl Iterator<Item = usize> {
    let mut ret = vec![true; upper_bound + 1];
    
    for i in range_with_step(3..=usize::MAX, 2).take_while(|x|x * x <= upper_bound) {
        for j in range_with_step(i * i..=upper_bound, i) {
            ret[j] = false;
        }
    }

    ret.into_iter()
        .enumerate()
        .skip(2)
        .filter(|(x, y)|x % 2 != 0 && *y || *x == 2)
        .map(|x|x.0)
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

    // Zips the cycling sequence into a `Range`, indicating the current iteration,
    // appending and returning the values from the cycled sequence
    // or returning the current index if values were empty.
    (1..=usize::MAX).zip(fizzbuzz)
        .map(|(i, (x, y))| {
            let ret = x.to_owned() + y;

            if ret.is_empty() {
                i.to_string()
            } else {
                ret
            }
        })
}