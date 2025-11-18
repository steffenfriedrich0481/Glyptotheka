use crate::db::connection::DbPool;
use crate::db::repositories::file_repo::FileRepository;
use crate::db::repositories::project_repo::ProjectRepository;
use crate::models::project::CreateProject;
use crate::services::image_cache::ImageCacheService;
use crate::utils::error::AppError;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{info, warn};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct RescanResult {
    pub projects_found: usize,
    pub projects_added: usize,
    pub projects_updated: usize,
    pub projects_removed: usize,
    pub files_processed: usize,
    pub files_added: usize,
    pub files_updated: usize,
    pub files_removed: usize,
    pub errors: Vec<String>,
}

pub struct RescanService {
    project_repo: ProjectRepository,
    file_repo: FileRepository,
    image_cache_service: Option<ImageCacheService>,
    preview_repo: crate::db::repositories::preview_repo::PreviewRepository,
    composite_service: Option<crate::services::composite_preview::CompositePreviewService>,
}

impl RescanService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            project_repo: ProjectRepository::new(pool.clone()),
            file_repo: FileRepository::new(pool.clone()),
            image_cache_service: None,
            preview_repo: crate::db::repositories::preview_repo::PreviewRepository::new(pool),
            composite_service: None,
        }
    }

    pub fn with_cache(pool: DbPool, cache_service: ImageCacheService) -> Self {
        Self {
            project_repo: ProjectRepository::new(pool.clone()),
            file_repo: FileRepository::new(pool.clone()),
            image_cache_service: Some(cache_service),
            preview_repo: crate::db::repositories::preview_repo::PreviewRepository::new(pool),
            composite_service: None,
        }
    }

    pub fn with_composite_preview(mut self, cache_dir: std::path::PathBuf) -> Self {
        self.composite_service =
            Some(crate::services::composite_preview::CompositePreviewService::new(cache_dir));
        self
    }

    pub fn rescan(&self, root_path: &str) -> Result<RescanResult, AppError> {
        let root = Path::new(root_path);

        if !root.exists() {
            return Err(AppError::ValidationError(format!(
                "Root path does not exist: {}",
                root_path
            )));
        }

        if !root.is_dir() {
            return Err(AppError::ValidationError(format!(
                "Root path is not a directory: {}",
                root_path
            )));
        }

        let mut result = RescanResult {
            projects_found: 0,
            projects_added: 0,
            projects_updated: 0,
            projects_removed: 0,
            files_processed: 0,
            files_added: 0,
            files_updated: 0,
            files_removed: 0,
            errors: Vec::new(),
        };

        // Clear all inherited images before rescanning (they will be regenerated)
        if let Err(e) = self.clear_inherited_images() {
            let error_msg = format!("Error clearing inherited images: {}", e);
            warn!("{}", error_msg);
            result.errors.push(error_msg);
        } else {
            info!("Cleared inherited images for rebuild");
        }

        // Get all existing projects from database
        let existing_projects = self.get_all_projects_map()?;
        let mut found_project_paths = HashSet::new();

        // Scan file system for current state
        let mut project_folders = HashMap::new();

        for entry in WalkDir::new(root).follow_links(false) {
            match entry {
                Ok(e) => {
                    if e.file_type().is_file() {
                        if let Some(ext) = e.path().extension() {
                            if ext.eq_ignore_ascii_case("stl") {
                                if let Some(parent) = e.path().parent() {
                                    project_folders
                                        .entry(parent.to_path_buf())
                                        .or_insert_with(Vec::new)
                                        .push(e.path().to_path_buf());
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    result
                        .errors
                        .push(format!("Error walking directory: {}", e));
                }
            }
        }

        // Track all processed paths
        let mut path_to_id = HashMap::new();
        let mut processed_paths = HashSet::new();

        // Process each project folder
        for (folder, stl_files) in project_folders.iter() {
            let full_path = folder.to_str().unwrap_or("").to_string();
            found_project_paths.insert(full_path.clone());

            match self.create_or_update_project_hierarchy(
                folder,
                root,
                &mut path_to_id,
                &mut processed_paths,
                &existing_projects,
                &mut result,
            ) {
                Ok(project_id) => {
                    result.projects_found += 1;

                    // Get existing STL files for this project
                    let existing_stl_files = self.get_existing_stl_files(project_id)?;
                    let mut found_stl_paths = HashSet::new();

                    // Process STL files
                    for stl_file in stl_files {
                        let file_path = stl_file.to_str().unwrap_or("");
                        found_stl_paths.insert(file_path.to_string());

                        match self.process_stl_file(
                            project_id,
                            stl_file,
                            &existing_stl_files,
                            &mut result,
                        ) {
                            Ok(_) => result.files_processed += 1,
                            Err(e) => result
                                .errors
                                .push(format!("Error processing STL file: {}", e)),
                        }
                    }

                    // Remove deleted STL files
                    for (file_path, file_id) in existing_stl_files.iter() {
                        if !found_stl_paths.contains(file_path) {
                            if let Err(e) = self.file_repo.delete_stl_file(*file_id) {
                                result
                                    .errors
                                    .push(format!("Error removing deleted STL file: {}", e));
                            } else {
                                result.files_removed += 1;
                            }
                        }
                    }

                    // Process images
                    if let Err(e) = self.process_images_for_project(project_id, folder, &mut result)
                    {
                        result
                            .errors
                            .push(format!("Error processing images: {}", e));
                    }
                }
                Err(e) => {
                    result
                        .errors
                        .push(format!("Error processing project: {}", e));
                }
            }
        }

        // Remove projects that no longer exist
        for (project_path, project_id) in existing_projects.iter() {
            if !found_project_paths.contains(project_path)
                && !processed_paths.contains(Path::new(project_path))
            {
                // Check if path still exists in filesystem
                if !Path::new(project_path).exists() {
                    if let Err(e) = self.project_repo.delete(*project_id) {
                        result
                            .errors
                            .push(format!("Error removing deleted project: {}", e));
                    } else {
                        result.projects_removed += 1;
                    }
                }
            }
        }

        // First-and-a-half pass: Scan parent folders for images
        info!("Scanning parent folders for images during rescan");
        let mut scanned_parent_folders = HashSet::new();
        
        for (folder, _) in project_folders.iter() {
            let mut current: &Path = folder.as_path();
            
            while let Some(parent_folder) = current.parent() {
                if parent_folder < root {
                    break;
                }
                
                // Skip if already scanned
                if scanned_parent_folders.contains(parent_folder) {
                    current = parent_folder;
                    continue;
                }
                
                // Ensure parent project exists
                match self.ensure_project_exists(parent_folder, root, &path_to_id)
                {
                    Ok(parent_id) => {
                        // Process images in parent folder (checks for new/deleted images)
                        if let Err(e) = self.process_images_for_project(parent_id, parent_folder, &mut result) {
                            let error_msg = format!(
                                "Error processing images for parent folder {}: {}",
                                parent_folder.display(),
                                e
                            );
                            warn!("{}", error_msg);
                            result.errors.push(error_msg);
                        }
                        scanned_parent_folders.insert(parent_folder.to_path_buf());
                    }
                    Err(e) => {
                        let error_msg = format!(
                            "Error ensuring parent project exists for {}: {}",
                            parent_folder.display(),
                            e
                        );
                        warn!("{}", error_msg);
                        result.errors.push(error_msg);
                    }
                }
                
                current = parent_folder;
            }
        }

        // Second pass: Propagate images from parent folders to children
        info!("Propagating images from parent folders to children");
        for (folder, _) in project_folders.iter() {
            if let Some(&project_id) = path_to_id.get(folder) {
                if let Err(e) = self.inherit_images_from_parents(project_id, folder, root, &path_to_id)
                {
                    let error_msg = format!(
                        "Error inheriting images for project {}: {}",
                        folder.display(),
                        e
                    );
                    warn!("{}", error_msg);
                    result.errors.push(error_msg);
                }
            }
        }

        // Third pass: Generate/update composite previews for ALL projects
        if let Some(ref composite_service) = self.composite_service {
            info!("Generating composite previews for all projects during rescan");
            // Iterate over all projects, not just folders with STL files
            for (_folder, &project_id) in path_to_id.iter() {
                if let Err(e) = self.generate_preview_for_project(project_id, composite_service, &mut result) {
                    let error_msg = format!(
                        "Error generating preview for project {}: {}",
                        project_id, e
                    );
                    warn!("{}", error_msg);
                    result.errors.push(error_msg);
                }
            }
        }

        // Clean up orphaned cache files
        if let Some(ref cache_service) = self.image_cache_service {
            match cache_service.cleanup_orphaned() {
                Ok(count) => {
                    if count > 0 {
                        result
                            .errors
                            .push(format!("Cleaned up {} orphaned cache entries", count));
                    }
                }
                Err(e) => {
                    result.errors.push(format!("Error cleaning cache: {}", e));
                }
            }
        }

        Ok(result)
    }

    fn create_or_update_project_hierarchy(
        &self,
        folder: &Path,
        root: &Path,
        path_to_id: &mut HashMap<PathBuf, i64>,
        processed_paths: &mut HashSet<PathBuf>,
        existing_projects: &HashMap<String, i64>,
        result: &mut RescanResult,
    ) -> Result<i64, AppError> {
        if let Some(&existing_id) = path_to_id.get(folder) {
            return Ok(existing_id);
        }

        let full_path = folder.to_str().unwrap_or("").to_string();

        processed_paths.insert(folder.to_path_buf());

        let parent_id = if folder != root {
            if let Some(parent) = folder.parent() {
                if parent >= root {
                    Some(self.create_or_update_project_hierarchy(
                        parent,
                        root,
                        path_to_id,
                        processed_paths,
                        existing_projects,
                        result,
                    )?)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        let name = folder
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or("Unknown")
            .to_string();

        let project_id = if let Some(&existing_id) = existing_projects.get(&full_path) {
            // Project exists, update if needed
            path_to_id.insert(folder.to_path_buf(), existing_id);
            existing_id
        } else if let Some(project) = self.project_repo.get_by_path(&full_path)? {
            // Found by path query
            path_to_id.insert(folder.to_path_buf(), project.id);
            project.id
        } else {
            // Create new project
            let create_project = CreateProject {
                name,
                full_path: full_path.clone(),
                parent_id,
                is_leaf: true,
            };

            let new_id = self.project_repo.create(&create_project)?;
            path_to_id.insert(folder.to_path_buf(), new_id);
            result.projects_added += 1;
            new_id
        };

        Ok(project_id)
    }

    fn get_all_projects_map(&self) -> Result<HashMap<String, i64>, AppError> {
        let conn = self.project_repo.pool.get()?;
        let mut stmt = conn.prepare("SELECT id, full_path FROM projects")?;

        let projects = stmt
            .query_map([], |row| {
                Ok((row.get::<_, String>(1)?, row.get::<_, i64>(0)?))
            })?
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(projects)
    }

    fn get_existing_stl_files(&self, project_id: i64) -> Result<HashMap<String, i64>, AppError> {
        let conn = self.file_repo.pool.get()?;
        let mut stmt = conn.prepare("SELECT id, file_path FROM stl_files WHERE project_id = ?1")?;

        let files = stmt
            .query_map([project_id], |row| {
                Ok((row.get::<_, String>(1)?, row.get::<_, i64>(0)?))
            })?
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(files)
    }

    fn process_stl_file(
        &self,
        project_id: i64,
        stl_file: &Path,
        existing_files: &HashMap<String, i64>,
        result: &mut RescanResult,
    ) -> Result<(), AppError> {
        let file_path = stl_file.to_str().unwrap_or("");
        let filename = stl_file
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or("");
        let metadata = fs::metadata(stl_file)?;
        let file_size = metadata.len() as i64;

        if let Some(&_file_id) = existing_files.get(file_path) {
            // File exists - check if modified (size or timestamp changed)
            // For now, just count as processed
            // TODO: Could add modification detection based on size/mtime
        } else {
            // New file - add it
            self.file_repo
                .add_stl_file(project_id, filename, file_path, file_size)?;
            result.files_added += 1;
        }

        Ok(())
    }

    fn process_images_for_project(
        &self,
        project_id: i64,
        folder: &Path,
        result: &mut RescanResult,
    ) -> Result<(), AppError> {
        let image_extensions = ["jpg", "jpeg", "png", "gif", "webp"];

        // Get existing images
        let existing_images = self.get_existing_image_files(project_id)?;
        let mut found_image_paths = HashSet::new();

        if let Ok(entries) = fs::read_dir(folder) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(ext) = entry.path().extension() {
                            if image_extensions.iter().any(|e| ext.eq_ignore_ascii_case(e)) {
                                let file_path = entry.path().to_str().unwrap_or("").to_string();
                                found_image_paths.insert(file_path.clone());

                                if !existing_images.contains_key(&file_path) {
                                    let filename =
                                        entry.file_name().to_str().unwrap_or("").to_string();
                                    let file_size = fs::metadata(entry.path())
                                        .map(|m| m.len() as i64)
                                        .unwrap_or(0);

                                    self.file_repo.add_image_file(
                                        project_id, &filename, &file_path, file_size, "direct",
                                        None, 0,
                                    )?;
                                    result.files_added += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Remove deleted images
        for (image_path, image_id) in existing_images.iter() {
            if !found_image_paths.contains(image_path) {
                if let Err(e) = self.file_repo.delete_image_file(*image_id) {
                    result
                        .errors
                        .push(format!("Error removing deleted image: {}", e));
                } else {
                    result.files_removed += 1;
                }
            }
        }

        Ok(())
    }

    fn get_existing_image_files(&self, project_id: i64) -> Result<HashMap<String, i64>, AppError> {
        let conn = self.file_repo.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, file_path FROM image_files WHERE project_id = ?1 AND source_type = 'direct'"
        )?;

        let files = stmt
            .query_map([project_id], |row| {
                Ok((row.get::<_, String>(1)?, row.get::<_, i64>(0)?))
            })?
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(files)
    }

    /// Clear all inherited images from the database.
    /// This is done at the start of rescan to rebuild inheritance fresh.
    fn clear_inherited_images(&self) -> Result<(), AppError> {
        let conn = self.file_repo.pool.get()?;
        conn.execute("DELETE FROM image_files WHERE source_type = 'inherited'", [])?;
        Ok(())
    }

    /// Ensure a project exists for the given folder path.
    fn ensure_project_exists(
        &self,
        folder: &Path,
        root: &Path,
        path_to_id: &HashMap<PathBuf, i64>,
    ) -> Result<i64, AppError> {
        // Check cache first
        if let Some(&existing_id) = path_to_id.get(folder) {
            return Ok(existing_id);
        }

        // Check database
        let full_path = folder.to_str().unwrap().to_string();
        if let Some(project) = self.project_repo.get_by_path(&full_path)? {
            return Ok(project.id);
        }

        // Create project for this folder
        let parent_id = if folder != root {
            folder.parent().and_then(|p| {
                if p >= root {
                    path_to_id.get(p).copied()
                } else {
                    None
                }
            })
        } else {
            None
        };

        let name = folder
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or("Unknown")
            .to_string();

        let create_project = CreateProject {
            name,
            full_path,
            parent_id,
            is_leaf: false,
        };

        self.project_repo.create(&create_project)
    }

    /// Walk up the folder tree and inherit images from all ancestor folders.
    fn inherit_images_from_parents(
        &self,
        project_id: i64,
        folder: &Path,
        root: &Path,
        path_to_id: &HashMap<PathBuf, i64>,
    ) -> Result<(), AppError> {
        let image_extensions = ["jpg", "jpeg", "png", "gif", "webp"];
        let mut inherited_images = Vec::new();

        // Walk up the tree from current folder to root
        let mut current_folder = folder;
        while let Some(parent_folder) = current_folder.parent() {
            if parent_folder < root {
                break;
            }

            // Scan parent folder for images
            if let Ok(entries) = fs::read_dir(parent_folder) {
                for entry in entries.flatten() {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            if let Some(ext) = entry.path().extension() {
                                if image_extensions.iter().any(|e| ext.eq_ignore_ascii_case(e)) {
                                    // Get the parent project ID
                                    let source_project_id = path_to_id
                                        .get(parent_folder)
                                        .copied()
                                        .or_else(|| {
                                            self.ensure_project_exists(
                                                parent_folder,
                                                root,
                                                path_to_id,
                                            )
                                            .ok()
                                        });

                                    if let Some(source_id) = source_project_id {
                                        let filename =
                                            entry.file_name().to_str().unwrap_or("").to_string();
                                        let file_path =
                                            entry.path().to_str().unwrap_or("").to_string();
                                        let file_size = fs::metadata(entry.path())
                                            .map(|m| m.len() as i64)
                                            .unwrap_or(0);

                                        inherited_images.push((
                                            filename,
                                            file_path,
                                            file_size,
                                            source_id,
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }

            current_folder = parent_folder;
        }

        // Add all inherited images to this project
        for (filename, file_path, file_size, source_id) in inherited_images {
            self.file_repo.add_image_file(
                project_id,
                &filename,
                &file_path,
                file_size,
                "inherited",
                Some(source_id),
                0,
            )?;
        }

        Ok(())
    }

    /// Generate or update composite preview for a project during rescan
    fn generate_preview_for_project(
        &self,
        project_id: i64,
        composite_service: &crate::services::composite_preview::CompositePreviewService,
        result: &mut RescanResult,
    ) -> Result<(), AppError> {
        // Get first 4 direct images for this project
        let conn = self.file_repo.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, file_path FROM image_files 
             WHERE project_id = ?1 AND source_type = 'direct'
             ORDER BY created_at ASC
             LIMIT 4"
        )?;

        let images: Vec<(i64, String)> = stmt
            .query_map([project_id], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        // Check if we had a previous preview
        let had_preview = self.preview_repo.get_preview(project_id)?.is_some();

        if images.len() < 2 {
            // Not enough images for composite - delete preview if it exists
            if had_preview {
                composite_service.delete_preview(project_id)?;
                self.preview_repo.delete_preview(project_id)?;
            }
            return Ok(());
        }

        let image_paths: Vec<String> = images.iter().map(|(_, path)| path.clone()).collect();
        let image_ids: Vec<i64> = images.iter().map(|(id, _)| *id).collect();

        // Generate the composite preview
        let preview_path = composite_service.generate_preview(project_id, &image_paths)?;

        // Store in database
        let create_preview = crate::db::repositories::preview_repo::CreateProjectPreview {
            project_id,
            preview_path: preview_path.to_string_lossy().to_string(),
            image_count: image_ids.len() as i32,
            source_image_ids: image_ids,
        };

        self.preview_repo.store_preview(&create_preview)?;

        if !had_preview {
            result.files_added += 1; // Count new previews
        } else {
            result.files_updated += 1; // Count updated previews
        }

        Ok(())
    }
}
