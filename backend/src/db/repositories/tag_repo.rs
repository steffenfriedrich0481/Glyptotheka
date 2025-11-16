use crate::db::connection::DbPool;
use crate::models::tag::{CreateTag, Tag};
use crate::utils::error::AppError;
use rusqlite::params;

pub struct TagRepository {
    pool: DbPool,
}

impl TagRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn create(&self, tag: &CreateTag) -> Result<i64, AppError> {
        let conn = self.pool.get()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;

        conn.execute(
            "INSERT INTO tags (name, color, created_at, usage_count) VALUES (?1, ?2, ?3, 0)",
            params![tag.name, tag.color, now],
        )?;

        Ok(conn.last_insert_rowid())
    }

    pub fn get_or_create(&self, name: &str, color: Option<String>) -> Result<i64, AppError> {
        let conn = self.pool.get()?;
        
        let existing: Result<i64, _> = conn.query_row(
            "SELECT id FROM tags WHERE name = ?1 COLLATE NOCASE",
            params![name],
            |row| row.get(0),
        );

        match existing {
            Ok(id) => Ok(id),
            Err(_) => self.create(&CreateTag { name: name.to_string(), color }),
        }
    }

    pub fn list_all(&self) -> Result<Vec<Tag>, AppError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, color, created_at, usage_count 
             FROM tags ORDER BY usage_count DESC, name",
        )?;

        let tags = stmt
            .query_map([], |row| {
                Ok(Tag {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    color: row.get(2)?,
                    created_at: row.get(3)?,
                    usage_count: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(tags)
    }

    pub fn add_to_project(&self, project_id: i64, tag_id: i64) -> Result<(), AppError> {
        let conn = self.pool.get()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;

        conn.execute(
            "INSERT OR IGNORE INTO project_tags (project_id, tag_id, created_at) VALUES (?1, ?2, ?3)",
            params![project_id, tag_id, now],
        )?;

        Ok(())
    }

    pub fn remove_from_project(&self, project_id: i64, tag_id: i64) -> Result<(), AppError> {
        let conn = self.pool.get()?;
        conn.execute(
            "DELETE FROM project_tags WHERE project_id = ?1 AND tag_id = ?2",
            params![project_id, tag_id],
        )?;

        Ok(())
    }

    pub fn get_project_tags(&self, project_id: i64) -> Result<Vec<Tag>, AppError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT t.id, t.name, t.color, t.created_at, t.usage_count 
             FROM tags t
             INNER JOIN project_tags pt ON t.id = pt.tag_id
             WHERE pt.project_id = ?1
             ORDER BY t.name",
        )?;

        let tags = stmt
            .query_map(params![project_id], |row| {
                Ok(Tag {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    color: row.get(2)?,
                    created_at: row.get(3)?,
                    usage_count: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(tags)
    }
}
