use crate::db::connection::DbPool;
use crate::models::project::{CreateProject, Project, ProjectWithRelations};
use crate::models::tag::Tag;
use crate::utils::error::AppError;
use rusqlite::{params, OptionalExtension};

pub struct ProjectRepository {
    pub(crate) pool: DbPool,
}

impl ProjectRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn create(&self, project: &CreateProject) -> Result<i64, AppError> {
        let conn = self.pool.get()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;

        conn.execute(
            "INSERT INTO projects (name, full_path, parent_id, is_leaf, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                project.name,
                project.full_path,
                project.parent_id,
                project.is_leaf,
                now,
                now
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    pub fn get_by_id(&self, id: i64) -> Result<Option<Project>, AppError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, full_path, parent_id, is_leaf, description, created_at, updated_at
             FROM projects WHERE id = ?1",
        )?;

        let project = stmt
            .query_row(params![id], |row| {
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    full_path: row.get(2)?,
                    parent_id: row.get(3)?,
                    is_leaf: row.get(4)?,
                    description: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            })
            .optional()?;

        Ok(project)
    }

    pub fn get_by_path(&self, path: &str) -> Result<Option<Project>, AppError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, full_path, parent_id, is_leaf, description, created_at, updated_at
             FROM projects WHERE full_path = ?1",
        )?;

        let project = stmt
            .query_row(params![path], |row| {
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    full_path: row.get(2)?,
                    parent_id: row.get(3)?,
                    is_leaf: row.get(4)?,
                    description: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            })
            .optional()?;

        Ok(project)
    }

    pub fn list_root(&self) -> Result<Vec<Project>, AppError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, full_path, parent_id, is_leaf, description, created_at, updated_at
             FROM projects WHERE parent_id IS NULL ORDER BY name",
        )?;

        let projects = stmt
            .query_map([], |row| {
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    full_path: row.get(2)?,
                    parent_id: row.get(3)?,
                    is_leaf: row.get(4)?,
                    description: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(projects)
    }

    pub fn list_children(&self, parent_id: i64) -> Result<Vec<Project>, AppError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, full_path, parent_id, is_leaf, description, created_at, updated_at
             FROM projects WHERE parent_id = ?1 ORDER BY name",
        )?;

        let projects = stmt
            .query_map(params![parent_id], |row| {
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    full_path: row.get(2)?,
                    parent_id: row.get(3)?,
                    is_leaf: row.get(4)?,
                    description: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(projects)
    }

    pub fn get_with_relations(&self, id: i64) -> Result<Option<ProjectWithRelations>, AppError> {
        let project = match self.get_by_id(id)? {
            Some(p) => p,
            None => return Ok(None),
        };

        let children = self.list_children(id)?;

        let conn = self.pool.get()?;
        let stl_count: usize = conn.query_row(
            "SELECT COUNT(*) FROM stl_files WHERE project_id = ?1",
            params![id],
            |row| row.get(0),
        )?;

        let image_count: usize = conn.query_row(
            "SELECT COUNT(*) FROM image_files WHERE project_id = ?1",
            params![id],
            |row| row.get(0),
        )?;

        // Get tags for this project
        let mut stmt = conn.prepare(
            "SELECT t.id, t.name, t.color, t.created_at, t.usage_count 
             FROM tags t
             INNER JOIN project_tags pt ON t.id = pt.tag_id
             WHERE pt.project_id = ?1
             ORDER BY t.name",
        )?;

        let tags = stmt
            .query_map(params![id], |row| {
                Ok(Tag {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    color: row.get(2)?,
                    created_at: row.get(3)?,
                    usage_count: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Some(ProjectWithRelations {
            project,
            children,
            stl_count,
            image_count,
            tags,
        }))
    }

    pub fn delete(&self, id: i64) -> Result<(), AppError> {
        let conn = self.pool.get()?;
        conn.execute("DELETE FROM projects WHERE id = ?1", params![id])?;
        Ok(())
    }
}
