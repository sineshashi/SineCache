//! This module provides various eviction policies that can be used with a cache implementation.

//! An eviction policy defines the strategy for removing entries from a cache when it reaches its capacity.
//! Different eviction policies prioritize different criteria for eviction, such as least recently used (LRU) or first-in-first-out (FIFO) or Least Frequently Used(LFU).

pub mod fifo;  // FIFO eviction policy
pub mod common; // Common traits and structs used by eviction policies
pub mod lru;   // LRU eviction policy
pub mod lfu; //LFU Eviction policy
pub mod tests;