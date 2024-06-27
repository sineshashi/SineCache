//! Unit tests regarding FIFO

use crate::{common::KeyRef, eviction_policies::fifo::FIFO};
use crate::eviction_policies::common::EvictionPolicy;

#[test]
fn test_new_fifo() {
    let mut fifo: FIFO<i32> = FIFO::new();
    assert!(fifo.evict().is_none());
}

#[test]
fn test_on_set_and_evict() {
    let mut fifo: FIFO<i32> = FIFO::new();
    let key1 = KeyRef::new(&1);
    let key2 = KeyRef::new(&2);

    fifo.on_set(key1.clone());
    fifo.on_set(key2.clone());

    assert_eq!(fifo.evict(), Some(key1));
    assert_eq!(fifo.evict(), Some(key2));
    assert_eq!(fifo.evict(), None);
}

#[test]
fn test_evict() {
    let mut fifo: FIFO<i32> = FIFO::new();
    let key1 = KeyRef::new(&1);
    let key2 = KeyRef::new(&2);
    let key3 = KeyRef::new(&3);

    fifo.on_set(key1.clone());
    fifo.on_set(key2.clone());
    fifo.on_set(key3.clone());

    assert_eq!(fifo.evict(), Some(key1));
    assert_eq!(fifo.evict(), Some(key2));
    assert_eq!(fifo.evict(), Some(key3));
    assert_eq!(fifo.evict(), None);
}

#[test]
fn test_remove_and_evict() {
    let mut fifo: FIFO<i32> = FIFO::new();
    let key1 = KeyRef::new(&1);
    let key2 = KeyRef::new(&2);

    fifo.on_set(key1.clone());
    fifo.on_set(key2.clone());
    fifo.remove(key1.clone());

    assert_eq!(fifo.evict(), Some(key2));
    assert_eq!(fifo.evict(), None);
}

#[test]
fn test_evict_with_tombstones() {
    let mut fifo: FIFO<i32> = FIFO::new();
    let key1 = KeyRef::new(&1);
    let key2 = KeyRef::new(&2);
    let key3 = KeyRef::new(&3);

    fifo.on_set(key1.clone());
    fifo.on_set(key2.clone());
    fifo.on_set(key3.clone());
    fifo.remove(key1.clone());
    fifo.remove(key2.clone());

    assert_eq!(fifo.evict(), Some(key3));
    assert_eq!(fifo.evict(), None);
}

#[test]
fn test_on_get() {
    let mut fifo: FIFO<i32> = FIFO::new();
    let key1 = KeyRef::new(&1);
    fifo.on_set(key1.clone());
    fifo.on_get(&key1);
    // On_get should not affect the queue, no assertions needed here.
}
