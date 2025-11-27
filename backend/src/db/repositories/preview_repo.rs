use crate::db::connection::DbPool;
use crate::utils::error::AppError;
use rusqlite::params;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectPreview {
    pub id: i64,
    pub project_id: i64,
    pub preview_path: String,
    pub image_count: i32,
    pub source_image_ids: Vec<i64>,
    pub generated_at: i64,
}

#[derive(Debug, Clone)]
pub struct CreateProjectPreview {
    pub project_id: i64,
    pub preview_path: String,
    pub image_count: i32,
    pub source_image_ids: Vec<i64>,
}

pub struct PreviewRepository {
    pub pool: DbPool,
}

impl PreviewRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Store or update a preview for a project
    pub fn store_preview(&self, preview: &CreateProjectPreview) -> Result<i64, AppError> {
        let conn = self.pool.get()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;

        let source_ids_json = serde_json::to_string(&preview.source_image_ids).map_err(|e| {
            AppError::InternalServer(format!("Failed to serialize image IDs: {}", e))
        })?;

        // Delete existing preview for this project
        conn.execute(
            "DELETE FROM project_previews WHERE project_id = ?1",
            params![preview.project_id],
        )?;

        // Insert new preview
        conn.execute(
            "INSERT INTO project_previews (project_id, preview_path, image_count, source_image_ids, generated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                preview.project_id,
                preview.preview_path,
                preview.image_count,
                source_ids_json,
                now
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Get preview for a project
    pub fn get_preview(&self, project_id: i64) -> Result<Option<ProjectPreview>, AppError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, project_id, preview_path, image_count, source_image_ids, generated_at
             FROM project_previews
             WHERE project_id = ?1",
        )?;

        let result = stmt.query_row(params![project_id], |row| {
            let source_ids_str: String = row.get(4)?;
            let source_image_ids: Vec<i64> =
                serde_json::from_str(&source_ids_str).unwrap_or_default();

            Ok(ProjectPreview {
                id: row.get(0)?,
                project_id: row.get(1)?,
                preview_path: row.get(2)?,
                image_count: row.get(3)?,
                source_image_ids,
                generated_at: row.get(5)?,
            })
        });

        match result {
            Ok(preview) => Ok(Some(preview)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Delete preview for a project
    pub fn delete_preview(&self, project_id: i64) -> Result<(), AppError> {
        let conn = self.pool.get()?;
        conn.execute(
            "DELETE FROM project_previews WHERE project_id = ?1",
            params![project_id],
        )?;
        Ok(())
    }

    /// Get all previews that reference specific image IDs (for cleanup)
    pub fn get_previews_using_image(&self, image_id: i64) -> Result<Vec<i64>, AppError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, source_image_ids FROM project_previews WHERE source_image_ids LIKE ?1",
        )?;

        let search_pattern = format!("%{}%", image_id);
        let rows = stmt.query_map(params![search_pattern], |row| {
            let id: i64 = row.get(0)?;
            let source_ids_str: String = row.get(1)?;
            let source_image_ids: Vec<i64> =
                serde_json::from_str(&source_ids_str).unwrap_or_default();

            if source_image_ids.contains(&image_id) {
                Ok(Some(id))
            } else {
                Ok(None)
            }
        })?;

        let mut preview_ids = Vec::new();
        for row in rows {
            if let Ok(Some(id)) = row {
                preview_ids.push(id);
            }
        }

        Ok(preview_ids)
    }
}
