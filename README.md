# SineCache

SineCache is a high-performance, in-memory caching library for Rust, designed to efficiently store and manage key-value pairs with support for various eviction policies and persistence options using Append-Only Files (AOF).

## Features

### Powerful Caching Mechanism

SineCache provides a robust caching solution with flexible configurations and support for multiple eviction policies, ensuring optimal performance for varying application needs.

### Eviction Policies

Choose from FIFO (First-In, First-Out), LRU (Least Recently Used), and LFU (Least Frequently Used) eviction policies. Additionally, define custom eviction policies through a simple trait implementation.

### Asynchronous Support

- **AsyncCache**: Wraps the `Cache` struct with a `tokio::sync::Mutex`, enabling safe concurrent access with asynchronous operations (`async` versions of `get`, `put`, `remove`, etc.).

### Persistence with Append-Only Files (AOF)

Optionally persist cache data using AOF, ensuring durability and recovery of cache state across application restarts. If `flush_time` is provided (milliseconds), data is flushed to disk after every `flush_time` milliseconds to disk *without blocking the main thread*. In case of `None`, every operation is flushed to disk in the same thread.

### Thread Safety

Ensures thread safety with appropriate locking mechanisms (`tokio::sync::Mutex` for `AsyncCache`), making it suitable for multi-threaded environments.

### Configuration Flexibility

Configure cache size limits, eviction policies, AOF settings, and more through intuitive configuration structs (`CacheSyncConfig` and `AsyncCacheConfig`).

### Comprehensive Documentation

Extensive API documentation and examples facilitate easy integration and customization within applications.

### Safety and Reliability

Built with Rust's ownership model and type system, ensuring memory safety and preventing common bugs like null pointer dereferencing and data races.

## Getting Started

To use SineCache in your Rust project, add it to your `Cargo.toml`:

```toml
[dependencies]
sine_cache = "0.2.0"
```

## Examples

Some examples are listed below but for the more detailed documentation, visit: [https://docs.rs/sine_cache/latest/sine_cache/](https://docs.rs/sine_cache/latest/sine_cache/)

### `Cache` - Synchronous Cache:

Simple methods related to `Cache` . For using it in concurrent environment, customize on top of it like wrapping in Mutex and using `async` methods etc.

```rust
use sine_cache::{cache::Cache, config::CacheConfig};

fn main() {
    let capacity = 10; // Maximum number of entries in the cache.
    let mut cache = Cache::new(sine_cache::config::CacheSyncConfig::LFU(CacheConfig{max_size: capacity}));

    // Inserting key-value pairs into the cache
    cache.put(1, "One");
    cache.put(1, "one"); // Overwrites previous value
    cache.put(2, "Two");

    // Retrieving a value from the cache
    let value = cache.get(&1);
    assert!(value.is_some_and(|x| x == &"one"));
}
```

### `AsyncCache` - Asynchronous Cache:

Some examples related to `AsyncCache` are listed below:

- #### Without `AOF`:

  When `AOF` is not required:

```rust
use sine_cache::{cache::AsyncCache, config::{AsyncCacheConfig, EvictionAsyncConfig}};

#[tokio::main]
async fn main() {
    let capacity = 10; // Maximum number of entries in the cache.
    let mut cache = AsyncCache::new(AsyncCacheConfig::LFU(EvictionAsyncConfig {max_size: capacity, aof_config: None})).await;

    // Inserting key-value pairs into the cache
    cache.put(1, String::from("One")).await;
    cache.put(1, String::from("one")).await; // Overwrites previous value
    cache.put(2, String::from("Two")).await;

    // Retrieving a value from the cache
    let value = cache.get(&1).await;
    assert!(value.is_some_and(|x| x == "one"));
}
```

- #### With `AOF`:

  When AOF is required, we can pass details related to AOF in the configurations and set the periodic flushes to disk or each operation record to disk based on setting `flush_time` in milliseconds or `None`.

```rust
use sine_cache::{cache::AsyncCache, config::{AsyncCacheConfig, EvictionAsyncConfig, EvictionAOFConfig}};

#[tokio::main]
async fn main() {
  
    let capacity = 10; // Maximum number of entries in the cache.
    let mut cache = AsyncCache::new(AsyncCacheConfig::LFU(EvictionAsyncConfig {
        max_size: capacity,
        aof_config: Some(EvictionAOFConfig {
            folder: String::from("./data"), //folder in which persistent file should be written.
            cache_name: String::from("async_lof_cache"), //Unique cache name as with same name file will be created.
            flush_time: Some(5000) //After every 5000 milliseconds data will be flushed to disk.
        })
    })).await;

    // Inserting key-value pairs into the cache
    cache.put(1, String::from("One")).await;
    cache.put(1, String::from("one")).await; // Overwrites previous value
    cache.put(2, String::from("Two")).await;

    // Retrieving a value from the cache
    let value = cache.get(&1).await;
    assert!(value.is_some_and(|x| x == "one"));
}
```

### Custom eviction policy

Custom evicton policies can also be defined and used with all the features of `AsyncCache` and `Cache`.

```rust
use sine_cache::eviction_policies::common::EvictionPolicy;
use sine_cache::{cache::AsyncCache, config::{AsyncCacheConfig, CustomEvictionAsyncConfig, CustomEvictionAOFConfig}};

pub struct CustomEviction<K> {
    _phantom: std::marker::PhantomData<K>,
}

impl<K: Eq + std::hash::Hash + Clone> CustomEviction<K> {
    pub fn new() -> Self{
        Self{
            _phantom: std::marker::PhantomData
        }
    }
}

impl<K: Eq + std::hash::Hash + Clone> EvictionPolicy<K> for CustomEviction<K> {
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

#[tokio::main]
async fn main() {
  
    let capacity = 10; // Maximum number of entries in the cache.
    let mut cache = AsyncCache::new(AsyncCacheConfig::Custom(CustomEvictionAsyncConfig {
        max_size: capacity,
        aof_config: Some(CustomEvictionAOFConfig {
            folder: String::from("./data"), //folder in which persistent file should be written.
            cache_name: String::from("async_lof_custom_cache"), //Unique cache name as with same name file will be created.
            flush_time: Some(5000), //After every 5000 milliseconds data will be flushed to disk.
            persist_read_ops: true //whether to store reads also, true generally.
        }),
        policy: Box::new(CustomEviction::new())
    })).await;

    // Inserting key-value pairs into the cache
    cache.put(1, String::from("One")).await;
    cache.put(1, String::from("one")).await; // Overwrites previous value
    cache.put(2, String::from("Two")).await;

    // Retrieving a value from the cache
    let value = cache.get(&1).await;
    assert!(value.is_some_and(|x| x == "one"));
}

```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
