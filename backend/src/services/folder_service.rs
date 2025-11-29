use crate::db::connection::DbPool;
use crate::models::image_file::ImageFile;
use crate::models::project::{ImagePreview, Project, StlCategory};
use crate::models::stl_file::StlFile;
use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, serde::Serialize)]
pub struct FolderContents {
    pub folders: Vec<FolderInfo>,
    pub projects: Vec<ProjectWithPreview>,
    pub current_path: String,
    pub total_folders: usize,
    pub total_projects: usize,
    pub is_leaf_project: bool, // T042: Indicates if current path is a leaf project (should show project view, not browse view)
    pub project_details: Option<ProjectDetails>, // Project details when path IS a project
}

/// T038: Project with preview metadata for folder-level display
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProjectWithPreview {
    pub project: Project,
    pub preview_images: Vec<ImagePreview>,
}

/// Project details with files and categories when viewing a project
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProjectDetails {
    pub project: Project,
    pub stl_categories: Vec<StlCategory>,
    pub images: Vec<ImageFile>,
    pub total_images: i64,
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
    ignored_keywords: Vec<String>,
}

impl FolderService {
    pub fn new(pool: DbPool, root_path: PathBuf) -> Self {
        Self {
            pool,
            root_path,
            ignored_keywords: Vec::new(),
        }
    }

    pub fn with_ignored_keywords(mut self, keywords: Vec<String>) -> Self {
        self.ignored_keywords = keywords;
        self
    }

    /// Check if a folder name contains any ignored keyword (case-insensitive substring match)
    fn is_stl_category_folder(&self, folder_name: &str) -> bool {
        let normalized_name = folder_name.trim().to_lowercase();
        self.ignored_keywords
            .iter()
            .any(|keyword| normalized_name.contains(&keyword.trim().to_lowercase()))
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

        // Check if this path itself is a project (leaf node)
        let is_leaf_project = self.is_path_a_project(relative_path)?;

        let project_details = if is_leaf_project {
            // This path is a project - fetch its details
            Some(self.get_project_details_by_path(relative_path)?)
        } else {
            None
        };

        let folders = if !is_leaf_project {
            // Get immediate child folders only if this is not a project
            self.get_child_folders(&full_path)?
        } else {
            Vec::new()
        };

        // Get projects at this level
        let page = page.unwrap_or(1);
        let per_page = per_page.unwrap_or(50);
        let offset = (page - 1) * per_page;

        let projects = if !is_leaf_project {
            self.get_projects_at_path(relative_path, per_page, offset)?
        } else {
            Vec::new()
        };

        let total_projects = if !is_leaf_project {
            self.count_projects_at_path(relative_path)?
        } else {
            0
        };

        Ok(FolderContents {
            folders: folders.clone(),
            projects,
            current_path: relative_path.to_string(),
            total_folders: folders.len(),
            total_projects,
            is_leaf_project,
            project_details,
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
    /// Check if the given path corresponds to an existing project
    fn is_path_a_project(&self, relative_path: &str) -> Result<bool> {
        let conn = self.pool.get()?;

        // Build the full database path with /projects prefix
        let db_path = if relative_path.is_empty() {
            "/projects".to_string()
        } else {
            format!("/projects/{}", relative_path)
        };

        tracing::debug!(
            "Checking if path is a project: relative='{}', db_path='{}'",
            relative_path,
            db_path
        );

        let exists: bool = conn
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM projects WHERE full_path = ?1 AND is_leaf = 1)",
                [&db_path],
                |row| row.get(0),
            )
            .unwrap_or(false);

        tracing::debug!("Path '{}' is_project: {}", db_path, exists);

        Ok(exists)
    }

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
                    // Skip folders that are STL category keywords
                    if self.is_stl_category_folder(name) {
                        continue;
                    }

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

        // Build the full database path with /projects prefix
        let db_path = if path.is_empty() {
            "/projects".to_string()
        } else {
            format!("/projects/{}", path)
        };

        let query = "SELECT id, name, full_path, parent_id, is_leaf, description, folder_level, created_at, updated_at
             FROM projects 
             WHERE full_path LIKE ?1 || '/%' AND full_path NOT LIKE ?1 || '/%/%'
             ORDER BY name COLLATE NOCASE
             LIMIT ?2 OFFSET ?3";

        tracing::info!(
            "Querying projects at path: db_path='{}', limit={}, offset={}, query='{}'",
            db_path,
            limit,
            offset,
            query
        );

        // First get projects - look for immediate children only
        let mut stmt = conn.prepare(query)?;

        let projects: Vec<Project> = stmt
            .query_map([&db_path, &limit.to_string(), &offset.to_string()], |row| {
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

        tracing::info!("Found {} projects at path '{}'", projects.len(), db_path);
        for project in &projects {
            tracing::debug!(
                "  - Project: id={}, name='{}', full_path='{}'",
                project.id,
                project.name,
                project.full_path
            );
        }

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

        // Build the full database path with /projects prefix
        let db_path = if path.is_empty() {
            "/projects".to_string()
        } else {
            format!("/projects/{}", path)
        };

        let count: usize = conn.query_row(
            "SELECT COUNT(*) FROM projects 
             WHERE full_path LIKE ?1 || '/%' AND full_path NOT LIKE ?1 || '/%/%'",
            [&db_path],
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

    /// Get full project details by path (for when browse path IS a project)
    fn get_project_details_by_path(&self, relative_path: &str) -> Result<ProjectDetails> {
        let conn = self.pool.get()?;

        // Build the full database path with /projects prefix
        let db_path = if relative_path.is_empty() {
            "/projects".to_string()
        } else {
            format!("/projects/{}", relative_path)
        };

        // Get the project
        let project: Project = conn.query_row(
            "SELECT id, name, full_path, parent_id, is_leaf, description, folder_level, created_at, updated_at
             FROM projects 
             WHERE full_path = ?1",
            [&db_path],
            |row| {
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
            },
        )?;

        // Get STL files and group by category
        let mut stmt = conn.prepare(
            "SELECT id, filename, file_path, file_size, category, project_id, created_at, updated_at, preview_path, preview_generated_at
             FROM stl_files
             WHERE project_id = ?1
             ORDER BY category, filename",
        )?;

        let stl_files: Vec<StlFile> = stmt
            .query_map([project.id], |row| {
                Ok(StlFile {
                    id: row.get(0)?,
                    filename: row.get(1)?,
                    file_path: row.get(2)?,
                    file_size: row.get(3)?,
                    category: row.get(4)?,
                    project_id: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                    preview_path: row.get(8)?,
                    preview_generated_at: row.get(9)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        // Group STL files by category
        let mut category_map: HashMap<Option<String>, Vec<StlFile>> = HashMap::new();
        for file in stl_files {
            category_map
                .entry(file.category.clone())
                .or_insert_with(Vec::new)
                .push(file);
        }

        // Convert to Vec<StlCategory>, with uncategorized files first
        let mut stl_categories: Vec<StlCategory> = category_map
            .into_iter()
            .map(|(category, files)| StlCategory { category, files })
            .collect();

        // Sort: uncategorized (None) first, then alphabetically by category name
        stl_categories.sort_by(|a, b| match (&a.category, &b.category) {
            (None, None) => std::cmp::Ordering::Equal,
            (None, Some(_)) => std::cmp::Ordering::Less,
            (Some(_), None) => std::cmp::Ordering::Greater,
            (Some(a_cat), Some(b_cat)) => a_cat.cmp(b_cat),
        });

        // Get images (priority-sorted)
        let mut stmt = conn.prepare(
            "SELECT id, filename, file_path, file_size, project_id, source_type, image_source, image_priority, source_project_id, display_order, created_at, updated_at
             FROM image_files
             WHERE project_id = ?1
             ORDER BY image_priority DESC, display_order ASC, filename ASC",
        )?;

        let images: Vec<ImageFile> = stmt
            .query_map([project.id], |row| {
                Ok(ImageFile {
                    id: row.get(0)?,
                    filename: row.get(1)?,
                    file_path: row.get(2)?,
                    file_size: row.get(3)?,
                    project_id: row.get(4)?,
                    source_type: row.get(5)?,
                    image_source: row.get(6)?,
                    image_priority: row.get(7)?,
                    source_project_id: row.get(8)?,
                    display_order: row.get(9)?,
                    created_at: row.get(10)?,
                    updated_at: row.get(11)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let total_images = images.len() as i64;

        Ok(ProjectDetails {
            project,
            stl_categories,
            images,
            total_images,
        })
    }
}
