use axum::{extract::State, Json};
use crate::api::routes::AppState;
use crate::utils::error::AppError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanStatus {
    pub is_scanning: bool,
    pub projects_found: Option<usize>,
    pub files_processed: Option<usize>,
    pub errors: Option<Vec<String>>,
}

pub struct ScanState {
    pub is_scanning: bool,
    pub result: Option<crate::services::scanner::ScanResult>,
}

pub async fn start_scan(
    State(state): State<AppState>,
) -> Result<Json<ScanStatus>, AppError> {
    let mut scan_state = state.scan_state.lock().await;
    
    if scan_state.is_scanning {
        return Ok(Json(ScanStatus {
            is_scanning: true,
            projects_found: None,
            files_processed: None,
            errors: None,
        }));
    }

    let config = state.config_service.get_config()?;
    let root_path = config.root_path.ok_or_else(|| {
        AppError::ValidationError("Root path not configured".to_string())
    })?;

    scan_state.is_scanning = true;
    scan_state.result = None;
    drop(scan_state);

    let scanner = state.scanner_service.clone();
    let scan_state_arc = state.scan_state.clone();
    let config_service = state.config_service.clone();
    
    tokio::spawn(async move {
        let result = scanner.scan(&root_path);
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
        files_processed: None,
        errors: None,
    }))
}

pub async fn get_scan_status(
    State(state): State<AppState>,
) -> Result<Json<ScanStatus>, AppError> {
    let scan_state = state.scan_state.lock().await;
    
    let status = if let Some(ref result) = scan_state.result {
        ScanStatus {
            is_scanning: scan_state.is_scanning,
            projects_found: Some(result.projects_found),
            files_processed: Some(result.files_processed),
            errors: if result.errors.is_empty() {
                None
            } else {
                Some(result.errors.clone())
            },
        }
    } else {
        ScanStatus {
            is_scanning: scan_state.is_scanning,
            projects_found: None,
            files_processed: None,
            errors: None,
        }
    };

    Ok(Json(status))
}
