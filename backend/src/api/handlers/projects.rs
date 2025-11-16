use axum::{
    extract::{Path, Query, State},
    Json,
};
use crate::api::routes::AppState;
use crate::models::project::{Project, ProjectWithRelations};
use crate::models::stl_file::StlFile;
use crate::models::image_file::ImageFile;
use crate::utils::error::AppError;
use serde::{Deserialize, Serialize};

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
    pub stl_files: Vec<StlFile>,
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
    let project = state.project_repo
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
    state.project_repo
        .get_by_id(id)?
        .ok_or_else(|| AppError::NotFound(format!("Project {} not found", id)))?;

    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(20);
    let offset = (page - 1) * per_page;

    let stl_files = state.file_repo.get_stl_files_by_project(id)?;
    let images = state.file_repo.get_image_files_by_project(id, per_page, offset)?;
    let total_images = state.file_repo.count_images_by_project(id)?;

    Ok(Json(FilesResponse {
        stl_files,
        images,
        total_images,
        page,
        per_page,
    }))
}




