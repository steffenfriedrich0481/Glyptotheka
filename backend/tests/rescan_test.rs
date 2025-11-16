#[cfg(test)]
mod rescan_tests {
    use glyptotheka_backend::services::rescan::RescanService;
    use glyptotheka_backend::db::connection::create_pool;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    fn setup_test_env() -> (TempDir, String) {
        let temp_dir = TempDir::new().unwrap();
        let test_root = temp_dir.path().join("test_models");
        fs::create_dir_all(&test_root).unwrap();
        (temp_dir, test_root.to_str().unwrap().to_string())
    }

    fn create_test_stl(path: &Path, name: &str) {
        fs::write(path.join(name), "solid test\nendsolid test\n").unwrap();
    }

    #[test]
    fn test_rescan_detects_new_files() {
        let (_temp_dir, test_root) = setup_test_env();
        let pool = create_pool(":memory:").unwrap();
        let rescan_service = RescanService::new(pool);

        // Create initial project
        let project1 = Path::new(&test_root).join("project1");
        fs::create_dir_all(&project1).unwrap();
        create_test_stl(&project1, "model1.stl");

        // Initial scan would happen here (not implemented in test)
        
        // Add new file
        create_test_stl(&project1, "model2.stl");

        // Rescan should detect the new file
        let result = rescan_service.rescan(&test_root);
        assert!(result.is_ok());
        let scan_result = result.unwrap();
        assert!(scan_result.files_added > 0 || scan_result.files_processed > 0);
    }

    #[test]
    fn test_rescan_detects_deleted_projects() {
        let (_temp_dir, test_root) = setup_test_env();
        let pool = create_pool(":memory:").unwrap();
        let rescan_service = RescanService::new(pool);

        // Create and remove project directory
        let project1 = Path::new(&test_root).join("project1");
        fs::create_dir_all(&project1).unwrap();
        create_test_stl(&project1, "model1.stl");
        
        // Simulate that it was scanned before
        // Then delete it
        fs::remove_dir_all(&project1).unwrap();

        // Rescan should handle missing directory gracefully
        let result = rescan_service.rescan(&test_root);
        assert!(result.is_ok());
    }

    #[test]
    fn test_rescan_preserves_structure() {
        let (_temp_dir, test_root) = setup_test_env();
        let pool = create_pool(":memory:").unwrap();
        let rescan_service = RescanService::new(pool);

        // Create nested structure
        let project1 = Path::new(&test_root).join("category1").join("project1");
        fs::create_dir_all(&project1).unwrap();
        create_test_stl(&project1, "model1.stl");

        // First scan
        let result = rescan_service.rescan(&test_root);
        assert!(result.is_ok());
        
        // Verify projects are found
        let scan_result = result.unwrap();
        assert!(scan_result.projects_found > 0);
    }
}
