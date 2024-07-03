//! Contains common structs and traits used throughout the library.

use serde::{Deserialize, Serialize};
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

/// Enum to indicate which operation is being performed.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Operation {
    Put,
    Get,
    Remove,
}

impl Operation {
    /// Provides corresponding number which can be saved in AOF to indentify the operation.
    /// `Get` = `0`
    /// `Put` = `1`
    /// `Remove` = `2`
    pub fn to_int(&self) -> i8 {
        match self {
            Self::Get => 0,
            Self::Put => 1,
            Self::Remove => 2,
        }
    }

    /// Loads the corresponding enum based on the provided integer.
    /// `Get` = `0`
    /// `Put` = `1`
    /// `Remove` = `2`
    pub fn from_int(i: u8) -> Self {
        if i == 0 {
            Self::Get
        } else if i == 1 {
            Self::Put
        } else if i == 2 {
            Self::Remove
        } else {
            panic!("Invalid integer {:?}", i);
        }
    }
}

/// struct to represent the single record in AOF.
#[derive(Clone)]
pub struct AOFRecord<K, V>
where
    for<'de> K: Deserialize<'de> + Serialize,
    for<'de> V: Deserialize<'de> + Serialize,
{
    pub key: K,
    pub value: Option<V>,
    pub operation: Operation,
}
