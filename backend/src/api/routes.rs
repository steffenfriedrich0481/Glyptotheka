use axum::{
    routing::{get, post},
    Router,
};
use crate::api::handlers::{config, files, projects, scan};
use crate::api::handlers::scan::ScanState;
use crate::config::ConfigService;
use crate::db::connection::DbPool;
use crate::db::repositories::file_repo::FileRepository;
use crate::db::repositories::project_repo::ProjectRepository;
use crate::services::image_cache::ImageCacheService;
use crate::services::scanner::ScannerService;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub project_repo: Arc<ProjectRepository>,
    pub file_repo: Arc<FileRepository>,
    pub config_service: Arc<ConfigService>,
    pub scanner_service: Arc<ScannerService>,
    pub image_cache_service: Arc<ImageCacheService>,
    pub scan_state: Arc<Mutex<ScanState>>,
}

pub fn create_router(pool: DbPool, cache_dir: PathBuf) -> Router {
    let state = AppState {
        project_repo: Arc::new(ProjectRepository::new(pool.clone())),
        file_repo: Arc::new(FileRepository::new(pool.clone())),
        config_service: Arc::new(ConfigService::new(pool.clone())),
        scanner_service: Arc::new(ScannerService::new(pool.clone())),
        image_cache_service: Arc::new(ImageCacheService::new(cache_dir, pool.clone())),
        scan_state: Arc::new(Mutex::new(ScanState {
            is_scanning: false,
            result: None,
        })),
    };

    Router::new()
        // Config routes
        .route("/api/config", get(config::get_config))
        .route("/api/config", post(config::update_config))
        // Scan routes
        .route("/api/scan", post(scan::start_scan))
        .route("/api/scan/status", get(scan::get_scan_status))
        // Project routes
        .route("/api/projects", get(projects::list_root_projects))
        .route("/api/projects/:id", get(projects::get_project))
        .route("/api/projects/:id/children", get(projects::get_project_children))
        .route("/api/projects/:id/files", get(projects::get_project_files))
        // File/Image routes
        .route("/api/images/:hash", get(files::serve_image))
        .with_state(state)
}
