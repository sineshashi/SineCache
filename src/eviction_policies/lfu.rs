//! Implements an LFU (Least Frequently Used) eviction policy for a cache.
//!
//! This module provides a struct `LFU<K>` that implements an LFU eviction policy for a cache.
//! It tracks the frequency of accesses to keys and evicts the least frequently accessed keys
//! when the cache reaches its capacity. The LFU policy uses an internal hashmap (`map`) to
//! keep track of each key's access frequency and a hashmap of LRU caches (`freq_nodes`) to
//! store keys grouped by their access frequencies.
//!
//! ## Implementation Details
//! - `LFU<K>` struct:
//!   - `map`: Maps each `K` to its access frequency.
//!   - `least_freq`: Tracks the smallest frequency among all keys.
//!   - `freq_nodes`: Maps access frequencies to LRU caches storing keys accessed at that frequency.
//!
//! - Methods:
//!   - `new()`: Creates a new instance of `LFU<K>` with empty internal structures.
//!   - `record_access(&mut self, key: &K)`: Records an access to a key, updating its frequency
//!     and moving it to the appropriate frequency list.
//!   - `remove_key(&mut self, key: K)`: Removes a key from the LFU cache and adjusts internal state.
//!   - `remove_lfu_key(&mut self) -> Option<K>`: Evicts the least frequently used key from the LFU cache.
//!
//!
//! This module is part of a larger caching library and is used to manage the eviction policy
//! within a cache system. It is designed to be efficient and scalable, providing fast access
//! to frequently accessed keys while managing memory usage effectively.
//!
//! ### Related Modules
//! - `lru`: Implements the LRU (Least Recently Used) eviction policy for comparison and internal use.
//!
//!
//! This LFU eviction policy is suitable for applications requiring efficient management of
//! frequently accessed data in memory, ensuring optimal performance under high load conditions.

use std::collections::HashMap;

use super::{
    common::EvictionPolicy,
    lru::LRU,
};

/// LFU (Least Frequently Used) eviction policy for a cache.
///
/// This struct, `LFU<K>`, implements an LFU eviction policy for a cache. It tracks the frequency
/// of accesses to keys and evicts keys that are least frequently accessed when space is needed.
pub struct LFU<K>
where
    K: Eq + std::hash::Hash + Clone + std::fmt::Debug, // Key requirements: Eq, Hash, Clone, Debug
{
    /// Maps each key to its access frequency count.
    map: HashMap<K, usize>,

    /// Tracks the smallest frequency of any key in the cache.
    least_freq: usize,

    /// Stores keys grouped by their access frequencies using LRU structures.
    /// Each frequency is associated with an LRU list containing keys accessed at that frequency.
    freq_nodes: HashMap<usize, LRU<K>>,
}

impl<K: Eq + std::hash::Hash + Clone + std::fmt::Debug> LFU<K> {
    /// Creates a new instance of `LFU`.
    ///
    /// Initializes an empty LFU cache with default values.
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            least_freq: 0,
            freq_nodes: HashMap::new(),
        }
    }

    /// Records an access to a key, incrementing its access frequency and updating internal structures.
    ///
    /// If the key exists in the LFU cache, its access frequency is incremented. The key is then moved
    /// to the appropriate frequency list in `freq_nodes` using an LRU strategy.
    fn record_access(&mut self, key: &K) {
        if let Some(freq) = self.map.get_mut(key) {
            // Remove the key from its current frequency list
            if *freq != 0 {
                if let Some(lru) = self.freq_nodes.get_mut(freq) {
                    lru.remove(key.clone()); // Remove key from the current LRU list
                                             // If the list becomes empty and it's the least frequent, adjust `least_freq`
                    if lru.len() == 0 && *freq == self.least_freq {
                        self.least_freq += 1;
                    }
                } else {
                    panic!("Should never happen!!!!!!!!!!11");
                }
            } else {
                self.least_freq += 1;
            }

            // Increment the frequency of access for the key
            *freq += 1;

            // Add the key to the new frequency list (create one if it doesn't exist)
            self.freq_nodes
                .entry(*freq)
                .or_insert_with(LRU::new) // Create a new LRU list if necessary
                .on_set(key.clone()); // Add key to the LRU list at the new frequency
        }
    }

    /// Removes a key from the LFU cache, adjusting internal state accordingly.
    ///
    /// Removes the key and its frequency count from `map` and removes the key from the appropriate
    /// frequency list in `freq_nodes`.
    fn remove_key(&mut self, key: K) {
        if let Some(freq) = self.map.remove(&key) {
            if let Some(lru) = self.freq_nodes.get_mut(&freq) {
                lru.remove(key.clone()); // Remove key from the LRU list at the specific frequency
            }

            if self.map.len() == 0 {
                self.least_freq = 0;
                return;
            };
            let mut lruopt = self.freq_nodes.get(&self.least_freq);
            while lruopt.is_none() || lruopt.is_some_and(|x| x.len() == 0) {
                self.least_freq += 1;
                lruopt = self.freq_nodes.get(&self.least_freq);
            }
        }
    }

    /// Evicts the least frequently used key from the LFU cache.
    ///
    /// Evicts the least frequently used key by removing it from the lowest frequency list in `freq_nodes`.
    fn remove_lfu_key(&mut self) -> Option<K> {
        if self.least_freq == 0 {
            return None; // No keys to evict if all frequencies are empty
        }
        let evicted = if let Some(lru) = self.freq_nodes.get_mut(&self.least_freq) {
            if let Some(evicted) = lru.evict() {
                // Evict the least used key from the LRU list
                self.map.remove(&evicted); // Remove the key from the frequency map
                Some(evicted)
            } else {
                None
            }
        } else {
            None
        };
        if self.map.len() == 0 {
            self.least_freq = 0;
            return evicted;
        };
        let mut lruopt = self.freq_nodes.get(&self.least_freq);
        while lruopt.is_none() || lruopt.is_some_and(|x| x.len() == 0) {
            self.least_freq += 1;
            lruopt = self.freq_nodes.get(&self.least_freq);
        }
        evicted
    }
}

impl<K: Eq + std::hash::Hash + Clone + std::fmt::Debug> EvictionPolicy<K> for LFU<K> {
    /// Called when a value associated with a key is retrieved from the cache.
    ///
    /// Records the access of the key to adjust its frequency in the LFU cache.
    fn on_get(&mut self, key: &K) {
        self.record_access(key);
    }

    /// Called when a new key-value pair is inserted into the cache.
    ///
    /// Inserts the key into the LFU cache and initializes its access frequency if it's new.
    fn on_set(&mut self, key: K) {
        if !self.map.contains_key(&key) {
            self.map.insert(key.clone(), 0); // Insert the key with an initial frequency of 0
            self.least_freq = 0; // Reset `least_freq` because a new key is added
        }
        self.record_access(&key); // Record the access of the key to adjust its frequency
    }

    /// Evicts a key-value pair from the cache based on the LFU eviction policy.
    ///
    /// Evicts the least frequently used key-value pair from the LFU cache.
    fn evict(&mut self) -> Option<K> {
        return self.remove_lfu_key();
    }

    /// Removes a key-value pair from the LFU cache based on the key.
    ///
    /// Removes the specified key and its associated value from the LFU cache.
    fn remove(&mut self, key: K) {
        self.remove_key(key);
    }
}
