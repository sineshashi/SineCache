//! Unit tests regarding LFU

use crate::{common::KeyRef, eviction_policies::lfu::LFU};
use crate::eviction_policies::common::EvictionPolicy;

#[test]
fn test_new_lfu() {
    let mut lfu: LFU<i32> = LFU::new();
    assert!(lfu.evict().is_none());
}

#[test]
fn test_on_set_and_evict() {
    let mut lfu: LFU<i32> = LFU::new();
    let key1 = KeyRef::new(&1);
    let key2 = KeyRef::new(&2);

    lfu.on_set(key1.clone());
    lfu.on_set(key2.clone());

    assert_eq!(lfu.evict(), Some(key1.clone()));
    lfu.on_set(key1.clone());
    assert_eq!(lfu.evict(), Some(key2));
    assert_eq!(lfu.evict(), Some(key1.clone()));
    assert_eq!(lfu.evict(), None);
}

#[test]
fn test_on_get() {
    let mut lfu: LFU<i32> = LFU::new();
    let key1 = KeyRef::new(&1);
    let key2 = KeyRef::new(&2);
    let key3 = KeyRef::new(&3);

    lfu.on_set(key1.clone());
    lfu.on_set(key2.clone());

    lfu.on_get(&key1);
    lfu.on_set(key3.clone());
    assert_eq!(lfu.evict(), Some(key2));
    assert_eq!(lfu.evict(), Some(key3));
    assert_eq!(lfu.evict(), Some(key1));
    assert_eq!(lfu.evict(), None);
}

#[test]
fn test_remove_and_evict() {
    let mut lfu: LFU<i32> = LFU::new();
    let key1 = KeyRef::new(&1);
    let key2 = KeyRef::new(&2);

    lfu.on_set(key1.clone());
    lfu.on_set(key2.clone());
    lfu.remove(key1.clone());

    assert_eq!(lfu.evict(), Some(key2));
    assert_eq!(lfu.evict(), None);
}

#[test]
fn test_evict_with_multiple_keys() {
    let mut lfu: LFU<i32> = LFU::new();
    let key1 = KeyRef::new(&1);
    let key2 = KeyRef::new(&2);
    let key3 = KeyRef::new(&3);
    let key4 = KeyRef::new(&4);

    lfu.on_set(key1.clone());
    lfu.on_set(key2.clone());
    lfu.on_set(key3.clone());
    lfu.on_set(key4.clone());

    assert_eq!(lfu.evict(), Some(key1));
    assert_eq!(lfu.evict(), Some(key2));
    assert_eq!(lfu.evict(), Some(key3));
    assert_eq!(lfu.evict(), Some(key4));
    assert_eq!(lfu.evict(), None);
}
