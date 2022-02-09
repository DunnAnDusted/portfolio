#![deny(missing_docs)]

/// A meta iterator, its closure recieves a referance to the iterator,
/// allowing bespoke advancing behaviour to be defined.
/// This enables use of methods on the adapted iterator,
/// or batched advancement, whilst utilising declarative patterns.
/// 
/// # Examples
/// 
/// Batching elements:
/// ```
/// # use reitertools::ReItertools;
/// #
/// let batches = (0..4).next_with(|iter|some((iter.next()?, iter.next()?)));
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
///     .for_each(|i|assert!(i < 2));
/// 
/// assert!(peekable_range.eq([2, 3]));
/// ```
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct NextWith<I, F> {
    next: F,
    inner: I,
}

impl<T, I, F> NextWith<I, F> where
I: Iterator,
F: FnMut(&mut I) -> Option<T>, {
    /// Constructs a new [`NextWith`] iterator from the passed closure and iterator.
    pub(super) fn new(next: F, inner: I) -> NextWith<I, F> {
        NextWith{
            next,
            inner,
        }
    }

    /// Deconstructs the iterator into its inner iterator.
    #[inline]
    pub fn into_inner(self) -> I {
        self.inner
    }
}

impl<T, I, F> Iterator for NextWith<I, F> where
I: Iterator,
F: FnMut(&mut I) -> Option<T>, {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        (self.next)(self.inner.by_ref())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}