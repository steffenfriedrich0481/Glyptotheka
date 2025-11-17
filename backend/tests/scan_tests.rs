use glyptotheka_backend::config::Config;
use glyptotheka_backend::db::connection::init_pool;
use glyptotheka_backend::services::scanner::ScannerService;
use tempfile::TempDir;
use std::fs;

fn setup_test_env() -> (TempDir, Config) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    
    let config = Config {
        database_url: db_path.to_str().unwrap().to_string(),
        cache_dir: temp_dir.path().join("cache"),
        root_path: None,
        stl_thumb_path: None,
    };
    
    fs::create_dir_all(&config.cache_dir).unwrap();
    
    (temp_dir, config)
}

fn create_test_project(base_path: &std::path::Path, name: &str, stl_count: usize, img_count: usize) {
    let project_path = base_path.join(name);
    fs::create_dir_all(&project_path).unwrap();
    
    // Create STL files
    for i in 0..stl_count {
        let stl_path = project_path.join(format!("model{}.stl", i));
        fs::write(stl_path, b"FAKE STL DATA").unwrap();
    }
    
    // Create image files
    for i in 0..img_count {
        let img_path = project_path.join(format!("image{}.jpg", i));
        fs::write(img_path, b"FAKE IMAGE DATA").unwrap();
    }
}

#[test]
fn test_scan_empty_directory() {
    let (temp_dir, config) = setup_test_env();
    let pool = init_pool(&config.database_url).unwrap();
    let scanner = ScannerService::new(pool);
    
    let scan_path = temp_dir.path().join("empty");
    fs::create_dir_all(&scan_path).unwrap();
    
    let result = scanner.scan(scan_path.to_str().unwrap()).unwrap();
    
    assert_eq!(result.projects_found, 0);
    assert_eq!(result.files_processed, 0);
}

#[test]
fn test_scan_single_project() {
    let (temp_dir, config) = setup_test_env();
    let pool = init_pool(&config.database_url).unwrap();
    let scanner = ScannerService::new(pool);
    
    let scan_path = temp_dir.path().join("projects");
    fs::create_dir_all(&scan_path).unwrap();
    
    // Create a project with 2 STL files and 3 images
    create_test_project(&scan_path, "project1", 2, 3);
    
    let result = scanner.scan(scan_path.to_str().unwrap()).unwrap();
    
    assert_eq!(result.projects_found, 1);
    assert_eq!(result.files_processed, 2); // Only STL files are counted
}

#[test]
fn test_scan_nested_projects() {
    let (temp_dir, config) = setup_test_env();
    let pool = init_pool(&config.database_url).unwrap();
    let scanner = ScannerService::new(pool);
    
    let scan_path = temp_dir.path().join("projects");
    fs::create_dir_all(&scan_path).unwrap();
    
    // Create nested structure
    create_test_project(&scan_path, "parent", 1, 1);
    create_test_project(&scan_path.join("parent"), "child", 2, 2);
    
    let result = scanner.scan(scan_path.to_str().unwrap()).unwrap();
    
    assert!(result.projects_found >= 2); // At least parent and child
}

#[test]
fn test_scan_multiple_projects() {
    let (temp_dir, config) = setup_test_env();
    let pool = init_pool(&config.database_url).unwrap();
    let scanner = ScannerService::new(pool);
    
    let scan_path = temp_dir.path().join("projects");
    fs::create_dir_all(&scan_path).unwrap();
    
    // Create multiple independent projects
    create_test_project(&scan_path, "project1", 1, 1);
    create_test_project(&scan_path, "project2", 2, 2);
    create_test_project(&scan_path, "project3", 3, 3);
    
    let result = scanner.scan(scan_path.to_str().unwrap()).unwrap();
    
    assert_eq!(result.projects_found, 3);
    assert_eq!(result.files_processed, 6); // 1 + 2 + 3 STL files
}

#[test]
fn test_scan_invalid_path() {
    let (_temp_dir, config) = setup_test_env();
    let pool = init_pool(&config.database_url).unwrap();
    let scanner = ScannerService::new(pool);
    
    let result = scanner.scan("/nonexistent/path");
    
    assert!(result.is_err());
}

#[test]
fn test_rescan_updates_existing() {
    let (temp_dir, config) = setup_test_env();
    let pool = init_pool(&config.database_url).unwrap();
    let scanner = ScannerService::new(pool);
    
    let scan_path = temp_dir.path().join("projects");
    fs::create_dir_all(&scan_path).unwrap();
    
    // First scan
    create_test_project(&scan_path, "project1", 1, 1);
    let result1 = scanner.scan(scan_path.to_str().unwrap()).unwrap();
    
    // Add more files
    let project_path = scan_path.join("project1");
    fs::write(project_path.join("model2.stl"), b"FAKE STL DATA").unwrap();
    
    // Rescan
    let result2 = scanner.scan(scan_path.to_str().unwrap()).unwrap();
    
    assert_eq!(result1.projects_found, 1);
    assert_eq!(result2.projects_found, 1);
    assert_eq!(result2.files_processed, 2);
}
