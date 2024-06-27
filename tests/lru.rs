use std::{
    collections::HashMap, sync::{Arc, Mutex}, thread
};

use sine_cache::{
    cache::Cache,
    common::CacheEntry,
    eviction_policies::{common::EvictionPolicy, lru::LRU},
};

/// Test basic functionality of putting and getting items from the cache.
#[test]
fn test_basic_get_put() {
    let mut cache = Cache::new(2, LRU::new());

    cache.put("K1".to_string(), 1);
    cache.put("K2".to_string(), 2);

    assert_eq!(cache.get(&"K1".to_string()), Some(&1));
    assert_eq!(cache.get(&"K2".to_string()), Some(&2));
}

/// Test LRU eviction policy when inserting more items than the cache capacity.
#[test]
fn test_lru_eviction() {
    let mut cache = Cache::new(2, LRU::new());

    cache.put("K1".to_string(), 1);
    cache.put("K2".to_string(), 2);
    cache.put("K1".to_string(), 10);
    cache.put("K3".to_string(), 3);

    assert_eq!(cache.get(&"K1".to_string()), Some(&10));
    assert!(cache.contains_key(&"K1".to_string()));
    assert_eq!(cache.get(&"K2".to_string()), None);
    assert_eq!(cache.get(&"K3".to_string()), Some(&3));
    cache.put("K4".to_string(), 4);
    assert_eq!(cache.get(&"K4".to_string()), Some(&4));
    assert_eq!(cache.get(&"K1".to_string()), None);
    assert!(!cache.contains_key(&"K1".to_string()));
}

/// Test getting mutable reference and removing items from the cache.
#[test]
fn test_get_mut_and_remove() {
    let mut cache = Cache::new(2, LRU::new());

    cache.put("K1".to_string(), 1);
    cache.put("K2".to_string(), 2);

    if let Some(value) = cache.get_mut(&"K1".to_string()) {
        *value = 10;
    }

    cache.remove(&"K2".to_string());

    assert_eq!(cache.get(&"K1".to_string()), Some(&10));
    assert_eq!(cache.get(&"K2".to_string()), None);
}

#[test]
fn test_contains_key() {
    let mut cache = Cache::new(2, LRU::new());

    cache.put("K1".to_string(), 1);
    cache.put("K2".to_string(), 2);

    assert!(cache.contains_key(&"K1".to_string()));
    assert!(!cache.contains_key(&"K3".to_string()));
}

#[test]
fn test_size() {
    let mut cache = Cache::new(2, LRU::new());

    cache.put("K1".to_string(), 1);
    cache.put("K2".to_string(), 2);

    assert_eq!(cache.size(), 2);
}
