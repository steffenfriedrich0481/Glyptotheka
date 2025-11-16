use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::api::routes::AppState;
use crate::models::tag::CreateTag;

#[derive(Debug, Deserialize)]
pub struct TagsQuery {
    pub q: Option<String>,
    #[serde(default = "default_sort_by")]
    pub sort_by: String,
}

fn default_sort_by() -> String {
    "name".to_string()
}

#[derive(Debug, Serialize)]
pub struct TagsResponse {
    pub data: Vec<serde_json::Value>,
}

pub async fn list_tags(
    State(state): State<AppState>,
    Query(query): Query<TagsQuery>,
) -> Result<Json<TagsResponse>, (StatusCode, Json<serde_json::Value>)> {
    let mut tags = state.tag_repo.list_all().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;

    // Filter by query prefix if provided
    if let Some(q) = query.q {
        let q_lower = q.to_lowercase();
        tags.retain(|tag| tag.name.to_lowercase().starts_with(&q_lower));
    }

    // Sort by specified field
    match query.sort_by.as_str() {
        "usage" => {
            tags.sort_by(|a, b| b.usage_count.cmp(&a.usage_count).then(a.name.cmp(&b.name)));
        }
        _ => {
            tags.sort_by(|a, b| a.name.cmp(&b.name));
        }
    }

    let data: Vec<serde_json::Value> = tags
        .into_iter()
        .map(|t| serde_json::to_value(t).unwrap())
        .collect();

    Ok(Json(TagsResponse { data }))
}

pub async fn autocomplete_tags(
    State(state): State<AppState>,
    Query(query): Query<TagsQuery>,
) -> Result<Json<TagsResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Autocomplete uses the same logic as list_tags but with a query filter
    list_tags(State(state), Query(query)).await
}

#[derive(Debug, Deserialize)]
pub struct TagProjectRequest {
    #[serde(rename = "tagName")]
    pub tag_name: String,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RemoveTagQuery {
    #[serde(rename = "tagName")]
    pub tag_name: String,
}

// T095: POST /api/projects/:id/tags - Add tag to project
pub async fn add_tag_to_project(
    State(state): State<AppState>,
    Path(project_id): Path<i64>,
    Json(payload): Json<TagProjectRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Verify project exists
    let _project = state.project_repo.get_by_id(project_id).map_err(|e| {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": format!("Project not found: {}", e) })),
        )
    })?;

    // Get or create tag
    let tag_id = state.tag_repo.get_or_create(&payload.tag_name, payload.color).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;

    // Add tag to project
    state.tag_repo.add_to_project(project_id, tag_id).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;

    // Return updated project with tags
    let tags = state.tag_repo.get_project_tags(project_id).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;

    Ok(Json(serde_json::json!({ "tags": tags })))
}

// T096: DELETE /api/projects/:id/tags - Remove tag from project
pub async fn remove_tag_from_project(
    State(state): State<AppState>,
    Path(project_id): Path<i64>,
    Query(query): Query<RemoveTagQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Verify project exists
    let _project = state.project_repo.get_by_id(project_id).map_err(|e| {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": format!("Project not found: {}", e) })),
        )
    })?;

    // Find tag by name
    let conn = state.pool.get().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;

    let tag_id: i64 = conn
        .query_row(
            "SELECT id FROM tags WHERE name = ?1 COLLATE NOCASE",
            [&query.tag_name],
            |row| row.get(0),
        )
        .map_err(|_| {
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": "Tag not found" })),
            )
        })?;

    // Remove tag from project
    state.tag_repo.remove_from_project(project_id, tag_id).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;

    // Return updated project with tags
    let tags = state.tag_repo.get_project_tags(project_id).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;

    Ok(Json(serde_json::json!({ "tags": tags })))
}

// T097: POST /api/tags - Create new tag
pub async fn create_tag(
    State(state): State<AppState>,
    Json(payload): Json<CreateTag>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let tag_id = state.tag_repo.create(&payload).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;

    let conn = state.pool.get().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;

    let tag = conn
        .query_row(
            "SELECT id, name, color, created_at, usage_count FROM tags WHERE id = ?1",
            [tag_id],
            |row| {
                Ok(serde_json::json!({
                    "id": row.get::<_, i64>(0)?,
                    "name": row.get::<_, String>(1)?,
                    "color": row.get::<_, Option<String>>(2)?,
                    "created_at": row.get::<_, i64>(3)?,
                    "usage_count": row.get::<_, i32>(4)?
                }))
            },
        )
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
        })?;

    Ok(Json(tag))
}
