use crate::db::connection::DbPool;
use crate::utils::error::AppError;
use rusqlite::params;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub id: i64,
    pub root_path: Option<String>,
    pub last_scan_at: Option<i64>,
    pub stl_thumb_path: Option<String>,
    pub cache_max_size_mb: i64,
    pub images_per_page: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfigRequest {
    pub root_path: Option<String>,
    pub stl_thumb_path: Option<String>,
    pub cache_max_size_mb: Option<i64>,
    pub images_per_page: Option<i64>,
}

pub struct ConfigService {
    pool: DbPool,
}

impl ConfigService {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn get_config(&self) -> Result<AppConfig, AppError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, root_path, last_scan_at, stl_thumb_path, cache_max_size_mb, images_per_page, created_at, updated_at
             FROM config WHERE id = 1"
        )?;

        let config = stmt.query_row([], |row| {
            Ok(AppConfig {
                id: row.get(0)?,
                root_path: row.get(1)?,
                last_scan_at: row.get(2)?,
                stl_thumb_path: row.get(3)?,
                cache_max_size_mb: row.get(4)?,
                images_per_page: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?;

        Ok(config)
    }

    pub fn update_config(&self, updates: &UpdateConfigRequest) -> Result<AppConfig, AppError> {
        let conn = self.pool.get()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;

        if let Some(ref root_path) = updates.root_path {
            conn.execute(
                "UPDATE config SET root_path = ?1, updated_at = ?2 WHERE id = 1",
                params![root_path, now],
            )?;
        }

        if let Some(ref stl_thumb_path) = updates.stl_thumb_path {
            conn.execute(
                "UPDATE config SET stl_thumb_path = ?1, updated_at = ?2 WHERE id = 1",
                params![stl_thumb_path, now],
            )?;
        }

        if let Some(cache_max_size_mb) = updates.cache_max_size_mb {
            conn.execute(
                "UPDATE config SET cache_max_size_mb = ?1, updated_at = ?2 WHERE id = 1",
                params![cache_max_size_mb, now],
            )?;
        }

        if let Some(images_per_page) = updates.images_per_page {
            conn.execute(
                "UPDATE config SET images_per_page = ?1, updated_at = ?2 WHERE id = 1",
                params![images_per_page, now],
            )?;
        }

        self.get_config()
    }

    pub fn update_last_scan(&self) -> Result<(), AppError> {
        let conn = self.pool.get()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;

        conn.execute(
            "UPDATE config SET last_scan_at = ?1, updated_at = ?2 WHERE id = 1",
            params![now, now],
        )?;

        Ok(())
    }
}
