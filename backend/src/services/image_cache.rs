use crate::db::connection::DbPool;
use crate::utils::error::AppError;
use rusqlite::params;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct ImageCacheService {
    cache_dir: PathBuf,
    pool: DbPool,
}

impl ImageCacheService {
    pub fn new(cache_dir: PathBuf, pool: DbPool) -> Self {
        // Ensure cache directories exist
        let _ = fs::create_dir_all(&cache_dir);
        let _ = fs::create_dir_all(cache_dir.join("images"));
        let _ = fs::create_dir_all(cache_dir.join("previews"));

        Self { cache_dir, pool }
    }

    pub fn get_cached_image(&self, original_path: &str) -> Result<Option<PathBuf>, AppError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT cache_path FROM cached_files WHERE original_path = ? AND file_type = 'image'",
        )?;

        let result = stmt.query_row(params![original_path], |row| {
            let cache_path: String = row.get(0)?;
            Ok(PathBuf::from(cache_path))
        });

        match result {
            Ok(path) if path.exists() => {
                self.update_access_time(original_path)?;
                Ok(Some(path))
            }
            _ => Ok(None),
        }
    }

    pub fn cache_image(&self, original_path: &str) -> Result<PathBuf, AppError> {
        if let Some(cached) = self.get_cached_image(original_path)? {
            return Ok(cached);
        }

        let original = Path::new(original_path);
        if !original.exists() {
            return Err(AppError::NotFound(format!(
                "Image not found: {}",
                original_path
            )));
        }

        let hash = self.hash_path(original_path);
        let ext = original
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("jpg");
        let cache_filename = format!("{}.{}", hash, ext);
        let cache_path = self.cache_dir.join("images").join(cache_filename);

        fs::copy(original, &cache_path)?;

        let file_size = fs::metadata(&cache_path)?.len() as i64;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;

        let conn = self.pool.get()?;
        conn.execute(
            "INSERT INTO cached_files (original_path, cache_path, file_type, file_size, checksum, cached_at, accessed_at)
             VALUES (?1, ?2, 'image', ?3, ?4, ?5, ?6)
             ON CONFLICT(original_path) DO UPDATE SET accessed_at = ?6",
            params![
                original_path,
                cache_path.to_str().unwrap(),
                file_size,
                hash,
                now,
                now
            ],
        )?;

        Ok(cache_path)
    }

    pub fn get_cached_preview(&self, stl_path: &str) -> Result<Option<PathBuf>, AppError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT cache_path FROM cached_files WHERE original_path = ? AND file_type = 'preview'",
        )?;

        let result = stmt.query_row(params![stl_path], |row| {
            let cache_path: String = row.get(0)?;
            Ok(PathBuf::from(cache_path))
        });

        match result {
            Ok(path) if path.exists() => {
                self.update_access_time(stl_path)?;
                Ok(Some(path))
            }
            _ => Ok(None),
        }
    }

    pub fn cache_preview(&self, stl_path: &str, preview_data: &[u8]) -> Result<PathBuf, AppError> {
        let hash = self.hash_path(stl_path);
        let cache_filename = format!("{}.png", hash);
        let cache_path = self.cache_dir.join("previews").join(cache_filename);

        fs::write(&cache_path, preview_data)?;

        let file_size = preview_data.len() as i64;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;

        let conn = self.pool.get()?;
        conn.execute(
            "INSERT INTO cached_files (original_path, cache_path, file_type, file_size, checksum, cached_at, accessed_at)
             VALUES (?1, ?2, 'preview', ?3, ?4, ?5, ?6)
             ON CONFLICT(original_path) DO UPDATE SET accessed_at = ?6",
            params![
                stl_path,
                cache_path.to_str().unwrap(),
                file_size,
                hash,
                now,
                now
            ],
        )?;

        Ok(cache_path)
    }

    fn hash_path(&self, path: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(path.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn update_access_time(&self, original_path: &str) -> Result<(), AppError> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;

        let conn = self.pool.get()?;
        conn.execute(
            "UPDATE cached_files SET accessed_at = ?1 WHERE original_path = ?2",
            params![now, original_path],
        )?;

        Ok(())
    }

    pub fn get_image_by_hash(&self, hash: &str) -> Result<Option<PathBuf>, AppError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT cache_path, original_path FROM cached_files WHERE checksum = ? LIMIT 1",
        )?;

        let result = stmt.query_row(params![hash], |row| {
            let cache_path: String = row.get(0)?;
            let original_path: String = row.get(1)?;
            Ok((PathBuf::from(cache_path), original_path))
        });

        match result {
            Ok((path, original_path)) if path.exists() => {
                self.update_access_time(&original_path)?;
                Ok(Some(path))
            }
            _ => Ok(None),
        }
    }

    /// Clean up orphaned cache entries where original files no longer exist
    pub fn cleanup_orphaned(&self) -> Result<usize, AppError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT id, original_path, cache_path FROM cached_files")?;

        let entries: Vec<(i64, String, String)> = stmt
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?
            .collect::<Result<Vec<_>, _>>()?;

        let mut removed = 0;

        for (id, original_path, cache_path) in entries {
            // Check if original file still exists
            if !Path::new(&original_path).exists() {
                // Remove cache file
                let cache_file = Path::new(&cache_path);
                if cache_file.exists() {
                    let _ = fs::remove_file(cache_file);
                }

                // Remove database entry
                conn.execute("DELETE FROM cached_files WHERE id = ?1", params![id])?;
                removed += 1;
            }
        }

        Ok(removed)
    }

    /// Clear all cached files (images and previews)
    pub fn clear_all(&self) -> Result<usize, AppError> {
        let mut removed = 0;

        // Clear images directory
        let images_dir = self.cache_dir.join("images");
        if images_dir.exists() {
            if let Ok(entries) = fs::read_dir(&images_dir) {
                for entry in entries.flatten() {
                    if fs::remove_file(entry.path()).is_ok() {
                        removed += 1;
                    }
                }
            }
        }

        // Clear previews directory
        let previews_dir = self.cache_dir.join("previews");
        if previews_dir.exists() {
            if let Ok(entries) = fs::read_dir(&previews_dir) {
                for entry in entries.flatten() {
                    if fs::remove_file(entry.path()).is_ok() {
                        removed += 1;
                    }
                }
            }
        }

        // Also clear the database entries if the table exists
        if let Ok(conn) = self.pool.get() {
            let _ = conn.execute("DELETE FROM cached_files", []);
        }

        tracing::info!("Cleared {} cached files", removed);

        Ok(removed)
    }
}
