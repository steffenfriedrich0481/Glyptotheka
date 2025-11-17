use crate::api::routes::AppState;
use crate::config::{AppConfig, UpdateConfigRequest};
use crate::utils::error::AppError;
use axum::{extract::State, Json};
use tracing::{error, info};

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
