use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageFile {
    pub id: i64,
    pub project_id: i64,
    pub filename: String,
    pub file_path: String,
    pub file_size: i64,
    pub source_type: String,
    pub source_project_id: Option<i64>,
    pub display_order: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone)]
pub struct CreateImageFile {
    pub project_id: i64,
    pub filename: String,
    pub file_path: String,
    pub file_size: i64,
    pub source_type: String,
    pub source_project_id: Option<i64>,
    pub display_order: i32,
}
