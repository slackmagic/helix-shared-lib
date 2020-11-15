use crate::business::error::*;
use crate::business::traits::TrackerDomainTrait;
use crate::core::item::*;
use crate::core::log::*;
use crate::storage::traits::*;
use async_trait::async_trait;
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

#[async_trait]
impl<I: Serialize + DeserializeOwned, L: Serialize + DeserializeOwned + std::marker::Sync>
    TrackerDomainTrait<I, L> for TrackerDomain<I, L>
{
    async fn get_items(
        &self,
        type_id: &String,
        owner_uuid: &uuid::Uuid,
    ) -> TrackerDomainResult<Vec<Item<I>>> {
        match self.item_storage.get_items(type_id, owner_uuid).await {
            Ok(result) => Ok(result),
            Err(_) => Err(TrackerDomainError::StorageError),
        }
    }

    async fn add_log(
        &self,
        item_id: &uuid::Uuid,
        payload: &L,
    ) -> TrackerDomainResult<Option<Log<L>>> {
        Ok(self.log_storage.add_log(item_id, payload).await?)
    }

    async fn get_logs_by_item(
        &self,
        item_id: &uuid::Uuid,
        owner_uuid: &uuid::Uuid,
    ) -> TrackerDomainResult<Vec<Log<L>>> {
        match self.log_storage.get_logs_by_item(item_id, owner_uuid).await {
            Ok(result) => Ok(result),
            Err(_) => Err(TrackerDomainError::StorageError),
        }
    }

    async fn get_logs_by_type(
        &self,
        type_id: &String,
        owner_uuid: &uuid::Uuid,
    ) -> TrackerDomainResult<Vec<Log<L>>> {
        match self.log_storage.get_logs_by_type(type_id, owner_uuid).await {
            Ok(result) => Ok(result),
            Err(_) => Err(TrackerDomainError::StorageError),
        }
    }

    async fn get_last_logs_by_item(
        &self,
        item_id: &uuid::Uuid,
        owner_uuid: &uuid::Uuid,
    ) -> TrackerDomainResult<Vec<Log<L>>> {
        match self
            .log_storage
            .get_last_logs_by_item(item_id, owner_uuid)
            .await
        {
            Ok(result) => Ok(result),
            Err(_) => Err(TrackerDomainError::StorageError),
        }
    }

    async fn get_last_logs_by_type(
        &self,
        type_id: &String,
        owner_uuid: &uuid::Uuid,
    ) -> TrackerDomainResult<Vec<Log<L>>> {
        match self
            .log_storage
            .get_last_logs_by_type(type_id, owner_uuid)
            .await
        {
            Ok(result) => Ok(result),
            Err(_) => Err(TrackerDomainError::StorageError),
        }
    }
}
