use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::services::folder_service::FolderService;

#[derive(Clone)]
pub struct BrowseState {
    pub folder_service: Arc<FolderService>,
}

#[derive(Debug, Deserialize)]
pub struct FolderQuery {
    page: Option<usize>,
    per_page: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error: String,
}

/// GET /api/browse/*path - Get folder contents
pub async fn get_folder_contents(
    State(state): State<BrowseState>,
    path: Option<Path<String>>,
    Query(query): Query<FolderQuery>,
) -> impl IntoResponse {
    let folder_path = path.map(|p| p.0).unwrap_or_default();

    match state
        .folder_service
        .get_folder_contents(&folder_path, query.page, query.per_page)
    {
        Ok(contents) => (StatusCode::OK, Json(contents)).into_response(),
        Err(e) => {
            tracing::error!("Failed to get folder contents: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to get folder contents: {}", e),
                }),
            )
                .into_response()
        }
    }
}

/// GET /api/browse/breadcrumb/*path - Get breadcrumb trail
pub async fn get_breadcrumb(
    State(state): State<BrowseState>,
    path: Option<Path<String>>,
) -> impl IntoResponse {
    let folder_path = path.map(|p| p.0).unwrap_or_default();

    match state.folder_service.get_breadcrumb_trail(&folder_path) {
        Ok(breadcrumbs) => (StatusCode::OK, Json(breadcrumbs)).into_response(),
        Err(e) => {
            tracing::error!("Failed to get breadcrumb: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to get breadcrumb: {}", e),
                }),
            )
                .into_response()
        }
    }
}
