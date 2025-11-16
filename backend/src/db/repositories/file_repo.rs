use crate::db::connection::DbPool;
use crate::models::stl_file::{CreateStlFile, StlFile};
use crate::models::image_file::{CreateImageFile, ImageFile};
use crate::utils::error::AppError;
use rusqlite::params;

pub struct FileRepository {
    pub(crate) pool: DbPool,
}

impl FileRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    // STL Files
    pub fn create_stl_file(&self, file: &CreateStlFile) -> Result<i64, AppError> {
        let conn = self.pool.get()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;

        conn.execute(
            "INSERT INTO stl_files (project_id, filename, file_path, file_size, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![file.project_id, file.filename, file.file_path, file.file_size, now, now],
        )?;

        Ok(conn.last_insert_rowid())
    }

    pub fn get_stl_files_by_project(&self, project_id: i64) -> Result<Vec<StlFile>, AppError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, project_id, filename, file_path, file_size, preview_path, 
                    preview_generated_at, created_at, updated_at
             FROM stl_files WHERE project_id = ?1 ORDER BY filename",
        )?;

        let files = stmt
            .query_map(params![project_id], |row| {
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
            .collect::<Result<Vec<_>, _>>()?;

        Ok(files)
    }

    // Image Files
    pub fn create_image_file(&self, file: &CreateImageFile) -> Result<i64, AppError> {
        let conn = self.pool.get()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;

        conn.execute(
            "INSERT INTO image_files (project_id, filename, file_path, file_size, source_type, 
                                     source_project_id, display_order, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                file.project_id,
                file.filename,
                file.file_path,
                file.file_size,
                file.source_type,
                file.source_project_id,
                file.display_order,
                now,
                now
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    pub fn get_image_files_by_project(
        &self,
        project_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ImageFile>, AppError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, project_id, filename, file_path, file_size, source_type, 
                    source_project_id, display_order, created_at, updated_at
             FROM image_files 
             WHERE project_id = ?1 
             ORDER BY display_order, filename
             LIMIT ?2 OFFSET ?3",
        )?;

        let files = stmt
            .query_map(params![project_id, limit, offset], |row| {
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
            .collect::<Result<Vec<_>, _>>()?;

        Ok(files)
    }

    // Helper methods for scanner
    pub fn add_stl_file(
        &self,
        project_id: i64,
        filename: &str,
        file_path: &str,
        file_size: i64,
    ) -> Result<i64, AppError> {
        let file = CreateStlFile {
            project_id,
            filename: filename.to_string(),
            file_path: file_path.to_string(),
            file_size,
        };
        self.create_stl_file(&file)
    }

    pub fn add_image_file(
        &self,
        project_id: i64,
        filename: &str,
        file_path: &str,
        file_size: i64,
        source_type: &str,
        source_project_id: Option<i64>,
        display_order: i32,
    ) -> Result<i64, AppError> {
        let file = CreateImageFile {
            project_id,
            filename: filename.to_string(),
            file_path: file_path.to_string(),
            file_size,
            source_type: source_type.to_string(),
            source_project_id,
            display_order,
        };
        self.create_image_file(&file)
    }

    pub fn count_images_by_project(&self, project_id: i64) -> Result<i64, AppError> {
        let conn = self.pool.get()?;
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM image_files WHERE project_id = ?1",
            params![project_id],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    pub fn delete_stl_file(&self, id: i64) -> Result<(), AppError> {
        let conn = self.pool.get()?;
        conn.execute("DELETE FROM stl_files WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn delete_image_file(&self, id: i64) -> Result<(), AppError> {
        let conn = self.pool.get()?;
        conn.execute("DELETE FROM image_files WHERE id = ?1", params![id])?;
        Ok(())
    }
}
