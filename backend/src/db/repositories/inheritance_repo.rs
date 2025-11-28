use crate::db::connection::DbPool;
use crate::models::project::ImageInheritance;
use rusqlite::params;
use std::sync::Arc;

pub struct ImageInheritanceRepository {
    pool: Arc<DbPool>,
}

impl ImageInheritanceRepository {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }

    /// Create a new image inheritance entry
    pub fn create(
        &self,
        project_id: i64,
        image_id: i64,
        source_project_id: i64,
        inherited_from_path: &str,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let conn = self.pool.get()?;
        let created_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;

        conn.execute(
            "INSERT INTO image_inheritance (project_id, image_id, source_project_id, inherited_from_path, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(project_id, image_id) DO NOTHING",
            params![project_id, image_id, source_project_id, inherited_from_path, created_at],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Get all inherited images for a project
    pub fn get_inherited_images_for_project(
        &self,
        project_id: i64,
    ) -> Result<Vec<ImageInheritance>, Box<dyn std::error::Error>> {
        let conn = self.pool.get()?;

        let mut stmt = conn.prepare(
            "SELECT id, project_id, image_id, source_project_id, inherited_from_path, created_at
             FROM image_inheritance
             WHERE project_id = ?1",
        )?;

        let inheritance_iter = stmt.query_map(params![project_id], |row| {
            Ok(ImageInheritance {
                id: row.get(0)?,
                project_id: row.get(1)?,
                image_id: row.get(2)?,
                source_project_id: row.get(3)?,
                inherited_from_path: row.get(4)?,
                created_at: row.get(5)?,
            })
        })?;

        let mut result = Vec::new();
        for inheritance in inheritance_iter {
            result.push(inheritance?);
        }

        Ok(result)
    }

    /// Delete all inheritance entries for a project
    pub fn delete_by_project(&self, project_id: i64) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.pool.get()?;
        conn.execute(
            "DELETE FROM image_inheritance WHERE project_id = ?1",
            params![project_id],
        )?;
        Ok(())
    }

    /// Delete all inheritance entries for an image
    pub fn delete_by_image(&self, image_id: i64) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.pool.get()?;
        conn.execute(
            "DELETE FROM image_inheritance WHERE image_id = ?1",
            params![image_id],
        )?;
        Ok(())
    }

    /// Clear all inheritance entries (useful for rescanning)
    pub fn clear_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.pool.get()?;
        conn.execute("DELETE FROM image_inheritance", [])?;
        Ok(())
    }
}
