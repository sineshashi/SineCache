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
//! ## Warnings
//!
//! - **Thread Safety**: **Warning**: This library does not employ locking mechanisms. As a result, concurrent
//!   access from multiple threads may lead to undefined behavior or data corruption. For multi-threaded applications,
//!   consider implementing appropriate locking strategies or using thread-safe alternatives.
//!
//! ## Example
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
//!
//! This example demonstrates basic usage of the cache with LFU eviction policy. Users can easily switch to
//! other eviction policies like FIFO or LRU by replacing `LFU::new()` with the desired policy constructor.
//!
//! For detailed API documentation and further customization options, refer to the library's documentation and
//! examples provided.

pub mod common;      // Common types and utilities used throughout the library
pub mod eviction_policies; // Implementations of different eviction policies for cache management
pub mod cache;        // Core functionalities for creating and managing in-memory caches
pub mod tests;       // Unit tests for the library components
