use std::fs::File;
use std::io::{Read, Write};
use zip::write::{FileOptions, ZipWriter};
use zip::CompressionMethod;

use crate::db::connection::DbPool;
use crate::models::image_file::ImageFile;
use crate::models::stl_file::StlFile;
use crate::utils::error::AppError;

pub struct DownloadService {
    pool: DbPool,
}

impl DownloadService {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn create_project_zip(
        &self,
        project_id: i64,
        output_path: &std::path::Path,
    ) -> Result<(), AppError> {
        let stl_files = {
            let conn = self.pool.get()?;
            let mut stmt = conn.prepare(
                "SELECT id, project_id, filename, file_path, file_size, preview_path, preview_generated_at, created_at, updated_at
                 FROM stl_files
                 WHERE project_id = ?1
                 ORDER BY filename",
            )?;

            let files: Result<Vec<StlFile>, rusqlite::Error> = stmt
                .query_map([project_id], |row| {
                    Ok(StlFile {
                        id: row.get(0)?,
                        project_id: row.get(1)?,
                        filename: row.get(2)?,
                        file_path: row.get(3)?,
                        file_size: row.get(4)?,
                        preview_path: row.get(5)?,
                        preview_generated_at: row.get(6)?,
                        created_at: row.get(7)?,
                        updated_at: row.get(8)?,
                    })
                })?
                .collect();

            files?
        };

        let image_files = {
            let conn = self.pool.get()?;
            let mut stmt = conn.prepare(
                "SELECT id, project_id, filename, file_path, file_size, source_type, source_project_id, display_order, created_at, updated_at
                 FROM image_files
                 WHERE project_id = ?1
                 ORDER BY filename",
            )?;

            let files: Result<Vec<ImageFile>, rusqlite::Error> = stmt
                .query_map([project_id], |row| {
                    Ok(ImageFile {
                        id: row.get(0)?,
                        project_id: row.get(1)?,
                        filename: row.get(2)?,
                        file_path: row.get(3)?,
                        file_size: row.get(4)?,
                        source_type: row.get(5)?,
                        source_project_id: row.get(6)?,
                        display_order: row.get(7)?,
                        created_at: row.get(8)?,
                        updated_at: row.get(9)?,
                    })
                })?
                .collect();

            files?
        };

        // Create ZIP file in blocking task to avoid blocking async runtime
        let output_path = output_path.to_path_buf();
        tokio::task::spawn_blocking(move || {
            let file = File::create(&output_path).map_err(|e| {
                AppError::InternalServer(format!("Failed to create ZIP file: {}", e))
            })?;
            let mut zip = ZipWriter::new(file);

            let options = FileOptions::default().compression_method(CompressionMethod::Deflated);

            // Add STL files
            for stl_file in stl_files {
                Self::add_file_to_zip_sync(
                    &mut zip,
                    &stl_file.file_path,
                    &stl_file.filename,
                    options,
                )?;
            }

            // Add image files
            for image_file in image_files {
                Self::add_file_to_zip_sync(
                    &mut zip,
                    &image_file.file_path,
                    &image_file.filename,
                    options,
                )?;
            }

            zip.finish()
                .map_err(|e| AppError::InternalServer(format!("Failed to finish ZIP: {}", e)))?;

            Ok::<(), AppError>(())
        })
        .await
        .map_err(|e| AppError::InternalServer(format!("ZIP task failed: {}", e)))??;

        Ok(())
    }

    fn add_file_to_zip_sync(
        zip: &mut ZipWriter<File>,
        file_path: &str,
        filename: &str,
        options: FileOptions<()>,
    ) -> Result<(), AppError> {
        let mut file = File::open(file_path).map_err(|e| {
            AppError::InternalServer(format!("Failed to open file {}: {}", file_path, e))
        })?;

        zip.start_file(filename, options)
            .map_err(|e| AppError::InternalServer(format!("Failed to start ZIP entry: {}", e)))?;

        let mut buffer = vec![0u8; 8192];
        loop {
            let n = file
                .read(&mut buffer)
                .map_err(|e| AppError::InternalServer(format!("Failed to read file: {}", e)))?;
            if n == 0 {
                break;
            }
            zip.write_all(&buffer[..n])
                .map_err(|e| AppError::InternalServer(format!("Failed to write to ZIP: {}", e)))?;
        }

        Ok(())
    }
}
