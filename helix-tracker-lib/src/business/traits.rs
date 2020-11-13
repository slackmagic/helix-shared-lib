use crate::business::error::TrackerDomainResult;
use crate::core::item::Item;
use crate::core::log::Log;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait TrackerInteractorTrait {
    //fn create_item<T: DeserializeOwned>(&self, item: Item<T>) -> TrackerDomainResult<Item<T>>;

    fn get_items<T: DeserializeOwned>(
        &self,
        type_id: &String,
        owner_uuid: &uuid::Uuid,
    ) -> TrackerDomainResult<Vec<Item<T>>>;

    fn add_log<T: Serialize + DeserializeOwned>(
        &self,
        item_id: &i32,
        payload: &T,
    ) -> TrackerDomainResult<Option<Log<T>>>;

    fn get_last_logs_by_type<T: DeserializeOwned>(
        &self,
        type_id: &String,
        owner_uuid: &uuid::Uuid,
    ) -> TrackerDomainResult<Vec<Log<T>>>;
}
