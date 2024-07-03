//! Contains formal implementation fo NoEviction.

use super::common::EvictionPolicy;

/// No eviction. Just a formal implementation
pub struct NoEviction<K> {
    _phantom: std::marker::PhantomData<K>,
}

impl<K: Eq + std::hash::Hash + Clone> NoEviction<K> {
    pub fn new() -> Self{
        Self{
            _phantom: std::marker::PhantomData
        }
    }
}

impl<K: Eq + std::hash::Hash + Clone> EvictionPolicy<K> for NoEviction<K> {
    fn on_get(&mut self, key: &K) {
        // nothing to do.
    }

    fn on_set(&mut self, key: K) {
        // nothing to do.
    }

    fn evict(&mut self) -> Option<K> {
        // nothing to do
        None
    }

    fn remove(&mut self, key: K) {
        //nothing to do
    }
}