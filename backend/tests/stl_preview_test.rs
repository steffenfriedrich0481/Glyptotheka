use glyptotheka_backend::db::connection::create_pool;
use glyptotheka_backend::services::image_cache::ImageCacheService;
use glyptotheka_backend::services::stl_preview::StlPreviewService;
use std::path::PathBuf;
use tempfile::TempDir;

#[tokio::test]
async fn test_stl_preview_generation() {
    // Setup temporary database and cache directory
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let cache_dir = temp_dir.path().join("cache");
    std::fs::create_dir_all(&cache_dir).unwrap();

    // Initialize database pool
    let pool = create_pool(db_path.to_str().unwrap()).unwrap();

    // Run migrations
    glyptotheka_backend::db::migrations::run_migrations(&pool).unwrap();

    // Create services
    let image_cache = ImageCacheService::new(cache_dir.clone(), pool.clone());
    let preview_service = StlPreviewService::new(image_cache, pool);

    // Find a small test STL file from the example directory
    let test_stl = find_test_stl_file();

    if test_stl.is_none() {
        println!("Skipping test: No STL files found in example directory");
        return;
    }

    let test_stl = test_stl.unwrap();
    println!("Testing with STL file: {}", test_stl);

    // Generate preview
    let result = preview_service.generate_preview(&test_stl).await;

    match result {
        Ok(preview_path) => {
            println!(
                "✓ Preview generated successfully: {}",
                preview_path.display()
            );

            // Verify preview file exists
            assert!(preview_path.exists(), "Preview file should exist");

            // Verify it's a PNG file (basic check)
            let metadata = std::fs::metadata(&preview_path).unwrap();
            assert!(metadata.len() > 0, "Preview file should not be empty");

            // Check that it's actually a PNG by reading the header
            let file_data = std::fs::read(&preview_path).unwrap();
            assert!(file_data.len() >= 8, "File should be at least 8 bytes");
            assert_eq!(
                &file_data[0..4],
                &[0x89, 0x50, 0x4E, 0x47],
                "Should be PNG format"
            );

            println!("✓ Preview validation passed");
            println!("  - File size: {} bytes", metadata.len());
            println!("  - Format: PNG");
        }
        Err(e) => {
            panic!("Preview generation failed: {}", e);
        }
    }
}

fn find_test_stl_file() -> Option<String> {
    // Try to find a small STL file in the example directory
    let example_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("example");

    if !example_dir.exists() {
        return None;
    }

    // Find first STL file (preferably small)
    walkdir::WalkDir::new(example_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("stl"))
                .unwrap_or(false)
        })
        .filter(|e| {
            // Try to find files smaller than 10MB for faster testing
            e.metadata().map(|m| m.len() < 10_000_000).unwrap_or(false)
        })
        .next()
        .and_then(|e| e.path().to_str().map(|s| s.to_string()))
}
