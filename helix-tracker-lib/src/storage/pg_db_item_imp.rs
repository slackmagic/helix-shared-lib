use crate::core::item::*;
use crate::storage::error::*;
use crate::storage::traits::ItemStorageTrait;
use postgres::{Connection, TlsMode};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
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
        let mut result: Vec<Item<T>> = Vec::new();
        let query: String = "
        select id, uuid, configuration, expired_after, refresh_every, created_on, updated_on, owner_, type_
        from tracker.item
        where 
        1=1
        and type_ = $1 
        and owner_ = $2
        order by id asc;"
            .to_string();

        for row in &self
            .db_conn
            .query(&query, &[&type_id, &owner_uuid])
            .unwrap()
        {
            let parsed_config: Option<T> = match serde_json::from_value(row.get("configuration")) {
                Ok(config) => Some(config),
                Err(_) => None,
            };

            let item: Item<T> = Item {
                id: row.get("id"),
                configuration: parsed_config,
                expired_after: row.get("expired_after"),
                refresh_every: row.get("refresh_every"),
                created_on: row.get("created_on"),
                updated_on: row.get("updated_on"),
                owner: row.get("owner_"),
                type_id: row.get("type_"),
            };

            result.push(item);
        }

        Ok(result)
    }
}
