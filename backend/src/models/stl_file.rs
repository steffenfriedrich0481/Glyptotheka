use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StlFile {
    pub id: i64,
    pub project_id: i64,
    pub filename: String,
    pub file_path: String,
    pub file_size: i64,
    pub category: Option<String>,
    pub preview_path: Option<String>,
    pub preview_generated_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone)]
pub struct CreateStlFile {
    pub project_id: i64,
    pub filename: String,
    pub file_path: String,
    pub file_size: i64,
    pub category: Option<String>,
}
