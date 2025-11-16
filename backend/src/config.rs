#[derive(Debug, Clone)]
pub struct Config {
    pub database_path: String,
    pub cache_dir: String,
    pub stl_thumb_path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_path: "glyptotheka.db".to_string(),
            cache_dir: "cache".to_string(),
            stl_thumb_path: Some("stl-thumb".to_string()),
        }
    }
}
