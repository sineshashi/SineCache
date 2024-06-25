use std::{sync::{Arc, Mutex}, thread};

use crate::{cache::Cache, common::CacheEntry, eviction_policies::{common::EvictionPolicy, lru::LRU}};

/// Test basic functionality of putting and getting items from the cache.
#[test]
fn test_basic_get_put() {
    // Create a new cache with LRU eviction policy and capacity of 2
    let mut cache = Cache::new(2, LRU::new());

    // Insert two items into the cache
    cache.put("K1".to_string(), 1);
    cache.put("K2".to_string(), 2);

    // Assert that the items can be retrieved correctly
    assert_eq!(cache.get(&"K1".to_string()), Some(&1));
    assert_eq!(cache.get(&"K2".to_string()), Some(&2));
}

/// Test LRU eviction policy when inserting more items than the cache capacity.
#[test]
fn test_lru_eviction() {
    // Create a new cache with LRU eviction policy and capacity of 2
    let mut cache = Cache::new(2, LRU::new());

    // Insert three items into the cache, exceeding its capacity
    cache.put("K1".to_string(), 1);
    cache.put("K2".to_string(), 2);
    cache.put("K3".to_string(), 3);

    // Assert that the least recently used item "K1" has been evicted
    assert_eq!(cache.get(&"K1".to_string()), None);
    // Assert that "K2" is still in the cache
    assert_eq!(cache.get(&"K2".to_string()), Some(&2));
    // Assert that "K3" is in the cache
    assert_eq!(cache.get(&"K3".to_string()), Some(&3));
}

/// Test getting mutable reference and removing items from the cache.
#[test]
fn test_get_mut_and_remove() {
    // Create a new cache with LRU eviction policy and capacity of 2
    let mut cache = Cache::new(2, LRU::new());

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
    // Create a new cache with LRU eviction policy and capacity of 2
    let mut cache = Cache::new(2, LRU::new());

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
    // Create a new cache with LRU eviction policy and capacity of 2
    let mut cache = Cache::new(2, LRU::new());

    // Insert two items into the cache
    cache.put("K1".to_string(), 1);
    cache.put("K2".to_string(), 2);

    // Assert that the size of the cache is 2
    assert_eq!(cache.size(), 2);
}
