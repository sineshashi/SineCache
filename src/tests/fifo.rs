//!Includes tests regarding Caching with FIFO Policy.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::cache::Cache;
use crate::common::CacheEntry;
use crate::eviction_policies::fifo::FIFO;

// Basic functionality tests
#[test]
fn test_put_get() {
    let mut cache = Cache::new(2, FIFO::new());
    let key1 = "key1".to_string();
    let value1 = CacheEntry { value: "value1".to_string() };
    cache.put(key1.clone(), value1.clone());

    assert!(
        cache.get(&key1).is_some_and(|x| {x.value == value1.value})
    );

    let key2 = "key2".to_string();
    let value2 = CacheEntry { value: "value2".to_string() };
    cache.put(key2.clone(), value2.clone());

    assert!(
        cache.get(&key1).is_some_and(|x| x.value == value1.value)
    );
    assert!(
        cache.get(&key2).is_some_and(|x| x.value == value2.value)
    );
}

#[test]
fn test_eviction() {
    let mut cache = Cache::new(2, FIFO::new());
    let key1 = "key1".to_string();
    let value1 = CacheEntry { value: "value1".to_string() };
    cache.put(key1.clone(), value1.clone());

    let key2 = "key2".to_string();
    let value2 = CacheEntry { value: "value2".to_string() };
    cache.put(key2.clone(), value2.clone());

    let key3 = "key3".to_string();
    let value3 = CacheEntry { value: "value3".to_string() };
    cache.put(key3.clone(), value3.clone());

    assert!(
        cache.get(&key1).is_none()
    );
    assert!(
        cache.get(&key2).is_some_and(|x| x.value == value2.value)
    );
    assert!(
        cache.get(&key3).is_some_and(|x| x.value == value3.value)
    );
}

/// Test getting mutable reference and removing items from the cache.
#[test]
fn test_get_mut_and_remove() {
    // Create a new cache with FIFO eviction policy and capacity of 2
    let mut cache = Cache::new(2, FIFO::new());

    // Insert two items into the cache
    cache.put("K1".to_string(), 1);
    cache.put("K2".to_string(), 2);

    // Get mutable reference to "K1" and modify its value
    if let Some(value) = cache.get_mut(&"K1".to_string()) {
        *value = 10;
    }

    // Remove "K2" from the cache
    cache.remove(&"K2".to_string());

    // Assert that "K1" has been updated and "K2" has been removed
    assert_eq!(cache.get(&"K1".to_string()), Some(&10));
    assert_eq!(cache.get(&"K2".to_string()), None);
}

/// Test checking if a key exists in the cache.
#[test]
fn test_contains_key() {
    // Create a new cache with FIFO eviction policy and capacity of 2
    let mut cache = Cache::new(2, FIFO::new());

    // Insert two items into the cache
    cache.put("K1".to_string(), 1);
    cache.put("K2".to_string(), 2);

    // Assert that "K1" exists in the cache and "K3" does not
    assert!(cache.contains_key(&"K1".to_string()));
    assert!(!cache.contains_key(&"K3".to_string()));
}

/// Test getting the current size of the cache.
#[test]
fn test_size() {
    // Create a new cache with FIFO eviction policy and capacity of 2
    let mut cache = Cache::new(2, FIFO::new());

    // Insert two items into the cache
    cache.put("K1".to_string(), 1);
    cache.put("K2".to_string(), 2);

    // Assert that the size of the cache is 2
    assert_eq!(cache.size(), 2);
}
