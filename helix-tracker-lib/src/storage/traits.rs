use crate::core::item::Item;
use crate::core::log::Log;
use crate::storage::error::*;
use async_trait::async_trait;
use serde::de::DeserializeOwned;

#[async_trait]
pub trait ItemStorageTrait<T: DeserializeOwned>: Send + Sync {
    async fn get_items(
        &self,
        type_id: &String,
        owner_uuid: &uuid::Uuid,
    ) -> StorageResult<Vec<Item<T>>>;
}

#[async_trait]
pub trait LogStorageTrait<T: DeserializeOwned>: Send + Sync {
    async fn add_log(&self, item_id: &uuid::Uuid, payload: &T) -> StorageResult<Option<Log<T>>>;

    async fn get_logs_by_item(
        &self,
        item_id: &uuid::Uuid,
        owner_uuid: &uuid::Uuid,
    ) -> StorageResult<Vec<Log<T>>>;

    async fn get_logs_by_type(
        &self,
        type_id: &String,
        owner_uuid: &uuid::Uuid,
    ) -> StorageResult<Vec<Log<T>>>;

    async fn get_last_logs_by_type(
        &self,
        type_id: &String,
        owner_uuid: &uuid::Uuid,
        steps: i64,
    ) -> StorageResult<Vec<Log<T>>>;

    async fn get_last_logs_by_item(
        &self,
        item_id: &uuid::Uuid,
        owner_uuid: &uuid::Uuid,
        steps: i64,
    ) -> StorageResult<Vec<Log<T>>>;
}
