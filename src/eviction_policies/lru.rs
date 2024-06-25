//! This module provides a simple LRU (Least Recently Used) cache implementation using a HashMap
//! for fast lookups and a doubly linked list for efficient eviction of least recently used items.
//!
//! ## Explanation
//!
//! The LRU cache (`LRU`) maintains a doubly linked list to track the access order of items, with
//! a `HashMap` (`map`) to quickly access items by their keys.
//!
//! - `LinkedListNode`: Represents a node in the doubly linked list.
//! - `LRU`: Implements the `EvictionPolicy` trait for managing the cache's eviction policies.
//!
//! ### Safety
//!
//! This module uses `unsafe` Rust code to manipulate raw pointers (`*mut LinkedListNode<K>`) and
//! manage lifetimes carefully. Proper synchronization (via `Send` and `Sync` traits) is ensured
//! for multi-threaded environments.
//!
//! ## Usage
//!
//! Create a new LRU cache, interact with it using `on_get` and `on_set` methods, and manage evictions
//! using the `evict` method. The cache can be safely shared across threads when `K` implements
//! `Send` and `Sync`.
//!

use std::{collections::HashMap, ptr::NonNull};

use crate::common::KeyRef;

use super::common::EvictionPolicy;

/// Represents a node in the doubly linked list used within the LRU cache.
pub struct LinkedListNode<K>
where
    K: Eq + std::hash::Hash + Clone,
{
    pub key: KeyRef<K>,
    pub pre: Option<*mut LinkedListNode<K>>,
    pub next: Option<*mut LinkedListNode<K>>,
}

impl<K> LinkedListNode<K>
where
    K: Eq + std::hash::Hash + Clone,
{
    /// Creates a new `LinkedListNode` with the provided key.
    pub fn new(key_ref: KeyRef<K>) -> Self {
        Self {
            key: key_ref,
            pre: None,
            next: None
        }
    }
}

/// Represents an LRU (Least Recently Used) cache implementation.
pub struct LRU<K>
where
    K: Eq + std::hash::Hash + Clone ,
{
    map: HashMap<KeyRef<K>, NonNull<LinkedListNode<K>>>,
    head: Option<*mut LinkedListNode<K>>,
    tail: Option<*mut LinkedListNode<K>>,
}

impl<K> LRU<K>
where
    K: Eq + std::hash::Hash + Clone ,
{
    /// Creates a new instance of `LRU`.
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    /// Removes a node from the linked list.
    fn remove_node(&mut self, node: &NonNull<LinkedListNode<K>>) {
        let curr = node.as_ptr();
        if let Some(pre) = unsafe { (*curr).pre } {
            unsafe { (*pre).next = (*curr).next };
        } else {
            self.head = unsafe { (*curr).next };
        }
        if let Some(next) = unsafe { (*curr).next } {
            unsafe { (*next).pre = (*curr).pre }
        } else {
            self.tail = unsafe { (*curr).pre };
        }
    }

    /// Inserts a node at the front of the linked list.
    fn insert_at_front(&mut self, node: &NonNull<LinkedListNode<K>>) {
        let mut current = node.as_ptr();

        unsafe {
            (*current).next = self.head;
        }
        if let Some(head) = self.head {
            unsafe {
                (*head).pre = Some(current);
            }
        }
        self.head = Some(current);

        // Update tail if the list was previously empty
        if self.tail.is_none() {
            self.tail = Some(current);
        }
        unsafe {
            self.map.insert((*current).key.clone(), NonNull::new(current).unwrap());
        }
    }

    /// Moves a node with the given key to the front of the linked list.
    pub fn move_to_front(&mut self, key: &KeyRef<K>) {
        if let Some(node) = self.map.remove(key) {
            // Remove the node from its current position
            self.remove_node(&node);

            // Insert the node at the head of the linked list
            self.insert_at_front(&node);
        }
    }

    /// Removes the least recently used node from the linked list and returns its key.
    fn remove_from_last(&mut self) -> Option<KeyRef<K>> {
        if let Some(tail) = self.tail {
            self.map.remove(unsafe { &(*tail).key });
            if let Some(pre) = unsafe { (*tail).pre } {
                unsafe {
                    (*pre).next = None;
                }
            } else {
                self.tail = None;
                self.head = None; // Because there was only a single element.
            };
            return Some(unsafe { (*tail).key.clone() });
        }
        return None;
    }
}

/// Implements the `EvictionPolicy` trait for `LRU`, providing methods for managing cache
/// evictions based on key access patterns.
impl<K> EvictionPolicy<K> for LRU<K>
where
    K: Eq + std::hash::Hash + Clone ,
{
    /// Adjusts the cache structure when a key is accessed.
    fn on_get(&mut self, key: &KeyRef<K>) {
        if self.map.contains_key(key) {
            self.move_to_front(key);
        }
    }

    /// Adjusts the cache structure when a new key-value pair is set.
    fn on_set(&mut self, key: KeyRef<K>) {
        if let Some(node) = self.map.remove(&key) {
            self.remove_node(&node);
        }
        self.insert_at_front(&NonNull::new(Box::into_raw(Box::new(
            LinkedListNode::new(key),
        )))
        .unwrap());
    }

    /// Removes and returns the least recently used key from the cache.
    fn evict(&mut self) -> Option<KeyRef<K>> {
        self.remove_from_last()
    }

    /// Removes a specific key from the cache.
    fn remove(&mut self, key: KeyRef<K>) {
        if let Some(removed) = self.map.remove(&key) {
            self.remove_node(&removed);
        }
    }
}

/// Enables safe concurrent access to `LRU` instances across threads when `K` is `Send`.
unsafe impl<K: Eq + std::hash::Hash + Clone  + Send> Send for LRU<K> {}

/// Enables safe concurrent access to `LRU` instances across threads when `K` is `Sync`.
unsafe impl<K: Eq + std::hash::Hash + Clone  + Sync> Sync for LRU<K> {}
