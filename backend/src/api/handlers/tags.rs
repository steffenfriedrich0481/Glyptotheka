use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::api::routes::AppState;

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
