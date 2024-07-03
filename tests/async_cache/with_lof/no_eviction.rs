use core::num;

use rand::{random, Rng};
use rand::{distributions::WeightedIndex, thread_rng};
use rand::distributions::Distribution;
use sine_cache::{cache::{AsyncCache, Cache}, common::Operation, config::{AsyncCacheConfig, CacheSyncConfig, NoEvictionAOFConfig, NoEvictionAsyncConfig}};

#[tokio::test]
async fn test_no_eviction_async_cache_with_periodic_flush()  -> Result<(), tokio::io::Error> {
    let cache_name = "test_no_eviction_async_cache_with_periodic_flush";
    let folder = ".";
    let _ = tokio::fs::remove_file(format!("{}/{}.dat", folder, cache_name)).await;
    let async_cache = AsyncCache::new(
        AsyncCacheConfig::NoEviction(NoEvictionAsyncConfig {
            aof_config: Some(NoEvictionAOFConfig {
                folder: String::from(folder),
                cache_name:  String::from(cache_name),
                flush_time: Some(100),
                persist_read_ops: false
            })
        })
    ).await;
    let mut cache = Cache::new(CacheSyncConfig::NoEviction);
    // Define weights for different operations (adjust weights as needed)
    let weights = &[0.3, 0.5, 0.2];

    // Define possible operations
    let operations = vec![Operation::Put, Operation::Get, Operation::Remove];

    let weighted_dist = WeightedIndex::new(weights).unwrap();
    let mut rng = thread_rng();
    let mut  rng1 = thread_rng();

    let num_ops = 200; // Adjust the number of random operations

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
                async_cache.put(key.clone(), value.clone()).await;
                cache.put(key.clone(), value.clone());
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    drop(async_cache);
    let async_cache = AsyncCache::new(
        AsyncCacheConfig::NoEviction(NoEvictionAsyncConfig {
            aof_config: Some(NoEvictionAOFConfig {
                folder: String::from(folder),
                cache_name:  String::from(cache_name),
                flush_time: Some(100),
                persist_read_ops: false
            })
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
async fn test_no_eviction_async_cache_with_periodic_flush_with_persistent_reads()  -> Result<(), tokio::io::Error> {
    let cache_name = "test_no_eviction_async_cache_with_periodic_flush_with_persistent_reads";
    let folder = ".";
    let _ = tokio::fs::remove_file(format!("{}/{}.dat", folder, cache_name)).await;
    let async_cache = AsyncCache::new(
        AsyncCacheConfig::NoEviction(NoEvictionAsyncConfig {
            aof_config: Some(NoEvictionAOFConfig {
                folder: String::from(folder),
                cache_name:  String::from(cache_name),
                flush_time: Some(100),
                persist_read_ops: true
            })
        })
    ).await;
    let mut cache = Cache::new(CacheSyncConfig::NoEviction);
    // Define weights for different operations (adjust weights as needed)
    let weights = &[0.3, 0.5, 0.2];

    // Define possible operations
    let operations = vec![Operation::Put, Operation::Get, Operation::Remove];

    let weighted_dist = WeightedIndex::new(weights).unwrap();
    let mut rng = thread_rng();

    let num_ops = 200; // Adjust the number of random operations
    let mut rng1 = thread_rng();

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
                async_cache.put(key.clone(), value.clone()).await;
                cache.put(key.clone(), value.clone());
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    drop(async_cache);
    let async_cache = AsyncCache::new(
        AsyncCacheConfig::NoEviction(NoEvictionAsyncConfig {
            aof_config: Some(NoEvictionAOFConfig {
                folder: String::from(folder),
                cache_name:  String::from(cache_name),
                flush_time: Some(100),
                persist_read_ops: false
            })
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
async fn test_no_eviction_async_cache_with_instant_flush_with_persistent_reads()  -> Result<(), tokio::io::Error> {
    let cache_name = "test_no_eviction_async_cache_with_instant_flush_with_persistent_reads";
    let folder = ".";
    let _ = tokio::fs::remove_file(format!("{}/{}.dat", folder, cache_name)).await;
    let async_cache = AsyncCache::new(
        AsyncCacheConfig::NoEviction(NoEvictionAsyncConfig {
            aof_config: Some(NoEvictionAOFConfig {
                folder: String::from(folder),
                cache_name:  String::from(cache_name),
                flush_time: None,
                persist_read_ops: true
            })
        })
    ).await;
    let mut cache = Cache::new(CacheSyncConfig::NoEviction);
    // Define weights for different operations (adjust weights as needed)
    let weights = &[0.3, 0.5, 0.2];

    // Define possible operations
    let operations = vec![Operation::Put, Operation::Get, Operation::Remove];

    let weighted_dist = WeightedIndex::new(weights).unwrap();
    let mut rng = thread_rng();
    let mut rng1 = thread_rng();

    let num_ops = 200; // Adjust the number of random operations

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
                async_cache.put(key.clone(), value.clone()).await;
                cache.put(key.clone(), value.clone());
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    drop(async_cache);
    let async_cache = AsyncCache::new(
        AsyncCacheConfig::NoEviction(NoEvictionAsyncConfig {
            aof_config: Some(NoEvictionAOFConfig {
                folder: String::from(folder),
                cache_name:  String::from(cache_name),
                flush_time: Some(100),
                persist_read_ops: false
            })
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
async fn test_no_eviction_async_cache_with_instant_flush()  -> Result<(), tokio::io::Error> {
    let cache_name = "test_no_eviction_async_cache_with_instant_flush";
    let folder = ".";
    let _ = tokio::fs::remove_file(format!("{}/{}.dat", folder, cache_name)).await;
    let async_cache = AsyncCache::new(
        AsyncCacheConfig::NoEviction(NoEvictionAsyncConfig {
            aof_config: Some(NoEvictionAOFConfig {
                folder: String::from(folder),
                cache_name:  String::from(cache_name),
                flush_time: None,
                persist_read_ops: false
            })
        })
    ).await;
    let mut cache = Cache::new(CacheSyncConfig::NoEviction);
    // Define weights for different operations (adjust weights as needed)
    let weights = &[0.3, 0.5, 0.2];

    // Define possible operations
    let operations = vec![Operation::Put, Operation::Get, Operation::Remove];

    let weighted_dist = WeightedIndex::new(weights).unwrap();
    let mut rng = thread_rng();
    let mut rng1 = thread_rng();

    let num_ops = 200; // Adjust the number of random operations

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
                async_cache.put(key.clone(), value.clone()).await;
                cache.put(key.clone(), value.clone());
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    drop(async_cache);
    let async_cache = AsyncCache::new(
        AsyncCacheConfig::NoEviction(NoEvictionAsyncConfig {
            aof_config: Some(NoEvictionAOFConfig {
                folder: String::from(folder),
                cache_name:  String::from(cache_name),
                flush_time: Some(100),
                persist_read_ops: false
            })
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