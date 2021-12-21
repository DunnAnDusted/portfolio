//! Interfaces for summarising collections,
//! and their implementations.
use std::{
    collections::HashMap,
    hash::Hash,
    iter::Iterator,
    borrow::Borrow,
};

/// An interface for counting the number of times
/// the referanced item appears in the collection.
pub trait TallyItems<Q: ?Sized> 
where
    Self::Item: Borrow<Q>,
    Q: Eq, {
    /// The type of items being tallied.
    type Item;

        /// Counts the number of times the referanced item
        /// appears in the collection.
        /// 
        /// The value may be any borrowed form of the collections item type, 
        /// but [`Eq`] on the borrowed form must match that for the item type.
        /// 
        /// # Example
        /// ```
        /// use my_rusttools::traits::TallyItems;
        /// 
        /// let a = ["One", "Two", "Three", "Three"];
        /// let count = a.iter().tally_item(&"Three");
        /// 
        /// assert_eq!(2, count);
        /// ```
        fn tally_item(self, find: &Q) -> usize;
}

impl<T: Iterator, Q> TallyItems<Q> for T
where
    T::Item: Borrow<Q>,
    Q: Eq + ?Sized, {
        type Item = T::Item;

        fn tally_item(self, find: &Q) -> usize
        where
            Self::Item: Borrow<Q>,
            Q: Eq, {
                self.into_iter()
                    .filter(|x|x.borrow() == find)
                    .count()
            }
    }

/// An interface for summarising collections of values.
pub trait SummariseCollection 
where
    Self::Item: Eq + Hash, {
        /// The type of item in the collection being summarised.
        type Item;

        /// Counts the number of times a value appears in a collection.
        /// 
        /// Returns a [`HashMap`] with keys defined as the type,
        /// `Self::Item`, counting how many times each value appeared as a `usize`.
        /// 
        /// # Examples
        /// ```
        /// use my_rusttools::traits::SummariseCollection;
        /// 
        /// let a = ["One", "Two", "Three", "Three"];
        /// 
        /// let iter = a.iter();
        /// let counts = iter.count_items();
        /// 
        /// assert_eq!(Some(&1), counts.get(&a[0]));
        /// assert_eq!(Some(&1), counts.get(&a[1]));
        /// assert_eq!(Some(&2), counts.get(&a[2]));
        /// ```
        fn count_items(self) -> HashMap<Self::Item, usize>;

        /// Finds the most common item in a collection.
        /// 
        /// If multiple items are the most common,
        /// only one of them is returned.
        /// If the collection is empty,
        /// [`None`] is returned.
        /// 
        /// # Examples
        /// ```
        /// use my_rusttools::traits::SummariseCollection;
        /// 
        /// let a = ["One", "Two", "Two", "Three", "Three"];
        /// 
        /// let iter = a.iter();
        /// let most_common = iter.most_common();
        /// 
        /// assert!(most_common.is_some());
        /// ```
        #[inline]
        fn most_common(self) -> Option<Self::Item>
        where
            Self: Sized {
                self.most_common_count()
                    .map(|x|x.0)
            }

        /// Finds the most common item in a collection,
        /// listing the number of times it occurs.
        /// 
        /// If multiple items are the most common,
        /// only one of them is returned.
        /// If the collection is empty,
        /// [`None`] is returned.
        /// 
        /// # Examples
        /// ```
        /// use my_rusttools::traits::SummariseCollection;
        /// 
        /// let a = ["One", "Two", "Two", "Three", "Three"];
        /// 
        /// let iter = a.iter();
        /// let most_common_with_count = iter.most_common_count();
        /// 
        /// assert!(most_common_with_count.is_some());
        /// ```
        fn most_common_count(self) -> Option<(Self::Item, usize)>
        where 
            Self: Sized {
                self.count_items()
                    .into_iter()
                    .max_by(|x, y|x.1.cmp(&y.1))
            }

        /// Find the least common item in a collection.
        /// 
        /// If multiple items are the least common,
        /// only one of them is returned.
        /// If the collection is empty, [`None`] is returned.
        /// 
        /// # Examples
        /// ```
        /// use my_rusttools::traits::SummariseCollection;
        /// 
        /// let a = ["One", "Two", "Three", "Three"];
        /// 
        /// let iter = a.iter();
        /// let least_common = iter.least_common();
        /// 
        /// assert!(least_common.is_some());
        /// ```
        #[inline]
        fn least_common(self) -> Option<Self::Item>
        where
            Self: Sized {
                self.least_common_count()
                    .map(|x|x.0)
            }

        /// Finds the least common item in a collection,
        /// listing the number of times it occurs.
        /// 
        /// If multiple items are the least common,
        /// only one of them is returned.
        /// If the collection is empty,
        /// [`None`] is returned.
        /// 
        /// # Examples
        /// ```
        /// use my_rusttools::traits::SummariseCollection;
        /// 
        /// let a = ["One", "Two", "Three", "Three"];
        /// 
        /// let iter = a.iter();
        /// let least_common_with_count = iter.least_common_count();
        /// 
        /// assert!(least_common_with_count.is_some());
        /// ```
        fn least_common_count(self) -> Option<(Self::Item, usize)>
        where
            Self: Sized {
                self.count_items()
                    .into_iter()
                    .min_by(|x, y|x.1.cmp(&y.1))
            }
    }

impl<T: Iterator> SummariseCollection for T 
where
    T::Item: Eq + Hash {
        type Item = T::Item;

        fn count_items(self) -> HashMap<T::Item, usize> {
            let mut ret: HashMap<T::Item, usize> = HashMap::new();

            for item in self {
                ret.entry(item)
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
            }

            ret
        }
    }