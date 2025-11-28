use serde::{Deserialize, Serialize};

use super::project::Project;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    pub path: String,
    pub name: String,
    pub level: usize,
    pub parent_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderContents {
    pub folder: Folder,
    pub subfolders: Vec<Folder>,
    pub projects: Vec<Project>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreadcrumbItem {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breadcrumb {
    pub items: Vec<BreadcrumbItem>,
}

impl Folder {
    pub fn from_path(path: &str) -> Self {
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let level = parts.len();
        let name = parts.last().map(|s| s.to_string()).unwrap_or_default();
        let parent_path = if parts.len() > 1 {
            Some(parts[..parts.len() - 1].join("/"))
        } else {
            None
        };

        Self {
            path: path.to_string(),
            name,
            level,
            parent_path,
        }
    }

    pub fn get_breadcrumb(&self) -> Breadcrumb {
        let parts: Vec<&str> = self.path.split('/').filter(|s| !s.is_empty()).collect();
        let mut items = vec![BreadcrumbItem {
            name: "Root".to_string(),
            path: "/".to_string(),
        }];

        let mut current_path = String::new();
        for part in parts {
            current_path.push('/');
            current_path.push_str(part);
            items.push(BreadcrumbItem {
                name: part.to_string(),
                path: current_path.clone(),
            });
        }

        Breadcrumb { items }
    }
}
