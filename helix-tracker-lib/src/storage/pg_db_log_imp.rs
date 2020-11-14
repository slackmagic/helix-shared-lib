use crate::core::log::*;
use crate::storage::error::*;
use crate::storage::traits::LogStorageTrait;
use postgres::{Connection, TlsMode};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use std::mem;
use uuid;

pub struct PgDbLogTrackerStorage<T: DeserializeOwned> {
    pub db_conn: Connection,
    item_type: PhantomData<T>,
}

impl<T: DeserializeOwned> PgDbLogTrackerStorage<T> {
    pub fn new(conn_string: String) -> PgDbLogTrackerStorage<T> {
        let t_connection: Connection = Connection::connect(conn_string, TlsMode::None).unwrap();
        PgDbLogTrackerStorage {
            db_conn: t_connection,
            item_type: PhantomData,
        }
    }
}

impl<T: DeserializeOwned + std::marker::Send> LogStorageTrait<T> for PgDbLogTrackerStorage<T> {
    fn add_log(&self, item_id: &i32, payload: &T) -> StorageResult<Option<Log<T>>> {
        Err(StorageError::NotImplemented)
    }

    fn get_last_logs_by_type(
        &self,
        type_id: &String,
        owner_uuid: &uuid::Uuid,
    ) -> StorageResult<Vec<Log<T>>> {
        Err(StorageError::NotImplemented)
    }
}
