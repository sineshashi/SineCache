//!Includes tests regarding Caching with FIFO Policy.

use sine_cache::cache::Cache;
use sine_cache::common::CacheEntry;
use sine_cache::config::CacheConfig;

// Basic functionality tests
#[test]
fn test_put_get() {
    let mut cache = Cache::new(sine_cache::config::CacheSyncConfig::FIFO(CacheConfig{max_size: 2}));
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
    let mut cache = Cache::new(sine_cache::config::CacheSyncConfig::FIFO(CacheConfig{max_size: 2}));
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
    let mut cache = Cache::new(sine_cache::config::CacheSyncConfig::FIFO(CacheConfig{max_size: 2}));

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
    let mut cache = Cache::new(sine_cache::config::CacheSyncConfig::FIFO(CacheConfig{max_size: 2}));

    cache.put("K1".to_string(), 1);
    cache.put("K2".to_string(), 2);

    assert!(cache.contains_key(&"K1".to_string()));
    assert!(!cache.contains_key(&"K3".to_string()));
}

/// Test getting the current size of the cache.
#[test]
fn test_size() {
    let mut cache = Cache::new(sine_cache::config::CacheSyncConfig::FIFO(CacheConfig{max_size: 2}));

    cache.put("K1".to_string(), 1);
    cache.put("K2".to_string(), 2);

    assert_eq!(cache.size(), 2);
}
