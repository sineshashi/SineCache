//! Contains code for AOF for persisting data.

use std::collections::VecDeque;
use std::path::Path;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::fs::{File, OpenOptions};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

use crate::common::{AOFRecord, Operation};

/// This struct represents an Append-only File (AOF) for persistent storage
pub struct AOF {
    filedir: String,
    writer: Mutex<File>,
}

impl AOF {
    /// Opens an existing AOF file or creates a new one at the specified path
    pub async fn new(filedir: String) -> Self {
        return Self {
            writer: Mutex::new(OpenOptions::new()
                .create(true)
                .append(true)
                .open(&filedir)
                .await
                .expect(&format!("Error in opening aof {} file", filedir))),
            filedir: filedir,
        };
    }

    async fn object_to_bytes<O: Serialize>(obj: &O) -> Vec<u8> {
        serde_json::to_vec(obj).unwrap()
    }

    async fn to_single_record_bytes<K: Serialize, V: Serialize>(
        operation: Operation,
        key: &K,
        value: &Option<V>,
    ) -> Vec<u8> {
        let key_bytes = Self::object_to_bytes(key).await;
        let operation_byte_size = operation.to_int().to_le_bytes();
        let key_bytes_size = (key_bytes.len() as u32).to_le_bytes();
        let mut bytes = vec![];
        bytes.extend(operation_byte_size);
        bytes.extend(key_bytes_size);
        bytes.extend(key_bytes);
        if value.is_some() {
            let value_bytes = Self::object_to_bytes(value.as_ref().unwrap()).await;
            bytes.extend((value_bytes.len() as u64).to_le_bytes());
            bytes.extend(value_bytes);
        };
        bytes
    }

    pub async fn on_event<K, V>(&self, r: AOFRecord<K, V>, flush: bool)
    where
        for<'de> K: Deserialize<'de> + Serialize,
        for<'de> V: Deserialize<'de> + Serialize,
    {
        let mut gaurd = self.writer.lock().await;
        gaurd
            .write_all(&Self::to_single_record_bytes(r.operation, &r.key, &r.value).await)
            .await
            .unwrap();
        if flush {
            gaurd.flush().await.unwrap();
        }
    }

    pub async fn on_event_multi<K, V>(&self, records: Vec<AOFRecord<K, V>>, flush: bool)
    where
        for<'de> K: Deserialize<'de> + Serialize,
        for<'de> V: Deserialize<'de> + Serialize,
    {
        let mut bytes = vec![];
        for r in records {
            bytes.extend(Self::to_single_record_bytes(r.operation, &r.key, &r.value).await)
        }
        let mut gaurd = self.writer.lock().await;
        gaurd.write_all(&bytes).await.unwrap();
        if flush {
            gaurd.flush().await.unwrap();
        }
    }

    pub async fn flush(&mut self) {
        self.writer.lock().await.flush().await.unwrap();
    }

    pub async fn into_iter(&self) -> io::Result<AOFIterator> {
        let reader = File::open(&self.filedir).await?;
        Ok(AOFIterator { reader })
    }
}

/// Iterator which helps in iterating all the recorded options one by one.
pub struct AOFIterator {
    reader: File,
}

impl AOFIterator {
    /// Next record in the sequence.
    pub async fn next<K, V>(&mut self) -> io::Result<Option<AOFRecord<K, V>>>
    where
        for<'de> K: Deserialize<'de> + Serialize,
        for<'de> V: Deserialize<'de> + Serialize,
    {
        let mut ops_int_bytes = [0u8; 1];
        if self.reader.read_exact(&mut ops_int_bytes).await.is_err() {
            return Ok(None);
        };
        let ops_int = u8::from_le_bytes(ops_int_bytes);
        let operation = Operation::from_int(ops_int);
        let mut key_size_buf = [0u8; 4];
        self.reader.read_exact(&mut key_size_buf).await?;
        let key_size = u32::from_le_bytes(key_size_buf);
        let mut key_buf = vec![0u8; key_size as usize];
        self.reader.read_exact(&mut key_buf).await?;
        let key: K = serde_json::from_slice(&key_buf)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let value;
        if let Operation::Put = operation {
            let mut value_size_buf = [0u8; 8];
            self.reader.read_exact(&mut value_size_buf).await?;
            let value_size = u64::from_le_bytes(value_size_buf);
            let mut value_buf = vec![0u8; value_size as usize];
            self.reader.read_exact(&mut value_buf).await?;
            value = Some(
                serde_json::from_slice(&value_buf)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?,
            );
        } else {
            value = None
        }
        return Ok(Some(AOFRecord {
            key,
            value,
            operation,
        }));
    }
}

/// This struct is a facade to use `AOF`. 
/// 
pub struct AOFSubscriber<K, V>
where
    for<'de> K: Deserialize<'de> + Serialize,
    for<'de> V: Deserialize<'de> + Serialize,
{
    aof: Option<AOF>,
    pub flush_time: Option<u32>,
    unwritten_inmemory_records: Mutex<VecDeque<AOFRecord<K, V>>>,
}

impl<K, V> AOFSubscriber<K, V>
where
    for<'de> K: Deserialize<'de> + Serialize,
    for<'de> V: Deserialize<'de> + Serialize,
{
    pub async fn new(
        filedir: Option<String>,
        cache_name: Option<String>,
        flush_time: Option<u32>,
    ) -> Self {
        if !Path::new(filedir.as_ref().unwrap()).exists() {
            let _ = tokio::fs::create_dir_all(filedir.as_ref().unwrap()).await;
        };
        Self {
            aof: if filedir.as_ref().is_some() {
                Some(
                    AOF::new(format!("{}/{}.dat", filedir.unwrap(), cache_name.unwrap())).await,
                )
            } else {
                None
            },
            flush_time: flush_time,
            unwritten_inmemory_records: Mutex::new(VecDeque::new()),
        }
    }

    pub async fn on_event(&self, r: AOFRecord<K, V>) {
        if self.aof.as_ref().is_some() {
            if self.flush_time.is_some() {
                self.unwritten_inmemory_records.lock().await.push_back(r);
            } else {
                self.aof
                    .as_ref()
                    .unwrap()
                    .on_event(r, true)
                    .await;
            }
        }
    }

    /// Copies all the deque to vectore sequentially and empties the deque.
    async fn get_current_records_and_empty_it(&self) -> Vec<AOFRecord<K, V>> {
        let mut records_guard = self.unwritten_inmemory_records.lock().await;
        let mut records = vec![];
        while let Some(r) = records_guard.pop_front() {
            records.push(r);
        }
        records
    }

    /// Flushes the in memory data to disk and empties in memory. Call this function carefully as it does
    /// not check whether it is ok to call this or not. For e.g. in case of no flush time or no AOF, it must not be called.
    pub async fn flush_to_disk(&self) {
        let records = self.get_current_records_and_empty_it().await;
        self.aof
            .as_ref()
            .unwrap()
            .on_event_multi(records, true)
            .await;
    }

    pub async fn into_iter(&self) -> io::Result<AOFIterator> {
        if self.aof.as_ref().is_some() {
            self.aof.as_ref().unwrap().into_iter().await
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "AOF isn inited."))
        }
    }
}

use async_recursion::async_recursion;

/// Flushes periodically to disk.
#[async_recursion]
pub async fn periodic_flush<K, V>(aof_subscriber: Arc<AOFSubscriber<K, V>>)
where
    for<'de> K: Deserialize<'de> + Serialize + Send + Sync,
    for<'de> V: Deserialize<'de> + Serialize + Send + Sync,
{
    if aof_subscriber.flush_time.as_ref().is_none() {
        return;
    }
    tokio::time::sleep(tokio::time::Duration::from_millis(
        aof_subscriber.flush_time.unwrap() as u64,
    ))
    .await;
    aof_subscriber.flush_to_disk().await;
    periodic_flush(aof_subscriber).await;
}
