//! Code of Cache struct which provides functionalities of caching.

use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::{common::CacheEntry, eviction_policies::common::EvictionPolicy};

/// A thread-safe key-value cache with configurable eviction policy.

/// This struct, `Cache<K, V, P>`, implements a generic in-memory cache with thread-safety. It utilizes a `HashMap` to store key-value pairs and allows customization of the eviction policy through the `P` generic type, which must implement the `EvictionPolicy<K>` trait.

pub struct Cache<K, V, P>
where
    K: Eq + std::hash::Hash + Clone,
    P: EvictionPolicy<K>,
{
    /// The maximum size of the cache in number of entries.
    max_size: usize,

    /// The internal HashMap storing key-value pairs with associated cache entries.
    cache: HashMap<K, CacheEntry<V>>,

    /// The eviction policy instance used by the cache to determine eviction behavior.
    eviction_policy: P,

    /// A Mutex for thread-safe access to the cache's internal state.
    lock: Arc<Mutex<()>>,
}

impl<K, V, P> Cache<K, V, P>
where
    K: Eq + std::hash::Hash + Clone,
    P: EvictionPolicy<K>,
{
    /// Creates a new `Cache` instance.

    /// This function constructs a new cache with the provided `max_size`, `eviction_policy`. An internal Mutex is created for thread-safe access.

    pub fn new(max_size: usize, eviction_policy: P) -> Self {
        Cache {
            max_size,
            cache: HashMap::new(),
            eviction_policy,
            lock: Arc::new(Mutex::new(())),
        }
    }

    /// Retrieves the value associated with the given key from the cache.

    /// This function attempts to retrieve the value for the provided `key`. It acquires the lock, checks if the key exists in the cache, and if so, calls the eviction policy's `on_get` method. If the key is found, a cloned copy of the value is returned. Otherwise, `None` is returned.

    pub fn get(&mut self, key: &K) -> Option<V>
    where
        V: Clone,
    {
        let _guard = self.lock.lock().unwrap();
        if let Some(entry) = self.cache.get(key) {
            self.eviction_policy.on_get(key);
            return Some(entry.value.clone());
        }
        None
    }

    /// Inserts a new key-value pair into the cache.

    /// This function inserts a new key-value pair into the cache. It acquires the lock, checks if the cache is at its maximum size, and if necessary, evicts an entry using the eviction policy. The new key-value pair is then inserted into the cache along with a `CacheEntry` and the eviction policy's `on_set` method is called.

    pub fn put(&mut self, key: K, value: V) {
        let _guard = self.lock.lock().unwrap();
        if self.cache.len() >= self.max_size {
            if let Some(key) = self.eviction_policy.evict() {
                self.cache.remove(&key);
            }
        }
        self.cache.insert(key.clone(), CacheEntry::new(value));
        self.eviction_policy.on_set(&key);
    }

    /// Removes the entry with the given key from the cache.

    /// This function removes the entry associated with the provided `key` from the cache. It acquires the lock and removes the entry if it exists. If an entry is removed, the eviction policy's `remove` method is called.

    pub fn remove(&mut self, key: &K) {
        let _guard = self.lock.lock().unwrap();
        if self.cache.remove(key).is_some() {
            self.eviction_policy.remove(key);
        }
    }

    ///Checks if key is already in cache.

    pub fn contains_key(&self, key: &K) -> bool {
        let _guard = self.lock.lock().unwrap();
        return self.cache.contains_key(key);
    }

    ///Returns the current size of the cache. The number of keys in the cache at the moment.

    pub fn size(&self) -> usize {
        let _guard = self.lock.lock().unwrap();
        return self.cache.len();
    }
}

