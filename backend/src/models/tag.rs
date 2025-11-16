use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub color: Option<String>,
    pub created_at: i64,
    pub usage_count: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTag {
    pub name: String,
    pub color: Option<String>,
}
