use crate::core::log::*;
use crate::storage::error::*;
use crate::storage::traits::LogStorageTrait;
use async_trait::async_trait;
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod};
use serde::de::DeserializeOwned;
use serde::Serialize;
use sha1::{Digest, Sha1};
use std::marker::PhantomData;
use tokio_postgres::NoTls;
use uuid;

pub struct PgDbLogTrackerStorage<T: Serialize + DeserializeOwned> {
    pub pool: Pool,
    item_type: PhantomData<T>,
}

impl<T: Serialize + DeserializeOwned> PgDbLogTrackerStorage<T> {
    pub fn new(conn_string: String) -> StorageResult<PgDbLogTrackerStorage<T>> {
        let mut cfg = Config::new();
        cfg.dbname = Some(conn_string);
        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });

        Ok(PgDbLogTrackerStorage {
            item_type: PhantomData,
            pool: cfg.create_pool(NoTls).unwrap(),
        })
    }
}

#[async_trait]
impl<T: Serialize + DeserializeOwned + std::marker::Send + std::marker::Sync> LogStorageTrait<T>
    for PgDbLogTrackerStorage<T>
{
    async fn add_log(&self, item_id: &uuid::Uuid, payload: &T) -> StorageResult<Option<Log<T>>> {
        let query = "
        INSERT INTO tracker.log
        VALUES (DEFAULT,$1, $2, DEFAULT,$3)
        RETURNING uuid, created_on, data, item_;";

        let json_data = serde_json::to_value(payload).unwrap();

        let mut hasher = Sha1::new();
        hasher.update(json_data.to_string().as_bytes());
        let hash = &hasher.finalize()[..];
        let hash = std::str::from_utf8(hash).unwrap();

        let client = &self.pool.get().await.unwrap();
        let row_inserted = client.query(query, &[&hash, &json_data, &item_id]).await?;

        row_inserted.iter().next().unwrap();

        match row_inserted.iter().next() {
            Some(row) => {
                let parsed_payload: Option<T> = match serde_json::from_value(row.get("data")) {
                    Ok(payload) => Some(payload),
                    Err(_) => None,
                };
                Ok(Some(Log {
                    uuid: row.get("uuid"),
                    created_on: row.get("created_on"),
                    hash: row.get("hash"),
                    data: parsed_payload,
                    item_id: row.get("item_"),
                }))
            }
            None => {
                println!("HELIX Error impossible to create LOG");
                Err(StorageError::CreationImpossible)
            }
        }
    }

    async fn get_logs_by_type(
        &self,
        type_id: &String,
        owner_uuid: &uuid::Uuid,
    ) -> StorageResult<Vec<Log<T>>> {
        let mut result: Vec<Log<T>> = Vec::new();

        let query = "
        SELECT *
        FROM
            tracker.log,
            tracker.item
        WHERE 1=1
        AND tracker.item.id = tracker.log.item_
        AND tracker.item.type_ = $1
        AND tracker.item.owner_ = $2
        ORDER BY tracker.item.id asc ";

        let client = &self.pool.get().await.unwrap();
        let rows = client.query(query, &[&type_id, &owner_uuid]).await?;

        for row in rows {
            let parsed_payload: Option<T> = match serde_json::from_value(row.get("data")) {
                Ok(payload) => Some(payload),
                Err(_) => None,
            };
            result.push(Log {
                uuid: row.get("uuid"),
                created_on: row.get("created_on"),
                hash: row.get("hash"),
                data: parsed_payload,
                item_id: row.get("item_"),
            });
        }

        Ok(result)
    }

    async fn get_logs_by_item(
        &self,
        item_id: &uuid::Uuid,
        owner_uuid: &uuid::Uuid,
    ) -> StorageResult<Vec<Log<T>>> {
        let mut result: Vec<Log<T>> = Vec::new();

        let query = "
        SELECT *
        FROM
            tracker.log,
            tracker.item
        WHERE 1=1
        AND tracker.log.item_ =$1
        AND tracker.item.owner_ = $2
        ORDER BY tracker.item.id asc ";

        let client = &self.pool.get().await.unwrap();
        let rows = client.query(query, &[&item_id, &owner_uuid]).await?;

        for row in rows {
            let parsed_payload: Option<T> = match serde_json::from_value(row.get("data")) {
                Ok(payload) => Some(payload),
                Err(_) => None,
            };
            result.push(Log {
                uuid: row.get("uuid"),
                created_on: row.get("created_on"),
                hash: row.get("hash"),
                data: parsed_payload,
                item_id: row.get("item_"),
            });
        }

        Ok(result)
    }

    async fn get_last_logs_by_type(
        &self,
        type_id: &String,
        owner_uuid: &uuid::Uuid,
    ) -> StorageResult<Vec<Log<T>>> {
        let mut result: Vec<Log<T>> = Vec::new();

        let query = "
        SELECT logs_with_col_numbers.uuid as log_uuid, *
        FROM 
            (SELECT *, ROW_NUMBER() OVER (PARTITION BY item_ ORDER BY created_on DESC) col 
            FROM tracker.log
            ORDER BY uuid ASC) logs_with_col_numbers,
            tracker.item
        where 1=1
        AND tracker.item.id = logs_with_col_numbers.item_
        AND tracker.item.type_ = $1
        AND logs_with_col_numbers.col = 1
        AND tracker.item.owner_ = $2
        ORDER BY tracker.item.id asc ";

        let client = &self.pool.get().await.unwrap();
        let rows = client.query(query, &[&type_id, &owner_uuid]).await?;

        for row in rows {
            let parsed_payload: Option<T> = match serde_json::from_value(row.get("data")) {
                Ok(payload) => Some(payload),
                Err(_) => None,
            };
            result.push(Log {
                uuid: row.get("uuid"),
                created_on: row.get("created_on"),
                hash: row.get("hash"),
                data: parsed_payload,
                item_id: row.get("item_"),
            });
        }

        Ok(result)
    }

    async fn get_last_logs_by_item(
        &self,
        item_id: &uuid::Uuid,
        owner_uuid: &uuid::Uuid,
    ) -> StorageResult<Vec<Log<T>>> {
        let mut result: Vec<Log<T>> = Vec::new();

        let query = "
        SELECT logs_with_col_numbers.uuid as log_uuid, *
        FROM 
            (SELECT *, ROW_NUMBER() OVER (PARTITION BY item_ ORDER BY created_on DESC) col 
            FROM tracker.log
            ORDER BY uuid ASC) logs_with_col_numbers,
            tracker.item
        where 1=1
        AND logs_with_col_numbers.item_ = $1
        AND logs_with_col_numbers.col = 1
        AND tracker.item.owner_ = $2
        ORDER BY tracker.item.id asc ";

        let client = &self.pool.get().await.unwrap();
        let rows = client.query(query, &[&item_id, &owner_uuid]).await?;

        for row in rows {
            let parsed_payload: Option<T> = match serde_json::from_value(row.get("data")) {
                Ok(payload) => Some(payload),
                Err(_) => None,
            };
            result.push(Log {
                uuid: row.get("uuid"),
                created_on: row.get("created_on"),
                hash: row.get("hash"),
                data: parsed_payload,
                item_id: row.get("item_"),
            });
        }

        Ok(result)
    }
}
