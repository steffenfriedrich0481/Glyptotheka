use crate::api::routes::AppState;
use crate::models::image_file::ImageFile;
use crate::models::project::{Project, ProjectWithRelations, StlCategory};
use crate::models::stl_file::StlFile;
use crate::utils::error::AppError;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectListResponse {
    pub projects: Vec<Project>,
}

#[derive(Debug, Deserialize)]
pub struct FilesPaginationParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilesResponse {
    pub stl_categories: Vec<StlCategory>,
    pub images: Vec<ImageFile>,
    pub total_images: i64,
    pub page: i64,
    pub per_page: i64,
}

pub async fn list_root_projects(
    State(state): State<AppState>,
) -> Result<Json<ProjectListResponse>, AppError> {
    let projects = state.project_repo.list_root()?;
    Ok(Json(ProjectListResponse { projects }))
}

pub async fn get_project(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ProjectWithRelations>, AppError> {
    let project = state
        .project_repo
        .get_with_relations(id)?
        .ok_or_else(|| AppError::NotFound(format!("Project {} not found", id)))?;
    Ok(Json(project))
}

pub async fn get_project_children(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ProjectListResponse>, AppError> {
    let projects = state.project_repo.list_children(id)?;
    Ok(Json(ProjectListResponse { projects }))
}

pub async fn get_project_files(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Query(pagination): Query<FilesPaginationParams>,
) -> Result<Json<FilesResponse>, AppError> {
    // Verify project exists
    state
        .project_repo
        .get_by_id(id)?
        .ok_or_else(|| AppError::NotFound(format!("Project {} not found", id)))?;

    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(20);
    let offset = (page - 1) * per_page;

    let stl_files = state.file_repo.get_stl_files_by_project(id)?;

    // Group STL files by category
    let mut category_map: HashMap<Option<String>, Vec<StlFile>> = HashMap::new();
    for file in stl_files {
        category_map
            .entry(file.category.clone())
            .or_insert_with(Vec::new)
            .push(file);
    }

    // Convert to Vec<StlCategory>, with uncategorized files first
    let mut stl_categories: Vec<StlCategory> = category_map
        .into_iter()
        .map(|(category, files)| StlCategory { category, files })
        .collect();

    // Sort: uncategorized (None) first, then alphabetically by category name
    stl_categories.sort_by(|a, b| match (&a.category, &b.category) {
        (None, None) => std::cmp::Ordering::Equal,
        (None, Some(_)) => std::cmp::Ordering::Less,
        (Some(_), None) => std::cmp::Ordering::Greater,
        (Some(a_cat), Some(b_cat)) => a_cat.cmp(b_cat),
    });

    // T030: Use priority-sorted images (regular images before STL previews)
    let images = state
        .file_repo
        .get_images_by_priority(id, per_page, offset)?;
    let total_images = state.file_repo.count_images_by_project(id)?;

    Ok(Json(FilesResponse {
        stl_categories,
        images,
        total_images,
        page,
        per_page,
    }))
}

pub async fn get_project_preview(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl axum::response::IntoResponse, AppError> {
    use axum::body::Body;
    use axum::http::{header, StatusCode};
    use axum::response::Response;
    use tokio::fs::File;
    use tokio_util::io::ReaderStream;

    // Get preview from database
    let preview = state
        .preview_repo
        .get_preview(id)?
        .ok_or_else(|| AppError::NotFound(format!("Preview not found for project {}", id)))?;

    // Check if file exists
    let preview_path = std::path::Path::new(&preview.preview_path);
    if !preview_path.exists() {
        return Err(AppError::NotFound(format!(
            "Preview file not found at {:?}",
            preview_path
        )));
    }

    // Serve the image
    let file = File::open(preview_path)
        .await
        .map_err(|e| AppError::InternalServer(format!("Failed to open preview file: {}", e)))?;

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "image/png")
        .header(header::CACHE_CONTROL, "public, max-age=3600")
        .body(body)
        .unwrap())
}
