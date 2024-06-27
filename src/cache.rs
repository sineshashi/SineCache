//! Code of Cache struct which provides functionalities of caching.

use std::collections::HashMap;

use crate::{common::{CacheEntry, KeyRef}, eviction_policies::common::EvictionPolicy};

/// A thread-safe key-value cache with configurable eviction policy.
/// This struct, `Cache<K, V, P>`, implements a generic in-memory cache with thread-safety. It utilizes a `HashMap` to store key-value pairs and allows customization of the eviction policy through the `P` generic type, which must implement the `EvictionPolicy<K>` trait.
/// 


pub struct Cache<K, V, P>
where
    K: Eq + std::hash::Hash + Clone ,
    P: EvictionPolicy<K>,
{
    /// The maximum size of the cache in number of entries.
    max_size: usize,

    /// The internal HashMap storing key-value pairs with associated cache entries.
    cache: HashMap<K, CacheEntry<V>>,

    /// The eviction policy instance used by the cache to determine eviction behavior.
    eviction_policy: P,
}

impl<K, V, P> Cache<K, V, P>
where
    K: Eq + std::hash::Hash + Clone + std::fmt::Debug,
    P: EvictionPolicy<K>,
{
    /// Creates a new `Cache` instance.

    /// This function constructs a new cache with the provided `max_size`, `eviction_policy`. An internal Mutex is created for thread-safe access.

    pub fn new(max_size: usize, eviction_policy: P) -> Self {
        Cache {
            max_size,
            cache: HashMap::new(),
            eviction_policy,
        }
    }

    /// Retrieves the value associated with the given key from the cache.

    /// This function attempts to retrieve the value for the provided `key`. It acquires the lock, checks if the key exists in the cache, and if so, calls the eviction policy's `on_get` method. If the key is found, an immuatable reference to the value is returned. Otherwise, `None` is returned.

    pub fn get(&mut self, key: &K) -> Option<&V>
    {
        if let Some((key, val)) = self.cache.get_key_value(key) {
            self.eviction_policy.on_get(&KeyRef::new(key));
            return Some(&val.value);
        }
        None
    }

    /// Retrieves mutable pointer to the value associated with the given key from the cache.

    /// This function attempts to retrieve the value for the provided `key`. It acquires the lock, checks if the key exists in the cache, and if so, calls the eviction policy's `on_get` method. If the key is found, an muatable reference to the value is returned. Otherwise, `None` is returned.

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V>
    {
        if let Some((k, _)) = self.cache.get_key_value(key) {
            self.eviction_policy.on_get(&KeyRef::new(k));
            return Some(&mut self.cache.get_mut(&key).unwrap().value);
        } else {
            None
        }

    }

    /// Inserts a new key-value pair into the cache.

    /// This function inserts a new key-value pair into the cache. It acquires the lock, checks if the cache is at its maximum size, and if necessary, evicts an entry using the eviction policy. The new key-value pair is then inserted into the cache along with a `CacheEntry` and the eviction policy's `on_set` method is called.

    pub fn put(&mut self, key: K, value: V) {
        if self.cache.len() >= self.max_size && !self.contains_key(&key){
            if let Some(evicted) = self.eviction_policy.evict() {
                self.cache.remove(unsafe{&*evicted.key});
            }
        }
        match self.cache.get_mut(&key) {
            Some(v) => {
                v.value = value;
            },
            None => {
                self.cache.insert(key.clone(), CacheEntry::new(value));
            }
        };
        match self.cache.get_key_value(&key){
            None => {},
            Some((k, _)) => {
                let keyref = KeyRef::new(k);
                self.eviction_policy.on_set(keyref);
            }
        };
    }

    /// Removes the entry with the given key from the cache.

    /// This function removes the entry associated with the provided `key` from the cache. It acquires the lock and removes the entry if it exists. If an entry is removed, the eviction policy's `remove` method is called.

    pub fn remove(&mut self, key: &K) {
        if let Some((k, _)) = self.cache.remove_entry(key) {
            let key = KeyRef::new(&k);
            self.eviction_policy.remove(key);
        }
    }

    ///Checks if key is already in cache.

    pub fn contains_key(&self, key: &K) -> bool {
        return self.cache.contains_key(&key);
    }

    ///Returns the current size of the cache. The number of keys in the cache at the moment.
    pub fn size(&self) -> usize {
        return self.cache.len();
    }
}

