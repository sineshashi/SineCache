//! Contains logic what to do when some event take place in `ThreadSafeCache.`
//! 

use std::{io, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{aof::{periodic_flush, AOFIterator, AOFSubscriber}, common::AOFRecord};

/// Struct to perform operations after some event takes place in `ThreadSafeCache`
/// For now it handles the `AOF` and when to write to disk.
pub struct CacheEventSubscriber<K, V>
where
    for<'de> K: Deserialize<'de> + Serialize + Send + Sync,
    for<'de> V: Deserialize<'de> + Serialize + Send + Sync,
{
    aof_subscriber: Option<Arc<AOFSubscriber<K, V>>>
}

impl<K, V> CacheEventSubscriber<K, V> 
where
    for<'de> K: Deserialize<'de> + Serialize + Send + Sync + 'static,
    for<'de> V: Deserialize<'de> + Serialize + Send + Sync + 'static,
{
    /// Creates new instance of `CacheEventSubscriber`
    /// 
    /// `filedir`: Folder where the persistent file should be created.
    /// 
    /// `cache_name`: Unique cache_name as with same name file will be created.
    /// 
    /// `flush_time`: Periodic time to flush data. If `None`, it will flush every operation which will make it
    /// really slow. don't do that untill you know what you are doing.
    /// 
    /// If both `filedir` and `cache_name` are `None`, no `AOF` will be created.
    /// 
    /// In case of invalid inputs, it will panic.
    /// 
    pub async fn new(
        filedir: Option<String>,
        cache_name: Option<String>,
        flush_time: Option<u32>,
    ) -> Self {
        if (cache_name.as_ref().is_none() && filedir.as_ref().is_some())
            || (filedir.as_ref().is_none() && cache_name.as_ref().is_some())
            || (flush_time.is_some_and(|x| x == 0))
        {
            panic!("Either both File dir and cache name are None or neither one. flush time must be greater than zero.");
        } else if filedir.as_ref().is_some() && cache_name.as_ref().is_some() {
            let aof_subscriber = Arc::new(AOFSubscriber::new(filedir, cache_name, flush_time).await);
            let instance = Self {
                aof_subscriber: Some(aof_subscriber.clone())
            };
            tokio::spawn(async move {periodic_flush(aof_subscriber.clone()).await});
            instance
        } else {
            Self {
                aof_subscriber: None
            }
        }
    }

    /// Method will be called when something happens in the cache.
    pub async fn on_event(&self, r: AOFRecord<K, V>) {
        if self.aof_subscriber.as_ref().is_some(){
            self.aof_subscriber.as_ref().unwrap().on_event(r).await;
        }
    }

    /// Returns Iterator with all the operations sequentially. Throws error if AOF has not been initialized.
    pub async fn into_iter(&self) -> std::io::Result<AOFIterator> {
        if self.aof_subscriber.as_ref().is_some(){
            self.aof_subscriber.as_ref().unwrap().into_iter().await
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "AOF isn inited."))
        }
    }
}