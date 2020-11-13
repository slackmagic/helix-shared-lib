#[derive(Debug, Serialize, Deserialize)]
pub struct ItemType {
    pub id: String,
    pub name: String,
    pub expired_after: Option<String>,
    pub refresh_every: Option<String>,
}
