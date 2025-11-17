use crate::db::connection::DbPool;
use crate::db::repositories::file_repo::FileRepository;
use crate::db::repositories::project_repo::ProjectRepository;
use crate::models::project::CreateProject;
use crate::utils::error::AppError;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{error, info, warn};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub projects_found: usize,
    pub files_processed: usize,
    pub errors: Vec<String>,
}

pub struct ScannerService {
    project_repo: ProjectRepository,
    file_repo: FileRepository,
}

impl ScannerService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            project_repo: ProjectRepository::new(pool.clone()),
            file_repo: FileRepository::new(pool),
        }
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

        // Build project hierarchy
        let mut path_to_id = HashMap::new();
        let mut processed_paths = HashSet::new();

        // Process each project folder
        for (folder, stl_files) in project_folders.iter() {
            match self.create_project_hierarchy(folder, root, &mut path_to_id, &mut processed_paths)
            {
                Ok(project_id) => {
                    projects_found += 1;

                    // Add STL files
                    for stl_file in stl_files {
                        match self.file_repo.add_stl_file(
                            project_id,
                            stl_file.file_name().unwrap().to_str().unwrap(),
                            stl_file.to_str().unwrap(),
                            fs::metadata(stl_file).map(|m| m.len() as i64).unwrap_or(0),
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

        info!(
            "Scan complete: {} projects found, {} files processed, {} errors",
            projects_found,
            files_processed,
            errors.len()
        );

        if !errors.is_empty() {
            warn!("Scan completed with {} errors", errors.len());
        }

        Ok(ScanResult {
            projects_found,
            files_processed,
            errors,
        })
    }

    fn create_project_hierarchy(
        &self,
        folder: &Path,
        root: &Path,
        path_to_id: &mut HashMap<PathBuf, i64>,
        processed_paths: &mut HashSet<PathBuf>,
    ) -> Result<i64, AppError> {
        if let Some(&existing_id) = path_to_id.get(folder) {
            return Ok(existing_id);
        }

        let full_path = folder.to_str().unwrap().to_string();

        if processed_paths.contains(folder) {
            if let Some(project) = self.project_repo.get_by_path(&full_path)? {
                path_to_id.insert(folder.to_path_buf(), project.id);
                return Ok(project.id);
            }
        }

        processed_paths.insert(folder.to_path_buf());

        let parent_id = if folder != root {
            if let Some(parent) = folder.parent() {
                if parent >= root {
                    Some(self.create_project_hierarchy(
                        parent,
                        root,
                        path_to_id,
                        processed_paths,
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

        let create_project = CreateProject {
            name,
            full_path: full_path.clone(),
            parent_id,
            is_leaf: true,
        };

        let project_id = if let Some(existing) = self.project_repo.get_by_path(&full_path)? {
            existing.id
        } else {
            self.project_repo.create(&create_project)?
        };

        path_to_id.insert(folder.to_path_buf(), project_id);

        Ok(project_id)
    }

    fn add_images_for_project(&self, project_id: i64, folder: &Path) -> Result<(), AppError> {
        let image_extensions = ["jpg", "jpeg", "png", "gif", "webp"];

        if let Ok(entries) = fs::read_dir(folder) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(ext) = entry.path().extension() {
                            if image_extensions.iter().any(|e| ext.eq_ignore_ascii_case(e)) {
                                let filename = entry.file_name().to_str().unwrap_or("").to_string();
                                let file_path = entry.path().to_str().unwrap_or("").to_string();
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
}
