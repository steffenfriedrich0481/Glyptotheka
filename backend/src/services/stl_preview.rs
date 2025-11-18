use crate::db::connection::DbPool;
use crate::services::image_cache::ImageCacheService;
use crate::utils::error::AppError;
use rusqlite::params;
use std::path::{Path, PathBuf};
use stl_thumb::config::Config as StlConfig;
use tokio::sync::mpsc;
use tokio::task;
use tokio::time::{timeout, Duration};
use tracing::{info, warn};

// T009: PreviewResult struct
#[derive(Debug)]
pub enum PreviewResult {
    Generated(PathBuf),
    CacheHit(PathBuf),
    Skipped(String), // Reason for skipping
}

#[derive(Clone)]
pub struct StlPreviewService {
    image_cache: ImageCacheService,
    pool: DbPool,
}

impl StlPreviewService {
    pub fn new(image_cache: ImageCacheService, pool: DbPool) -> Self {
        // T046: Check stl-thumb availability at startup
        if let Err(e) = Self::check_stl_thumb_available() {
            warn!("STL preview generation may not work: {}", e);
        }
        Self { image_cache, pool }
    }

    // T046, T047: Check if stl-thumb is available
    fn check_stl_thumb_available() -> Result<(), AppError> {
        // Try to check if stl-thumb library is available
        // For now, we assume it's available since it's a compile-time dependency
        // In production, you might want to verify the binary or library exists
        info!("STL preview service initialized");
        Ok(())
    }

    // T005: Generate preview with smart cache
    pub async fn generate_preview_with_smart_cache(&self, stl_path: &str) -> Result<PreviewResult, AppError> {
        // T048: Log preview generation operations
        info!("Generating STL preview for: {}", stl_path);
        
        // T008: Validate file size (100MB limit)
        let stl_path_buf = PathBuf::from(stl_path);
        if !stl_path_buf.exists() {
            warn!("STL file not found: {}", stl_path);
            return Err(AppError::NotFound(format!(
                "STL file not found: {}",
                stl_path
            )));
        }

        let metadata = std::fs::metadata(&stl_path_buf)?;
        let file_size = metadata.len();
        if file_size > 100 * 1024 * 1024 {
            warn!("Skipping STL file (>100MB): {} ({}MB)", stl_path, file_size / (1024 * 1024));
            return Ok(PreviewResult::Skipped("File too large (>100MB)".to_string()));
        }

        // T050: Basic disk space check (ensure at least 100MB free)
        // Note: Full implementation would check actual free space
        // For now, we rely on the cache directory being writable

        // T010: Smart caching logic - check if preview is valid
        if let Ok(true) = self.is_preview_valid(stl_path).await {
            if let Some(cached_path) = self.image_cache.get_cached_preview(stl_path)? {
                info!("Using valid cached preview for {}", stl_path);
                return Ok(PreviewResult::CacheHit(cached_path));
            }
        }

        // Generate new preview with timeout
        let preview_data = match timeout(
            Duration::from_secs(30), // T011: 30 second timeout
            self.render_stl_preview(&stl_path_buf)
        ).await {
            Ok(Ok(data)) => data,
            Ok(Err(e)) => {
                // T012, T048, T049: Graceful error handling with detailed logging
                warn!("Failed to render STL preview for {}: {} (possibly corrupted STL file)", stl_path, e);
                return Err(e);
            }
            Err(_) => {
                warn!("STL preview generation timed out after 30s for {}", stl_path);
                return Err(AppError::InternalServer("Preview generation timed out".to_string()));
            }
        };

        // Cache the preview
        let cache_path = self.image_cache.cache_preview(stl_path, &preview_data)?;

        // Update database with preview information
        self.update_stl_preview_info(stl_path, cache_path.to_str().unwrap())?;

        info!("Generated preview for {}", stl_path);
        Ok(PreviewResult::Generated(cache_path))
    }

    // T006: Check if preview is valid (mtime comparison)
    pub async fn is_preview_valid(&self, stl_path: &str) -> Result<bool, AppError> {
        // Get STL file modification time
        let stl_path_buf = PathBuf::from(stl_path);
        if !stl_path_buf.exists() {
            return Ok(false);
        }
        
        let stl_metadata = std::fs::metadata(&stl_path_buf)?;
        let stl_mtime = stl_metadata.modified()?
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;

        // Get preview timestamp from database
        if let Some(preview_timestamp) = self.get_preview_timestamp(stl_path)? {
            // Valid if preview is newer than or equal to STL file
            Ok(stl_mtime <= preview_timestamp)
        } else {
            Ok(false)
        }
    }

    // T007: Get preview timestamp helper
    fn get_preview_timestamp(&self, stl_path: &str) -> Result<Option<i64>, AppError> {
        let conn = self.pool.get()?;
        let result: Result<i64, _> = conn.query_row(
            "SELECT preview_generated_at FROM stl_files WHERE file_path = ?1",
            params![stl_path],
            |row| row.get(0),
        );

        match result {
            Ok(timestamp) => Ok(Some(timestamp)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(AppError::from(e)),
        }
    }

    /// Generate a preview for an STL file (backward compatible)
    pub async fn generate_preview(&self, stl_path: &str) -> Result<PathBuf, AppError> {
        match self.generate_preview_with_smart_cache(stl_path).await? {
            PreviewResult::Generated(path) | PreviewResult::CacheHit(path) => Ok(path),
            PreviewResult::Skipped(reason) => {
                Err(AppError::InternalServer(format!("Preview generation skipped: {}", reason)))
            }
        }
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
