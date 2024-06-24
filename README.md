# SineCache: A Simple and Efficient In-Memory Cache Library for Rust (Unpublished)

**SineCache** is a work-in-progress Rust library designed to provide a user-friendly and efficient way to store key-value pairs in memory with optional eviction policies. It's being built with thread-safety in mind to handle concurrent access from multiple threads in your Rust applications.

## **Planned Features:**

* **In-Memory Cache:** Stores data efficiently in memory for fast access.
* **Thread-Safe:** Ensures safe concurrent access to the cache.
* **Configurable Eviction Policies (Under Development):** Choose from different eviction strategies like First-In-First-Out (FIFO) (implemented), Least Recently Used (LRU), and Least Frequently Used (LFU) to manage cache size.
* **Easy to Use API:** Simple methods for adding, retrieving, and removing data from the cache.

## **Current Stage**

SineCache is currently under development. The core functionality with FIFO eviction policy is implemented. Support for additional eviction policies like LRU and LFU is planned for future releases.

## **Getting Started (Placeholder)**

*This section will be filled with instructions once the library is published on crates.io*

## **Basic Usage Example (Placeholder)**

*This section will include a code example demonstrating usage once the library is published*

## **Future Roadmap**

* Implement Least Recently Used (LRU) and Least Frequently Used (LFU) eviction policies.
* Integrate Time-To-Live (TTL) functionality for automatic cache entry expiration.
* Publish the library on crates.io for easy installation.

## **Contributing**

We welcome contributions to SineCache! If you'd like to help with development or have suggestions, feel free to reach out (mention preferred communication method, e.g., GitHub discussions, email).

**Note:** This library is currently under development and not yet published. Stay tuned for updates
