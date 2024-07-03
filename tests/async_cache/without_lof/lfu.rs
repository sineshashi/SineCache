use sine_cache::{
    cache::AsyncCache, config::{AsyncCacheConfig, EvictionAsyncConfig}
};
use std::sync::Arc;
use tokio::{sync::Semaphore, test};

#[test]
async fn test_basic_get_put() {
    let cache = Arc::new(AsyncCache::new(AsyncCacheConfig::LFU(EvictionAsyncConfig {max_size: 2, aof_config: None})).await);

    // Insert two items into the cache
    cache.put("K1".to_string(), 1).await;
    cache.put("K2".to_string(), 2).await;

    // Assert that the items can be retrieved correctly
    assert_eq!(cache.get(&"K1".to_string()).await, Some(1));
    assert_eq!(cache.get(&"K2".to_string()).await, Some(2));
}

#[test]
async fn test_get_ref_and_remove() {
    let cache = Arc::new(AsyncCache::new(AsyncCacheConfig::LFU(EvictionAsyncConfig {max_size: 2, aof_config: None})).await);

    cache.put("K1".to_string(), 1).await;
    cache.put("K2".to_string(), 2).await;

    assert_eq!(cache.get_ref(&"K1".to_string()).await, Some(&1));

    cache.remove(&"K2".to_string()).await;
    assert_eq!(cache.get_ref(&"K2".to_string()).await, None);
}

#[test]
async fn test_contains_key() {
    let cache = Arc::new(AsyncCache::new(AsyncCacheConfig::LFU(EvictionAsyncConfig {max_size: 2, aof_config: None})).await);

    cache.put("K1".to_string(), 1).await;
    cache.put("K2".to_string(), 2).await;

    assert!(cache.contains_key(&"K1".to_string()).await);
    assert!(!cache.contains_key(&"K3".to_string()).await);
}

#[test]
async fn test_size() {
    let cache = Arc::new(AsyncCache::new(AsyncCacheConfig::LFU(EvictionAsyncConfig {max_size: 2, aof_config: None})).await);

    cache.put("K1".to_string(), 1).await;
    cache.put("K2".to_string(), 2).await;

    assert_eq!(cache.size().await, 2);
}

#[test]
async fn test_lfu_eviction() {
    let cache = Arc::new(AsyncCache::new(AsyncCacheConfig::LFU(EvictionAsyncConfig {max_size: 2, aof_config: None})).await);

    cache.put("K1".to_string(), 1).await;
    cache.put("K2".to_string(), 2).await;
    cache.put("K3".to_string(), 3).await;

    assert_eq!(cache.get(&"K1".to_string()).await, None);
    assert_eq!(cache.get(&"K2".to_string()).await, Some(2));
    assert_eq!(cache.get(&"K3".to_string()).await, Some(3));

    cache.put("K2".to_string(), 20).await;
    cache.put("K4".to_string(), 4).await;

    assert_eq!(cache.get(&"K3".to_string()).await, None);
    assert_eq!(cache.get(&"K4".to_string()).await, Some(4));
    assert_eq!(cache.get(&"K2".to_string()).await, Some(20));
}


#[test]
async fn test_thread_safe_lru_cache() {
    const NUM_THREADS: usize = 10;
    const MAX_KEYS_PER_THREAD: usize = 100;

    // Create an LRU eviction policy with a max capacity
    let cache = Arc::new(AsyncCache::new(AsyncCacheConfig::FIFO(EvictionAsyncConfig {max_size: MAX_KEYS_PER_THREAD, aof_config: None})).await);

    let semaphore = Arc::new(Semaphore::new(NUM_THREADS/3+1));

    let mut handles = vec![];
    for thread_id in 0..NUM_THREADS {
        let cache = Arc::clone(&cache);

        let handle = tokio::spawn(async move {

            for i in 0..MAX_KEYS_PER_THREAD {
                let value = format!("Value{}_{}", thread_id, i);
                cache.put(i, value.clone()).await;
                assert_eq!(cache.get(&i).await, Some(value));
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        let semaphore = Arc::clone(&semaphore);
        let _permit = semaphore.acquire().await.unwrap();
        handle.await.unwrap();
    }

    assert_eq!(cache.size().await, MAX_KEYS_PER_THREAD);

    for i in 0..MAX_KEYS_PER_THREAD {
        assert!(cache.contains_key(&i).await);
        if let Some(value) = cache.get(&i).await {
            println!("{:?}", value);
        }
    }
}
