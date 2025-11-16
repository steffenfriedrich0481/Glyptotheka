use axum::{
    routing::{delete, get, post},
    Router,
};
use crate::api::handlers::{config, files, projects, scan, search, tags};
use crate::api::handlers::scan::ScanState;
use crate::config::ConfigService;
use crate::db::connection::DbPool;
use crate::db::repositories::file_repo::FileRepository;
use crate::db::repositories::project_repo::ProjectRepository;
use crate::db::repositories::tag_repo::TagRepository;
use crate::services::download::DownloadService;
use crate::services::image_cache::ImageCacheService;
use crate::services::scanner::ScannerService;
use crate::services::rescan::RescanService;
use crate::services::search::SearchService;
use crate::services::stl_preview::StlPreviewService;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub project_repo: Arc<ProjectRepository>,
    pub file_repo: Arc<FileRepository>,
    pub tag_repo: Arc<TagRepository>,
    pub config_service: Arc<ConfigService>,
    pub scanner_service: Arc<ScannerService>,
    pub rescan_service: Arc<RescanService>,
    pub image_cache_service: Arc<ImageCacheService>,
    pub search_service: Arc<SearchService>,
    pub download_service: Arc<DownloadService>,
    pub stl_preview_service: Arc<StlPreviewService>,
    pub scan_state: Arc<Mutex<ScanState>>,
}

pub fn create_router(pool: DbPool, cache_dir: PathBuf) -> Router {
    let image_cache = Arc::new(ImageCacheService::new(cache_dir, pool.clone()));
    
    // Get stl_thumb_path from config
    let stl_thumb_path = {
        let conn = pool.get().ok();
        conn.and_then(|c| {
            c.query_row(
                "SELECT stl_thumb_path FROM config WHERE id = 1",
                [],
                |row| row.get::<_, Option<String>>(0)
            ).ok().flatten()
        }).map(PathBuf::from)
    };
    
    let stl_preview = Arc::new(StlPreviewService::new(
        stl_thumb_path,
        (*image_cache).clone(),
        pool.clone(),
    ));
    
    let state = AppState {
        pool: pool.clone(),
        project_repo: Arc::new(ProjectRepository::new(pool.clone())),
        file_repo: Arc::new(FileRepository::new(pool.clone())),
        tag_repo: Arc::new(TagRepository::new(pool.clone())),
        config_service: Arc::new(ConfigService::new(pool.clone())),
        scanner_service: Arc::new(ScannerService::new(pool.clone())),
        rescan_service: Arc::new(RescanService::with_cache(
            pool.clone(),
            (*image_cache).clone(),
        )),
        image_cache_service: image_cache,
        search_service: Arc::new(SearchService::new(pool.clone())),
        download_service: Arc::new(DownloadService::new(pool.clone())),
        stl_preview_service: stl_preview,
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
        .route("/api/projects/:id/download", get(files::download_project_zip))
        // File/Image routes
        .route("/api/images/:hash", get(files::serve_image))
        .route("/api/previews/:hash", get(files::serve_preview))
        .route("/api/files/:id", get(files::download_file))
        // Search routes
        .route("/api/search", get(search::search_projects))
        // Tags routes
        .route("/api/tags", get(tags::list_tags))
        .route("/api/tags", post(tags::create_tag))
        .route("/api/tags/autocomplete", get(tags::autocomplete_tags))
        .route("/api/projects/:id/tags", post(tags::add_tag_to_project))
        .route("/api/projects/:id/tags", delete(tags::remove_tag_from_project))
        .with_state(state)
}
