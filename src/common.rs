//! Contains common structs and traits used throughout the library.

/// A cached entry representing a key-value pair.
///
/// This struct, `CacheEntry<T>`, stores a cached value of type `T` along
/// with any additional information needed by the cache implementation.
#[derive(Clone, Debug)]
pub struct CacheEntry<T> {
    /// The actual value stored in the cache entry.
    pub value: T,
}

impl<T> CacheEntry<T> {
    /// Creates a new `CacheEntry` instance.
    ///
    /// This function constructs a new `CacheEntry` with the provided `value`
    /// of type `T`.
    pub fn new(value: T) -> Self {
        CacheEntry { value }
    }
}

/// An opaque reference to a key type `K`.
///
/// This struct, `KeyRef<K>`, provides an opaque reference to a key of type `K`.
/// It allows for hashing and equality comparison while hiding the actual key type.
/// This can be useful for cache implementations that need to store keys in a
/// specific way, but still want to leverage the `Hash` and `PartialEq` traits.
#[derive(Clone)]
pub struct KeyRef<K> {
    pub key: *const K,
}

impl<K> KeyRef<K> {
    /// Creates a new `KeyRef` instance from a reference to a key.
    ///
    /// This function constructs a new `KeyRef<K>` by taking a reference to a
    /// value of type `K`. The actual key data is not copied, but rather referenced.
    pub fn new(key: &K) -> Self {
        KeyRef { key }
    }
}

impl<K: std::hash::Hash> std::hash::Hash for KeyRef<K> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unsafe { (*self.key).hash(state) }
    }
}

impl<K: PartialEq> PartialEq for KeyRef<K> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { (*self.key).eq(&*other.key) }
    }
}

impl<K: Eq> Eq for KeyRef<K> {}

impl<K: std::fmt::Debug> std::fmt::Debug for KeyRef<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("KeyRef { key: ")?;
        // Since we have a raw pointer, we can't directly print its value
        // for security reasons.
        unsafe {
            // This is safe only if the pointer points to valid memory
            // and the type K implements Debug.
            f.write_fmt(format_args!("{:?}", *self.key))?;
        }
        f.write_str(" }")
    }
}