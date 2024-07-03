//! Contains code to define different configurations to use `Cache` and `AsyncCache
//! `

use crate::eviction_policies::{common::EvictionPolicy, fifo::FIFO, lfu::LFU, lru::LRU, noevicton::NoEviction};

/// Lists all supported policies
pub enum EvictionPolicyEnum <K> {
    NoEviction,
    LRU,
    LFU,
    FIFO,
    Custom(Box<dyn EvictionPolicy<K> + Send>)
}

impl<K: std::hash::Hash + Eq + PartialEq + Eq + Send + Sync + Clone + core::fmt::Debug + 'static,> EvictionPolicyEnum<K> {
    /// get empty policy instance based on the value of enum.
    pub fn create_policy(
        self,
    ) -> Box<dyn EvictionPolicy<K> + Send> {
        match self {
            Self::FIFO => Box::new(FIFO::new()),
            Self::LFU => Box::new(LFU::new()),
            Self::LRU => Box::new(LRU::new()),
            Self::NoEviction => Box::new(NoEviction::new()),
            Self::Custom(e) => e
        }
    }
}

/// Config for `Cache` struct.
pub struct CacheConfig {
    pub max_size: usize,
}

/// Cache configuration to handle custom policies
pub struct CustomCacheConfig<K> {
    pub max_size: usize,
    pub policy: Box<dyn EvictionPolicy<K> + Send>
}

/// Eviction policy based config for `Cache` struct.
pub enum CacheSyncConfig<K> {
    NoEviction,
    LRU(CacheConfig),
    LFU(CacheConfig),
    FIFO(CacheConfig),
    Custom(CustomCacheConfig<K>)
}

impl<K> CacheSyncConfig<K> {
    /// Returns the `CacheConfig` to use in `Cache` struct
    pub fn get_config(&self) -> CacheConfig {
        match self {
            Self::NoEviction => CacheConfig { max_size: 0 }, // setting max size 0 as it will not impact the process.
            Self::FIFO(v) => CacheConfig {
                max_size: v.max_size
            },
            Self::LRU(v) => CacheConfig {
                max_size: v.max_size
            },
            Self::LFU(v) => CacheConfig {
                max_size: v.max_size
            },
            Self::Custom(v) => CacheConfig {
                max_size: v.max_size
            }
        }
    }

    /// Returns the eviction policy type.
    pub fn get_policy_type(self) -> EvictionPolicyEnum<K> {
        match self {
            Self::NoEviction => EvictionPolicyEnum::NoEviction,
            Self::FIFO(_) => EvictionPolicyEnum::FIFO,
            Self::LRU(_) => EvictionPolicyEnum::LRU,
            Self::LFU(_) => EvictionPolicyEnum::LFU,
            Self::Custom(v) => EvictionPolicyEnum::Custom(v.policy)
        }
    }
}

/// `AOF` related configurations for no eviction.
pub struct NoEvictionAOFConfig {
    pub folder: String, // folder in which persistent data will be written. e.g. "./folder"
    pub cache_name: String, //unique cache name as the file with same name will be created and utilized upon restart.
    pub flush_time: Option<u32>, // time in milliseconds in which data will be periodically flushed to disk. In case of `None`, data will be flushed on every event.
    pub persist_read_ops: bool, // If `false`, get operations will be not be recorded in AOF file. Setting it `false` increases speed of reads specially in case of flushing every write.
}

/// No eviction configurations for `AsyncCache`
///
pub struct NoEvictionAsyncConfig {
    pub aof_config: Option<NoEvictionAOFConfig>,
}

/// `AOF` related configurations for evictions.
pub struct EvictionAOFConfig {
    pub folder: String, // folder in which persistent data will be written. e.g. "./folder"
    pub cache_name: String, //unique cache name as the file with same name will be created and utilized upon restart.
    pub flush_time: Option<u32>, // time in milliseconds in which data will be periodically flushed to disk. In case of `None`, data will be flushed on every event.
}

/// Evictions related `Async` configurations.
///
pub struct EvictionAsyncConfig {
    pub max_size: usize, // maximum number of keys to store before starting evictions on new keys.
    pub aof_config: Option<EvictionAOFConfig>,
}

/// `AOF` related configurations for custom eviction.
pub struct CustomEvictionAOFConfig {
    pub folder: String, // folder in which persistent data will be written. e.g. "./folder"
    pub cache_name: String, //unique cache name as the file with same name will be created and utilized upon restart.
    pub flush_time: Option<u32>, // time in milliseconds in which data will be periodically flushed to disk. In case of `None`, data will be flushed on every event.
    pub persist_read_ops: bool, // If `false`, get operations will be not be recorded in AOF file. Setting it `false` increases speed of reads specially in case of flushing every write.
}

/// Eviction related configurations for custom policies.
/// 
pub struct CustomEvictionAsyncConfig<K> {
    pub max_size: usize, // maximum number of keys to store before starting evictions on new keys.
    pub aof_config: Option<CustomEvictionAOFConfig>,
    pub policy: Box<dyn EvictionPolicy<K> + Send>
}

/// Config for `AsyncCache`
///
pub enum AsyncCacheConfig<K> {
    NoEviction(NoEvictionAsyncConfig),
    LFU(EvictionAsyncConfig),
    LRU(EvictionAsyncConfig),
    FIFO(EvictionAsyncConfig),
    Custom(CustomEvictionAsyncConfig<K>)
}

impl<K> AsyncCacheConfig<K> {
    /// get config for `Cache`
    ///
    pub fn get_sync_config(self) -> CacheSyncConfig<K> {
        match self {
            Self::NoEviction(_) => CacheSyncConfig::NoEviction,
            Self::FIFO(v) => CacheSyncConfig::FIFO(CacheConfig {
                max_size: v.max_size,
            }),
            Self::LFU(v) => CacheSyncConfig::LFU(CacheConfig {
                max_size: v.max_size,
            }),
            Self::LRU(v) => CacheSyncConfig::LRU(CacheConfig {
                max_size: v.max_size,
            }),
            Self::Custom(v) => CacheSyncConfig::Custom(CustomCacheConfig {
                max_size: v.max_size,
                policy: v.policy
            })
        }
    }

    /// get whether to include read ops or not in `AOF`. In case of no-evictions and aof not configured, returns `None`.
    ///
    pub fn persist_read_ops(&self) -> Option<bool> {
        match self {
            Self::NoEviction(v) => v.aof_config.as_ref().map(|x| x.persist_read_ops),
            Self::Custom(v) => v.aof_config.as_ref().map(|x| x.persist_read_ops),
            _ => Some(true),
        }
    }

    /// get `AOF` related config.
    ///
    /// Returns a tuple Option<(`folder`, `cache_name`, `flush_time`)>
    ///
    /// In case of no `AOF`, returns None
    ///
    pub fn get_aof_config(&self) -> Option<(String, String, Option<u32>)> {
        match self {
            Self::NoEviction(v) => v.aof_config.as_ref().map(|x| (x.folder.clone(), x.cache_name.clone(), x.flush_time)),
            Self::FIFO(v) => v.aof_config.as_ref().map(|x| (x.folder.clone(), x.cache_name.clone(), x.flush_time)),
            Self::LFU(v) => v.aof_config.as_ref().map(|x| (x.folder.clone(), x.cache_name.clone(), x.flush_time)),
            Self::LRU(v) => v.aof_config.as_ref().map(|x| (x.folder.clone(), x.cache_name.clone(), x.flush_time)),
            Self::Custom(v) => v.aof_config.as_ref().map(|x| (x.folder.clone(), x.cache_name.clone(), x.flush_time)),
        }
    }

    /// Returns eviction policy type
    /// 
    pub fn get_policy_type(self) -> EvictionPolicyEnum<K> {
        match self {
            Self::NoEviction(_) => EvictionPolicyEnum::NoEviction,
            Self::FIFO(_) => EvictionPolicyEnum::FIFO,
            Self::LRU(_) => EvictionPolicyEnum::LRU,
            Self::LFU(_) => EvictionPolicyEnum::LFU,
            Self::Custom(v) => EvictionPolicyEnum::Custom(v.policy)
        }
    }
}
