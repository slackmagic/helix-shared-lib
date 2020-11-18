use crate::core::log::*;
use crate::storage::error::*;
use crate::storage::traits::LogStorageTrait;
use async_trait::async_trait;
use blake2b_simd::blake2b;
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::PhantomData;
use tokio_postgres::NoTls;
use uuid;

pub struct PgDbLogTrackerStorage<T: Serialize + DeserializeOwned> {
    pub pool: Pool,
    item_type: PhantomData<T>,
}

impl<T: Serialize + DeserializeOwned> PgDbLogTrackerStorage<T> {
    pub fn new(
        database: String,
        host: String,
        port: u16,
        user: String,
        password: String,
    ) -> PgDbLogTrackerStorage<T> {
        let mut cfg = Config::new();
        cfg.dbname = Some(database);
        cfg.host = Some(host);
        cfg.port = Some(port);
        cfg.user = Some(user);
        cfg.password = Some(password);
        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });

        PgDbLogTrackerStorage {
            item_type: PhantomData,
            pool: cfg.create_pool(NoTls).unwrap(),
        }
    }
}

#[async_trait]
impl<T: Serialize + DeserializeOwned + std::marker::Send + std::marker::Sync> LogStorageTrait<T>
    for PgDbLogTrackerStorage<T>
{
    async fn add_log(&self, item_id: &uuid::Uuid, payload: &T) -> StorageResult<Option<Log<T>>> {
        let json_data = serde_json::to_value(payload).unwrap();

        let hash = blake2b(json_data.to_string().as_bytes())
            .to_hex()
            .to_string();

        let client = &self.pool.get().await.unwrap();

        let query = "SELECT * FROM tracker.log where log.hash = $1;";
        let existing_log = client.query(query, &[&hash]).await?;
        match existing_log.iter().next() {
            None => {
                let query = "
                INSERT INTO tracker.log
                VALUES (DEFAULT,$1, $2, DEFAULT,$3)
                RETURNING uuid, hash, created_on, data, item_;";
                let row_inserted = client.query(query, &[&hash, &json_data, &item_id]).await?;
                row_inserted.iter().next().unwrap();
                match row_inserted.iter().next() {
                    Some(row) => {
                        let parsed_payload: Option<T> =
                            match serde_json::from_value(row.get("data")) {
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
        steps: u32,
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
        AND logs_with_col_numbers.col = $3
        AND tracker.item.owner_ = $2
        ORDER BY tracker.item.id asc ";

        let client = &self.pool.get().await.unwrap();
        let rows = client
            .query(query, &[&type_id, &owner_uuid, &steps])
            .await?;

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
        steps: u32,
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
        AND logs_with_col_numbers.col = $3
        AND tracker.item.owner_ = $2
        ORDER BY tracker.item.id asc ";

        let client = &self.pool.get().await.unwrap();
        let rows = client.query(query, &[&item_id, &owner_uuid, &1]).await?;

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hash_from_json() {
        let expected = "5c8c3952796b2bc109182132acd0c9d2b4006f2733a1238a2bc904552314aa1cd2057e47407796993c9cbbd240a16731ce64081ab6fb2808117c9ca7e2a65588";
        let hash = blake2b(b"{a json}");
        println!("{:?}", &hash.to_hex().to_string());
        assert_eq!(expected, &hash.to_hex());
    }
}
