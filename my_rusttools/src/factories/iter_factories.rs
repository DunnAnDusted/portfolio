use std::{
    ops::{RangeBounds, RangeFrom},
    iter::{
        self, 
        FilterMap,
        Enumerate,
        Map,
        Zip,
        Cycle,
        Chain,
        Take,
        Repeat,
        Once,
        RepeatWith,
        OnceWith,
        FlatMap
    }, 
    vec::IntoIter,
};

/// A specialised iterator type for returning prime numbers.
/// 
/// This typedef is used to give the return of [`sieve_primes`],
/// a concrete return type, allowing the usage of methods defined on the aliased type,
/// without needing to list every trait in the function signiture.
/// 
/// # Examples
/// 
/// ```
/// use my_rusttools::factories::sieve_primes;
/// 
/// let primes = sieve_primes(10);
/// 
/// assert!(primes.eq(vec![2, 3, 5, 7]));
/// ```
pub type SievePrimes<F> = FilterMap<Enumerate<IntoIter<bool>>, F>;

/// A specialised iterator type for returning the **FizzBuzz** sequence.
/// 
/// This typedef is used to give the return of [`fizzbuzz`],
/// a concrete return type, allowing the usage of methods defined on the aliased type,
/// without needing to list every trait in the function signiture.
/// 
/// # Examples
/// 
/// ```
/// use my_rusttools::factories::fizzbuzz;
/// 
/// assert_eq!(Some("FizzBuzz".to_string()), fizzbuzz().nth(14));
/// ```
pub type FizzBuzz<'a, F> = Map<Zip<RangeFrom<usize>, Zip<RepeatInterval<&'a str>, RepeatInterval<&'a str>>>, F>;

/// A specialised iterator type for cycling a distinct value into a sequence,
/// at a regular interval.
/// 
/// This typedef is used to give the return of [`repeat_interval`],
/// a concrete return type, allowing the usage of methods defined on the aliased type,
/// without needing to list every trait in the function signiture.
/// 
/// # Examples
/// 
/// ```
/// use my_rusttools::factories::repeat_interval;
/// 
/// let mut fizzy = repeat_interval("Fizz", 3);
/// 
/// // First two values are empty string slices.
/// assert_eq!(fizzy.next(), Some(""));
/// assert_eq!(fizzy.next(), Some(""));
/// 
/// // The third value is now `"Fizz"`.
/// assert_eq!(fizzy.next(), Some("Fizz"));
/// 
/// // The fourth and fifth values are empty string slices again.
/// assert_eq!(fizzy.next(), Some(""));
/// assert_eq!(fizzy.next(), Some(""));
/// ```
pub type RepeatInterval<T> = Cycle<Chain<Take<Repeat<T>>, Once<T>>>;

/// A specialised iterator type for cycling the result of a closure into a sequence,
/// at a regular interval.
/// 
/// This typedef is used to give the return of [`repeat_interval_with`],
/// a concrete return type, allowing the usage of methods defined on the aliased type,
/// without needing to list every trait in the function signiture.
/// 
/// # Examples
/// 
/// ```
/// use my_rusttools::factories::repeat_interval_with;
/// 
/// let mut fizzy = repeat_interval_with(||String::from("Fizz"), 3);
/// 
/// // First two values are empty strings.
/// assert_eq!(fizzy.next(), Some("".to_owned()));
/// assert_eq!(fizzy.next(), Some("".to_owned()));
/// 
/// // The third value is now `"A"`.
/// assert_eq!(fizzy.next(), Some("Fizz".to_owned()));
/// 
/// // The fourth and fifth values are empty strings again.
/// assert_eq!(fizzy.next(), Some("".to_owned()));
/// assert_eq!(fizzy.next(), Some("".to_owned()));
/// ```
pub type RepeatIntervalWith<D, F> = Cycle<Chain<Take<RepeatWith<D>>, OnceWith<F>>>;

/// A specialised iterator type for repeating provided value,
/// a specified number of times.
/// 
/// This typedef is used to give the return of [`repeat_values`],
/// a concrete return type, allowing the usage of methods defined on the aliased type,
/// without needing to list every trait in the function signiture.
/// 
/// # Examples
/// 
/// ```
/// use my_rusttools::factories::repeat_values;
/// 
/// let foobar_1 = repeat_values(&[("Foo", 2), ("Bar", 3)]);
/// let foobar_2 = ["Foo", "Foo", "Bar", "Bar", "Bar"];
/// 
/// asser
pub type RepeatValues<T, F> = FlatMap<IntoIter<(T, usize)>, Take<Repeat<T>>, F>;

/// Creates an iterator which returns all the primes,
/// less than or equal to `upper_bound`.
/// 
/// # Examples
/// 
/// ```
/// # use my_rusttools::factories::sieve_primes;
/// #
/// let primes = sieve_primes(10);
/// 
/// assert!(primes.eq(vec![2, 3, 5, 7]));
/// ```
pub fn sieve_primes(upper_bound: usize) -> SievePrimes<impl FnMut((usize, bool)) -> Option<usize>> {
    let mut primes_markers = vec![true; upper_bound.checked_add(1).unwrap_or(upper_bound)];
    
    primes_markers.iter_mut()
        .take(2)
        .for_each(|x|*x = false); // Unmarks indexes 0 and 1, which are excluded from the definition of primes.

    // 9 is the first index which is a multiple of an odd number, 
    // meaning bounds any lower, don't need to be processed.
    if upper_bound > 8 {
        let validate_multiple_bound = |i: &usize|i <= &upper_bound;

        let next_sucessor = |(i, _): &(usize, _)|{
            let next = i + 2;

            next.checked_pow(2) // The indexes power will overflow first, meaning the next index can be calculated safely.
                .filter(&validate_multiple_bound) // Otherwise, it will otherwise exceed the specified upper bound.
                .map(|x|(next, x))
        };
            
        // Only steps over odd indexes, starting at 3, even values being inherrantly divisible by 2.
        iter::successors(Some((3, 9)), next_sucessor)
            .for_each(|(i, x)|
                // Only needs to process index sequences which aren't marked yet.
                if primes_markers[i] {
                    primes_markers.iter_mut()
                        .skip(x)
                        .step_by(i)
                        .for_each(|y|*y = false);
                }
            );
    }
        
    primes_markers.into_iter()
        .enumerate()
        .filter_map(|x|match x {
            // Explicitly excluding indexes marked as `false`, excludes the single largest group of indexes, odd or even.
            // 7*2 == 14, for example...
            (_, false) => None,
            (x, _) if x % 2 != 0 => Some(x),
            (2, _) => Some(2), // Explicitly includes the index `2`, due to being the only even prime number.
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
/// 
/// ```
/// # use my_rusttools::factories::range_with_step;
/// #
/// let range = range_with_step(0..=10, 2);
/// let nums = vec![0, 2, 4, 6, 8, 10];
///
/// assert!(range.eq(nums));
/// ```
#[inline]
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
/// 
/// ```
/// # use my_rusttools::factories::fizzbuzz;
/// #
/// assert_eq!(Some("FizzBuzz".to_string()), fizzbuzz().nth(14));
/// ```
#[inline]
pub fn fizzbuzz() -> FizzBuzz<'static, impl FnMut((usize,(&'static str, &'static str))) -> String> {
    // Sets up cycling iterators, with `Fizz` and `Buzz` values at the appropriate intervals,
    // zipping them into a single iterator.
    let fizzbuzz = repeat_interval("Fizz", 3).zip(repeat_interval("Buzz", 5));

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

/// Creates an iterator that repeats a default value,
/// inserting the `repeat` value, every `interval` iterations.
/// 
/// # Panics
/// 
/// The function does not guard against underflows,
/// so passing a value of `0` either produces the wrong interval, or panics.
/// If debug assertions are enabled, a panic is guaranteed.
/// 
/// # Examples
/// 
/// ```
/// # use my_rusttools::factories::repeat_interval;
/// #
/// let mut fizzy = repeat_interval("Fizz", 3);
/// 
/// // First two values are empty string slices.
/// assert_eq!(fizzy.next(), Some(""));
/// assert_eq!(fizzy.next(), Some(""));
/// 
/// // The third value is now `"Fizz"`.
/// assert_eq!(fizzy.next(), Some("Fizz"));
/// 
/// // The fourth and fifth values are empty string slices again.
/// assert_eq!(fizzy.next(), Some(""));
/// assert_eq!(fizzy.next(), Some(""));
/// ```
#[inline]
pub fn repeat_interval<T: Clone + Default>(repeat: T, interval: usize) -> RepeatInterval<T> {
    iter::repeat(Default::default())
        .take(interval - 1)
        .chain(iter::once(repeat))
        .cycle()
}

/// Creates an iterator that repeats a default value,
/// inserting the result of the `repeat` closure,
/// every `interval` iterations.
/// 
/// # Panics
/// 
/// The function does not guard against underflows,
/// so passing a value of `0` either produces the wrong interval, or panics.
/// If debug assertions are enabled, a panic is guaranteed.
/// 
/// # Examples
/// 
/// ```
/// # use my_rusttools::factories::repeat_interval_with;
/// #
/// let mut fizzy = repeat_interval_with(||String::from("Fizz"), 3);
/// 
/// // First two values are empty strings.
/// assert_eq!(fizzy.next(), Some("".to_owned()));
/// assert_eq!(fizzy.next(), Some("".to_owned()));
/// 
/// // The third value is now `"A"`.
/// assert_eq!(fizzy.next(), Some("Fizz".to_owned()));
/// 
/// // The fourth and fifth values are empty strings again.
/// assert_eq!(fizzy.next(), Some("".to_owned()));
/// assert_eq!(fizzy.next(), Some("".to_owned()));
/// ```
#[inline]
pub fn repeat_interval_with<T: Default, F: Clone>(repeat: F, interval: usize) -> RepeatIntervalWith<impl FnMut() -> T + Clone, F> where
F: FnMut() -> T, {
    iter::repeat_with(Default::default)
        .take(interval - 1)
        .chain(iter::once_with(repeat))
        .cycle()
}

/// Creates an iterator that takes `repeat`, 
/// repeating the first value of each tuple, for the count of the second.
/// 
/// # Examples
/// 
/// ```
/// # use my_rusttools::factories::repeat_values;
/// #
/// let foobar_1 = repeat_values(&[("Foo", 2), ("Bar", 3)]);
/// let foobar_2 = ["Foo", "Foo", "Bar", "Bar", "Bar"];
/// 
/// assert!(foobar_1.eq(foobar_2));
/// ```
pub fn repeat_values<T: Clone>(repeat: &[(T, usize)]) -> RepeatValues<T, impl FnMut((T, usize)) -> Take<Repeat<T>>> {
    repeat.to_owned()
        .into_iter()
        .flat_map(|(x, y)|iter::repeat(x).take(y))
}