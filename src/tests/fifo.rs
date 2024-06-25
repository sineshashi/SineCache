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
        cache.get(&key1).is_some_and(|x| x.value == value1.value)
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
