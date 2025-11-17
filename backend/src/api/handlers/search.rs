use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::api::routes::AppState;
use crate::services::search::SearchParams;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub tags: Option<String>,
    #[serde(default = "default_page")]
    pub page: usize,
    #[serde(default = "default_per_page")]
    pub per_page: usize,
}

fn default_page() -> usize {
    1
}

fn default_per_page() -> usize {
    20
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub data: Vec<serde_json::Value>,
    pub meta: SearchMeta,
}

#[derive(Debug, Serialize)]
pub struct SearchMeta {
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
    pub total_pages: usize,
}

pub async fn search_projects(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<SearchResponse>, (StatusCode, Json<serde_json::Value>)> {
    let tags = if let Some(tags_str) = query.tags {
        tags_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        Vec::new()
    };

    let params = SearchParams {
        query: query.q,
        tags,
        page: query.page,
        per_page: query.per_page.min(100),
    };

    let result = state.search_service.search(&params).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;

    let data: Vec<serde_json::Value> = result
        .projects
        .into_iter()
        .map(|p| serde_json::to_value(p).unwrap())
        .collect();

    Ok(Json(SearchResponse {
        data,
        meta: SearchMeta {
            total: result.total,
            page: result.page,
            per_page: result.per_page,
            total_pages: result.total_pages,
        },
    }))
}
