//! Implements a First-In-First-Out (FIFO) eviction policy for a cache.
//!
//! This module provides a FIFO (First-In-First-Out) eviction policy implementation (`FIFO<K>`) for a cache.
//! It maintains a queue using `VecDeque<K>` to store keys in the order they were inserted. The eviction policy
//! evicts the least recently accessed key (the one at the front of the queue).

use std::collections::{HashSet, VecDeque};

use super::common::EvictionPolicy;

/// A First-In-First-Out (FIFO) eviction policy for a cache.
///
/// This struct, `FIFO<K>`, implements a FIFO eviction policy for a cache. It maintains a queue using `VecDeque<K>`
/// to store keys in the order of insertion. The eviction policy evicts the least recently accessed key (the one at
/// the front of the queue).
pub struct FIFO<K> {
    /// The queue that stores keys in the order of insertion (FIFO).
    queue: VecDeque<K>,

    /// A set containing keys that have been logically removed from the queue but not yet evicted.
    tombstones: HashSet<K>,
}

impl<K: Eq + std::hash::Hash + Clone > FIFO<K> {
    /// Creates a new `FIFO` eviction policy instance.
    ///
    /// Constructs a new `FIFO` eviction policy with an empty queue and an empty tombstone set.
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            tombstones: HashSet::new(),
        }
    }
}

impl<K: Eq + std::hash::Hash + Clone > EvictionPolicy<K> for FIFO<K> {
    /// Called when a value is retrieved from the cache using the given key.
    ///
    /// In a FIFO policy, there's no specific action required upon a get operation. This function is a placeholder.
    fn on_get(&mut self, _key: &K) {}

    /// Called when a new value is inserted into the cache using the given key.
    ///
    /// Adds a cloned copy of the `key` to the back of the `queue`, maintaining the FIFO order of key insertion.
    fn on_set(&mut self, key: K) {
        self.queue.push_back(key);
    }

    /// Attempts to evict a key-value pair from the cache according to the FIFO policy.
    ///
    /// Iteratively removes keys from the front of the `queue` until it encounters a non-tombstone key.
    /// It then evicts that key and returns `Some(key)`. If the queue becomes empty or only contains tombstones,
    /// it returns `None`.
    fn evict(&mut self) -> Option<K> {
        let mut evicted_key = None;
        while let Some(key) = self.queue.pop_front() {
            if self.tombstones.contains(&key) {
                self.tombstones.remove(&key);
            } else {
                evicted_key = Some(key);
                break;
            }
        }
        evicted_key
    }

    /// Removes the entry with the given key from the cache (logically).
    ///
    /// Marks the key for eviction by adding it to the `tombstones` set. The actual eviction happens during the `evict` function.
    fn remove(&mut self, key: K) {
        self.tombstones.insert(key);
    }
}
