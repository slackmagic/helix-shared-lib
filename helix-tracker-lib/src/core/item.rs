use chrono::prelude::*;
use uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Item<T> {
    pub id: uuid::Uuid,
    pub configuration: Option<T>,
    pub expired_after: Option<String>,
    pub refresh_every: Option<String>,
    pub created_on: Option<DateTime<Utc>>,
    pub updated_on: Option<DateTime<Utc>>,
    pub owner: uuid::Uuid,
    pub type_id: String,
}

impl<T> Item<T> {
    pub fn new(
        id: uuid::Uuid,
        configuration: Option<T>,
        expired_after: Option<String>,
        refresh_every: Option<String>,
        created_on: Option<DateTime<Utc>>,
        updated_on: Option<DateTime<Utc>>,
        owner: uuid::Uuid,
        type_id: String,
    ) -> Item<T> {
        Item {
            id: id,
            configuration: configuration,
            expired_after: expired_after,
            refresh_every: refresh_every,
            created_on: created_on,
            updated_on: updated_on,
            owner: owner,
            type_id: type_id,
        }
    }
}
