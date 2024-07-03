//! Unit tests regarding LRU

use crate::{ eviction_policies::lru::LRU};
use crate::eviction_policies::common::EvictionPolicy;

#[test]
fn test_new_lru() {
    let mut lru: LRU<i32> = LRU::new();
    assert_eq!(lru.len(), 0);
    assert!(lru.evict().is_none());
}

#[test]
fn test_on_set_and_evict() {
    let mut lru: LRU<i32> = LRU::new();
    let key1 = 1;
    let key2 = 2;

    lru.on_set(key1.clone());
    lru.on_set(key2.clone());

    assert_eq!(lru.evict(), Some(key1));
    assert_eq!(lru.evict(), Some(key2));
    assert_eq!(lru.evict(), None);
}

#[test]
fn test_on_get() {
    let mut lru: LRU<i32> = LRU::new();
    let key1 = 1;
    let key2 = 2;
    let key3 = 3;

    lru.on_set(key1.clone());
    lru.on_set(key2.clone());
    lru.on_set(key3.clone());

    lru.on_get(&key1);
    assert_eq!(lru.evict(), Some(key2));
    assert_eq!(lru.evict(), Some(key3));
    assert_eq!(lru.evict(), Some(key1));
    assert_eq!(lru.evict(), None);
}

#[test]
fn test_remove_and_evict() {
    let mut lru: LRU<i32> = LRU::new();
    let key1 = 1;
    let key2 = 2;

    lru.on_set(key1.clone());
    lru.on_set(key2.clone());
    lru.remove(key1.clone());

    assert_eq!(lru.evict(), Some(key2));
    assert_eq!(lru.evict(), None);
}

#[test]
fn test_evict_with_multiple_keys() {
    let mut lru: LRU<i32> = LRU::new();
    let key1 = 1;
    let key2 = 2;
    let key3 = 3;
    let key4 = 4;

    lru.on_set(key1.clone());
    lru.on_set(key2.clone());
    lru.on_set(key3.clone());
    lru.on_set(key4.clone());

    assert_eq!(lru.evict(), Some(key1));
    assert_eq!(lru.evict(), Some(key2));
    assert_eq!(lru.evict(), Some(key3));
    assert_eq!(lru.evict(), Some(key4));
    assert_eq!(lru.evict(), None);
}
