use crate::db::connection::DbPool;
use crate::services::image_cache::ImageCacheService;
use crate::utils::error::AppError;
use rusqlite::params;
use std::path::{Path, PathBuf};
use stl_thumb::config::Config as StlConfig;
use tokio::sync::mpsc;
use tokio::task;
use tracing::{info, warn};

#[derive(Clone)]
pub struct StlPreviewService {
    image_cache: ImageCacheService,
    pool: DbPool,
}

impl StlPreviewService {
    pub fn new(image_cache: ImageCacheService, pool: DbPool) -> Self {
        Self { image_cache, pool }
    }

    /// Generate a preview for an STL file
    pub async fn generate_preview(&self, stl_path: &str) -> Result<PathBuf, AppError> {
        // Check if preview already exists in cache
        if let Some(cached_path) = self.image_cache.get_cached_preview(stl_path)? {
            info!("Using cached preview for {}", stl_path);
            return Ok(cached_path);
        }

        let stl_path_buf = PathBuf::from(stl_path);
        if !stl_path_buf.exists() {
            return Err(AppError::NotFound(format!(
                "STL file not found: {}",
                stl_path
            )));
        }

        // Generate preview using stl-thumb library
        let preview_data = self.render_stl_preview(&stl_path_buf).await?;

        // Cache the preview
        let cache_path = self.image_cache.cache_preview(stl_path, &preview_data)?;

        // Update database with preview information
        self.update_stl_preview_info(stl_path, cache_path.to_str().unwrap())?;

        info!("Generated preview for {}", stl_path);
        Ok(cache_path)
    }

    /// Render STL file to PNG using stl-thumb library
    async fn render_stl_preview(&self, stl_path: &Path) -> Result<Vec<u8>, AppError> {
        let stl_path = stl_path.to_path_buf();
        let stl_path_str = stl_path.to_string_lossy().to_string();

        // Render in blocking thread (CPU-bound OpenGL work)
        task::spawn_blocking(move || {
            // Generate temporary output path
            let temp_dir = std::env::temp_dir();
            let temp_filename = format!("stl_preview_{}.png", std::process::id());
            let output_path = temp_dir.join(temp_filename);

            // Configure stl-thumb to render at 512x512
            let config = StlConfig {
                stl_filename: stl_path_str.clone(),
                img_filename: output_path.to_string_lossy().to_string(),
                width: 512,
                height: 512,
                visible: false, // Headless rendering
                verbosity: 0,
                ..Default::default()
            };

            // Render directly to file
            stl_thumb::render_to_file(&config)
                .map_err(|e| format!("STL rendering failed: {}", e))?;

            // Read the generated file
            let data = std::fs::read(&output_path)
                .map_err(|e| format!("Failed to read generated preview: {}", e))?;

            // Clean up temporary file
            let _ = std::fs::remove_file(&output_path);

            Ok(data)
        })
        .await
        .map_err(|e| AppError::InternalServer(format!("Task join error: {}", e)))?
        .map_err(AppError::InternalServer)
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
                        info!(
                            "Generated preview: {} -> {}",
                            stl_path,
                            preview_path.display()
                        );
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
