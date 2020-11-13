use crate::core::item::Item;
use crate::core::log::Log;
use crate::storage::error::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait ItemStorageTrait<T: DeserializeOwned>: Send {
    fn get_items(&self, type_id: &String, owner_uuid: &uuid::Uuid) -> StorageResult<Vec<Item<T>>>;
}
/*
pub trait LogStorageTrait<T: DeserializeOwned>: Send {
        &self,
        item_id: &i32,
        payload: &I,
    ) -> StorageResult<Option<Log<T>>>;

    fn get_last_logs_by_type(
        &self,
        type_id: &String,
        owner_uuid: &uuid::Uuid,
    ) -> StorageResult<Vec<Log<T>>>;
}*/
