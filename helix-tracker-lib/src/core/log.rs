use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Log<T> {
    pub uuid: uuid::Uuid,
    pub hash: Option<String>,
    pub data: Option<T>,
    pub created_on: Option<DateTime<Utc>>,
    pub item_id: i32,
}
