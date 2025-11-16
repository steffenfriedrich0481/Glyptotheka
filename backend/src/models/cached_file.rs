use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedFile {
    pub id: i64,
    pub original_path: String,
    pub cache_path: String,
    pub file_type: String,
    pub file_size: i64,
    pub checksum: Option<String>,
    pub cached_at: i64,
    pub accessed_at: i64,
}

#[derive(Debug, Clone)]
pub struct CreateCachedFile {
    pub original_path: String,
    pub cache_path: String,
    pub file_type: String,
    pub file_size: i64,
    pub checksum: Option<String>,
}
