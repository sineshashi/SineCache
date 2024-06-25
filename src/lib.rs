//! This library provides an in-memory caching solution with support for various eviction policies.

//! The library offers functionalities for creating caches that store key-value pairs in memory.
//! It allows you to define eviction policies that determine how entries are removed from the cache
//! when it reaches its capacity.

pub mod common;      // Common types and utilities used throughout the library
pub mod eviction_policies; // Implementations of different eviction policies for cache management
pub mod cache;        // Core functionalities for creating and managing in-memory caches
pub mod tests;       // Unit tests for the library components
