use crate::db::connection::DbPool;
use crate::services::image_cache::ImageCacheService;
use crate::utils::error::AppError;
use rusqlite::params;
use std::path::{Path, PathBuf};
use std::process::Command;
use tokio::sync::mpsc;
use tokio::task;
use tracing::{info, warn};

#[derive(Clone)]
pub struct StlPreviewService {
    stl_thumb_path: Option<PathBuf>,
    image_cache: ImageCacheService,
    pool: DbPool,
}

impl StlPreviewService {
    pub fn new(stl_thumb_path: Option<PathBuf>, image_cache: ImageCacheService, pool: DbPool) -> Self {
        Self {
            stl_thumb_path,
            image_cache,
            pool,
        }
    }

    /// Generate a preview for an STL file
    pub async fn generate_preview(&self, stl_path: &str) -> Result<PathBuf, AppError> {
        // Check if preview already exists in cache
        if let Some(cached_path) = self.image_cache.get_cached_preview(stl_path)? {
            info!("Using cached preview for {}", stl_path);
            return Ok(cached_path);
        }

        // Check if stl-thumb is available
        let stl_thumb_path = self.stl_thumb_path.as_ref().ok_or_else(|| {
            AppError::InternalServer("stl-thumb is not configured".to_string())
        })?;

        if !stl_thumb_path.exists() {
            return Err(AppError::InternalServer(format!(
                "stl-thumb not found at: {}",
                stl_thumb_path.display()
            )));
        }

        let stl_path_buf = PathBuf::from(stl_path);
        if !stl_path_buf.exists() {
            return Err(AppError::NotFound(format!("STL file not found: {}", stl_path)));
        }

        // Generate preview using stl-thumb
        let preview_data = self.run_stl_thumb(&stl_path_buf, stl_thumb_path).await?;

        // Cache the preview
        let cache_path = self.image_cache.cache_preview(stl_path, &preview_data)?;

        // Update database with preview information
        self.update_stl_preview_info(stl_path, cache_path.to_str().unwrap())?;

        info!("Generated preview for {}", stl_path);
        Ok(cache_path)
    }

    /// Run stl-thumb command to generate preview
    async fn run_stl_thumb(&self, stl_path: &Path, stl_thumb_path: &Path) -> Result<Vec<u8>, AppError> {
        let stl_path = stl_path.to_path_buf();
        let stl_thumb_path = stl_thumb_path.to_path_buf();

        task::spawn_blocking(move || {
            let output = Command::new(&stl_thumb_path)
                .arg(stl_path.as_os_str())
                .arg("-") // Output to stdout
                .arg("-s")
                .arg("512") // 512x512 preview size
                .output()
                .map_err(|e| AppError::InternalServer(format!("Failed to run stl-thumb: {}", e)))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(AppError::InternalServer(format!(
                    "stl-thumb failed: {}",
                    stderr
                )));
            }

            Ok(output.stdout)
        })
        .await
        .map_err(|e| AppError::InternalServer(format!("Task join error: {}", e)))?
    }

    /// Update STL file record with preview information
    fn update_stl_preview_info(&self, stl_path: &str, preview_path: &str) -> Result<(), AppError> {
        let conn = self.pool.get()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;

        conn.execute(
            "UPDATE stl_files SET preview_path = ?1, preview_generated_at = ?2 WHERE file_path = ?3",
            params![preview_path, now, stl_path],
        )?;

        Ok(())
    }

    /// Check if a preview exists for an STL file
    pub fn has_preview(&self, stl_path: &str) -> Result<bool, AppError> {
        Ok(self.image_cache.get_cached_preview(stl_path)?.is_some())
    }

    /// Get the path to an existing preview, if available
    pub fn get_preview(&self, stl_path: &str) -> Result<Option<PathBuf>, AppError> {
        self.image_cache.get_cached_preview(stl_path)
    }
}

/// Background job queue for preview generation
pub struct PreviewQueue {
    sender: mpsc::Sender<String>,
}

impl PreviewQueue {
    pub fn new(preview_service: StlPreviewService, queue_size: usize) -> Self {
        let (sender, mut receiver) = mpsc::channel::<String>(queue_size);

        // Spawn background worker
        tokio::spawn(async move {
            while let Some(stl_path) = receiver.recv().await {
                match preview_service.generate_preview(&stl_path).await {
                    Ok(preview_path) => {
                        info!("Generated preview: {} -> {}", stl_path, preview_path.display());
                    }
                    Err(e) => {
                        warn!("Failed to generate preview for {}: {}", stl_path, e);
                    }
                }
            }
        });

        Self { sender }
    }

    /// Queue an STL file for preview generation
    pub async fn queue_preview(&self, stl_path: String) -> Result<(), AppError> {
        self.sender
            .send(stl_path)
            .await
            .map_err(|e| AppError::InternalServer(format!("Failed to queue preview: {}", e)))
    }
}
