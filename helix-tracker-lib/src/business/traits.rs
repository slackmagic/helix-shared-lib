use crate::business::error::TrackerDomainResult;
use crate::core::item::Item;
use crate::core::log::Log;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait TrackerDomainTrait<I: Serialize + DeserializeOwned, L: Serialize + DeserializeOwned>:
    Send
{
    fn create_item(&self, item: Item<I>) -> TrackerDomainResult<Item<I>>;

    fn get_items(
        &self,
        type_id: &String,
        owner_uuid: &uuid::Uuid,
    ) -> TrackerDomainResult<Vec<Item<I>>>;

    fn add_log(&self, item_id: &uuid::Uuid, payload: &L) -> TrackerDomainResult<Option<Log<L>>>;

    fn udpate_log(&self, item_id: &uuid::Uuid, payload: &L) -> TrackerDomainResult<Option<Log<L>>>;

    fn get_logs_by_item(
        &self,
        item_id: &uuid::Uuid,
        owner_uuid: &uuid::Uuid,
    ) -> TrackerDomainResult<Vec<Log<L>>>;

    fn get_logs_by_type(
        &self,
        type_id: &String,
        owner_uuid: &uuid::Uuid,
    ) -> TrackerDomainResult<Vec<Log<L>>>;

    fn get_last_logs_by_item(
        &self,
        item_id: &uuid::Uuid,
        owner_uuid: &uuid::Uuid,
    ) -> TrackerDomainResult<Vec<Log<L>>>;

    fn get_last_logs_by_type(
        &self,
        type_id: &String,
        owner_uuid: &uuid::Uuid,
    ) -> TrackerDomainResult<Vec<Log<L>>>;
}
