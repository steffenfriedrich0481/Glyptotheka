use crate::models::tag::Tag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub full_path: String,
    pub parent_id: Option<i64>,
    pub is_leaf: bool,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectWithRelations {
    #[serde(flatten)]
    pub project: Project,
    pub children: Vec<Project>,
    pub stl_count: usize,
    pub image_count: usize,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProject {
    pub name: String,
    pub full_path: String,
    pub parent_id: Option<i64>,
    pub is_leaf: bool,
}
