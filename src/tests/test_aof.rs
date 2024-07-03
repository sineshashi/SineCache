use crate::{
    aof::{AOFSubscriber, AOF}, cache_events::CacheEventSubscriber, common::{AOFRecord, Operation}
};
use rand::distributions::WeightedIndex;
use rand::prelude::*;

#[tokio::test]
async fn test_aof_new_creates_file() -> Result<(), tokio::io::Error> {
    let test_file = "test_aof1.dat";
    let _aof = AOF::new(test_file.to_string()).await;
    // Check if the file exists
    let metadata = tokio::fs::metadata(test_file).await?;
    assert!(metadata.is_file());

    // Cleanup: Delete the test file
    tokio::fs::remove_file(test_file).await?;
    Ok(())
}

#[tokio::test]
async fn test_aof_on_event_put() -> Result<(), tokio::io::Error> {
    let test_file = "test_aof2.dat";
    let ao_file = AOF::new(test_file.to_string()).await;

    let test_key = String::from("key1");
    let test_value = String::from("value1");

    let record = AOFRecord {
        key: test_key.clone(),
        value: Some(test_value.clone()),
        operation: Operation::Put,
    };
    ao_file.on_event(record, true).await;

    let test_key1 = String::from("key2");
    let test_value1 = String::from("value2");

    let record = AOFRecord {
        key: test_key1.clone(),
        value: Some(test_value1.clone()),
        operation: Operation::Put,
    };
    ao_file.on_event(record, true).await;

    let mut total_records = 0;
    if let Ok(mut record_iter) = ao_file.into_iter().await {
        while let Ok(Some(r)) = record_iter.next::<String, String>().await {
            total_records += 1;
            if total_records == 1 {
                assert!(r.key == test_key);
                assert_eq!(r.value, Some(test_value.clone()));
                assert_eq!(r.operation, Operation::Put);
            } else if total_records == 2 {
                assert!(r.key == test_key1);
                assert_eq!(r.value, Some(test_value1.clone()));
                assert_eq!(r.operation, Operation::Put);
            } else {
                assert!(false);
            }
        }
    }

    tokio::fs::remove_file(test_file).await?;
    Ok(())
}

#[tokio::test]
async fn test_aof_random_ops_and_iteration_with_write_and_flush() -> Result<(), tokio::io::Error> {
    let test_file = "test_aof3.dat";
    let _ = tokio::fs::remove_file(test_file).await; //clean the file if exists
    let aof = AOF::new(test_file.to_string()).await;

    // Define weights for different operations (adjust weights as needed)
    let weights = &[0.3, 0.5, 0.2];

    // Define possible operations
    let operations = vec![Operation::Put, Operation::Get, Operation::Remove];

    let weighted_dist = WeightedIndex::new(weights).unwrap();
    let mut rng = thread_rng();

    let mut written_records = Vec::new();
    let num_ops = 200; // Adjust the number of random operations

    // Generate random operations and write to AOF
    for _ in 0..num_ops {
        let op = weighted_dist.sample(&mut rng);
        let key = format!("key{}", written_records.len());
        let value = match &operations[op] {
            Operation::Put => Some(format!("value{}", written_records.len())),
            _ => None,
        };
        written_records.push(AOFRecord {
            key: key.clone(),
            value: value.clone(),
            operation: operations[op].clone(),
        });
        aof.on_event(
            AOFRecord {
                key: key.clone(),
                value: value.clone(),
                operation: operations[op].clone(),
            },
            true,
        )
        .await;
    }

    // Read records from AOF and check order
    let mut iter = aof.into_iter().await.unwrap();
    for record in written_records {
        let next_record = iter.next::<String, String>().await.unwrap().unwrap();
        assert_eq!(next_record.key, record.key);
        assert_eq!(next_record.value, record.value);
        assert_eq!(next_record.operation, record.operation);
    }

    // Cleanup: Delete the test file
    tokio::fs::remove_file(test_file).await?;
    Ok(())
}

#[tokio::test]
async fn test_aof_random_ops_and_iteration_with_single_flush() -> Result<(), tokio::io::Error> {
    let test_file = "test_aof4.dat";
    let _ = tokio::fs::remove_file(test_file).await; //clean the file if exists
    let mut aof = AOF::new(test_file.to_string()).await;

    // Define weights for different operations (adjust weights as needed)
    let weights = &[0.3, 0.5, 0.2];

    // Define possible operations
    let operations = vec![Operation::Put, Operation::Get, Operation::Remove];

    let weighted_dist = WeightedIndex::new(weights).unwrap();
    let mut rng = thread_rng();

    let mut written_records = Vec::new();
    let num_ops = 200; // Adjust the number of random operations

    // Generate random operations and write to AOF
    for _ in 0..num_ops {
        let op = weighted_dist.sample(&mut rng);
        let key = format!("key{}", written_records.len());
        let value = match &operations[op] {
            Operation::Put => Some(format!("value{}", written_records.len())),
            _ => None,
        };
        written_records.push(AOFRecord {
            key: key.clone(),
            value: value.clone(),
            operation: operations[op].clone(),
        });
        aof.on_event(
            AOFRecord {
                key: key.clone(),
                value: value.clone(),
                operation: operations[op].clone(),
            },
            false,
        )
        .await;
    }
    aof.flush().await;

    // Read records from AOF and check order
    let mut iter = aof.into_iter().await.unwrap();
    for record in written_records {
        let next_record = iter.next::<String, String>().await.unwrap().unwrap();
        assert_eq!(next_record.key, record.key);
        assert_eq!(next_record.value, record.value);
        assert_eq!(next_record.operation, record.operation);
    }

    // Cleanup: Delete the test file
    tokio::fs::remove_file(test_file).await?;
    Ok(())
}


#[tokio::test]
async fn test_aof_random_ops_and_iteration_with_multi() -> Result<(), tokio::io::Error> {
    let test_file = "test_aof5.dat";
    let _ = tokio::fs::remove_file(test_file).await; //clean the file if exists
    let aof = AOF::new(test_file.to_string()).await;

    // Define weights for different operations (adjust weights as needed)
    let weights = &[0.3, 0.5, 0.2];

    // Define possible operations
    let operations = vec![Operation::Put, Operation::Get, Operation::Remove];

    let weighted_dist = WeightedIndex::new(weights).unwrap();
    let mut rng = thread_rng();

    let mut written_records = Vec::new();
    let num_ops = 200; // Adjust the number of random operations

    // Generate random operations and write to AOF
    for _ in 0..num_ops {
        let op = weighted_dist.sample(&mut rng);
        let key = format!("key{}", written_records.len());
        let value = match &operations[op] {
            Operation::Put => Some(format!("value{}", written_records.len())),
            _ => None,
        };
        written_records.push(AOFRecord {
            key: key.clone(),
            value: value.clone(),
            operation: operations[op].clone(),
        });
    }
    aof.on_event_multi(written_records.clone(), true).await;

    // Read records from AOF and check order
    let mut iter = aof.into_iter().await.unwrap();
    for record in written_records {
        let next_record = iter.next::<String, String>().await.unwrap().unwrap();
        assert_eq!(next_record.key, record.key);
        assert_eq!(next_record.value, record.value);
        assert_eq!(next_record.operation, record.operation);
    }

    // Cleanup: Delete the test file
    tokio::fs::remove_file(test_file).await?;
    Ok(())
}

#[tokio::test]
async fn test_aof_subscriber_with_disk_and_flush_time() -> Result<(), tokio::io::Error> {
    let test_file = String::from("test_aof6");
    let _ = tokio::fs::remove_file(test_file.clone()+".dat").await; //clean the file if exists
    let subscriber = CacheEventSubscriber::new(
        Some(String::from(".")),
        Some(String::from(test_file.clone())),
        Some(100)
    ).await;

    // Define weights for different operations (adjust weights as needed)
    let weights = &[0.3, 0.5, 0.2];

    // Define possible operations
    let operations = vec![Operation::Put, Operation::Get, Operation::Remove];

    let weighted_dist = WeightedIndex::new(weights).unwrap();
    let mut rng = thread_rng();

    let mut written_records = Vec::new();
    let num_ops = 200; // Adjust the number of random operations

    // Generate random operations and write to AOF
    for _ in 0..num_ops {
        let op = weighted_dist.sample(&mut rng);
        let key = format!("key{}", written_records.len());
        let value = match &operations[op] {
            Operation::Put => Some(format!("value{}", written_records.len())),
            _ => None,
        };
        let r = AOFRecord {
            key: key,
            value: value,
            operation: operations[op].clone(),
        };
        written_records.push(r.clone());
        subscriber.on_event(r).await;
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }
    // Read records from AOF and check order
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    let mut iter = subscriber.into_iter().await?;
    for record in written_records {
        let next_record = iter.next::<String, String>().await.unwrap().unwrap();
        assert_eq!(next_record.key, record.key);
        assert_eq!(next_record.value, record.value);
        assert_eq!(next_record.operation, record.operation);
    }

    // Cleanup: Delete the test file
    tokio::fs::remove_file(test_file+".dat").await?;
    Ok(())
}


#[tokio::test]
async fn test_aof_subscriber_with_disk() -> Result<(), tokio::io::Error> {
    let test_file = "test_aof7";
    let _ = tokio::fs::remove_file(format!("{}.dat", test_file)).await; //clean the file if exists
    let subscriber = CacheEventSubscriber::new(
        Some(String::from(".")),
        Some(String::from(test_file)),
        None
    ).await;

    // Define weights for different operations (adjust weights as needed)
    let weights = &[0.3, 0.5, 0.2];

    // Define possible operations
    let operations = vec![Operation::Put, Operation::Get, Operation::Remove];

    let weighted_dist = WeightedIndex::new(weights).unwrap();
    let mut rng = thread_rng();

    let mut written_records = Vec::new();
    let num_ops = 200; // Adjust the number of random operations

    // Generate random operations and write to AOF
    for _ in 0..num_ops {
        let op = weighted_dist.sample(&mut rng);
        let key = format!("key{}", written_records.len());
        let value = match &operations[op] {
            Operation::Put => Some(format!("value{}", written_records.len())),
            _ => None,
        };
        let r = AOFRecord {
            key: key,
            value: value,
            operation: operations[op].clone(),
        };
        written_records.push(r.clone());
        subscriber.on_event(r).await;
    }
    // Read records from AOF and check order
    let mut iter = subscriber.into_iter().await?;
    for record in written_records {
        let next_record = iter.next::<String, String>().await.unwrap().unwrap();
        assert_eq!(next_record.key, record.key);
        assert_eq!(next_record.value, record.value);
        assert_eq!(next_record.operation, record.operation);
    }

    // Cleanup: Delete the test file
    tokio::fs::remove_file(format!("{}.dat", test_file)).await?;
    Ok(())
}
