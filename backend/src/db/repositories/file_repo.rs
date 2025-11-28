use crate::db::connection::DbPool;
use crate::models::image_file::{CreateImageFile, ImageFile};
use crate::models::stl_file::{CreateStlFile, StlFile};
use crate::utils::error::AppError;
use rusqlite::params;
use std::collections::HashMap;

#[derive(Clone)]
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
                                     source_project_id, display_order, image_priority, image_source, 
                                     created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                file.project_id,
                file.filename,
                file.file_path,
                file.file_size,
                file.source_type,
                file.source_project_id,
                file.display_order,
                file.image_priority,
                file.image_source,
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
                    source_project_id, display_order, image_priority, image_source, 
                    created_at, updated_at
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
                    image_priority: row.get(8)?,
                    image_source: row.get(9)?,
                    created_at: row.get(10)?,
                    updated_at: row.get(11)?,
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

    #[allow(clippy::too_many_arguments)]
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
            image_priority: 100, // Default: regular images
            image_source: "regular".to_string(),
        };
        self.create_image_file(&file)
    }

    // T013: Insert STL preview image with priority 50
    pub fn insert_stl_preview_image(
        &self,
        project_id: i64,
        filename: &str,
        file_path: &str,
        file_size: i64,
    ) -> Result<i64, AppError> {
        let file = CreateImageFile {
            project_id,
            filename: filename.to_string(),
            file_path: file_path.to_string(),
            file_size,
            source_type: "direct".to_string(),
            source_project_id: None,
            display_order: 0,
            image_priority: 50, // STL previews
            image_source: "stl_preview".to_string(),
        };
        self.create_image_file(&file)
    }

    // T014: Get images sorted by priority (higher priority first)
    pub fn get_images_by_priority(
        &self,
        project_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ImageFile>, AppError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, project_id, filename, file_path, file_size, source_type, 
                    source_project_id, display_order, image_priority, image_source, 
                    created_at, updated_at
             FROM image_files 
             WHERE project_id = ?1 
             ORDER BY image_priority DESC, display_order ASC, created_at ASC
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
                    image_priority: row.get(8)?,
                    image_source: row.get(9)?,
                    created_at: row.get(10)?,
                    updated_at: row.get(11)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(files)
    }

    // T015: Delete STL preview image by file path
    pub fn delete_stl_preview_image(&self, file_path: &str) -> Result<(), AppError> {
        let conn = self.pool.get()?;
        conn.execute(
            "DELETE FROM image_files WHERE file_path = ?1 AND image_source = 'stl_preview'",
            params![file_path],
        )?;
        Ok(())
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

    pub fn get_aggregated_images(
        &self,
        project_id: i64,
        limit: i64,
    ) -> Result<Vec<crate::models::project::ImagePreview>, AppError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "WITH RECURSIVE parent_chain AS (
                SELECT id, parent_id, 0 as level
                FROM projects
                WHERE id = ?1
                UNION ALL
                SELECT p.id, p.parent_id, pc.level + 1
                FROM projects p
                JOIN parent_chain pc ON p.id = pc.parent_id
            )
            SELECT 
                img.id, 
                img.filename, 
                img.source_type, 
                img.image_source, 
                img.image_priority
            FROM image_files img
            JOIN parent_chain pc ON img.project_id = pc.id
            ORDER BY img.image_priority DESC, pc.level ASC, img.display_order ASC
            LIMIT ?2",
        )?;

        let images = stmt
            .query_map(params![project_id, limit], |row| {
                Ok(crate::models::project::ImagePreview {
                    id: row.get(0)?,
                    filename: row.get(1)?,
                    source_type: row.get(2)?,
                    image_source: row.get(3)?,
                    priority: row.get(4)?,
                    inherited_from: None,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(images)
    }

    pub fn get_aggregated_images_batch(
        &self,
        project_ids: &[i64],
        limit_per_project: i64,
    ) -> Result<HashMap<i64, Vec<crate::models::project::ImagePreview>>, AppError> {
        if project_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let conn = self.pool.get()?;

        let placeholders = project_ids
            .iter()
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(",");

        let sql = format!(
            "WITH RECURSIVE parent_chain AS (
                SELECT id, parent_id, id as original_project_id, 0 as level
                FROM projects
                WHERE id IN ({})
                UNION ALL
                SELECT p.id, p.parent_id, pc.original_project_id, pc.level + 1
                FROM projects p
                JOIN parent_chain pc ON p.id = pc.parent_id
            ),
            ranked_images AS (
                SELECT 
                    pc.original_project_id,
                    img.id, 
                    img.filename, 
                    img.source_type, 
                    img.image_source, 
                    img.image_priority,
                    ROW_NUMBER() OVER (PARTITION BY pc.original_project_id ORDER BY img.image_priority DESC, pc.level ASC, img.display_order ASC) as rn
                FROM image_files img
                JOIN parent_chain pc ON img.project_id = pc.id
            )
            SELECT 
                original_project_id,
                id, 
                filename, 
                source_type, 
                image_source, 
                image_priority
            FROM ranked_images 
            WHERE rn <= ?",
            placeholders
        );

        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::with_capacity(project_ids.len() + 1);
        for id in project_ids {
            params.push(id);
        }
        params.push(&limit_per_project);

        let mut stmt = conn.prepare(&sql)?;

        let rows = stmt.query_map(rusqlite::params_from_iter(params.iter()), |row| {
            let original_project_id: i64 = row.get(0)?;
            let image = crate::models::project::ImagePreview {
                id: row.get(1)?,
                filename: row.get(2)?,
                source_type: row.get(3)?,
                image_source: row.get(4)?,
                priority: row.get(5)?,
                inherited_from: None,
            };
            Ok((original_project_id, image))
        })?;

        let mut result: HashMap<i64, Vec<crate::models::project::ImagePreview>> = HashMap::new();

        for row in rows {
            let (project_id, image) = row?;
            result.entry(project_id).or_default().push(image);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use r2d2_sqlite::SqliteConnectionManager;

    fn setup_db() -> DbPool {
        let manager = SqliteConnectionManager::memory();
        let pool = r2d2::Pool::new(manager).unwrap();
        let conn = pool.get().unwrap();

        conn.execute(
            "CREATE TABLE projects (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                full_path TEXT NOT NULL,
                parent_id INTEGER,
                is_leaf BOOLEAN NOT NULL,
                description TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )
        .unwrap();

        conn.execute(
            "CREATE TABLE image_files (
                id INTEGER PRIMARY KEY,
                project_id INTEGER NOT NULL,
                filename TEXT NOT NULL,
                file_path TEXT NOT NULL,
                file_size INTEGER NOT NULL,
                source_type TEXT NOT NULL,
                source_project_id INTEGER,
                display_order INTEGER NOT NULL,
                image_priority INTEGER NOT NULL,
                image_source TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                FOREIGN KEY(project_id) REFERENCES projects(id)
            )",
            [],
        )
        .unwrap();

        pool
    }

    #[test]
    fn test_get_aggregated_images() {
        let pool = setup_db();
        let repo = FileRepository::new(pool.clone());
        let conn = pool.get().unwrap();

        // Create project hierarchy: Root -> Child -> Leaf
        conn.execute("INSERT INTO projects (id, name, full_path, parent_id, is_leaf, created_at, updated_at) VALUES (1, 'Root', '/root', NULL, 0, 0, 0)", []).unwrap();
        conn.execute("INSERT INTO projects (id, name, full_path, parent_id, is_leaf, created_at, updated_at) VALUES (2, 'Child', '/root/child', 1, 0, 0, 0)", []).unwrap();
        conn.execute("INSERT INTO projects (id, name, full_path, parent_id, is_leaf, created_at, updated_at) VALUES (3, 'Leaf', '/root/child/leaf', 2, 1, 0, 0)", []).unwrap();

        // Add images
        // Root image (inherited)
        repo.add_image_file(1, "root.jpg", "/root/root.jpg", 100, "direct", None, 0)
            .unwrap();

        // Child image (inherited)
        repo.add_image_file(
            2,
            "child.jpg",
            "/root/child/child.jpg",
            100,
            "direct",
            None,
            0,
        )
        .unwrap();

        // Leaf image (direct)
        repo.add_image_file(
            3,
            "leaf.jpg",
            "/root/child/leaf/leaf.jpg",
            100,
            "direct",
            None,
            0,
        )
        .unwrap();

        // Get aggregated images for Leaf
        let images = repo.get_aggregated_images(3, 15).unwrap();

        assert_eq!(images.len(), 3);
        // Order should be: Leaf (level 0), Child (level 1), Root (level 2)
        // Note: The query orders by image_priority DESC, pc.level ASC, img.display_order ASC
        // All have priority 100.
        // Leaf is level 0. Child is level 1. Root is level 2.
        assert_eq!(images[0].filename, "leaf.jpg");
        assert_eq!(images[1].filename, "child.jpg");
        assert_eq!(images[2].filename, "root.jpg");
    }

    #[test]
    fn test_image_priority() {
        let pool = setup_db();
        let repo = FileRepository::new(pool.clone());
        let conn = pool.get().unwrap();

        conn.execute("INSERT INTO projects (id, name, full_path, parent_id, is_leaf, created_at, updated_at) VALUES (1, 'P1', '/p1', NULL, 1, 0, 0)", []).unwrap();

        // Regular image (priority 100)
        repo.add_image_file(1, "regular.jpg", "/p1/regular.jpg", 100, "direct", None, 1)
            .unwrap();

        // STL preview (priority 50) - using insert_stl_preview_image
        repo.insert_stl_preview_image(1, "preview.png", "/p1/preview.png", 100)
            .unwrap();

        let images = repo.get_aggregated_images(1, 15).unwrap();

        assert_eq!(images.len(), 2);
        // Regular image (100) should come before preview (50)
        assert_eq!(images[0].filename, "regular.jpg");
        assert_eq!(images[1].filename, "preview.png");
    }

    #[test]
    fn test_get_aggregated_images_batch() {
        let pool = setup_db();
        let repo = FileRepository::new(pool.clone());
        let conn = pool.get().unwrap();

        // Create projects
        conn.execute("INSERT INTO projects (id, name, full_path, parent_id, is_leaf, created_at, updated_at) VALUES (1, 'Root', '/root', NULL, 0, 0, 0)", []).unwrap();
        conn.execute("INSERT INTO projects (id, name, full_path, parent_id, is_leaf, created_at, updated_at) VALUES (2, 'Child1', '/root/child1', 1, 1, 0, 0)", []).unwrap();
        conn.execute("INSERT INTO projects (id, name, full_path, parent_id, is_leaf, created_at, updated_at) VALUES (3, 'Child2', '/root/child2', 1, 1, 0, 0)", []).unwrap();

        // Add images
        repo.add_image_file(1, "root.jpg", "/root/root.jpg", 100, "direct", None, 0)
            .unwrap();
        repo.add_image_file(
            2,
            "child1.jpg",
            "/root/child1/child1.jpg",
            100,
            "direct",
            None,
            0,
        )
        .unwrap();
        repo.add_image_file(
            3,
            "child2.jpg",
            "/root/child2/child2.jpg",
            100,
            "direct",
            None,
            0,
        )
        .unwrap();

        // Get batch images for Child1 and Child2
        let images_map = repo.get_aggregated_images_batch(&[2, 3], 15).unwrap();

        assert_eq!(images_map.len(), 2);

        // Child1 should have child1.jpg and root.jpg
        let child1_images = images_map.get(&2).unwrap();
        assert_eq!(child1_images.len(), 2);
        // Order: Child1 (level 0), Root (level 1)
        assert_eq!(child1_images[0].filename, "child1.jpg");
        assert_eq!(child1_images[1].filename, "root.jpg");

        // Child2 should have child2.jpg and root.jpg
        let child2_images = images_map.get(&3).unwrap();
        assert_eq!(child2_images.len(), 2);
        assert_eq!(child2_images[0].filename, "child2.jpg");
        assert_eq!(child2_images[1].filename, "root.jpg");
    }
}
