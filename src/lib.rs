//! This library provides a powerful in-memory caching solution with support for customizable eviction policies
//! and persistence using Append-Only Files (AOF). It allows efficient management of key-value pairs,
//! ensuring both performance optimization and data persistence across application lifecycles.
//!
//! ## Features
//!
//! - **Multiple Eviction Policies**: Choose from FIFO (First-In, First-Out), LRU (Least Recently Used), and LFU
//!   (Least Frequently Used) eviction policies to suit different data access patterns.
//!
//! - **Customizable Eviction Strategies**: Implement custom eviction policies by defining types that adhere to the
//!   `EvictionPolicy` trait, allowing tailored cache management.
//!
//! - **Asynchronous Support**: `AsyncCache` struct provides `async` methods for operations like `get`, `put`, and
//!   `remove`, ensuring efficient handling of concurrent requests in async-await contexts.
//!
//! - **Persistence with Append-Only Files (AOF)**: Optionally persist cache state across restarts using AOF, ensuring
//!   data integrity and recovery after crashes.
//!
//! - **Thread Safety**: `AsyncCache` utilizes `tokio::sync::Mutex` to manage concurrent access safely, making it
//!   suitable for multi-threaded environments.
//!
//! - **Efficient Memory Management**: Optimizes memory usage with smart pointers and references, reducing redundancy
//!   and improving overall performance.
//!
//! - **Configuration Flexibility**: Configure cache size limits, eviction policies, and persistence settings through
//!   intuitive configuration structs (`CacheSyncConfig` and `AsyncCacheConfig`).
//!
//! - **Detailed Documentation**: Comprehensive API documentation and examples facilitate easy integration and usage
//!   within applications.
//!
//! - **Safety and Reliability**: Built with Rust's strong type system and ownership model, ensuring memory safety and
//!   preventing common bugs like null pointer dereferencing and data races.
//!
//! ## Examples
//!
//! ### `Cache` - Synchronous Cache:
//!
//! ```rust
//! use sine_cache::{cache::Cache, config::CacheConfig};
//!
//! fn main() {
//!     let capacity = 10; // Maximum number of entries in the cache.
//!     let mut cache = Cache::new(sine_cache::config::CacheSyncConfig::LFU(CacheConfig{max_size: capacity}));
//!
//!     // Inserting key-value pairs into the cache
//!     cache.put(1, "One");
//!     cache.put(1, "one"); // Overwrites previous value
//!     cache.put(2, "Two");
//!
//!     // Retrieving a value from the cache
//!     let value = cache.get(&1);
//!     assert!(value.is_some_and(|x| x == &"one"));
//! }
//! ```
//!
//! ### `AsyncCache` - Asynchronous Cache:
//!
//! - #### Without `AOF`:
//!
//! ```rust
//! use sine_cache::{cache::AsyncCache, config::{AsyncCacheConfig, EvictionAsyncConfig}};
//!
//! #[tokio::main]
//! async fn main() {
//!     let capacity = 10; // Maximum number of entries in the cache.
//!     let mut cache = AsyncCache::new(AsyncCacheConfig::LFU(EvictionAsyncConfig {max_size: capacity, aof_config: None})).await;
//!
//!     // Inserting key-value pairs into the cache
//!     cache.put(1, String::from("One")).await;
//!     cache.put(1, String::from("one")).await; // Overwrites previous value
//!     cache.put(2, String::from("Two")).await;
//!
//!     // Retrieving a value from the cache
//!     let value = cache.get(&1).await;
//!     assert!(value.is_some_and(|x| x == "one"));
//! }
//! ```
//! 
//! - #### With `AOF`:
//!
//! ```rust
//! use sine_cache::{cache::AsyncCache, config::{AsyncCacheConfig, EvictionAsyncConfig, EvictionAOFConfig}};
//!
//! #[tokio::main]
//! async fn main() {
//!     
//!     let capacity = 10; // Maximum number of entries in the cache.
//!     let mut cache = AsyncCache::new(AsyncCacheConfig::LFU(EvictionAsyncConfig {
//!         max_size: capacity,
//!         aof_config: Some(EvictionAOFConfig {
//!             folder: String::from("./data"), //folder in which persistent file should be written.
//!             cache_name: String::from("async_lof_cache"), //Unique cache name as with same name file will be created.
//!             flush_time: Some(5000) //After every 5000 milliseconds data will be flushed to disk.
//!         })
//!     })).await;
//!
//!     // Inserting key-value pairs into the cache
//!     cache.put(1, String::from("One")).await;
//!     cache.put(1, String::from("one")).await; // Overwrites previous value
//!     cache.put(2, String::from("Two")).await;
//!
//!     // Retrieving a value from the cache
//!     let value = cache.get(&1).await;
//!     assert!(value.is_some_and(|x| x == "one"));
//! }
//! ```
//! 
//! ### Custom eviction policy
//! ```rust
//! use sine_cache::eviction_policies::common::EvictionPolicy;
//! use sine_cache::{cache::AsyncCache, config::{AsyncCacheConfig, CustomEvictionAsyncConfig, CustomEvictionAOFConfig}};
//! 
//! pub struct CustomEviction<K> {
//!     _phantom: std::marker::PhantomData<K>,
//! }

//! impl<K: Eq + std::hash::Hash + Clone> CustomEviction<K> {
//!     pub fn new() -> Self{
//!         Self{
//!             _phantom: std::marker::PhantomData
//!         }
//!     }
//! }
//! 
//! impl<K: Eq + std::hash::Hash + Clone> EvictionPolicy<K> for CustomEviction<K> {
//!     fn on_get(&mut self, key: &K) {
//!         // nothing to do.
//!     }
//! 
//!     fn on_set(&mut self, key: K) {
//!         // nothing to do.
//!     }
//! 
//!     fn evict(&mut self) -> Option<K> {
//!         // nothing to do
//!         None
//!     }
//! 
//!     fn remove(&mut self, key: K) {
//!         //nothing to do
//!     }
//! }
//! 
//! #[tokio::main]
//! async fn main() {
//!     
//!     let capacity = 10; // Maximum number of entries in the cache.
//!     let mut cache = AsyncCache::new(AsyncCacheConfig::Custom(CustomEvictionAsyncConfig {
//!         max_size: capacity,
//!         aof_config: Some(CustomEvictionAOFConfig {
//!             folder: String::from("./data"), //folder in which persistent file should be written.
//!             cache_name: String::from("async_lof_custom_cache"), //Unique cache name as with same name file will be created.
//!             flush_time: Some(5000), //After every 5000 milliseconds data will be flushed to disk.
//!             persist_read_ops: true //whether to store reads also, true generally.
//!         }),
//!         policy: Box::new(CustomEviction::new())
//!     })).await;
//!
//!     // Inserting key-value pairs into the cache
//!     cache.put(1, String::from("One")).await;
//!     cache.put(1, String::from("one")).await; // Overwrites previous value
//!     cache.put(2, String::from("Two")).await;
//!
//!     // Retrieving a value from the cache
//!     let value = cache.get(&1).await;
//!     assert!(value.is_some_and(|x| x == "one"));
//! }
//! 
//! ```
//!
//! For detailed API documentation and further customization options, refer to the library's documentation.
//! For more examples, go through test modules on github library

pub mod aof; //Contains code of append only files
pub mod cache; // Core functionalities for creating and managing in-memory caches
pub mod cache_events; //Event manager which do things upon each event in cache.
pub mod common; // Common types and utilities used throughout the library
pub mod config;
pub mod eviction_policies; // Implementations of different eviction policies for cache management
mod tests; //Contains different configuration structs and enums.
