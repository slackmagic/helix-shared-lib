use crate::core::log::*;
use crate::storage::error::*;
use crate::storage::traits::LogStorageTrait;
use postgres::{Connection, TlsMode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::PhantomData;
use uuid;

pub struct PgDbLogTrackerStorage<T: Serialize + DeserializeOwned> {
    pub db_conn: Connection,
    item_type: PhantomData<T>,
}

impl<T: Serialize + DeserializeOwned> PgDbLogTrackerStorage<T> {
    pub fn new(conn_string: String) -> PgDbLogTrackerStorage<T> {
        let t_connection: Connection = Connection::connect(conn_string, TlsMode::None).unwrap();
        PgDbLogTrackerStorage {
            db_conn: t_connection,
            item_type: PhantomData,
        }
    }
}

impl<T: Serialize + DeserializeOwned + std::marker::Send> LogStorageTrait<T>
    for PgDbLogTrackerStorage<T>
{
    fn add_log(&self, item_id: &i32, payload: &T) -> StorageResult<Option<Log<T>>> {
        let query: String = "
        INSERT INTO tracker.log
        VALUES (DEFAULT,$1,DEFAULT,$2)
        RETURNING uuid, created_on, data, item_;"
            .to_string();

        let json_data = serde_json::to_value(payload).unwrap();
        let stmt = &self.db_conn.prepare(&query).unwrap();
        let row_inserted = stmt.query(&[&json_data, &item_id]).unwrap();

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

    fn get_logs_by_type(
        &self,
        type_id: &String,
        owner_uuid: &uuid::Uuid,
    ) -> StorageResult<Vec<Log<T>>> {
        let mut result: Vec<Log<T>> = Vec::new();

        let query: String = "
        SELECT *
        FROM
            tracker.log,
            tracker.item,
        WHERE 1=1
        AND tracker.item.id = tracker.log.item_
        AND tracker.item.type_ = $1
        AND tracker.item.owner_ = $2
        ORDER BY tracker.item.id asc "
            .to_string();

        let rows = &self
            .db_conn
            .query(query.as_str(), &[&type_id, &owner_uuid])
            .unwrap();

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

    fn get_last_logs_by_type(
        &self,
        type_id: &String,
        owner_uuid: &uuid::Uuid,
    ) -> StorageResult<Vec<Log<T>>> {
        let mut result: Vec<Log<T>> = Vec::new();

        let query: String = "
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
        ORDER BY tracker.item.id asc "
            .to_string();

        let rows = &self
            .db_conn
            .query(query.as_str(), &[&type_id, &owner_uuid])
            .unwrap();

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
