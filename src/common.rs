//! Contains code of some common structs and traits which will be used across the library.

/// A cached entry representing a key-value pair.

/// This struct, `CacheEntry<T>`, is used to store a cached value of type `T` along with any additional information needed for the cache implementation. 

#[derive(Clone, Debug)]
pub struct CacheEntry<T> {
    /// The actual value stored in the cache entry.
    pub value: T,
}

impl<T> CacheEntry<T> {
    /// Creates a new `CacheEntry` instance.

    /// This function constructs a new `CacheEntry` with the provided `value` of type `T`.

    pub fn new(value: T) -> Self {
        CacheEntry { value }
    }
}
