//! This library provides an efficient in-memory caching solution with support for various eviction policies.
//!
//! The library allows for the creation of caches that store key-value pairs in memory, providing fast access
//! and configurable eviction strategies to manage cache size and performance.
//!
//! ## Features
//!
//! - **Multiple Eviction Policies**: Supports FIFO (First In, First Out), LRU (Least Recently Used), and LFU
//!   (Least Frequently Used) eviction policies out of the box.
//!
//! - **Customizable**: Users can define their own eviction policies by implementing a simple trait, enabling
//!   tailored cache behavior to suit specific application needs.
//!
//! - **Efficient Memory Management**: The library optimizes memory usage by using references (`KeyRef`) to keys
//!   stored in the cache, reducing redundancy and improving performance.
//! 
//! - **Async/Await and Concurrency Support**: The library provides two structs for in-memory caching:
//!   - *`Cache`*: This struct implements various eviction policies for in-memory caching without using locks.
//!     Users have the flexibility to implement their own locking mechanisms if needed.
//!   - *`ThreadSafeCache`*: This struct wraps the `Cache` struct with a `tokio::sync::Mutex`, enabling safe concurrent access.
//!     It offers `async` versions of methods like `get` and `put` to support asynchronous operations.
//!     The mutex ensures thread safety, making it suitable for concurrent environments.
//!
//! ## Examples
//!
//! - # `Cache`:
//! 
//! ```rust
//! use sine_cache::cache::Cache;
//! use sine_cache::eviction_policies::lfu::LFU;
//!
//! fn main() {
//!     let capacity = 10; // Maximum number of entries in the cache.
//!     let mut cache = Cache::new(capacity, LFU::new());
//!
//!     // Inserting key-value pairs into the cache
//!     cache.put(1, "One");
//!     cache.put(1, "one"); // Overwrites previous value
//!     cache.put(2, "Two");
//!
//!     // Retrieving a value from the cache
//!     let value = cache.get(&1);
//!     println!("{:?}", value); // Output: Some("one")
//!     assert!(value.is_some_and(|x| x == &"one"));
//! }
//! ```
//! This example demonstrates basic usage of the cache with LFU eviction policy. Users can easily switch to
//! other eviction policies like FIFO or LRU by replacing `LFU::new()` with the desired policy constructor.
//! 
//! - # `ThreadSafeCache`:
//! 
//! ```rust
//! use sine_cache::cache::ThreadSafeCache;
//! use sine_cache::eviction_policies::lfu::LFU;
//! 
//! #[tokio::main]
//! async fn main() {
//!     let capacity = 10; // Maximum number of entries in the cache.
//!     let mut cache = ThreadSafeCache::new(capacity, LFU::new());
//!
//!     // Inserting key-value pairs into the cache
//!     cache.put(1, "One").await;
//!     cache.put(1, "one").await; // Overwrites previous value
//!     cache.put(2, "Two").await;
//!
//!     // Retrieving a value from the cache
//!     let value = cache.get(&1).await;
//!     println!("{:?}", value); // Output: Some("one")
//!     assert!(value.is_some_and(|x| x == "one"));
//! }
//! ```
//!
//! For detailed API documentation and further customization options, refer to the library's documentation.

pub mod common;      // Common types and utilities used throughout the library
pub mod eviction_policies; // Implementations of different eviction policies for cache management
pub mod cache;        // Core functionalities for creating and managing in-memory caches
