use axum::{extract::State, Json};
use crate::api::routes::AppState;
use crate::config::{AppConfig, UpdateConfigRequest};
use crate::utils::error::AppError;

pub async fn get_config(
    State(state): State<AppState>,
) -> Result<Json<AppConfig>, AppError> {
    let config = state.config_service.get_config()?;
    Ok(Json(config))
}

pub async fn update_config(
    State(state): State<AppState>,
    Json(request): Json<UpdateConfigRequest>,
) -> Result<Json<AppConfig>, AppError> {
    let config = state.config_service.update_config(&request)?;
    Ok(Json(config))
}

