//! Code of `Cache` and `AsyncCache` struct which provides functionalities of caching.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{cache_events::CacheEventSubscriber, common::{AOFRecord, CacheEntry, Operation}, config::{AsyncCacheConfig, CacheSyncConfig}, eviction_policies::common::EvictionPolicy};

/// This struct, `Cache<K, V, P>`, implements a generic in-memory cache. It utilizes a `HashMap` to store key-value pairs and allows customization of the eviction policy through the `P` generic type, which must implement the `EvictionPolicy<K>` trait.
/// 
/// This is basic Cache to use. For using cache with persistence with append only files or using in async env,
/// please use `AsyncCache`
/// 


pub struct Cache<K, V>
where
    K: Eq + std::hash::Hash + Clone ,
{
    /// The maximum size of the cache in number of entries.
    max_size: usize,

    /// The internal HashMap storing key-value pairs with associated cache entries.
    cache: HashMap<K, CacheEntry<V>>,

    /// The eviction policy instance used by the cache to determine eviction behavior.
    eviction_policy: Box<dyn EvictionPolicy<K> + Send>,
}

impl<K, V> Cache<K, V>
where
    K: Eq + std::hash::Hash + Clone + std::fmt::Debug + Send + Sync + 'static,
{
    /// Creates a new `Cache` instance.

    /// This function constructs a new cache with the provided `config`.
    /// 
    pub fn new(config: CacheSyncConfig<K>) -> Self {
        let max_size = config.get_config().max_size;
        let policy_type = config.get_policy_type();
        Cache {
            cache: HashMap::new(),
            max_size,
            eviction_policy: policy_type.create_policy()
        }
    }
}

impl<K, V> Cache<K, V>
where
    K: Eq + std::hash::Hash + Clone + std::fmt::Debug
{
    /// Retrieves the value associated with the given key from the cache.

    /// This function attempts to retrieve the value for the provided `key`. It checks if the key exists in the cache, and if so, calls the eviction policy's `on_get` method. If the key is found, an immuatable reference to the value is returned. Otherwise, `None` is returned.

    pub fn get(&mut self, key: &K) -> Option<&V>
    {
        self.eviction_policy.on_get(key);
        self.cache.get(key).map(|x| &x.value)
    }

    /// Retrieves mutable pointer to the value associated with the given key from the cache.

    /// This function attempts to retrieve the value for the provided `key`. It checks if the key exists in the cache, and if so, calls the eviction policy's `on_get` method. If the key is found, an muatable reference to the value is returned. Otherwise, `None` is returned.

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V>
    {
        self.eviction_policy.on_get(key);
        self.cache.get_mut(key).map(|x| &mut x.value)
    }

    /// Inserts a new key-value pair into the cache.

    /// This function inserts a new key-value pair into the cache. It checks if the cache is at its maximum size, and if necessary, evicts an entry using the eviction policy. The new key-value pair is then inserted into the cache along with a `CacheEntry` and the eviction policy's `on_set` method is called.
    /// 

    pub fn put(&mut self, key: K, value: V) {
        if self.cache.len() >= self.max_size && !self.contains_key(&key){
            if let Some(evicted) = self.eviction_policy.evict() {
                self.cache.remove(&evicted);
            }
        }
        match self.cache.get_mut(&key) {
            Some(v) => {
                v.value = value;
            },
            None => {
                self.cache.insert(key.clone(), CacheEntry::new(value));
            }
        };

        self.eviction_policy.on_set(key);
    }

    /// Removes the entry with the given key from the cache.

    /// This function removes the entry associated with the provided `key` from the cache. It removes the entry if it exists. If an entry is removed, the eviction policy's `remove` method is called.

    pub fn remove(&mut self, key: &K) {
        self.cache.remove(key);
        self.eviction_policy.remove(key.clone());
    }

    ///Checks if key is already in cache.
    /// 
    /// This does not account for access.
    /// 

    pub fn contains_key(&self, key: &K) -> bool {
        return self.cache.contains_key(&key);
    }

    ///Returns the current size of the cache. The number of keys in the cache at the moment.
    pub fn size(&self) -> usize {
        return self.cache.len();
    }

    /// Returns a raw pointer to the value associated with the given key.
    ///
    /// Returns a raw pointer to the value associated with the given key, if it exists
    /// in the cache. This method is unsafe due to potential dangling pointers and should
    /// only be used in environments where it is safe to manage raw pointers manually.
    fn get_raw(&mut self, key: &K) -> Option<*const V> {
        self.get(key).map(|x| x as *const V)
    }
}


/// A more advanced cache exposing `async` functions, suitable for concurrent environments.
/// 
/// It uses `Mutex` around `Cache` to provide synchronization.
/// 
/// `AOF` related configurations can be passed in `new()` method to persist data to restart the cache
/// from the same point where it was stopped or crashed. Although some data may be lost, please go through
/// `AsyncCacheConfig` for more info.
/// 

pub struct AsyncCache<K, V>
where
    for<'de> K: Eq + std::hash::Hash + Clone + Deserialize<'de> + Serialize + Send + Sync,
    for<'de> V: Deserialize<'de> + Serialize + Send + Sync,
{
    cache: Mutex<Cache<K, V>>,
    persist_read_ops: Option<bool>,
    subscriber_manager: CacheEventSubscriber<K, V>
}

impl<K, V> AsyncCache <K, V>
where
    for<'de> K: Eq + std::hash::Hash + Clone + std::fmt::Debug + Send + Sync + Deserialize<'de> + Serialize + 'static,
    for<'de> V: Clone + Deserialize<'de> + Serialize + Send + Sync + 'static
{
    /// Creates a new `AsyncCache` instance based on configurations.
    /// 
    /// In case of `AOF`, if given `cache_name` already exists in persistent files, it goes through all the
    /// operations sequentially and performs those on the newly created instance to get the latest cache.
    /// 
    /// Data may be lost in case of `flush_time` being not `None` for the last `flush_time` milliseconds before
    /// crash or stop.
    /// 
    /// Changing `EvictionPolicy` may load different keys as no meta data regarding policy, flushtime etc
    /// is persisted.
    ///
    /// In case of `NoEviction` and `read heavy` cache, using `flush_time = None` with `persist_read_ops = false`
    /// i.e. flush on every write but reads will not be persisted remove may be useful as `writes` 
    /// speed will be slow but `reads` will become faster.
    /// 
    /// In case of eviction policies, setting `flush_time` as `None` is *NOT RECOMMENDED* as it will make it as slow
    /// as disk io.
    /// 
    pub async fn new(config: AsyncCacheConfig<K>) -> Self {
        let instance = Self {
            persist_read_ops: config.persist_read_ops(),
            subscriber_manager: match config.get_aof_config() {
                Some(v) => CacheEventSubscriber::new(Some(v.0), Some(v.1), v.2).await,
                None => CacheEventSubscriber::new(None, None, None).await
            },
            cache: Mutex::new(Cache::new(config.get_sync_config()))
        };
        // performing operations sequentially as per `AOF`.
        let mut gaurd = instance.cache.lock().await;
        if let Ok(mut iter) = instance.subscriber_manager.into_iter().await {
            while let Ok(Some(record)) = iter.next().await {
                match record.operation {
                    Operation::Get => {
                        let _ = gaurd.get(&record.key);
                    },
                    Operation::Put => gaurd.put(record.key, record.value.unwrap()),
                    Operation::Remove => gaurd.remove(&record.key)
                }
            }
        }
        drop(gaurd);
        instance
    }

    /// Retrieves the value associated with the given key from the cache.
    ///
    /// Asynchronously retrieves the value associated with the provided `key` from the cache.
    /// Returns `None` if the key is not found.
    

    pub async fn get(&self, key: &K) -> Option<V>
    {
        let mut guard = self.cache.lock().await;
        let value = guard.get(key).cloned();
        if self.persist_read_ops.as_ref().is_some_and(|x| x.clone()) {
            self.subscriber_manager.on_event(AOFRecord {
                key: key.clone(),
                value: None,
                operation: crate::common::Operation::Get
            }).await;
        };
        drop(guard);
        value
    }

    /// Retrieves a reference to the value associated with the given key from the cache.
    ///
    /// Asynchronously retrieves a reference to the value associated with the provided `key` from the cache.
    /// Returns `None` if the key is not found.
    ///
    /// **Safety Note:** This method returns a reference that may become invalid in a multithreaded environment
    /// due to potential concurrent modifications. Use with caution in single-threaded environments only.
    
    pub async fn get_ref(&self, key: &K) -> Option<&V>
    {
        let mut gaurd = self.cache.lock().await;
        let val = gaurd.get_raw(key).map(|x| unsafe{x.as_ref()}).flatten();
        if self.persist_read_ops.as_ref().is_some_and(|x| x.clone()) {
            self.subscriber_manager.on_event(AOFRecord {
                key: key.clone(),
                value: None,
                operation: crate::common::Operation::Get
            }).await;
        };
        drop(gaurd);
        val
    }

    /// Inserts a new key-value pair into the cache.
    ///
    /// Asynchronously inserts a new key-value pair into the cache.
     
    pub async fn put(&self, key: K, value: V) {
        let mut gaurd = self.cache.lock().await;
        gaurd.put(key.clone(), value.clone());
        self.subscriber_manager.on_event(AOFRecord {
            key: key,
            value: Some(value),
            operation: crate::common::Operation::Put
        }).await;
        drop(gaurd);
    }

    /// Removes the entry with the given key from the cache.
    ///
    /// Asynchronously removes the entry associated with the provided `key` from the cache.
    pub async fn remove(&self, key: &K) {
        let mut gaurd = self.cache.lock().await;
        gaurd.remove(key);
        self.subscriber_manager.on_event(AOFRecord {
            key: key.clone(),
            value: None,
            operation: crate::common::Operation::Remove
        }).await;
        drop(gaurd);
    }

    /// Checks if the cache contains the given key.
    ///
    /// Asynchronously checks if the cache contains the provided `key`.
    /// 
    /// This does not account for access.
    /// 
    pub async fn contains_key(&self, key: &K) -> bool {
        return self.cache.lock().await.contains_key(&key);
    }

    /// Returns the current size of the cache.
    ///
    /// Asynchronously returns the current number of entries in the cache.
    pub async fn size(&self) -> usize {
        return self.cache.lock().await.size();
    }
}

