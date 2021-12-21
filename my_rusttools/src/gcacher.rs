//! A generic caching struct.
//! 
//! This module contains the [`GCacher`] type, a wrapper for [`HashMap`],
//! designed for storing the result of expensive closures,
//! retrieving an accurate return of the closure, without recalling it.
//! 
//! # Examples
//! 
//! The `value_from` method is primary route of addition and access to the caches content,
//! returning a pure referance to the arguments associated value in the cache,
//! creating one if it couldn't be found.
//! 
//! ```
//! use my_rusttools::gcacher::GCacher;
//! 
//! // Instances the cacher.
//! let mut squares = GCacher::new(|x: &usize|{
//!     println!("Closure ran!");
//!     x * x
//! });
//! 
//! // New values are added, then retried without running the closure again,
//! // via the `value_from()` method.
//! assert_eq!(&4, squares.value_from(2));
//! assert_eq!(&4, squares.value_from(2));
//! ```
use std::{
    borrow::Borrow,
    collections::{
        HashMap,
        hash_map::{
            RandomState,
            Drain,
        },
    },
    hash::Hash,
    ops::Deref,
    convert::From,
};

/// A generic caching struct.
/// 
/// Written as a wrapper to an underlying [`HashMap`],
/// it is designed for storing the result of expensive closures once executed,
/// as to allow it to be retrieved later, without recalling the closure.
/// 
/// # Examples
/// 
/// The [`value_from`] method is primary route of addition and access to the caches content,
/// returning a pure referance to the arguments associated value in the cache,
/// creating one if it couldn't be found.
/// 
/// ```
/// use my_rusttools::gcacher::GCacher;
/// 
/// // Instances the cacher.
/// let mut squares = GCacher::new(|x: &usize|{
///     println!("Closure ran!");
///     x * x
/// });
/// 
/// // New values are added, then retried without running the closure again,
/// // via the `value_from()` method.
/// assert_eq!(&4, squares.value_from(2));
/// assert_eq!(&4, squares.value_from(2));
/// ```
/// 
/// # Type Annotation
/// 
/// When instancing a `GCacher`, a closure argument is always required.
/// 
/// While closures can tyically be defined with implicit type definitions for their paramaters,
/// as part of the initialisation, the generic definitions of the struct haven't been defined,
/// and are instead defined in reverse, derived from the closures parameter and return types.
/// 
/// From these types, underlying `HashMap`s key-value pair types and, in turn, 
/// the parameter and return types of the [`value_from`] method, are derived.
/// 
/// ```
/// use my_rusttools::gcacher::GCacher;
/// 
/// // Type inferance is derived from the required type
/// // annotation on the closure, and its return type,
/// // rather than typing the variable declaration.
/// let mut squares = GCacher::new(|x: &usize|x * x);
/// 
/// // Automatically caching a value after a,
/// // potentially expensive, closure operation.
/// assert_eq!(&4, squares.value_from(2));
/// assert_eq!(&4, squares.value_from(2));
/// ```
/// ```compile_fail,E0282
/// use my_rusttools::gcacher::GCacher;
/// 
/// // Type cannout be infered, without
/// // annotating the type on the closure.
/// let squares = GCacher::new(|x|x * x);
/// ```
/// 
/// # Deref and the Pledge of Correctness
/// 
/// Being designed to store return values from a process, where avoiding repetion is desirable,
/// `GCacher` comes with the expectation its stored values be accurate with the return,
/// were the process to be repeated.
/// 
/// To this end, `GCacher` is written so the correctness of the underlying `HashMap`
/// is enforced by encapsulation, where access to the underlying cache is only ever immutable,
/// provided through implementation of the [`Deref`]`<Target=HashMap<K, V>>` trait.
/// 
/// Otherwise, limited access to constructor and destructor methods are provided through overriding methods,
/// implemented for `GCacher`, such as [`clear`] and [`drain`], where entries are removed from the cache.
/// ```
/// use my_rusttools::gcacher::GCacher;
/// 
/// // Instances the cacher.
/// let mut cacher = GCacher::new(|x: &usize|x * x);
/// 
/// // Caches a new value through the controlled mutator method,
/// // `value_from`.
/// cacher.value_from(2);
/// 
/// // The cache is locked, becoming immutable.
/// let cacher = cacher;
/// 
/// // The value cached while unlocked,
/// // is still accessible, via immutable deferance
/// // to the underlying `HashMap`.
/// assert_eq!(Some(&4), cacher.get(&2));
/// ```
/// 
/// Lastly, due to the nature of closures,
/// possesing the ability to capture variables from the environment they were defined in,
/// the results from the caching process can be affected
/// by the value captured variables are assigned before the first value is cached.
/// 
/// Due to the fact any variables captured by the closure,
/// can't be mutated again until the lifetime of the cache expires,
/// this does not outright violate the Pledge of Correctness,
/// though, writing closures dependant exclusively on their parameter,
/// would be recommended.
/// ```compile_fail,E0506
/// use my_rusttools::gcacher::GCacher;
/// 
/// // Environment variable is initialised and incrimented.
/// let mut num = 0;
/// num += 1;
/// 
/// // Cacher initialised, capturing `num`.
/// let mut cacher = GCacher::new(|x: &usize|x + num);
/// cacher.value_from(2);
/// 
/// // Environment variable is incrimented again.
/// num += 1;
/// assert_eq!(3, num);
/// 
/// // Cacher is erroneously called again, invalidating prior incriment.
/// assert_eq!(&3, cacher.value_from(2));
/// ```
/// [`value_from`]: Self::value_from
/// [`clear`]: Self::clear
/// [`drain`]: Self::drain
#[derive(Clone)]
pub struct GCacher<K, F, V, S = RandomState> 
where
    K: Hash + Eq,
    F: Fn(&K) -> V, {
        pub instancer: F,
        cache: HashMap<K, V, S>,
    }

impl<K, F, V> GCacher<K, F, V> 
where
    K: Hash + Eq,
    F: Fn(&K) -> V, {
        /// Creates a `GCacher` with an empty `HashMap`.
        /// 
        /// The cache `HashMap` is initially created with a capacity of 0,
        /// so will not alocate until the first value is cached.
        /// 
        /// # Examples
        /// ```
        /// use my_rusttools::gcacher::GCacher;
        /// let mut cacher = GCacher::new(|x: &usize|x * x);
        /// ```
        pub fn new(instancer: F) -> GCacher<K, F, V> {
            Self::create(instancer, HashMap::new())
        }

        /// Creates a `GCacher` with a `HashMap` of the specified capacity.
        /// 
        /// The cache `HashMap` will be able to hold at least `capacity` elements,
        /// without reallocating. If capacity is 0, the hash map will not allocate.
        /// 
        /// # Example
        /// ```
        /// use my_rusttools::gcacher::GCacher;
        /// let mut cacher = GCacher::with_capacity(|x: &usize|x * x, 10);
        /// ```
        pub fn with_capacity(instancer: F, capacity: usize) -> GCacher<K, F, V> {
            Self::create(instancer, HashMap::with_capacity(capacity))
        }

        /// Returns a reference to the value corresponding to the key,
        /// instancing a new one, if a key value pairing does not already exist.
        /// 
        /// The types of `val` and `V` are determined by the parameter,
        /// and return type of the closure passed upon initialisation of the cacher.
        /// 
        /// # Example
        /// ```
        /// use my_rusttools::gcacher::GCacher;
        /// 
        /// let mut cacher = GCacher::new(|x: &usize|x * x);
        ///
        /// assert_eq!(&4, cacher.value_from(2));
        /// assert_eq!(&16, cacher.value_from(4));
        /// ```
        pub fn value_from(&mut self, val: K) -> &V {
            self.cache.entry(val)
                .or_insert_with_key(&self.instancer)          
        }

        /// Clears the cache hash map, removing all key-value pairs.
        /// Keeps the allocated memory for reuse.
        /// 
        /// # Example
        /// ```
        /// use my_rusttools::gcacher::GCacher;
        /// 
        /// let mut cacher = GCacher::new(|x: &usize|x * x);
        /// cacher.value_from(2);
        /// cacher.clear();
        /// assert!(cacher.is_empty());
        /// ```
        pub fn clear(&mut self) {
            self.cache.clear();
        }

        /// Clears the cache hash map, returning all the  kay-value pairs as an iterator.
        /// Keeps the allocated memory for resuse.GCacher
        /// 
        /// # Example
        /// ```
        /// use my_rusttools::gcacher::GCacher;
        /// 
        /// let mut cacher = GCacher::new(|x: &usize|x * x);
        /// cacher.value_from(2);
        /// cacher.value_from(4);
        /// 
        /// for (k, v) in cacher.drain().take(1) {
        ///     assert!(k == 2 || k == 4);
        ///     assert!(v == 4 || v == 16);
        /// }
        /// 
        /// assert!(cacher.is_empty());
        /// ```
        pub fn drain(&mut self) -> Drain<'_, K, V> {
            self.cache.drain()
        }

        /// Reserves capacity for at least `additional` more elements to be inserted
        /// in the caches underlying `HashMap`. The collection may reserve more space to avoid
        /// frequent reallocations.
        ///
        /// # Panics
        ///
        /// Panics if the new allocation size overflows [`usize`].
        ///
        /// # Examples
        ///
        /// ```
        /// use my_rusttools::gcacher::GCacher;
        /// let mut cacher = GCacher::new(|x: &usize|x * x);
        /// cacher.reserve(10);
        /// ```
        pub fn reserve(&mut self, additional: usize) {
            self.cache.reserve(additional);
        }

        /// Shrinks the capacity of the caches hash map as much as possible.
        /// As much as possible will be dropped, while maintaining the internal rules
        /// and possibly leaving some space in accordance with the resize policy.
        /// 
        /// # Examples
        /// ```
        /// use my_rusttools::gcacher::GCacher;
        /// 
        /// let mut cacher = GCacher::with_capacity(|x: &usize|x * x, 100);
        /// cacher.value_from(2);
        /// cacher.value_from(4);
        /// assert!(cacher.capacity() >= 100);
        /// cacher.shrink_to_fit();
        /// assert!(cacher.capacity() >= 2);
        /// ```
        pub fn shrink_to_fit(&mut self) {
            self.cache.shrink_to_fit();
        }

        /// Shrinks the cacpacity of the caches internal hash map, with a lower limit.
        /// It will drop no more than the passed lower limit,
        /// while maintaining the internal rules and possibly leaving some space in accordance with the resize policy.
        /// 
        /// If the current capacity is less than the lower limit, this is a no-op.
        /// 
        /// # Examples
        /// ```
        /// use my_rusttools::gcacher::GCacher;
        /// 
        /// let mut cacher = GCacher::with_capacity(|x: &usize|x * x, 100);
        /// cacher.value_from(2);
        /// cacher.value_from(4);
        /// assert!(cacher.capacity() >= 100);
        /// cacher.shrink_to(10);
        /// assert!(cacher.capacity() >= 10);
        /// cacher.shrink_to(0);
        /// assert!(cacher.capacity() >= 2);
        /// ```
        pub fn shrink_to(&mut self, min_capacity: usize) {
            self.cache.shrink_to(min_capacity);
        }

        /// Removes a key from the cache's underlying hash map,
        /// returning the associated value when there is one cached.
        /// 
        /// The key may be an borrowed form of the maps key type,
        /// but [`Hash`] and [`Eq`] on the borrowed form *must* match those for the key type.
        /// 
        /// # Examples
        /// ```
        /// use my_rusttools::gcacher::GCacher;
        /// 
        /// let mut cacher = GCacher::new(|x: &usize|x * x);
        /// cacher.value_from(2);
        /// assert_eq!(cacher.remove(&2), Some(4));
        /// assert_eq!(cacher.remove(&2), None);
        /// ```
        pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
        where
            K: Borrow<Q>,
            Q: Eq + Hash, {
                self.cache.remove(k)
            }

        /// Removes a key from the cache's underlying hash map,
        /// returning the key and associated value when they were cached.
        /// 
        /// The key may be an borrowed form of the maps key type,
        /// but [`Hash`] and [`Eq`] on the borrowed form *must* match those for the key type.
        /// 
        /// # Examples
        /// ```
        /// use my_rusttools::gcacher::GCacher;
        /// 
        /// let mut cacher = GCacher::new(|x: &usize|x * x);
        /// cacher.value_from(2);
        /// assert_eq!(cacher.remove_entry(&2), Some((2, 4)));
        /// assert_eq!(cacher.remove_entry(&2), None);
        /// ```
        pub fn remove_entry<Q: ?Sized>(&mut self, k: &Q) -> Option<(K, V)>
        where
            K: Borrow<Q>,
            Q: Eq + Hash, {
                self.cache.remove_entry(k)
            }

        /// Retains only elements specified by the predicate.
        /// 
        /// In effect, remove all `(k, v)` pairs, such that `f(&k, &mut v)` returns `false`.
        /// The elements are visited in an unsorted (and unspecified) order.
        /// 
        /// # Examples
        /// ```
        /// use my_rusttools::gcacher::GCacher;
        /// 
        /// let mut cacher = GCacher::new(|x: &usize|x * x);
        /// cacher.value_from(1);
        /// cacher.value_from(2);
        /// cacher.value_from(3);
        /// cacher.value_from(4);
        /// cacher.retain(|&k, _|k % 2 == 0);
        /// assert_eq!(cacher.len(), 2);
        /// ```
        pub fn retain<U>(&mut self, f: U)
        where
            U: FnMut(&K, &mut V) -> bool {
                self.cache.retain(f);
            }

        /// An explicit alias of [`deref`].
        /// 
        /// Returns a referance to the structs underlying `HashMap`.
        /// 
        /// # Example
        /// ```
        /// use my_rusttools::gcacher::GCacher;
        /// 
        /// let mut cacher = GCacher::new(|x: &usize|x * x);
        /// cacher.value_from(2);
        /// 
        /// let cache = cacher.cache();
        /// assert_eq!(Some(&4), cache.get(&2));
        /// ```
        /// [`deref`]: Self::deref
        pub fn cache(&self) -> &HashMap<K, V> {
            &self.cache
        }

        /// Consumes the cacher,
        /// returning its internal cache.
        /// 
        /// # Example
        /// ```
        /// use my_rusttools::gcacher::GCacher;
        /// 
        /// let mut cacher = GCacher::new(|x: &usize|x * x);
        /// cacher.value_from(2);
        /// 
        /// let cache = cacher.into_cache();
        /// assert_eq!(Some(&4), cache.get(&2));
        /// ```
        pub fn into_cache(self) -> HashMap<K, V> {
            self.cache
        }

        /// Consumes the cacher,
        /// returning its inner values,
        /// as a tuple.
        /// 
        /// # Example
        /// ```
        /// use my_rusttools::gcacher::GCacher;
        /// 
        /// let mut cacher = GCacher::new(|x: &usize|x * x);
        /// cacher.value_from(2);
        /// 
        /// let (instancer, cache) = cacher.into_inner();
        /// assert_eq!(4, instancer(&2));
        /// assert_eq!(Some(&4), cache.get(&2));
        /// ```
        pub fn into_inner(self) -> (F, HashMap<K, V>) {
            (self.instancer, self.cache)
        }
    }

impl<K, F, V, S> GCacher<K, F, V, S> 
where
    K: Hash + Eq,
    F: Fn(&K) -> V, {
        /// The base, associated function, for instancing new caches.
        /// Allows super-functions to pass down their closure and a `HashMap`,
        /// to instance the cache cleanly.
        fn create(instancer: F, cache: HashMap<K, V, S>) -> GCacher<K, F, V, S> {
            Self {
                instancer: instancer,
                cache: cache,
            }
        }

        /// Creates a new cache with an empty `HashMap`, 
        /// using the given hash builder to hash keys.
        /// 
        /// The created map has the default initial capacity.
        /// 
        /// Warning: `hash_builder` is normally randomly generated, and is designed to allow HashMaps to be resistant to attacks that cause many collisions and very poor performance. Setting it manually using this function can expose a DoS attack vector.
        ///
        /// The `hash_builder` passed should implement the [`BuildHasher`] trait for the HashMap to be useful, see its documentation for details.
        /// 
        /// # Example
        /// ```
        /// use my_rusttools::gcacher::GCacher;
        /// use std::collections::hash_map::RandomState;
        /// 
        /// let s = RandomState::new();
        /// let mut cacher = GCacher::with_hasher(|x: &usize|x * x, s);
        /// cacher.value_from(2);
        /// ```
        /// [`BuildHasher`]: std::hash::BuildHasher
        pub fn with_hasher(instancer: F, hash_builder: S) -> GCacher<K, F, V, S> {
            Self::create(instancer, HashMap::with_hasher(hash_builder))
        }

        /// Creates a new cache with a `HashMap` of the specified capacity, 
        /// using the given hash builder to hash keys.
        /// 
        /// The hash map will be able to hold at least `capacity` elements without reallocating.
        /// If `capacity` is 0, the hash map will not allocate.
        /// 
        /// The created map has the default initial capacity.
        /// 
        /// Warning: `hash_builder` is normally randomly generated, and is designed to allow HashMaps to be resistant to attacks that cause many collisions and very poor performance. Setting it manually using this function can expose a DoS attack vector.
        ///
        /// The `hash_builder` passed should implement the [`BuildHasher`] trait for the HashMap to be useful, see its documentation for details.
        /// 
        /// # Example
        /// ```
        /// use my_rusttools::gcacher::GCacher;
        /// use std::collections::hash_map::RandomState;
        /// 
        /// let s = RandomState::new();
        /// let mut cacher = GCacher::with_capacity_and_hasher(|x: &usize|x * x, 10, s);
        /// cacher.value_from(2);
        /// ```
        /// [`BuildHasher`]: std::hash::BuildHasher
        pub fn with_capacity_and_hasher(instancer: F, capacity: usize, hash_builder: S) -> GCacher<K, F, V, S> {
            Self::create(instancer, HashMap::with_capacity_and_hasher(capacity, hash_builder))
        }
    }

impl<K, F, V> Deref for GCacher<K, F, V>
where
    K: Eq + Hash,
    F: Fn(&K) -> V {
        type Target = HashMap<K, V>;

        fn deref(&self) -> &Self::Target {
            &self.cache
        }
    }

impl<K, F, V> From<GCacher<K, F, V>> for HashMap<K, V>
where
    K: Eq + Hash,
    F: Fn(&K) -> V {
        fn from(unwrap: GCacher<K, F, V>) -> HashMap<K, V> {
            unwrap.cache
        }
    }

impl<K, F, V> From<GCacher<K, F, V>> for (F, HashMap<K, V>) 
where
    K: Eq + Hash,
    F: Fn(&K) -> V {
        fn from(unwrap: GCacher<K, F, V>) -> (F, HashMap<K, V>) {
            (unwrap.instancer, unwrap.cache)
        }
    }