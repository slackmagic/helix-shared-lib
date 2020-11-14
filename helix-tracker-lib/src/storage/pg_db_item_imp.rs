use crate::core::item::*;
use crate::storage::error::*;
use crate::storage::traits::ItemStorageTrait;
use postgres::{Connection, TlsMode};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use std::mem;
use uuid;

pub struct PgDbItemTrackerStorage<T: DeserializeOwned> {
    pub db_conn: Connection,
    item_type: PhantomData<T>,
}

impl<T: DeserializeOwned> PgDbItemTrackerStorage<T> {
    pub fn new(conn_string: String) -> PgDbItemTrackerStorage<T> {
        let t_connection: Connection = Connection::connect(conn_string, TlsMode::None).unwrap();
        PgDbItemTrackerStorage {
            db_conn: t_connection,
            item_type: PhantomData,
        }
    }
}

impl<T: DeserializeOwned + std::marker::Send> ItemStorageTrait<T> for PgDbItemTrackerStorage<T> {
    fn get_items(&self, type_id: &String, owner_uuid: &uuid::Uuid) -> StorageResult<Vec<Item<T>>> {
        Err(StorageError::NotImplemented)
    }
}
