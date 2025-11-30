use crate::db::connection::DbPool;
use crate::db::repositories::file_repo::FileRepository;
use crate::db::repositories::project_repo::ProjectRepository;
use crate::models::project::CreateProject;
use crate::utils::error::AppError;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tracing::{error, info, warn};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub projects_found: usize,
    pub files_processed: usize,
    pub stl_previews_generated: usize,
    pub stl_previews_queued: usize,
    pub errors: Vec<String>,
}

pub struct ScannerService {
    project_repo: ProjectRepository,
    file_repo: FileRepository,
    preview_repo: crate::db::repositories::preview_repo::PreviewRepository,
    composite_service: Option<crate::services::composite_preview::CompositePreviewService>,
    stl_preview_service: Option<crate::services::stl_preview::StlPreviewService>,
    preview_queue: Option<std::sync::Arc<crate::services::stl_preview::PreviewQueue>>,
    ignored_keywords: Vec<String>,
    preview_semaphore: Arc<Semaphore>,
}

impl ScannerService {
    pub fn new(pool: DbPool) -> Self {
        // Limit concurrent preview operations to prevent resource exhaustion (max 10)
        let preview_semaphore = Arc::new(Semaphore::new(10));
        Self {
            project_repo: ProjectRepository::new(pool.clone()),
            file_repo: FileRepository::new(pool.clone()),
            preview_repo: crate::db::repositories::preview_repo::PreviewRepository::new(pool),
            composite_service: None,
            stl_preview_service: None,
            preview_queue: None,
            ignored_keywords: Vec::new(),
            preview_semaphore,
        }
    }

    pub fn with_ignored_keywords(mut self, keywords: Vec<String>) -> Self {
        self.ignored_keywords = keywords;
        self
    }

    pub fn with_composite_preview(mut self, cache_dir: std::path::PathBuf) -> Self {
        self.composite_service =
            Some(crate::services::composite_preview::CompositePreviewService::new(cache_dir));
        self
    }

    pub fn with_stl_preview(
        mut self,
        stl_preview_service: crate::services::stl_preview::StlPreviewService,
        preview_queue: std::sync::Arc<crate::services::stl_preview::PreviewQueue>,
    ) -> Self {
        self.stl_preview_service = Some(stl_preview_service);
        self.preview_queue = Some(preview_queue);
        self
    }

    /// Check if a folder name contains any ignored keyword (case-insensitive substring match)
    fn is_stl_category_folder(&self, folder_name: &str) -> bool {
        let normalized_name = folder_name.trim().to_lowercase();
        self.ignored_keywords
            .iter()
            .any(|keyword| normalized_name.contains(&keyword.trim().to_lowercase()))
    }

    pub fn scan(&self, root_path: &str) -> Result<ScanResult, AppError> {
        let root = Path::new(root_path);

        info!("Starting scan of directory: {}", root_path);

        if !root.exists() {
            error!("Scan failed: Root path does not exist: {}", root_path);
            return Err(AppError::ValidationError(format!(
                "Root path does not exist: {}",
                root_path
            )));
        }

        if !root.is_dir() {
            error!("Scan failed: Root path is not a directory: {}", root_path);
            return Err(AppError::ValidationError(format!(
                "Root path is not a directory: {}",
                root_path
            )));
        }

        let mut projects_found = 0;
        let mut files_processed = 0;
        let mut errors = Vec::new();

        // Find all folders containing STL files
        let mut project_folders = HashMap::new();

        for entry in WalkDir::new(root).follow_links(false) {
            match entry {
                Ok(e) => {
                    if e.file_type().is_file() {
                        if let Some(ext) = e.path().extension() {
                            if ext.eq_ignore_ascii_case("stl") {
                                if let Some(parent) = e.path().parent() {
                                    // Find the actual project folder by traversing up
                                    // past any STL category folders
                                    let project_folder = self.find_project_folder(parent, root);
                                    project_folders
                                        .entry(project_folder)
                                        .or_insert_with(Vec::new)
                                        .push(e.path().to_path_buf());
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    let error_msg = format!("Error walking directory: {}", e);
                    error!("{}", error_msg);
                    errors.push(error_msg);
                }
            }
        }

        info!(
            "Directory walk complete. Found {} project folders",
            project_folders.len()
        );

        // Log some example project folders
        for (folder, stl_files) in project_folders.iter().take(5) {
            info!(
                "  Example project folder: {} ({} STL files)",
                folder.display(),
                stl_files.len()
            );
        }

        // Build project hierarchy
        let mut path_to_id = HashMap::new();
        let mut processed_paths = HashSet::new();

        // Process each project folder
        for (folder, stl_files) in project_folders.iter() {
            match self.create_project_hierarchy(folder, root, &mut path_to_id, &mut processed_paths)
            {
                Ok(project_id) => {
                    projects_found += 1;

                    // Add STL files and generate previews
                    let stl_files_vec: Vec<_> = stl_files.to_vec();
                    for stl_file in &stl_files_vec {
                        // Extract category from the immediate parent folder if it's an STL category folder
                        let category = stl_file
                            .parent()
                            .and_then(|parent| parent.file_name())
                            .and_then(|name| name.to_str())
                            .and_then(|name_str| {
                                if self.is_stl_category_folder(name_str) {
                                    Some(name_str)
                                } else {
                                    None
                                }
                            });

                        match self.file_repo.add_stl_file_with_category(
                            project_id,
                            stl_file.file_name().unwrap().to_str().unwrap(),
                            stl_file.to_str().unwrap(),
                            fs::metadata(stl_file).map(|m| m.len() as i64).unwrap_or(0),
                            category,
                        ) {
                            Ok(_) => files_processed += 1,
                            Err(e) => {
                                let error_msg =
                                    format!("Error adding STL file {}: {}", stl_file.display(), e);
                                error!("{}", error_msg);
                                errors.push(error_msg);
                            }
                        }
                    }

                    // Generate STL previews if service available (US1)
                    if self.stl_preview_service.is_some() && !stl_files_vec.is_empty() {
                        // Split: first 2 sync, rest async
                        let (sync_files, async_files) =
                            stl_files_vec.split_at(std::cmp::min(2, stl_files_vec.len()));

                        // Generate first 2 synchronously
                        for stl_file in sync_files {
                            if let Err(e) = self.generate_stl_preview_sync(project_id, stl_file) {
                                warn!(
                                    "Failed to generate preview for {}: {}",
                                    stl_file.display(),
                                    e
                                );
                            }
                        }

                        // Queue remaining for async generation
                        for stl_file in async_files {
                            if let Err(e) = self.queue_stl_preview(project_id, stl_file) {
                                warn!("Failed to queue preview for {}: {}", stl_file.display(), e);
                            }
                        }
                    }

                    // Find and add images
                    if let Err(e) = self.add_images_for_project(project_id, folder) {
                        let error_msg = format!(
                            "Error adding images for project {}: {}",
                            folder.display(),
                            e
                        );
                        warn!("{}", error_msg);
                        errors.push(error_msg);
                    }
                }
                Err(e) => {
                    let error_msg =
                        format!("Error creating project for {}: {}", folder.display(), e);
                    error!("{}", error_msg);
                    errors.push(error_msg);
                }
            }
        }

        // NEW: Scan parent folders for images
        info!("Scanning parent folders for images");
        let mut scanned_folders = HashSet::new();

        for (folder, _) in project_folders.iter() {
            let mut current: &Path = folder.as_path();

            while let Some(parent_folder) = current.parent() {
                if parent_folder < root {
                    break;
                }

                // Skip if already scanned
                if scanned_folders.contains(parent_folder) {
                    current = parent_folder;
                    continue;
                }

                // Ensure parent project exists
                match self.ensure_project_exists(parent_folder, root, &mut path_to_id) {
                    Ok(parent_id) => {
                        // Scan parent folder for images
                        if let Err(e) = self.add_images_for_project(parent_id, parent_folder) {
                            let error_msg = format!(
                                "Error adding images for parent folder {}: {}",
                                parent_folder.display(),
                                e
                            );
                            warn!("{}", error_msg);
                            errors.push(error_msg);
                        }
                        scanned_folders.insert(parent_folder.to_path_buf());
                    }
                    Err(e) => {
                        let error_msg = format!(
                            "Error ensuring parent project exists for {}: {}",
                            parent_folder.display(),
                            e
                        );
                        warn!("{}", error_msg);
                        errors.push(error_msg);
                    }
                }

                current = parent_folder;
            }
        }

        // Second pass: Propagate images from parent folders to children
        info!("Propagating images from parent folders to children");
        for (folder, _) in project_folders.iter() {
            if let Some(&project_id) = path_to_id.get(folder) {
                if let Err(e) =
                    self.inherit_images_from_parents(project_id, folder, root, &mut path_to_id)
                {
                    let error_msg = format!(
                        "Error inheriting images for project {}: {}",
                        folder.display(),
                        e
                    );
                    warn!("{}", error_msg);
                    errors.push(error_msg);
                }
            }
        }

        // Third pass: Generate composite previews for ALL projects
        if let Some(ref composite_service) = self.composite_service {
            info!("Generating composite previews for all projects");
            // Iterate over all projects, not just folders with STL files
            for (_folder, &project_id) in path_to_id.iter() {
                if let Err(e) = self.generate_preview_for_project(project_id, composite_service) {
                    let error_msg =
                        format!("Error generating preview for project {}: {}", project_id, e);
                    warn!("{}", error_msg);
                    errors.push(error_msg);
                }
            }
        }

        // Fourth pass: Backfill missing STL previews
        let (stl_previews_generated, stl_previews_queued) = if self.stl_preview_service.is_some() {
            info!("Backfilling missing STL previews");
            self.backfill_stl_previews(&mut errors)?
        } else {
            (0, 0)
        };

        info!(
            "Scan complete: {} projects found, {} files processed, {} STL previews generated, {} queued, {} errors",
            projects_found,
            files_processed,
            stl_previews_generated,
            stl_previews_queued,
            errors.len()
        );

        if !errors.is_empty() {
            warn!("Scan completed with {} errors", errors.len());
        }

        Ok(ScanResult {
            projects_found,
            files_processed,
            stl_previews_generated,
            stl_previews_queued,
            errors,
        })
    }

    /// Find the actual project folder by traversing up past STL category folders
    fn find_project_folder(&self, start_folder: &Path, root: &Path) -> PathBuf {
        let mut current = start_folder;

        // Traverse up while the current folder name matches an ignored keyword
        while let Some(folder_name) = current.file_name().and_then(|n| n.to_str()) {
            if self.is_stl_category_folder(folder_name) {
                // This is an STL category folder, go up one level
                if let Some(parent) = current.parent() {
                    if parent >= root {
                        current = parent;
                        continue;
                    }
                }
            }
            break;
        }

        current.to_path_buf()
    }

    fn create_project_hierarchy(
        &self,
        folder: &Path,
        root: &Path,
        path_to_id: &mut HashMap<PathBuf, i64>,
        processed_paths: &mut HashSet<PathBuf>,
    ) -> Result<i64, AppError> {
        let full_path = folder.to_str().unwrap().to_string();

        if let Some(&existing_id) = path_to_id.get(folder) {
            // Ensure is_leaf is true since this folder has STL files
            self.project_repo.update_is_leaf(existing_id, true)?;
            return Ok(existing_id);
        }

        if processed_paths.contains(folder) {
            if let Some(project) = self.project_repo.get_by_path(&full_path)? {
                path_to_id.insert(folder.to_path_buf(), project.id);
                // Ensure is_leaf is true
                if !project.is_leaf {
                    self.project_repo.update_is_leaf(project.id, true)?;
                }
                return Ok(project.id);
            }
        }

        processed_paths.insert(folder.to_path_buf());

        let parent_id = if folder != root {
            if let Some(parent) = folder.parent() {
                if parent >= root {
                    // Use ensure_project_exists for parent (creates with is_leaf=false)
                    Some(self.ensure_project_exists(parent, root, path_to_id)?)
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

        let create_project = CreateProject {
            name,
            full_path: full_path.clone(),
            parent_id,
            is_leaf: true,
        };

        let project_id = if let Some(existing) = self.project_repo.get_by_path(&full_path)? {
            if !existing.is_leaf {
                self.project_repo.update_is_leaf(existing.id, true)?;
            }
            existing.id
        } else {
            self.project_repo.create(&create_project)?
        };

        path_to_id.insert(folder.to_path_buf(), project_id);

        Ok(project_id)
    }

    fn add_images_for_project(&self, project_id: i64, folder: &Path) -> Result<(), AppError> {
        let image_extensions = ["jpg", "jpeg", "png", "gif", "webp"];

        // Get existing images for this project to avoid duplicates
        let conn = self.file_repo.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT file_path FROM image_files WHERE project_id = ?1 AND source_type = 'direct'",
        )?;
        let existing_images: HashSet<String> = stmt
            .query_map([project_id], |row| row.get::<_, String>(0))?
            .collect::<Result<HashSet<_>, _>>()?;

        if let Ok(entries) = fs::read_dir(folder) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(ext) = entry.path().extension() {
                            if image_extensions.iter().any(|e| ext.eq_ignore_ascii_case(e)) {
                                let filename = entry.file_name().to_str().unwrap_or("").to_string();
                                let file_path = entry.path().to_str().unwrap_or("").to_string();

                                // Skip if already exists
                                if existing_images.contains(&file_path) {
                                    continue;
                                }

                                let file_size = fs::metadata(entry.path())
                                    .map(|m| m.len() as i64)
                                    .unwrap_or(0);

                                self.file_repo.add_image_file(
                                    project_id, &filename, &file_path, file_size, "direct", None, 0,
                                )?;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Ensure a project exists for the given folder path.
    /// Creates a project entry for parent folders that may not have STL files.
    fn ensure_project_exists(
        &self,
        folder: &Path,
        root: &Path,
        path_to_id: &mut HashMap<PathBuf, i64>,
    ) -> Result<i64, AppError> {
        // Check cache first
        if let Some(&existing_id) = path_to_id.get(folder) {
            return Ok(existing_id);
        }

        // Check database
        let full_path = folder.to_str().unwrap().to_string();
        if let Some(project) = self.project_repo.get_by_path(&full_path)? {
            // Add to cache for future lookups
            path_to_id.insert(folder.to_path_buf(), project.id);
            return Ok(project.id);
        }

        // Create project for this folder
        // Recursively ensure parent exists first
        let parent_id = if folder != root {
            folder.parent().and_then(|p| {
                if p >= root {
                    // Recursively ensure parent exists
                    self.ensure_project_exists(p, root, path_to_id).ok()
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

        let project_id = self.project_repo.create(&create_project)?;

        // Add to cache
        path_to_id.insert(folder.to_path_buf(), project_id);

        Ok(project_id)
    }

    /// Walk up the folder tree and inherit images from all ancestor folders.
    /// Images from parent folders are added to the current project with source_type="inherited"
    /// and source_project_id pointing to the folder where the image was found.
    fn inherit_images_from_parents(
        &self,
        project_id: i64,
        folder: &Path,
        root: &Path,
        path_to_id: &mut HashMap<PathBuf, i64>,
    ) -> Result<(), AppError> {
        let image_extensions = ["jpg", "jpeg", "png", "gif", "webp"];
        let mut inherited_images = Vec::new();

        // Walk up the tree from current folder to root
        // Track depth: 1 = immediate parent, 2 = grandparent, etc.
        let mut current_folder = folder;
        let mut depth = 0;
        while let Some(parent_folder) = current_folder.parent() {
            if parent_folder < root {
                break;
            }

            depth += 1;

            // Scan parent folder for images
            if let Ok(entries) = fs::read_dir(parent_folder) {
                for entry in entries.flatten() {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            if let Some(ext) = entry.path().extension() {
                                if image_extensions.iter().any(|e| ext.eq_ignore_ascii_case(e)) {
                                    // Get the parent project ID
                                    let source_project_id =
                                        path_to_id.get(parent_folder).copied().or_else(|| {
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
                                            filename, file_path, file_size, source_id, depth,
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
        // display_order = depth (closer images have lower order, appear first)
        for (filename, file_path, file_size, source_id, depth) in inherited_images {
            self.file_repo.add_image_file(
                project_id,
                &filename,
                &file_path,
                file_size,
                "inherited",
                Some(source_id),
                depth,
            )?;
        }

        Ok(())
    }

    /// Generate composite preview for a project
    fn generate_preview_for_project(
        &self,
        project_id: i64,
        composite_service: &crate::services::composite_preview::CompositePreviewService,
    ) -> Result<(), AppError> {
        // T034, T037: Use priority-sorted images for composite preview
        let conn = self.file_repo.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, file_path FROM image_files 
             WHERE project_id = ?1 AND source_type = 'direct'
             ORDER BY image_priority DESC, display_order ASC, created_at ASC
             LIMIT 4",
        )?;

        let images: Vec<(i64, String)> = stmt
            .query_map([project_id], |row| Ok((row.get(0)?, row.get(1)?)))?
            .collect::<Result<Vec<_>, _>>()?;

        if images.len() < 2 {
            // Need at least 2 images for a composite
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

        Ok(())
    }

    // T021: Generate STL preview synchronously
    fn generate_stl_preview_sync(&self, project_id: i64, stl_file: &Path) -> Result<(), AppError> {
        if let Some(ref service) = self.stl_preview_service {
            let stl_path = stl_file.to_str().unwrap().to_string();
            let service_clone = service.clone();
            let stl_file_clone = stl_file.to_path_buf();
            let file_repo_clone = self.file_repo.clone();
            let semaphore = self.preview_semaphore.clone();

            // Spawn async task without blocking, but limit concurrency
            tokio::spawn(async move {
                // Acquire permit to limit concurrent preview operations
                let _permit = match semaphore.acquire().await {
                    Ok(p) => p,
                    Err(e) => {
                        warn!("Failed to acquire preview semaphore: {}", e);
                        return;
                    }
                };

                match service_clone
                    .generate_preview_with_smart_cache(&stl_path)
                    .await
                {
                    Ok(crate::services::stl_preview::PreviewResult::Generated(preview_path))
                    | Ok(crate::services::stl_preview::PreviewResult::CacheHit(preview_path)) => {
                        info!(
                            "Generated preview for {}: {}",
                            stl_path,
                            preview_path.display()
                        );

                        // Add preview to image_files database
                        let filename = format!(
                            "{}.png",
                            stl_file_clone.file_name().unwrap().to_str().unwrap()
                        );
                        let preview_path_str = preview_path.to_str().unwrap();
                        let file_size = std::fs::metadata(&preview_path)
                            .map(|m| m.len() as i64)
                            .unwrap_or(0);

                        if let Err(e) = file_repo_clone.insert_stl_preview_image(
                            project_id,
                            &filename,
                            preview_path_str,
                            file_size,
                        ) {
                            warn!(
                                "Failed to add STL preview to database for {}: {}",
                                stl_path, e
                            );
                        } else {
                            info!("Added STL preview to database for {}", stl_path);
                        }
                    }
                    Ok(crate::services::stl_preview::PreviewResult::Skipped(reason)) => {
                        warn!("Skipped preview for {}: {}", stl_path, reason);
                    }
                    Err(e) => {
                        warn!(
                            "Failed to generate preview for {}: {}",
                            stl_file_clone.display(),
                            e
                        );
                    }
                }
            });
        }
        Ok(())
    }

    // T022: Queue STL preview for async generation
    fn queue_stl_preview(&self, project_id: i64, stl_file: &Path) -> Result<(), AppError> {
        if let Some(ref queue) = self.preview_queue {
            let stl_path = stl_file.to_str().unwrap().to_string();
            let queue_clone = queue.clone();
            let stl_file_clone = stl_file.to_path_buf();
            let file_repo_clone = self.file_repo.clone();
            let service_clone = self.stl_preview_service.as_ref().unwrap().clone();
            let semaphore = self.preview_semaphore.clone();

            // Spawn async task to queue the preview, but limit concurrency
            tokio::spawn(async move {
                // Acquire permit to limit concurrent operations
                let _permit = match semaphore.acquire().await {
                    Ok(p) => p,
                    Err(e) => {
                        warn!("Failed to acquire preview semaphore: {}", e);
                        return;
                    }
                };

                if let Err(e) = queue_clone.queue_preview(stl_path.clone()).await {
                    warn!("Failed to queue preview for {}: {}", stl_path, e);
                } else {
                    info!("Queued preview generation for {}", stl_path);

                    // Wait a bit and check if preview was generated, then add to DB
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

                    // Check if preview exists and add to database
                    if let Ok(Some(preview_path)) = service_clone.get_preview(&stl_path) {
                        let filename = format!(
                            "{}.png",
                            stl_file_clone.file_name().unwrap().to_str().unwrap()
                        );
                        let preview_path_str = preview_path.to_str().unwrap();
                        let file_size = std::fs::metadata(&preview_path)
                            .map(|m| m.len() as i64)
                            .unwrap_or(0);

                        if let Err(e) = file_repo_clone.insert_stl_preview_image(
                            project_id,
                            &filename,
                            preview_path_str,
                            file_size,
                        ) {
                            warn!(
                                "Failed to add queued STL preview to database for {}: {}",
                                stl_path, e
                            );
                        } else {
                            info!("Added queued STL preview to database for {}", stl_path);
                        }
                    }
                }
            });
        }
        Ok(())
    }

    // T023: Add STL preview to database
    fn add_stl_preview_to_db(
        &self,
        project_id: i64,
        stl_file: &Path,
        preview_path: &Path,
    ) -> Result<(), AppError> {
        let filename = format!("{}.png", stl_file.file_name().unwrap().to_str().unwrap());
        let preview_path_str = preview_path.to_str().unwrap();
        let file_size = fs::metadata(preview_path)
            .map(|m| m.len() as i64)
            .unwrap_or(0);

        self.file_repo.insert_stl_preview_image(
            project_id,
            &filename,
            preview_path_str,
            file_size,
        )?;

        Ok(())
    }

    /// Backfill missing STL previews for all STL files in the database
    fn backfill_stl_previews(&self, errors: &mut Vec<String>) -> Result<(usize, usize), AppError> {
        let mut generated = 0;
        let mut queued = 0;

        // Get all STL files from database
        let conn = self.file_repo.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT s.id, s.project_id, s.file_path, s.preview_path 
             FROM stl_files s
             ORDER BY s.project_id",
        )?;

        let stl_files: Vec<(i64, i64, String, Option<String>)> = stmt
            .query_map([], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        drop(stmt);
        drop(conn);

        info!("Found {} STL files in database", stl_files.len());

        // Check which ones need preview generation
        for (_file_id, project_id, stl_path, existing_preview) in stl_files {
            let stl_path_buf = PathBuf::from(&stl_path);

            // Skip if file doesn't exist
            if !stl_path_buf.exists() {
                warn!("STL file not found, skipping preview: {}", stl_path);
                continue;
            }

            // Check if preview exists and is valid
            let needs_preview = if let Some(ref preview_path) = existing_preview {
                let preview_path_buf = PathBuf::from(preview_path);
                // Check if preview file exists
                if !preview_path_buf.exists() {
                    info!("Preview file missing for {}, will regenerate", stl_path);
                    true
                } else {
                    // Check if preview is older than STL file
                    match (fs::metadata(&stl_path_buf), fs::metadata(&preview_path_buf)) {
                        (Ok(stl_meta), Ok(preview_meta)) => {
                            match (stl_meta.modified(), preview_meta.modified()) {
                                (Ok(stl_time), Ok(preview_time)) => {
                                    if stl_time > preview_time {
                                        info!("Preview outdated for {}, will regenerate", stl_path);
                                        true
                                    } else {
                                        // Preview exists and is up to date, check if in image_files
                                        let conn = self.file_repo.pool.get()?;
                                        let count: i64 = conn.query_row(
                                            "SELECT COUNT(*) FROM image_files WHERE file_path = ?1 AND image_source = 'stl_preview'",
                                            [preview_path],
                                            |row| row.get(0)
                                        )?;

                                        if count == 0 {
                                            info!("Preview exists but not in image_files for {}, will add", stl_path);
                                            // Add existing preview to image_files
                                            if let Err(e) = self.add_stl_preview_to_db(
                                                project_id,
                                                &stl_path_buf,
                                                &preview_path_buf,
                                            ) {
                                                let error_msg = format!(
                                                    "Error adding existing STL preview to database for {}: {}",
                                                    stl_path, e
                                                );
                                                warn!("{}", error_msg);
                                                errors.push(error_msg);
                                            } else {
                                                generated += 1;
                                            }
                                        }
                                        false
                                    }
                                }
                                _ => true,
                            }
                        }
                        _ => true,
                    }
                }
            } else {
                info!("No preview exists for {}, will generate", stl_path);
                true
            };

            if needs_preview {
                // Generate first 5 synchronously, rest async
                if generated < 5 {
                    if let Err(e) = self.generate_stl_preview_sync(project_id, &stl_path_buf) {
                        let error_msg =
                            format!("Error generating STL preview for {}: {}", stl_path, e);
                        warn!("{}", error_msg);
                        errors.push(error_msg);
                    } else {
                        generated += 1;
                    }
                } else if let Err(e) = self.queue_stl_preview(project_id, &stl_path_buf) {
                    let error_msg = format!("Error queuing STL preview for {}: {}", stl_path, e);
                    warn!("{}", error_msg);
                    errors.push(error_msg);
                } else {
                    queued += 1;
                }
            }
        }

        info!(
            "Backfill complete: {} previews generated, {} queued",
            generated, queued
        );
        Ok((generated, queued))
    }
}
