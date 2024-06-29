//! Code of `Cache` and `ThreadSafeCache` struct which provides functionalities of caching.

use std::collections::HashMap;
use tokio::sync::Mutex;

use crate::{common::{CacheEntry, KeyRef}, eviction_policies::common::EvictionPolicy};

/// This struct, `Cache<K, V, P>`, implements a generic in-memory cache. It utilizes a `HashMap` to store key-value pairs and allows customization of the eviction policy through the `P` generic type, which must implement the `EvictionPolicy<K>` trait.
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

    /// This function constructs a new cache with the provided `max_size`, `eviction_policy`.

    pub fn new(max_size: usize, eviction_policy: P) -> Self {
        Cache {
            max_size,
            cache: HashMap::new(),
            eviction_policy,
        }
    }

    /// Retrieves the value associated with the given key from the cache.

    /// This function attempts to retrieve the value for the provided `key`. It checks if the key exists in the cache, and if so, calls the eviction policy's `on_get` method. If the key is found, an immuatable reference to the value is returned. Otherwise, `None` is returned.

    pub fn get(&mut self, key: &K) -> Option<&V>
    {
        if let Some((key, val)) = self.cache.get_key_value(key) {
            self.eviction_policy.on_get(&KeyRef::new(key));
            return Some(&val.value);
        }
        None
    }

    /// Retrieves mutable pointer to the value associated with the given key from the cache.

    /// This function attempts to retrieve the value for the provided `key`. It checks if the key exists in the cache, and if so, calls the eviction policy's `on_get` method. If the key is found, an muatable reference to the value is returned. Otherwise, `None` is returned.

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

    /// This function inserts a new key-value pair into the cache. It checks if the cache is at its maximum size, and if necessary, evicts an entry using the eviction policy. The new key-value pair is then inserted into the cache along with a `CacheEntry` and the eviction policy's `on_set` method is called.

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

    /// This function removes the entry associated with the provided `key` from the cache. It removes the entry if it exists. If an entry is removed, the eviction policy's `remove` method is called.

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

    /// Returns a raw pointer to the value associated with the given key.
    ///
    /// Returns a raw pointer to the value associated with the given key, if it exists
    /// in the cache. This method is unsafe due to potential dangling pointers and should
    /// only be used in environments where it is safe to manage raw pointers manually.
    fn get_raw(&mut self, key: &K) -> Option<*const V> {
        self.get(key).map(|x| x as *const V)
    }
}


/// A thread-safe and async wrapper around `Cache` using `Mutex` for synchronization.

pub struct ThreadSafeCache<K, V, P>
where
    K: Eq + std::hash::Hash + Clone ,
    P: EvictionPolicy<K>,
{
    cache: Mutex<Cache<K, V, P>>
}

impl<K, V, P> ThreadSafeCache <K, V, P>
where
    K: Eq + std::hash::Hash + Clone + std::fmt::Debug,
    P: EvictionPolicy<K>,
    V: Clone
{
    /// Creates a new `ThreadSafeCache` instance.
    ///
    /// Constructs a new thread-safe cache with the provided `max_size` and `eviction_policy`.

    pub fn new(max_size: usize, eviction_policy: P) -> Self {
        ThreadSafeCache {
            cache: Mutex::new(Cache::new(max_size, eviction_policy))
        }
    }

    /// Retrieves the value associated with the given key from the cache.
    ///
    /// Asynchronously retrieves the value associated with the provided `key` from the cache.
    /// Returns `None` if the key is not found.
    

    pub async fn get(&self, key: &K) -> Option<V>
    {
        self.cache.lock().await.get(key).cloned()
    }

    /// Retrieves a reference to the value associated with the given key from the cache.
    ///
    /// Asynchronously retrieves a reference to the value associated with the provided `key` from the cache.
    /// Returns `None` if the key is not found.
    ///
    /// **Safety Note:** This method returns a reference that may become invalid in a multithreaded environment
    /// due to potential concurrent modifications. Use with caution in single-threaded environments only.
    
    pub async fn get_ref(&self, key: &K) -> Option<&V>
    {
        self.cache.lock().await.get_raw(key).map(|x| unsafe{x.as_ref()}).flatten()
    }

    /// Inserts a new key-value pair into the cache.
    ///
    /// Asynchronously inserts a new key-value pair into the cache.
     
    pub async fn put(&self, key: K, value: V) {
        self.cache.lock().await.put(key, value)
    }

    /// Removes the entry with the given key from the cache.
    ///
    /// Asynchronously removes the entry associated with the provided `key` from the cache.
    pub async fn remove(&self, key: &K) {
        self.cache.lock().await.remove(key)
    }

    /// Checks if the cache contains the given key.
    ///
    /// Asynchronously checks if the cache contains the provided `key`.
    pub async fn contains_key(&self, key: &K) -> bool {
        return self.cache.lock().await.contains_key(&key);
    }

    /// Returns the current size of the cache.
    ///
    /// Asynchronously returns the current number of entries in the cache.
    pub async fn size(&self) -> usize {
        return self.cache.lock().await.size();
    }
}

