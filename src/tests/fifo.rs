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

// Concurrency test - concurrent put and get
#[test]
fn test_concurrent_put_get() {
    let cache = Arc::new(Mutex::new(Cache::new(10, FIFO::new())));
    let num_threads = 4;

    let mut threads = Vec::with_capacity(num_threads);
    for _ in 0..num_threads {
        let cache_clone = cache.clone();
        let thread = thread::spawn(move || {
            for i in 0..100 {
                let key = format!("key{}", i);
                let value = CacheEntry { value: key.clone() };
                cache_clone.lock().unwrap().put(key, value);
            }
        });
        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    // Verify some entries are still in the cache
    let lock = cache.lock().unwrap();
    assert!(lock.size() > 0);
}

// This is just an example, adjust based on your implementation
// You might need additional synchronization for eviction
#[test]
fn test_concurrent_eviction() {
    let cache = Arc::new(Mutex::new(Cache::new(2, FIFO::new())));
    let num_threads = 4;

    let mut threads = Vec::with_capacity(num_threads);
    for _ in 0..num_threads {
        let cache_clone = cache.clone();
        let thread = thread::spawn(move || {
            for i in 0..100 {
                let key = format!("key{}", i);
                let value = CacheEntry { value: key.clone() };
                cache_clone.lock().unwrap().put(key, value);
            }
        });
        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    // Verify only 2 entries are in the cache
    let lock = cache.lock().unwrap();
    assert_eq!(lock.size(), 2);
}