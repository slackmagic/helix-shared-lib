use crate::core::item::Item;
use crate::core::log::Log;
use crate::storage::error::*;
use serde::de::DeserializeOwned;

pub trait ItemStorageTrait<T: DeserializeOwned>: Send {
    fn get_items(&self, type_id: &String, owner_uuid: &uuid::Uuid) -> StorageResult<Vec<Item<T>>>;
}

pub trait LogStorageTrait<T: DeserializeOwned>: Send {
    fn add_log(&self, item_id: &uuid::Uuid, payload: &T) -> StorageResult<Option<Log<T>>>;

    fn get_logs_by_item(
        &self,
        item_id: &uuid::Uuid,
        owner_uuid: &uuid::Uuid,
    ) -> StorageResult<Vec<Log<T>>>;

    fn get_logs_by_type(
        &self,
        type_id: &String,
        owner_uuid: &uuid::Uuid,
    ) -> StorageResult<Vec<Log<T>>>;

    fn get_last_logs_by_type(
        &self,
        type_id: &String,
        owner_uuid: &uuid::Uuid,
    ) -> StorageResult<Vec<Log<T>>>;

    fn get_last_logs_by_item(
        &self,
        item_id: &uuid::Uuid,
        owner_uuid: &uuid::Uuid,
    ) -> StorageResult<Vec<Log<T>>>;
}
