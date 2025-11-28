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
    pub folder_level: i32,
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
    pub inherited_images: Vec<ImagePreview>,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProject {
    pub name: String,
    pub full_path: String,
    pub parent_id: Option<i64>,
    pub is_leaf: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagePreview {
    pub id: i64,
    pub filename: String,
    pub source_type: String,  // "direct", "inherited", "stl_preview"
    pub image_source: String, // "original", "stl_preview"
    pub priority: i32,
    pub inherited_from: Option<String>, // Path from which the image was inherited
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultProject {
    #[serde(flatten)]
    pub project: Project,
    pub stl_count: usize,
    pub image_count: usize,
    pub images: Vec<ImagePreview>,
}

#[derive(Debug, Clone)]
pub struct ImageInheritance {
    pub id: i64,
    pub project_id: i64,
    pub image_id: i64,
    pub source_project_id: i64,
    pub inherited_from_path: String,
    pub created_at: i64,
}
