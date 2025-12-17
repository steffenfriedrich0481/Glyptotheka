use crate::api::routes::AppState;
use crate::utils::error::AppError;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanRequest {
    pub force: Option<bool>,
    pub clean: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanStatus {
    pub is_scanning: bool,
    pub projects_found: Option<usize>,
    pub projects_added: Option<usize>,
    pub projects_updated: Option<usize>,
    pub projects_removed: Option<usize>,
    pub files_processed: Option<usize>,
    pub files_added: Option<usize>,
    pub files_updated: Option<usize>,
    pub files_removed: Option<usize>,
    pub errors: Option<Vec<String>>,
}

pub struct ScanState {
    pub is_scanning: bool,
    pub result: Option<ScanResult>,
}

#[derive(Debug, Clone)]
pub enum ScanResult {
    Initial(crate::services::scanner::ScanResult),
    Rescan(crate::services::rescan::RescanResult),
}

pub async fn start_scan(
    State(state): State<AppState>,
    Json(req): Json<ScanRequest>,
) -> Result<Json<ScanStatus>, AppError> {
    tracing::info!("Scan request received: force={:?}, clean={:?}", req.force, req.clean);
    
    let mut scan_state = state.scan_state.lock().await;

    if scan_state.is_scanning {
        tracing::info!("Scan already in progress, returning current status");
        return Ok(Json(ScanStatus {
            is_scanning: true,
            projects_found: None,
            projects_added: None,
            projects_updated: None,
            projects_removed: None,
            files_processed: None,
            files_added: None,
            files_updated: None,
            files_removed: None,
            errors: None,
        }));
    }

    tracing::info!("Getting config...");
    let config = state.config_service.get_config().map_err(|e| {
        tracing::error!("Failed to get config: {:?}", e);
        e
    })?;
    
    let root_path = config
        .root_path
        .ok_or_else(|| {
            tracing::error!("Root path not configured");
            AppError::ValidationError("Root path not configured".to_string())
        })?;
    
    tracing::info!("Root path: {}", root_path);

    let force = req.force.unwrap_or(false);
    let clean = req.clean.unwrap_or(false);
    let has_been_scanned = config.last_scan_at.is_some();
    
    tracing::info!("Scan settings: force={}, clean={}, has_been_scanned={}", force, clean, has_been_scanned);

    // If clean is requested, clear all database entries and cache before scanning
    if clean {
        tracing::info!("Clean rescan requested - clearing all data");
        tracing::info!("Clearing project repository...");
        if let Err(e) = state.project_repo.clear_all() {
            tracing::error!("Failed to clear project repository: {:?}", e);
            return Err(e);
        }
        tracing::info!("Project repository cleared successfully");
        
        tracing::info!("Clearing image cache...");
        if let Err(e) = state.image_cache_service.clear_all() {
            tracing::warn!("Failed to clear image cache: {}", e);
        }
        tracing::info!("Image cache cleared");
    }

    scan_state.is_scanning = true;
    scan_state.result = None;
    drop(scan_state);

    let scanner = state.scanner_service.clone();
    let rescan_service = state.rescan_service.clone();
    let scan_state_arc = state.scan_state.clone();
    let config_service = state.config_service.clone();

    tokio::spawn(async move {
        // If clean was requested, always do a full scan
        let result = if clean || force || !has_been_scanned {
            // Initial scan or forced full rescan
            scanner.scan(&root_path).map(ScanResult::Initial)
        } else {
            // Incremental rescan
            rescan_service.rescan(&root_path).map(ScanResult::Rescan)
        };

        let mut state = scan_state_arc.lock().await;
        state.is_scanning = false;

        if result.is_ok() {
            let _ = config_service.update_last_scan();
        }

        state.result = result.ok();
    });

    Ok(Json(ScanStatus {
        is_scanning: true,
        projects_found: None,
        projects_added: None,
        projects_updated: None,
        projects_removed: None,
        files_processed: None,
        files_added: None,
        files_updated: None,
        files_removed: None,
        errors: None,
    }))
}

pub async fn get_scan_status(State(state): State<AppState>) -> Result<Json<ScanStatus>, AppError> {
    let scan_state = state.scan_state.lock().await;

    let status = if let Some(ref result) = scan_state.result {
        match result {
            ScanResult::Initial(r) => ScanStatus {
                is_scanning: scan_state.is_scanning,
                projects_found: Some(r.projects_found),
                projects_added: Some(r.projects_found), // All are new on initial scan
                projects_updated: Some(0),
                projects_removed: Some(0),
                files_processed: Some(r.files_processed),
                files_added: Some(r.files_processed), // All are new on initial scan
                files_updated: Some(0),
                files_removed: Some(0),
                errors: if r.errors.is_empty() {
                    None
                } else {
                    Some(r.errors.clone())
                },
            },
            ScanResult::Rescan(r) => ScanStatus {
                is_scanning: scan_state.is_scanning,
                projects_found: Some(r.projects_found),
                projects_added: Some(r.projects_added),
                projects_updated: Some(r.projects_updated),
                projects_removed: Some(r.projects_removed),
                files_processed: Some(r.files_processed),
                files_added: Some(r.files_added),
                files_updated: Some(r.files_updated),
                files_removed: Some(r.files_removed),
                errors: if r.errors.is_empty() {
                    None
                } else {
                    Some(r.errors.clone())
                },
            },
        }
    } else {
        ScanStatus {
            is_scanning: scan_state.is_scanning,
            projects_found: None,
            projects_added: None,
            projects_updated: None,
            projects_removed: None,
            files_processed: None,
            files_added: None,
            files_updated: None,
            files_removed: None,
            errors: None,
        }
    };

    Ok(Json(status))
}
