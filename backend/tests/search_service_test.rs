use glyptotheka_backend::config::Config;
use glyptotheka_backend::db::connection::create_pool;
use glyptotheka_backend::db::migrations::run_migrations;
use glyptotheka_backend::services::search::{SearchParams, SearchService};
use std::fs;
use tempfile::TempDir;

fn setup_test_env() -> (TempDir, SearchService) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let cache_dir = temp_dir.path().join("cache");

    let config = Config {
        database_path: db_path.to_str().unwrap().to_string(),
        cache_dir: cache_dir.to_str().unwrap().to_string(),
    };

    fs::create_dir_all(&cache_dir).unwrap();
    let pool = create_pool(&config.database_path).unwrap();
    run_migrations(&pool).unwrap();

    // Load fixture
    let conn = pool.get().unwrap();
    let fixture = include_str!("fixtures/hierarchical_projects.sql");
    conn.execute_batch(fixture).unwrap();

    let service = SearchService::new(pool);
    (temp_dir, service)
}

#[test]
fn test_search_fts_leaf_only() {
    let (_temp_dir, service) = setup_test_env();

    // Search for "Car" - matches "Cars" (container) and "Sports Car" (leaf)
    let params = SearchParams {
        query: Some("Car".to_string()),
        tags: vec![],
        page: 1,
        per_page: 10,
        leaf_only: true,
    };

    let result = service.search(&params).unwrap();

    // Should only find "Sports Car"
    assert_eq!(result.total, 1);
    assert_eq!(result.projects[0].project.name, "Sports Car");
    assert!(result.projects[0].project.is_leaf);
}

#[test]
fn test_search_fts_all() {
    let (_temp_dir, service) = setup_test_env();

    // Search for "Car" - matches "Cars" (container) and "Sports Car" (leaf)
    let params = SearchParams {
        query: Some("Car".to_string()),
        tags: vec![],
        page: 1,
        per_page: 10,
        leaf_only: false,
    };

    let result = service.search(&params).unwrap();

    // Should find "Cars" and "Sports Car"
    assert_eq!(result.total, 2);
}

#[test]
fn test_search_all_leaf_only() {
    let (_temp_dir, service) = setup_test_env();

    let params = SearchParams {
        query: None,
        tags: vec![],
        page: 1,
        per_page: 10,
        leaf_only: true,
    };

    let result = service.search(&params).unwrap();

    // Should find "Sports Car" and "Truck"
    assert_eq!(result.total, 2);
    for p in result.projects {
        assert!(p.project.is_leaf);
    }
}
