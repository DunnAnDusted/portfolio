#![deny(missing_docs)]
use std::{
    borrow::Borrow,
    collections::HashMap,
    hash::Hash, 
    iter::Peekable,
};

use crate::NextWith;

/// An interface to extend the [`Iterator`] trait,
/// with additional adaptors and methods.
pub trait ReItertools: Iterator {
    /// Counts the number of times the referanced item
    /// appears in the iterator.
    /// 
    /// The value may be any borrowed form of the collections item type, 
    /// but [`Eq`] on the borrowed form must match that for the item type.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use reitertools::ReItertools;
    /// #
    /// let a = ["One", "Two", "Three", "Three"];
    /// let count = a.iter().tally_item(&"Three");
    /// 
    /// assert_eq!(2, count);
    /// ```
    #[inline]
    fn tally_item<Q>(self, find: &Q) -> usize where
    Self: Sized,
    Self::Item: Borrow<Q>,
    Q: Eq + ?Sized, {
        self.filter(|x|x.borrow() == find)
            .count()
    }

    /// Counts the number of times a unique elements
    /// is taken from the iterator.
    /// 
    /// Takes each elements, incriments their count in a [`HashMap`],
    /// or inserts unique elements,
    /// and returns the resulting `HashMap` once exhausted.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use reitertools::ReItertools;
    /// #
    /// let a = ["One", "Two", "Three", "Three"];
    /// 
    /// let iter = a.iter();
    /// let counts = iter.count_items();
    /// 
    /// assert_eq!(Some(&1), counts.get(&a[0]));
    /// assert_eq!(Some(&1), counts.get(&a[1]));
    /// assert_eq!(Some(&2), counts.get(&a[2]));
    /// ```
    #[inline]
    fn count_items(mut self) -> HashMap<Self::Item, usize> where
    Self: Sized,
    Self::Item: Eq + Hash, {
        let mut ret = HashMap::new();

        while let Some(key) = self.next() {
            ret.entry(key)
                .and_modify(|x|*x += 1)
                .or_insert(1);
        }

        ret

        // Alternative, more declarative, implementation.
        // I found this rather less readable, making an imperative option, preferable.
        //self.fold(HashMap::new(), |mut hm, key|{*hm.entry(key).or_default() += 1; hm})
        // OPTIONAL: See potential performance differences between approaches. Likely little to none,
    }

    /// Returns the element most commonly taken from an iterator.
    /// 
    /// If several elements are taken most commonly, only one is returned.
    /// If the iterator is empty, [`None`] is returned.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use reitertools::ReItertools;
    /// #
    /// let a = ["One", "Two", "Two", "Three", "Three"];
    /// 
    /// let iter = a.iter();
    /// assert!(iter.most_common().is_some());
    /// ```
    #[inline]
    fn most_common(self) -> Option<Self::Item> where
    Self: Sized,
    Self::Item: Eq + Hash, {
        self.most_common_with_count()
            .map(|x|x.0)
    }

    /// Returns the element most commonly taken from an iterator,
    /// with the number of times it was taken.
    /// 
    /// If several elements are taken most commonly, only one is returned.
    /// If the iterator is empty, [`None`] is returned.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use reitertools::ReItertools;
    /// #
    /// let a = ["One", "Two", "Two", "Three", "Three"];
    /// 
    /// let iter = a.iter();
    /// let most_common = iter.most_common_with_count();
    /// 
    /// assert_eq!(most_common.1, 2);
    /// ```
    #[inline]
    fn most_common_with_count(self) -> Option<(Self::Item, usize)> where
    Self: Sized,
    Self::Item: Eq + Hash, {
        self.count_items()
            .into_iter()
            .max_by(|x, y|x.1.cmp(&y.1))
    }

    /// Returns the element least commonly taken from an iterator.
    /// 
    /// If several elements are taken least commonly, only one is returned.
    /// If the iterator is empty, [`None`] is returned.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use reitertools::ReItertools;
    /// #
    /// let a = ["One", "Two", "Two", "Three", "Three"];
    /// 
    /// let iter = a.iter();
    /// assert!(iter.least_common().is_some());
    /// ```
    #[inline]
    fn least_common(self) -> Option<Self::Item> where
    Self: Sized,
    Self::Item: Eq + Hash, {
        self.least_common_with_count()
            .map(|x|x.0)
    }

    /// Returns the element least commonly taken from an iterator,
    /// with the number of times it was taken.
    /// 
    /// If several elements are taken least commonly, only one is returned.
    /// If the iterator is empty, [`None`] is returned.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use reitertools::ReItertools;
    /// #
    /// let a = ["One", "Two", "Two", "Three", "Three"];
    /// 
    /// let iter = a.iter();
    /// let lest_common = iter.least_common_with_count();
    /// 
    /// assert_eq!(least_common.1, 1);
    /// ```
    #[inline]
    fn least_common_with_count(self) -> Option<(Self::Item, usize)> where
    Self: Sized,
    Self::Item: Eq + Hash, {
        self.count_items()
            .into_iter()
            .min_by(|x, y|x.1.cmp(&y.1))
    }

    /// A meta iterator adaptor, its closure recieves a referance to the iterator,
    /// allowing bespoke advancing behaviour to be defined. 
    /// This enables use of methods implemented on the adapted iterator,
    /// or batched advancement, whilst utilising declarative patterns.
    /// 
    /// # Examples
    /// 
    /// Batching elements:
    /// ```
    /// # use reitertools::ReItertools;
    /// #
    /// let batches = (0..4).next_with(|iter|Some((iter.next()?, iter.next()?)));
    /// 
    /// assert!(batches.eq([(0, 1), (2, 3)]));
    /// ```
    /// 
    /// Adapted behaviour:
    /// ```
    /// # use reitertools::ReItertools;
    /// #
    /// let mut peekable_range = (0..4).peekable();
    /// 
    /// peekable_range.by_ref()
    ///     .next_with(|iter|iter.next_if(|&x|x < 2))
    ///     .for_each(|i| assert!(i < 2));
    /// 
    /// assert!(peekable_range.eq([2, 3]));
    /// ```
    #[inline]
    fn next_with<T,F>(self, next: F) -> NextWith<Self, F> where
    Self: Sized,
    F: FnMut(&mut Self) -> Option<T>, {
        NextWith::new(next, self)
    }

    /// Tests if the elements of the iterator are mirrored.
    /// 
    /// `palindrome_items()` adapts the iterator, to take elements from each end of the iterator in batches,
    /// eagerly testing for equality between the values in each batch.
    /// If all of them are equal until a `None` is reached, `palindrome_items()` will return `true`.
    /// If any of them are not equal, `palindrome_items()` will return `false`.
    /// 
    /// `palindrome_items()` is short-circuiting, and will halt once a pair is not equal,
    /// or the value from one end of the iterator, is exhausted.
    /// 
    /// An empty iterator returns `true`.
    /// 
    /// # Examples
    /// 
    /// Palindrome iterator:
    /// ```
    /// # use reitertools::ReItertools;
    /// #
    /// let a = [1, 0, 1].into_iter();
    /// 
    /// assert!(a.palindrome_items());
    /// ```
    /// 
    /// Non-Palindrome iterator:
    /// ```
    /// # use reitertools::ReItertools;
    /// #
    /// let a = [1, 2, 3].into_iter();
    /// 
    /// assert!(!a.palindrome_iterator());
    /// ```
    fn palidrome_items(&mut self) -> bool where
    Self: DoubleEndedIterator + Sized,
    Self::Item: PartialEq, {
        self.next_with(|iter|Some((iter.next()?, iter.next_back()?)))
            .all(|(x, y)|x == y)
    }
}

impl<T: ?Sized> ReItertools for T where
T: Iterator {}

/// An interface for iterator structs, with the capacity to retain values
/// which don't meet a condition to advance the iterator.
/// 
/// This is intended as an interface, to generalise the implementation
/// of the conditional advancing methods implemented for the [`Peekable`]
/// iterator in the standard library, where the structure allows equivilent behaviour to be implemented.
/// 
/// # Examples
/// 
/// Standard library inspiration:
/// ```
/// let peekable_range = (0..4).peekable();
/// 
/// while let Some(i) = peekable_range.next_if(|&x|x < 2) {
///     assert!(i < 2);
/// }
/// 
/// assert!(peekable_range.eq([2,3]));
/// ```
/// 
/// ReItertools `ConditionalAdvance` interface:
/// ```
/// use reitertools::*;
/// 
/// let peekable_range = (0..4).peekable();
/// 
/// peekable_range.by_ref()
///     .next_with(|iter|iter.next_if_lt(&2))
///     .for_each(|i|assert!(i < 2));
/// 
/// assert!(peekable_range.eq([2, 3]));
/// ```
pub trait ConditionalAdvance: Iterator {
    /// Consumes and returns the next value of this iterator,
    /// if a condition is true.
    /// 
    /// If `func` returns `true` for the next value of this iterator, consume and return it.
    /// Otherwise, return `None`.
    /// 
    /// # Examples
    /// 
    /// Consumes a number if it's equal to 0:
    /// ```
    /// use reitertools::ConditionalAdvance;
    /// 
    /// let mut peekable_range = (0..5).peekable();
    /// // The first item is 0:
    /// assert_eq!(peekable_range.next_if(|&i|i == 0), Some(0));
    /// // The next item is 1, so `func` will return `false`.
    /// assert_eq!(peekable_range.next_if(|&i|i == 0), None);
    /// // `next_if` retains the value on-top of the iterator if `func` returned `false`.
    /// assert_eq!(peekable_range.next(), Some(1));
    /// ```
    /// 
    /// Consume items less than 10:
    /// ```
    /// use reitertools::*;
    /// 
    /// let mut peekable_range = (1..20).peekable();
    /// // Consume all values less than 10.
    /// peekable_range.by_ref()
    ///     .next_with(|iter|iter.next_if(|&i|i < 10))
    ///     .for_each(|i|assert!(i < 10));
    /// // The next value returned will be 10.
    /// assert_eq!(peekable_range.next(), Some(10));
    /// ```
    fn next_if<F>(&mut self, func: F) -> Option<Self::Item> where
    F: FnOnce(&Self::Item) -> bool;

    /// Consume and return the next item, if equal to `expected`.
    /// 
    /// # Examples
    /// 
    /// Consumes a number if it's equal to 0:
    /// ```
    /// use reitertools::ConditionalAdvance;
    /// 
    /// let mut peekable_range = (0..5).peekable();
    /// // The first item is 0.
    /// assert_eq!(peekable_range.next_if_eq(&0), Some(0));
    /// // The next item is 1.
    /// assert_eq!(peekable_range.next_if_eq(&0), None);
    /// // `next_if_eq` retains the value on-top of the iterator.
    /// assert_eq!(peekable_range.next(), Some(1));
    /// ```
    fn next_if_eq<T>(&mut self, expected: &T) -> Option<Self::Item> where
    T: ?Sized,
    Self::Item: PartialEq<T>,
    Self: Sized, {
        self.next_if(|x|x == expected)
    }

    /// Consume and return the next item, if not equal to `expected`.
    /// 
    /// # Examples
    /// 
    /// Consumes a number if it's not equal to 1:
    /// ```
    /// use reitertools::ConditionalAdvance;
    /// 
    /// let mut peekable_range = (0..5).peekable();
    /// // The first item is 0.
    /// assert_eq!(peekable_range.next_if_ne(&1), Some(0));
    /// // The next item is 1.
    /// assert_eq!(peekable_range.next_if_ne(&1), None);
    /// // `next_if_ne` retains the value on-top of the iterator.
    /// assert_eq!(peekable_range.next(), Some(1));
    /// ```
    fn next_if_ne<T>(&mut self, expected: &T) -> Option<Self::Item> where
    T: ?Sized,
    Self::Item: PartialEq<T>,
    Self: Sized, {
        self.next_if(|x|x != expected)
    }

    /// Consume and return the next item, if less than or equal to `expected`.
    /// 
    /// # Examples
    /// 
    /// Consumes a number if it's less than or equal to 0:
    /// ```
    /// use reitertools::ConditionalAdvance;
    /// 
    /// let mut peekable_range = (0..5).peekable();
    /// // The first item is 0.
    /// assert_eq!(peekable_range.next_if_le(&0), Some(0));
    /// // The next item is 1.
    /// assert_eq!(peekable_range.next_if_le(&0), None);
    /// // `next_if_le` retains the value on-top of the iterator.
    /// assert_eq!(peekable_range.next(), Some(1));
    /// ```
    fn next_if_le<T>(&mut self, expected: &T) -> Option<Self::Item> where
    T: ?Sized,
    Self::Item: PartialOrd<T>,
    Self: Sized, {
        self.next_if(|x|x <= expected)
    }

    /// Consume and return the next item, if less than `expected`.
    /// 
    /// # Examples
    /// 
    /// Consumes a number if it's less than to 1:
    /// ```
    /// use reitertools::ConditionalAdvance;
    /// 
    /// let mut peekable_range = (0..5).peekable();
    /// // The first item is 0.
    /// assert_eq!(peekable_range.next_if_lt(&1), Some(0));
    /// // The next item is 1.
    /// assert_eq!(peekable_range.next_if_lt(&1), None);
    /// // `next_if_lt` retains the value on-top of the iterator.
    /// assert_eq!(peekable_range.next(), Some(1));
    /// ```
    fn next_if_lt<T>(&mut self, expected: &T) -> Option<Self::Item> where
    T: ?Sized,
    Self::Item: PartialOrd<T>,
    Self: Sized, {
        self.next_if(|x|x < expected)
    }

    /// Consume and return the next item, if greater or equal to `expected`.
    /// 
    /// # Examples
    /// 
    /// Consumes a number if it's greater or equal to 0:
    /// ```
    /// use reitertools::ConditionalAdvance;
    /// 
    /// let mut reverse_peekable_range = (-4..=0).rev().peekable();
    /// // The first item is 0.
    /// assert_eq!(reverse_peekable_range.next_if_ge(&0), Some(0));
    /// // The next item is -1.
    /// assert_eq!(reverse_peekable_range.next_if_ge(&0), None);
    /// // `next_if_ge` retains the value on-top of the iterator.
    /// assert_eq!(reverse_peekable_range.next(), Some(-1));
    /// ```
    fn next_if_ge<T>(&mut self, expected: &T) -> Option<Self::Item> where
    T: ?Sized,
    Self::Item: PartialOrd<T>,
    Self: Sized, {
        self.next_if(|x|x >= expected)
    }

    /// Consume and return the next item, if greater than `expected`.
    /// 
    /// # Examples
    /// 
    /// Consumes a number if it's greater than -1:
    /// ```
    /// use reitertools::ConditionalAdvance;
    /// 
    /// let mut reverse_peekable_range = (-4..=0).rev().peekable();
    /// // The first item is 0.
    /// assert_eq!(reverse_peekable_range.next_if_gt(&-1), Some(0));
    /// // The next item is -1.
    /// assert_eq!(reverse_peekable_range.next_if_gt(&-1), None);
    /// // `next_if_gt` retains the value on-top of the iterator.
    /// assert_eq!(reverse_peekable_range.next(), Some(-1));
    /// ```
    fn next_if_gt<T>(&mut self, expected: &T) -> Option<Self::Item> where
    T: ?Sized,
    Self::Item: PartialOrd<T>,
    Self: Sized, {
        self.next_if(|x|x > expected)
    }
}

impl<I :Iterator> ConditionalAdvance for Peekable<I> {
    fn next_if<F>(&mut self, func: F) -> Option<Self::Item> where
    F: FnOnce(&Self::Item) -> bool, {
        self.next_if(func)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_with_peeking() {
        let mut peekable_range = (0usize..4).peekable();

        peekable_range.by_ref()
            .next_with(|iter|iter.next_if_lt(&2))
            .for_each(|i|assert!(i < 2));

        assert!(peekable_range.eq(vec![2, 3]));
    }

    #[test]
    fn next_with_batching() {
        let batches = (0..4).next_with(|iter|Some((iter.next()?, iter.next()?)));

        assert!(batches.eq([(0, 1), (2, 3)]));
    }

    #[test]
    fn palimdrome_strings() {
        let wow = "Wow".chars()
            .map(|x|x.to_lowercase())
            .next_with(|iter|Some((iter.next()?, iter.next_back()?)))
            .all(|(x, y)|x.eq(y));

        assert!(wow);
    }

    #[test]
    fn palindrome_iterators() {
        let wow = [0, 1, 0].into_iter().palidrome_items();

        assert!(wow);
    }
}