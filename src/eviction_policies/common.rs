//!Traits and structs which will be used in eviction_policies modules.

/// A trait defining the eviction policy for a cache.

/// This trait, `EvictionPolicy<K>`, establishes a contract for different eviction policies that can be employed by a cache. It outlines the functions that an eviction policy must implement.

pub trait EvictionPolicy<K> {
    /// Called when a value is retrieved from the cache using the given key.

    /// This function is invoked whenever a value is looked up in the cache using the provided `key` of type `K`. The eviction policy can utilize this function to update its internal state or perform actions based on the get operation.

    fn on_get(&mut self, key: &K);

    /// Called when a new value is inserted into the cache using the given key.

    /// This function is called whenever a new key-value pair is inserted into the cache. The `key` argument represents the key of the new entry, and the eviction policy can leverage this function to update its internal state or perform actions based on the set operation.

    fn on_set(&mut self, key: &K);

    /// Attempts to evict a key-value pair from the cache according to the eviction policy.

    /// This function is responsible for selecting a key-value pair to evict from the cache based on the implemented eviction policy. It returns `Some(key)` if an eviction occurs, containing the evicted key of type `K`. If no eviction is necessary, it returns `None`.

    fn evict(&mut self) -> Option<K>;

    /// Removes the entry with the given key from the cache.

    /// This function explicitly removes the key-value entry associated with the provided `key` from the cache. The behavior upon encountering a missing key might vary based on the cache implementation.

    fn remove(&mut self, key: &K);
}
