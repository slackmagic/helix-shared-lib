use crate::core::item::*;
use crate::storage::error::*;
use crate::storage::traits::ItemStorageTrait;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio_postgres::{Client, NoTls};
use uuid;

pub struct PgDbItemTrackerStorage<T: DeserializeOwned> {
    pub client: Arc<Client>,
    item_type: PhantomData<T>,
}

impl<T: DeserializeOwned> PgDbItemTrackerStorage<T> {
    pub async fn new(conn_string: String) -> StorageResult<PgDbItemTrackerStorage<T>> {
        println!("INIT Connection to DB");
        let (client, connection) = tokio_postgres::connect(&conn_string, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        println!("DONE Connection to DB");
        Ok(PgDbItemTrackerStorage {
            item_type: PhantomData,
            client: Arc::new(client),
        })
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

        for row in &self.client.query(query, &[&type_id, &owner_uuid]).await? {
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
