use crate::business::error::*;
use crate::business::traits::TrackerDomainTrait;
use crate::core::item::*;
use crate::core::log::*;
use crate::storage::traits::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::boxed::Box;

pub struct TrackerDomain<I, L> {
    item_storage: Box<dyn ItemStorageTrait<I>>,
    log_storage: Box<dyn LogStorageTrait<L>>,
}

impl<I, L> TrackerDomain<I, L> {
    pub fn new(
        item_storage: Box<dyn ItemStorageTrait<I>>,
        log_storage: Box<dyn LogStorageTrait<L>>,
    ) -> Self {
        TrackerDomain {
            item_storage,
            log_storage,
        }
    }
}

impl<I: Serialize + DeserializeOwned, L: Serialize + DeserializeOwned> TrackerDomainTrait<I, L>
    for TrackerDomain<I, L>
{
    fn create_item<T: DeserializeOwned>(&self, item: Item<T>) -> TrackerDomainResult<Item<T>> {
        Err(TrackerDomainError::NotImplemented)
    }

    fn get_items(
        &self,
        type_id: &String,
        owner_uuid: &uuid::Uuid,
    ) -> TrackerDomainResult<Vec<Item<I>>> {
        Ok(self.item_storage.get_items(type_id, owner_uuid)?)
    }

    fn add_log(&self, item_id: &uuid::Uuid, payload: &L) -> TrackerDomainResult<Option<Log<L>>> {
        Ok(self.log_storage.add_log(item_id, payload)?)
    }

    fn udpate_log(&self, item_id: &uuid::Uuid, payload: &L) -> TrackerDomainResult<Option<Log<L>>> {
        Err(TrackerDomainError::NotImplemented)
    }

    fn get_logs_by_item(
        &self,
        item_id: &uuid::Uuid,
        owner_uuid: &uuid::Uuid,
    ) -> TrackerDomainResult<Vec<Log<L>>> {
        Ok(self.log_storage.get_logs_by_item(item_id, owner_uuid)?)
    }

    fn get_logs_by_type(
        &self,
        type_id: &String,
        owner_uuid: &uuid::Uuid,
    ) -> TrackerDomainResult<Vec<Log<L>>> {
        Ok(self.log_storage.get_logs_by_type(type_id, owner_uuid)?)
    }

    fn get_last_logs_by_item(
        &self,
        item_id: &uuid::Uuid,
        owner_uuid: &uuid::Uuid,
    ) -> TrackerDomainResult<Vec<Log<L>>> {
        Ok(self
            .log_storage
            .get_last_logs_by_item(item_id, owner_uuid)?)
    }

    fn get_last_logs_by_type(
        &self,
        type_id: &String,
        owner_uuid: &uuid::Uuid,
    ) -> TrackerDomainResult<Vec<Log<L>>> {
        Ok(self
            .log_storage
            .get_last_logs_by_type(type_id, owner_uuid)?)
    }
}
