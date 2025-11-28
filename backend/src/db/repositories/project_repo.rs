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
            "SELECT id, name, full_path, parent_id, is_leaf, description, folder_level, created_at, updated_at
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
                    folder_level: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            })
            .optional()?;

        Ok(project)
    }

    pub fn get_by_path(&self, path: &str) -> Result<Option<Project>, AppError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, full_path, parent_id, is_leaf, description, folder_level, created_at, updated_at
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
                    folder_level: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            })
            .optional()?;

        Ok(project)
    }

    pub fn list_root(&self) -> Result<Vec<Project>, AppError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, full_path, parent_id, is_leaf, description, folder_level, created_at, updated_at
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
                    folder_level: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(projects)
    }

    pub fn list_children(&self, parent_id: i64) -> Result<Vec<Project>, AppError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, full_path, parent_id, is_leaf, description, folder_level, created_at, updated_at
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
                    folder_level: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
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

        // T037: Fetch inherited images with preview metadata
        let inherited_images = self.get_project_preview_images(id)?;

        Ok(Some(ProjectWithRelations {
            project,
            children,
            stl_count,
            image_count,
            inherited_images,
            tags,
        }))
    }

    /// T037, T039: Get preview images for a project (optimized for folder-level display)
    /// Returns up to 5 images prioritized: direct images > inherited > STL previews
    pub fn get_project_preview_images(&self, id: i64) -> Result<Vec<crate::models::project::ImagePreview>, AppError> {
        use crate::models::project::ImagePreview;
        
        let conn = self.pool.get()?;
        
        // Query combines direct images, inherited images, and STL previews
        // Prioritizes: image_priority (10 for regular, 1 for STL preview)
        let mut stmt = conn.prepare(
            "SELECT 
                i.id,
                i.filename,
                i.source_type,
                i.image_source,
                i.image_priority,
                CASE 
                    WHEN i.source_type = 'inherited' THEN sp.full_path
                    ELSE NULL 
                END as inherited_from
             FROM image_files i
             LEFT JOIN projects sp ON i.source_project_id = sp.id
             WHERE i.project_id = ?1
             ORDER BY i.image_priority DESC, i.display_order ASC, i.filename ASC
             LIMIT 5",
        )?;

        let images = stmt
            .query_map(params![id], |row| {
                Ok(ImagePreview {
                    id: row.get(0)?,
                    filename: row.get(1)?,
                    source_type: row.get(2)?,
                    image_source: row.get(3)?,
                    priority: row.get(4)?,
                    inherited_from: row.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(images)
    }

    pub fn update_is_leaf(&self, id: i64, is_leaf: bool) -> Result<(), AppError> {
        let conn = self.pool.get()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;

        conn.execute(
            "UPDATE projects SET is_leaf = ?1, updated_at = ?2 WHERE id = ?3",
            params![is_leaf, now, id],
        )?;
        Ok(())
    }

    pub fn delete(&self, id: i64) -> Result<(), AppError> {
        let conn = self.pool.get()?;
        conn.execute("DELETE FROM projects WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn get_parent_chain(&self, project_id: i64) -> Result<Vec<Project>, AppError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "WITH RECURSIVE parent_chain AS (
                SELECT id, name, full_path, parent_id, is_leaf, description, folder_level, created_at, updated_at, 0 as level
                FROM projects
                WHERE id = ?1
                UNION ALL
                SELECT p.id, p.name, p.full_path, p.parent_id, p.is_leaf, p.description, p.created_at, p.updated_at, pc.level + 1
                FROM projects p
                JOIN parent_chain pc ON p.id = pc.parent_id
            )
            SELECT id, name, full_path, parent_id, is_leaf, description, folder_level, created_at, updated_at
            FROM parent_chain
            ORDER BY level DESC"
        )?;

        let projects = stmt
            .query_map(params![project_id], |row| {
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    full_path: row.get(2)?,
                    parent_id: row.get(3)?,
                    is_leaf: row.get(4)?,
                    description: row.get(5)?,
                    folder_level: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(projects)
    }
}
