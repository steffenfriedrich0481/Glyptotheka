use crate::db::connection::DbPool;
use crate::models::project::{ImagePreview, Project};
use anyhow::Result;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, serde::Serialize)]
pub struct FolderContents {
    pub folders: Vec<FolderInfo>,
    pub projects: Vec<ProjectWithPreview>,
    pub current_path: String,
    pub total_folders: usize,
    pub total_projects: usize,
}

/// T038: Project with preview metadata for folder-level display
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProjectWithPreview {
    #[serde(flatten)]
    pub project: Project,
    pub preview_images: Vec<ImagePreview>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct FolderInfo {
    pub name: String,
    pub path: String,
    pub project_count: usize,
    pub has_images: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct BreadcrumbItem {
    pub name: String,
    pub path: String,
}

pub struct FolderService {
    pool: DbPool,
    root_path: PathBuf,
}

impl FolderService {
    pub fn new(pool: DbPool, root_path: PathBuf) -> Self {
        Self { pool, root_path }
    }

    /// Get contents of a folder at the given path
    pub fn get_folder_contents(
        &self,
        relative_path: &str,
        page: Option<usize>,
        per_page: Option<usize>,
    ) -> Result<FolderContents> {
        // Validate path security
        self.validate_path(relative_path)?;

        let full_path = self.root_path.join(relative_path);

        // Get immediate child folders
        let folders = self.get_child_folders(&full_path)?;

        // Get projects at this level
        let page = page.unwrap_or(1);
        let per_page = per_page.unwrap_or(50);
        let offset = (page - 1) * per_page;

        let projects = self.get_projects_at_path(relative_path, per_page, offset)?;
        let total_projects = self.count_projects_at_path(relative_path)?;

        Ok(FolderContents {
            folders: folders.clone(),
            projects,
            current_path: relative_path.to_string(),
            total_folders: folders.len(),
            total_projects,
        })
    }

    /// Get breadcrumb trail for the given path
    pub fn get_breadcrumb_trail(&self, relative_path: &str) -> Result<Vec<BreadcrumbItem>> {
        let mut breadcrumbs = vec![BreadcrumbItem {
            name: "Root".to_string(),
            path: "".to_string(),
        }];

        if relative_path.is_empty() {
            return Ok(breadcrumbs);
        }

        let parts: Vec<&str> = relative_path.split('/').filter(|s| !s.is_empty()).collect();
        let mut current_path = String::new();

        for part in parts {
            if !current_path.is_empty() {
                current_path.push('/');
            }
            current_path.push_str(part);

            breadcrumbs.push(BreadcrumbItem {
                name: part.to_string(),
                path: current_path.clone(),
            });
        }

        Ok(breadcrumbs)
    }

    /// Validate path to prevent directory traversal attacks
    fn validate_path(&self, relative_path: &str) -> Result<()> {
        if relative_path.contains("..") {
            anyhow::bail!("Path contains invalid '..' sequence");
        }

        let full_path = self.root_path.join(relative_path);
        let canonical_path = full_path.canonicalize().ok();

        if let Some(canonical) = canonical_path {
            if !canonical.starts_with(&self.root_path) {
                anyhow::bail!("Path escapes root directory");
            }
        }

        Ok(())
    }

    /// Get immediate child folders
    fn get_child_folders(&self, full_path: &Path) -> Result<Vec<FolderInfo>> {
        if !full_path.exists() || !full_path.is_dir() {
            return Ok(vec![]);
        }

        let mut folders = Vec::new();
        let entries = std::fs::read_dir(full_path)?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    let relative_path = path
                        .strip_prefix(&self.root_path)
                        .unwrap_or(&path)
                        .to_string_lossy()
                        .to_string();

                    let project_count = self.count_projects_at_path(&relative_path).unwrap_or(0);
                    let has_images = self.folder_has_images(&relative_path).unwrap_or(false);

                    folders.push(FolderInfo {
                        name: name.to_string(),
                        path: relative_path,
                        project_count,
                        has_images,
                    });
                }
            }
        }

        folders.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        Ok(folders)
    }

    /// Get projects at specific path (not recursive)
    /// T038: Get projects at specific path with preview images
    fn get_projects_at_path(
        &self,
        path: &str,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<ProjectWithPreview>> {
        let conn = self.pool.get()?;

        // First get projects
        let mut stmt = conn.prepare(
            "SELECT id, name, full_path, parent_id, is_leaf, description, folder_level, created_at, updated_at
             FROM projects 
             WHERE full_path LIKE ?1 || '/%' AND full_path NOT LIKE ?1 || '/%/%'
             ORDER BY name COLLATE NOCASE
             LIMIT ?2 OFFSET ?3",
        )?;

        let projects: Vec<Project> = stmt
            .query_map([path, &limit.to_string(), &offset.to_string()], |row| {
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

        // T038, T039: Fetch preview images for each project (optimized batch query)
        let mut projects_with_previews = Vec::new();
        
        for project in projects {
            let preview_images = self.get_project_preview_images(project.id)?;
            projects_with_previews.push(ProjectWithPreview {
                project,
                preview_images,
            });
        }

        Ok(projects_with_previews)
    }

    /// T039: Optimized query for preview images (up to 3 images per project)
    fn get_project_preview_images(&self, project_id: i64) -> Result<Vec<ImagePreview>> {
        let conn = self.pool.get()?;
        
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
             LIMIT 3",
        )?;

        let images = stmt
            .query_map([project_id], |row| {
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

    /// Count projects at specific path
    fn count_projects_at_path(&self, path: &str) -> Result<usize> {
        let conn = self.pool.get()?;

        let count: usize = conn.query_row(
            "SELECT COUNT(*) FROM projects 
             WHERE full_path LIKE ?1 || '/%' AND full_path NOT LIKE ?1 || '/%/%'",
            [path],
            |row| row.get(0),
        )?;

        Ok(count)
    }

    /// Check if folder has any images
    fn folder_has_images(&self, path: &str) -> Result<bool> {
        let conn = self.pool.get()?;

        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM project_images pi
             JOIN projects p ON pi.project_id = p.id
             WHERE p.path LIKE ?1 || '/%'
             LIMIT 1",
            [path],
            |row| row.get(0),
        )?;

        Ok(count > 0)
    }
}
