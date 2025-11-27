use crate::api::routes::AppState;
use crate::config::{AppConfig, UpdateConfigRequest};
use crate::utils::error::AppError;
use axum::{extract::State, Json};
use std::path::Path;
use tracing::{error, info, warn};

pub async fn get_config(State(state): State<AppState>) -> Result<Json<AppConfig>, AppError> {
    info!("GET /api/config - Fetching configuration");
    match state.config_service.get_config() {
        Ok(config) => {
            info!("Configuration retrieved successfully");
            Ok(Json(config))
        }
        Err(e) => {
            error!("Failed to get configuration: {}", e);
            Err(e)
        }
    }
}

pub async fn update_config(
    State(state): State<AppState>,
    Json(request): Json<UpdateConfigRequest>,
) -> Result<Json<AppConfig>, AppError> {
    info!(
        "POST /api/config - Updating configuration with root_path: {:?}",
        request.root_path
    );

    // Validate root_path if provided
    if let Some(ref root_path) = request.root_path {
        let path = Path::new(root_path);
        if !path.exists() {
            warn!("Root path does not exist: {}", root_path);
            return Err(AppError::ValidationError(format!(
                "Path does not exist: {}. Please ensure the path is accessible within the container.",
                root_path
            )));
        }
        if !path.is_dir() {
            warn!("Root path is not a directory: {}", root_path);
            return Err(AppError::ValidationError(format!(
                "Path is not a directory: {}",
                root_path
            )));
        }
    }

    match state.config_service.update_config(&request) {
        Ok(config) => {
            info!("Configuration updated successfully");
            Ok(Json(config))
        }
        Err(e) => {
            error!("Failed to update configuration: {}", e);
            Err(e)
        }
    }
}
