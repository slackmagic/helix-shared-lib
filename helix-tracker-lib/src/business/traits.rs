use crate::business::error::TrackerDomainResult;
use crate::core::item::Item;
use crate::core::log::Log;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[async_trait]
pub trait TrackerDomainTrait<I: Serialize + DeserializeOwned, L: Serialize + DeserializeOwned>:
    Send + Sync
{
    async fn get_items(
        &self,
        type_id: &String,
        owner_uuid: &uuid::Uuid,
    ) -> TrackerDomainResult<Vec<Item<I>>>;

    async fn add_log(
        &self,
        item_id: &uuid::Uuid,
        payload: &L,
    ) -> TrackerDomainResult<Option<Log<L>>>;

    async fn get_logs_by_item(
        &self,
        item_id: &uuid::Uuid,
        owner_uuid: &uuid::Uuid,
    ) -> TrackerDomainResult<Vec<Log<L>>>;

    async fn get_logs_by_type(
        &self,
        type_id: &String,
        owner_uuid: &uuid::Uuid,
    ) -> TrackerDomainResult<Vec<Log<L>>>;

    async fn get_last_logs_by_item(
        &self,
        item_id: &uuid::Uuid,
        owner_uuid: &uuid::Uuid,
        steps: i64,
    ) -> TrackerDomainResult<Vec<Log<L>>>;

    async fn get_last_logs_by_type(
        &self,
        type_id: &String,
        owner_uuid: &uuid::Uuid,
        steps: i64,
    ) -> TrackerDomainResult<Vec<Log<L>>>;
}
