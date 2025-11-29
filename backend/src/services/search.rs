use crate::db::connection::DbPool;
use crate::db::repositories::file_repo::FileRepository;
use crate::models::project::{Project, SearchResultProject};
use crate::utils::error::AppError;

pub struct SearchService {
    pool: DbPool,
    ignored_keywords: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SearchParams {
    pub query: Option<String>,
    pub tags: Vec<String>,
    pub page: usize,
    pub per_page: usize,
    pub leaf_only: bool,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub projects: Vec<SearchResultProject>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
    pub total_pages: usize,
}

impl SearchService {
    pub fn new(pool: DbPool, ignored_keywords: Vec<String>) -> Self {
        Self {
            pool,
            ignored_keywords: ignored_keywords.iter().map(|k| k.to_lowercase()).collect(),
        }
    }

    fn resolve_display_name(&self, project: &Project) -> String {
        let name_lower = project.name.trim().to_lowercase();

        // If current name is not a keyword, return it
        if !self.ignored_keywords.contains(&name_lower) {
            return project.name.clone();
        }

        // Otherwise, traverse up the path
        let path = std::path::Path::new(&project.full_path);

        // Iterate components in reverse
        for component in path.components().rev() {
            if let Some(comp_str) = component.as_os_str().to_str() {
                let comp_lower = comp_str.trim().to_lowercase();
                if !self.ignored_keywords.contains(&comp_lower) {
                    return comp_str.to_string();
                }
            }
        }

        // Fallback to original name if everything is ignored (unlikely)
        project.name.clone()
    }

    pub fn search(&self, params: &SearchParams) -> Result<SearchResult, AppError> {
        let conn = self.pool.get()?;
        let offset = (params.page.saturating_sub(1)) * params.per_page;

        // Build query based on search parameters
        let (mut projects, total) = if params.query.is_some() && !params.tags.is_empty() {
            // Search by both name and tags
            self.search_combined(&conn, params, offset)?
        } else if params.query.is_some() {
            // Search by name only using FTS5
            self.search_fts(&conn, params, offset)?
        } else if !params.tags.is_empty() {
            // Filter by tags only
            self.search_by_tags(&conn, params, offset)?
        } else {
            // No filters - return all leaf projects
            self.search_all(&conn, params, offset)?
        };

        // Populate images for each project
        let file_repo = FileRepository::new(self.pool.clone());
        let project_ids: Vec<i64> = projects.iter().map(|p| p.project.id).collect();
        let mut images_map = file_repo.get_aggregated_images_batch(&project_ids, 15)?;

        for project in &mut projects {
            if let Some(images) = images_map.remove(&project.project.id) {
                project.image_count = images.len();
                project.images = images;
            }
        }

        let total_pages = if total > 0 {
            total.div_ceil(params.per_page)
        } else {
            0
        };

        Ok(SearchResult {
            projects,
            total,
            page: params.page,
            per_page: params.per_page,
            total_pages,
        })
    }

    fn search_fts(
        &self,
        conn: &rusqlite::Connection,
        params: &SearchParams,
        offset: usize,
    ) -> Result<(Vec<SearchResultProject>, usize), AppError> {
        let search_query = params.query.as_ref().unwrap();
        // Add wildcard for partial matching
        let fts_query = format!("{}*", search_query);
        let leaf_filter = if params.leaf_only {
            "AND p.is_leaf = 1"
        } else {
            ""
        };

        // Get total count
        let count_sql = format!(
            "SELECT COUNT(DISTINCT p.id)
             FROM projects p
             INNER JOIN projects_fts fts ON p.id = fts.project_id
             WHERE projects_fts MATCH ?1 {}",
            leaf_filter
        );

        let total: usize = conn.query_row(&count_sql, [&fts_query], |row| row.get(0))?;

        // Get projects
        let per_page_i64 = params.per_page as i64;
        let offset_i64 = offset as i64;

        let sql = format!(
            "SELECT DISTINCT p.id, p.name, p.full_path, p.parent_id, p.is_leaf, p.description, p.folder_level, p.created_at, p.updated_at,
             (SELECT COUNT(*) FROM stl_files WHERE project_id = p.id) as stl_count
             FROM projects p
             INNER JOIN projects_fts fts ON p.id = fts.project_id
             WHERE projects_fts MATCH ?1 {}
             ORDER BY p.name
             LIMIT ?2 OFFSET ?3",
            leaf_filter
        );

        let mut stmt = conn.prepare(&sql)?;

        let mut projects = stmt
            .query_map(
                rusqlite::params![&fts_query, per_page_i64, offset_i64],
                |row| {
                    Ok(SearchResultProject {
                        project: Project {
                            id: row.get(0)?,
                            name: row.get(1)?,
                            full_path: row.get(2)?,
                            parent_id: row.get(3)?,
                            is_leaf: row.get(4)?,
                            description: row.get(5)?,
                            folder_level: row.get(6)?,
                            created_at: row.get(7)?,
                            updated_at: row.get(8)?,
                        },
                        stl_count: row.get(9)?,
                        image_count: 0,
                        images: vec![],
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;

        // Update display names based on ignored keywords
        for p in &mut projects {
            p.project.name = self.resolve_display_name(&p.project);
        }

        Ok((projects, total))
    }

    fn search_by_tags(
        &self,
        conn: &rusqlite::Connection,
        params: &SearchParams,
        offset: usize,
    ) -> Result<(Vec<SearchResultProject>, usize), AppError> {
        // For simplicity with multiple tags, we'll filter projects that have ALL specified tags
        let placeholders = params
            .tags
            .iter()
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(", ");

        let leaf_filter = if params.leaf_only {
            "AND p.is_leaf = 1"
        } else {
            ""
        };

        let count_query = format!(
            "SELECT COUNT(*)
             FROM (
                 SELECT p.id
                 FROM projects p
                 INNER JOIN project_tags pt ON p.id = pt.project_id
                 INNER JOIN tags t ON pt.tag_id = t.id
                 WHERE t.name IN ({})
                 {}
                 GROUP BY p.id
                 HAVING COUNT(DISTINCT t.id) = ?
             )",
            placeholders, leaf_filter
        );

        let query = format!(
            "SELECT p.id, p.name, p.full_path, p.parent_id, p.is_leaf, p.description, p.folder_level, p.created_at, p.updated_at,
             (SELECT COUNT(*) FROM stl_files WHERE project_id = p.id) as stl_count
             FROM projects p
             INNER JOIN project_tags pt ON p.id = pt.project_id
             INNER JOIN tags t ON pt.tag_id = t.id
             WHERE t.name IN ({})
             {}
             GROUP BY p.id
             HAVING COUNT(DISTINCT t.id) = ?
             ORDER BY p.name
             LIMIT ? OFFSET ?",
            placeholders, leaf_filter
        );

        // Build params for count
        let tag_count = params.tags.len() as i64;
        let mut count_params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        for tag in &params.tags {
            count_params.push(tag);
        }
        count_params.push(&tag_count);

        let mut stmt = conn.prepare(&count_query)?;
        let total: usize = stmt
            .query_row(rusqlite::params_from_iter(count_params.iter()), |row| {
                row.get(0)
            })?;

        // Build params for query
        let per_page_i64 = params.per_page as i64;
        let offset_i64 = offset as i64;
        let mut query_params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        for tag in &params.tags {
            query_params.push(tag);
        }
        query_params.push(&tag_count);
        query_params.push(&per_page_i64);
        query_params.push(&offset_i64);

        let mut stmt = conn.prepare(&query)?;
        let mut projects = stmt
            .query_map(rusqlite::params_from_iter(query_params.iter()), |row| {
                Ok(SearchResultProject {
                    project: Project {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        full_path: row.get(2)?,
                        parent_id: row.get(3)?,
                        is_leaf: row.get(4)?,
                        description: row.get(5)?,
                        folder_level: row.get(6)?,
                        created_at: row.get(7)?,
                        updated_at: row.get(8)?,
                    },
                    stl_count: row.get(9)?,
                    image_count: 0,
                    images: vec![],
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        // Update display names based on ignored keywords
        for p in &mut projects {
            p.project.name = self.resolve_display_name(&p.project);
        }

        Ok((projects, total))
    }

    fn search_combined(
        &self,
        conn: &rusqlite::Connection,
        params: &SearchParams,
        offset: usize,
    ) -> Result<(Vec<SearchResultProject>, usize), AppError> {
        let search_query = params.query.as_ref().unwrap();
        let placeholders = params
            .tags
            .iter()
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(", ");

        let leaf_filter = if params.leaf_only {
            "AND p.is_leaf = 1"
        } else {
            ""
        };

        let count_query = format!(
            "SELECT COUNT(*)
             FROM (
                 SELECT p.id
                 FROM projects p
                 INNER JOIN projects_fts fts ON p.id = fts.project_id
                 INNER JOIN project_tags pt ON p.id = pt.project_id
                 INNER JOIN tags t ON pt.tag_id = t.id
                 WHERE projects_fts MATCH ?
                 AND t.name IN ({})
                 {}
                 GROUP BY p.id
                 HAVING COUNT(DISTINCT t.id) = ?
             )",
            placeholders, leaf_filter
        );

        let query = format!(
            "SELECT p.id, p.name, p.full_path, p.parent_id, p.is_leaf, p.description, p.folder_level, p.created_at, p.updated_at,
             (SELECT COUNT(*) FROM stl_files WHERE project_id = p.id) as stl_count
             FROM projects p
             INNER JOIN projects_fts fts ON p.id = fts.project_id
             INNER JOIN project_tags pt ON p.id = pt.project_id
             INNER JOIN tags t ON pt.tag_id = t.id
             WHERE projects_fts MATCH ?
             AND t.name IN ({})
             {}
             GROUP BY p.id
             HAVING COUNT(DISTINCT t.id) = ?
             ORDER BY p.name
             LIMIT ? OFFSET ?",
            placeholders, leaf_filter
        );

        // Build params for count
        let tag_count = params.tags.len() as i64;
        let mut count_params: Vec<&dyn rusqlite::ToSql> = vec![search_query];
        for tag in &params.tags {
            count_params.push(tag);
        }
        count_params.push(&tag_count);

        let mut stmt = conn.prepare(&count_query)?;
        let total: usize = stmt
            .query_row(rusqlite::params_from_iter(count_params.iter()), |row| {
                row.get(0)
            })?;

        // Build params for query
        let per_page_i64 = params.per_page as i64;
        let offset_i64 = offset as i64;
        let mut query_params: Vec<&dyn rusqlite::ToSql> = vec![search_query];
        for tag in &params.tags {
            query_params.push(tag);
        }
        query_params.push(&tag_count);
        query_params.push(&per_page_i64);
        query_params.push(&offset_i64);

        let mut stmt = conn.prepare(&query)?;
        let mut projects = stmt
            .query_map(rusqlite::params_from_iter(query_params.iter()), |row| {
                Ok(SearchResultProject {
                    project: Project {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        full_path: row.get(2)?,
                        parent_id: row.get(3)?,
                        is_leaf: row.get(4)?,
                        description: row.get(5)?,
                        folder_level: row.get(6)?,
                        created_at: row.get(7)?,
                        updated_at: row.get(8)?,
                    },
                    stl_count: row.get(9)?,
                    image_count: 0,
                    images: vec![],
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        // Update display names based on ignored keywords
        for p in &mut projects {
            p.project.name = self.resolve_display_name(&p.project);
        }

        Ok((projects, total))
    }

    fn search_all(
        &self,
        conn: &rusqlite::Connection,
        params: &SearchParams,
        offset: usize,
    ) -> Result<(Vec<SearchResultProject>, usize), AppError> {
        let leaf_filter = if params.leaf_only {
            "WHERE is_leaf = 1"
        } else {
            ""
        };

        let count_sql = format!("SELECT COUNT(*) FROM projects {}", leaf_filter);

        let total: usize = conn.query_row(&count_sql, [], |row| row.get(0))?;

        let sql = format!(
            "SELECT id, name, full_path, parent_id, is_leaf, description, folder_level, created_at, updated_at,
             (SELECT COUNT(*) FROM stl_files WHERE project_id = projects.id) as stl_count
             FROM projects
             {}
             ORDER BY name
             LIMIT ?1 OFFSET ?2",
            leaf_filter
        );

        let mut stmt = conn.prepare(&sql)?;

        let per_page_i64 = params.per_page as i64;
        let offset_i64 = offset as i64;

        let mut projects = stmt
            .query_map([per_page_i64, offset_i64], |row| {
                Ok(SearchResultProject {
                    project: Project {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        full_path: row.get(2)?,
                        parent_id: row.get(3)?,
                        is_leaf: row.get(4)?,
                        description: row.get(5)?,
                        folder_level: row.get(6)?,
                        created_at: row.get(7)?,
                        updated_at: row.get(8)?,
                    },
                    stl_count: row.get(9)?,
                    image_count: 0,
                    images: vec![],
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        // Update display names based on ignored keywords
        for p in &mut projects {
            p.project.name = self.resolve_display_name(&p.project);
        }

        Ok((projects, total))
    }
}
