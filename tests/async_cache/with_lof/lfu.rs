use rand::Rng;
use rand::{distributions::WeightedIndex, thread_rng};
use rand::distributions::Distribution;
use sine_cache::config::{CacheConfig, EvictionAOFConfig, EvictionAsyncConfig};
use sine_cache::{cache::{AsyncCache, Cache}, common::Operation, config::{AsyncCacheConfig, CacheSyncConfig}};

#[tokio::test]
async fn test_lfu_eviction_async_cache_with_periodic_flush()  -> Result<(), tokio::io::Error> {
    let cache_name = "test_lfu_eviction_async_cache_with_periodic_flush";
    let folder = ".";
    let flush_time = Some(500);
    let max_size = 50;
    let _ = tokio::fs::remove_file(format!("{}/{}.dat", folder, cache_name)).await;
    let async_cache: AsyncCache<String, String> = AsyncCache::new(
        AsyncCacheConfig::LFU(EvictionAsyncConfig {
            aof_config: Some(EvictionAOFConfig {
                folder: String::from(folder),
                cache_name:  String::from(cache_name),
                flush_time
            }),
            max_size: max_size
        })
    ).await;
    let mut cache: Cache<String, String> = Cache::new(CacheSyncConfig::LFU(CacheConfig{
        max_size
    }));
    // Define weights for different operations (adjust weights as needed)
    let weights = &[0.3, 0.5, 0.2];

    // Define possible operations
    let operations = vec![Operation::Put, Operation::Get, Operation::Remove];

    let weighted_dist = WeightedIndex::new(weights).unwrap();
    let mut rng = thread_rng();
    let mut rng1 = thread_rng();

    let num_ops = 250; // Adjust the number of random operations

    // Generate random operations and write to AOF
    for _ in 0..num_ops {
        let op = weighted_dist.sample(&mut rng);
        let i = rng1.gen_range(0..num_ops);
        let key = format!("key{}", i);
        let value = match &operations[op] {
            Operation::Put => Some(format!("value{}", i)),
            _ => None,
        };
        match operations[op].clone() {
            Operation::Get => {
                async_cache.get(&key).await;
                cache.get(&key);
            },
            Operation::Remove => {
                async_cache.remove(&key).await;
                cache.remove(&key);
            },
            Operation::Put => {
                cache.put(key.clone(), value.clone().unwrap());
                async_cache.put(key.clone(), value.clone().unwrap()).await;
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    drop(async_cache);
    let async_cache = AsyncCache::new(
        AsyncCacheConfig::LFU(EvictionAsyncConfig {
            aof_config: Some(EvictionAOFConfig {
                folder: String::from(folder),
                cache_name:  String::from(cache_name),
                flush_time
            }),
            max_size
        })
    ).await;
    
    for i in 0..num_ops {
        let key = format!("key{}", i);
        assert_eq!(cache.contains_key(&key), async_cache.contains_key(&key).await);
        assert_eq!(cache.get(&key).cloned(), async_cache.get(&key).await);
    };
    tokio::fs::remove_file(format!("{}/{}.dat", folder, cache_name)).await?;
    Ok(())
}

#[tokio::test]
async fn test_lfu_eviction_async_cache_with_instant_flush()  -> Result<(), tokio::io::Error> {
    let cache_name = "test_lfu_eviction_async_cache_with_instant_flush";
    let folder = ".";
    let flush_time = None;
    let max_size = 50;
    let _ = tokio::fs::remove_file(format!("{}/{}.dat", folder, cache_name)).await;
    let async_cache: AsyncCache<String, String> = AsyncCache::new(
        AsyncCacheConfig::LFU(EvictionAsyncConfig {
            aof_config: Some(EvictionAOFConfig {
                folder: String::from(folder),
                cache_name:  String::from(cache_name),
                flush_time
            }),
            max_size: max_size
        })
    ).await;
    let mut cache: Cache<String, String> = Cache::new(CacheSyncConfig::LFU(CacheConfig{
        max_size
    }));
    // Define weights for different operations (adjust weights as needed)
    let weights = &[0.3, 0.5, 0.2];

    // Define possible operations
    let operations = vec![Operation::Put, Operation::Get, Operation::Remove];

    let weighted_dist = WeightedIndex::new(weights).unwrap();
    let mut rng = thread_rng();
    let mut rng1 = thread_rng();

    let num_ops = 250; // Adjust the number of random operations

    // Generate random operations and write to AOF
    for _ in 0..num_ops {
        let op = weighted_dist.sample(&mut rng);
        let i = rng1.gen_range(0..num_ops);
        let key = format!("key{}", i);
        let value = match &operations[op] {
            Operation::Put => Some(format!("value{}", i)),
            _ => None,
        };
        match operations[op].clone() {
            Operation::Get => {
                async_cache.get(&key).await;
                cache.get(&key);
            },
            Operation::Remove => {
                async_cache.remove(&key).await;
                cache.remove(&key);
            },
            Operation::Put => {
                cache.put(key.clone(), value.clone().unwrap());
                async_cache.put(key.clone(), value.clone().unwrap()).await;
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    drop(async_cache);
    let async_cache = AsyncCache::new(
        AsyncCacheConfig::LFU(EvictionAsyncConfig {
            aof_config: Some(EvictionAOFConfig {
                folder: String::from(folder),
                cache_name:  String::from(cache_name),
                flush_time
            }),
            max_size
        })
    ).await;
    
    for i in 0..num_ops {
        let key = format!("key{}", i);
        assert_eq!(cache.contains_key(&key), async_cache.contains_key(&key).await);
        assert_eq!(cache.get(&key).cloned(), async_cache.get(&key).await);
    };
    tokio::fs::remove_file(format!("{}/{}.dat", folder, cache_name)).await?;
    Ok(())
}
