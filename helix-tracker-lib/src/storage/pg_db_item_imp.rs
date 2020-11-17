use crate::core::item::*;
use crate::storage::error::*;
use crate::storage::traits::ItemStorageTrait;
use async_trait::async_trait;
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use tokio_postgres::NoTls;
use uuid;

pub struct PgDbItemTrackerStorage<T: DeserializeOwned> {
    pub pool: Pool,
    item_type: PhantomData<T>,
}

impl<T: DeserializeOwned> PgDbItemTrackerStorage<T> {
    pub fn new(
        database: String,
        host: String,
        port: u16,
        user: String,
        password: String,
    ) -> PgDbItemTrackerStorage<T> {
        let mut cfg = Config::new();
        cfg.dbname = Some(database);
        cfg.host = Some(host);
        cfg.port = Some(port);
        cfg.user = Some(user);
        cfg.password = Some(password);

        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });

        PgDbItemTrackerStorage {
            item_type: PhantomData,
            pool: cfg.create_pool(NoTls).unwrap(),
        }
    }
}

#[async_trait]
impl<T: DeserializeOwned + std::marker::Send + std::marker::Sync> ItemStorageTrait<T>
    for PgDbItemTrackerStorage<T>
{
    async fn get_items(
        &self,
        type_id: &String,
        owner_uuid: &uuid::Uuid,
    ) -> StorageResult<Vec<Item<T>>> {
        let mut result: Vec<Item<T>> = Vec::new();
        let query = "
        select id, uuid, configuration, expired_after, refresh_every, created_on, updated_on, owner_, type_
        from tracker.item
        where 
        1=1
        and type_ = $1 
        and owner_ = $2
        order by id asc;";

        let client = self.pool.get().await.unwrap();
        for row in client.query(query, &[&type_id, &owner_uuid]).await? {
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
