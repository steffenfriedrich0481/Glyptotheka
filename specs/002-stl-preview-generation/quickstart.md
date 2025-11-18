# Quickstart Guide: STL Preview Image Generation

**Feature**: STL Preview Image Generation During Scanning  
**Date**: 2025-11-18  
**Phase**: 1 - Design & Contracts

## Overview

This quickstart guide provides developers with the essential information to implement and test the STL preview image generation feature.

---

## Prerequisites

### Required Knowledge

- Rust async programming (tokio runtime)
- SQLite database operations (rusqlite)
- File system operations
- OpenGL basics (for troubleshooting)

### Environment Setup

1. **Rust toolchain**: 1.75+ installed
2. **stl-thumb binary**: Available at `../stl-thumb` (already cloned)
3. **Database**: SQLite with migrations applied
4. **Cache directory**: `cache/stl-previews/` (auto-created)

### Dependencies

Already in `backend/Cargo.toml`:
```toml
stl-thumb = "0.5"
tokio = { version = "1.35", features = ["full"] }
rusqlite = { version = "0.31", features = ["bundled"] }
image = "0.24"  # For composite previews
walkdir = "2.4"
tracing = "0.1"
```

---

## 5-Minute Implementation Overview

### Step 1: Apply Database Migration

**File**: `backend/migrations/005_stl_preview_priority.sql`

```sql
-- Add image priority and source columns
ALTER TABLE image_files 
ADD COLUMN image_priority INTEGER NOT NULL DEFAULT 100;

ALTER TABLE image_files 
ADD COLUMN image_source TEXT NOT NULL DEFAULT 'regular' 
  CHECK (image_source IN ('regular', 'stl_preview', 'composite'));

-- Create index for priority-based queries
CREATE INDEX idx_image_files_priority 
  ON image_files(project_id, image_priority DESC, display_order ASC);

-- Update schema version
INSERT INTO schema_migrations (version, applied_at)
VALUES (5, strftime('%s', 'now'));
```

**Apply**:
```bash
cd backend
sqlite3 ../data/glyptotheka.db < migrations/005_stl_preview_priority.sql
```

---

### Step 2: Enhance StlPreviewService

**File**: `backend/src/services/stl_preview.rs`

**Add smart caching method**:

```rust
impl StlPreviewService {
    /// Generate preview with smart caching (NEW METHOD)
    pub async fn generate_preview_with_smart_cache(
        &self, 
        stl_path: &str
    ) -> Result<PreviewResult, AppError> {
        // 1. Check if preview exists and is valid
        if let Some(preview_path) = self.get_preview_path(stl_path)? {
            let stl_mtime = fs::metadata(stl_path)?.modified()?;
            let preview_generated_at = self.get_preview_timestamp(stl_path)?;
            
            if stl_mtime <= preview_generated_at {
                info!("Using cached STL preview for {}", stl_path);
                return Ok(PreviewResult {
                    preview_path,
                    was_cached: true,
                    generation_time_ms: 0,
                });
            }
            
            warn!("STL file modified, regenerating preview: {}", stl_path);
        }
        
        // 2. Check file size (skip if > 100MB)
        let file_size = fs::metadata(stl_path)?.len();
        if file_size > 100 * 1024 * 1024 {
            return Err(AppError::ValidationError(
                format!("STL file too large ({}MB)", file_size / 1024 / 1024)
            ));
        }
        
        // 3. Generate new preview
        let start = std::time::Instant::now();
        let preview_path = self.generate_preview(stl_path).await?;
        let generation_time_ms = start.elapsed().as_millis() as u64;
        
        Ok(PreviewResult {
            preview_path,
            was_cached: false,
            generation_time_ms,
        })
    }
    
    /// Get preview generation timestamp from database
    fn get_preview_timestamp(&self, stl_path: &str) -> Result<SystemTime, AppError> {
        let conn = self.pool.get()?;
        let timestamp: i64 = conn.query_row(
            "SELECT preview_generated_at FROM stl_files WHERE file_path = ?",
            params![stl_path],
            |row| row.get(0),
        )?;
        
        Ok(UNIX_EPOCH + Duration::from_secs(timestamp as u64))
    }
}

pub struct PreviewResult {
    pub preview_path: PathBuf,
    pub was_cached: bool,
    pub generation_time_ms: u64,
}
```

---

### Step 3: Integrate into ScannerService

**File**: `backend/src/services/scanner.rs`

**Add preview generation after STL file processing**:

```rust
impl ScannerService {
    pub fn scan_with_previews(
        &self,
        root_path: &str,
        preview_service: &StlPreviewService,
        preview_queue: &PreviewQueue,
    ) -> Result<ScanResult, AppError> {
        // ... existing scan logic ...
        
        // NEW: Generate STL previews for each project
        for (project_path, stl_files) in &project_folders {
            let project_id = path_to_id[project_path];
            
            // Split: first 2 sync, rest async
            let sync_count = stl_files.len().min(2);
            let (sync_files, async_files) = stl_files.split_at(sync_count);
            
            // Generate first 2 synchronously
            for stl_path in sync_files {
                match preview_service.generate_preview_with_smart_cache(
                    stl_path.to_str().unwrap()
                ).await {
                    Ok(result) => {
                        info!(
                            "Generated STL preview for {} in {}ms (cached: {})",
                            stl_path.display(),
                            result.generation_time_ms,
                            result.was_cached
                        );
                        
                        // Add to image_files table
                        self.add_stl_preview_to_db(
                            project_id, 
                            stl_path, 
                            &result.preview_path
                        )?;
                    }
                    Err(e) => {
                        warn!("Failed to generate STL preview for {}: {}", 
                              stl_path.display(), e);
                        errors.push(format!("STL preview failed: {}", stl_path.display()));
                    }
                }
            }
            
            // Queue remaining for async processing
            for stl_path in async_files {
                let stl_path_str = stl_path.to_str().unwrap().to_string();
                if let Err(e) = preview_queue.queue_preview(stl_path_str).await {
                    warn!("Failed to queue STL preview: {}", e);
                }
            }
        }
        
        // ... rest of scan logic ...
    }
    
    fn add_stl_preview_to_db(
        &self,
        project_id: i64,
        stl_path: &Path,
        preview_path: &Path,
    ) -> Result<(), AppError> {
        let conn = self.file_repo.pool.get()?;
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
        
        conn.execute(
            "INSERT INTO image_files (
                project_id, filename, file_path, file_size,
                source_type, image_priority, image_source,
                display_order, created_at, updated_at
            ) VALUES (?, ?, ?, ?, 'direct', 50, 'stl_preview', 0, ?, ?)
            ON CONFLICT(file_path) DO UPDATE SET updated_at = ?",
            params![
                project_id,
                preview_path.file_name().unwrap().to_str().unwrap(),
                preview_path.to_str().unwrap(),
                fs::metadata(preview_path)?.len(),
                now, now, now
            ],
        )?;
        
        Ok(())
    }
}
```

---

### Step 4: Update Image Queries

**File**: `backend/src/db/repositories/file_repo.rs`

**Modify image retrieval to sort by priority**:

```rust
impl FileRepository {
    pub fn get_images_by_priority(
        &self,
        project_id: i64,
    ) -> Result<Vec<ImageFile>, AppError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, project_id, filename, file_path, file_size,
                    source_type, source_project_id, display_order,
                    image_priority, image_source, created_at, updated_at
             FROM image_files
             WHERE project_id = ?
             ORDER BY image_priority DESC, display_order ASC, created_at ASC"
        )?;
        
        let images = stmt.query_map(params![project_id], |row| {
            Ok(ImageFile {
                id: row.get(0)?,
                project_id: row.get(1)?,
                filename: row.get(2)?,
                file_path: row.get(3)?,
                file_size: row.get(4)?,
                source_type: row.get(5)?,
                source_project_id: row.get(6)?,
                display_order: row.get(7)?,
                image_priority: row.get(8)?,
                image_source: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(images)
    }
}
```

---

### Step 5: Wire Up Dependencies

**File**: `backend/src/main.rs` (or wherever services are initialized)

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ... existing setup ...
    
    // Create services
    let image_cache = ImageCacheService::new(cache_dir.clone());
    let stl_preview_service = StlPreviewService::new(image_cache, pool.clone());
    let preview_queue = PreviewQueue::new(stl_preview_service.clone(), 100);
    
    let scanner = ScannerService::new(pool.clone());
    
    // Pass preview services to scan operations
    let scan_result = scanner.scan_with_previews(
        &root_path,
        &stl_preview_service,
        &preview_queue,
    )?;
    
    // ... rest of application ...
}
```

---

## Testing Quickstart

### Unit Test Example

**File**: `backend/src/services/stl_preview_test.rs`

```rust
#[tokio::test]
async fn test_smart_caching() {
    let service = setup_test_service().await;
    let test_stl = "tests/fixtures/test_model.stl";
    
    // First generation
    let result1 = service.generate_preview_with_smart_cache(test_stl).await.unwrap();
    assert!(!result1.was_cached, "First generation should not be cached");
    
    // Second generation (should use cache)
    let result2 = service.generate_preview_with_smart_cache(test_stl).await.unwrap();
    assert!(result2.was_cached, "Second generation should use cache");
    assert_eq!(result2.generation_time_ms, 0, "Cached generation should be instant");
    
    // Modify file
    touch_file(test_stl);
    
    // Third generation (cache miss due to modification)
    let result3 = service.generate_preview_with_smart_cache(test_stl).await.unwrap();
    assert!(!result3.was_cached, "Modified file should not use cache");
}
```

### Integration Test Example

**File**: `backend/tests/integration/scan_with_previews.rs`

```rust
#[tokio::test]
async fn test_scan_generates_stl_previews() {
    let test_dir = create_test_project_with_stls();
    let service = setup_scanner_service().await;
    let preview_service = setup_preview_service().await;
    let preview_queue = PreviewQueue::new(preview_service.clone(), 100);
    
    // Scan project
    let result = service.scan_with_previews(
        &test_dir,
        &preview_service,
        &preview_queue,
    ).await.unwrap();
    
    assert!(result.projects_found > 0);
    assert!(result.files_processed > 0);
    
    // Verify previews created
    let images = get_images_for_test_project().await.unwrap();
    let stl_previews: Vec<_> = images.iter()
        .filter(|img| img.image_source == "stl_preview")
        .collect();
    
    assert!(stl_previews.len() >= 2, "At least 2 STL previews should be generated");
    
    // Verify priority sorting
    let first_regular = images.iter().find(|img| img.image_source == "regular");
    let first_stl = images.iter().find(|img| img.image_source == "stl_preview");
    
    if let (Some(regular), Some(stl)) = (first_regular, first_stl) {
        assert!(regular.image_priority > stl.image_priority);
    }
}
```

---

## Manual Testing

### Test Case 1: Basic Preview Generation

```bash
# 1. Create test project
mkdir -p /tmp/test_project
cp /path/to/test.stl /tmp/test_project/

# 2. Run scan
curl -X POST http://localhost:8080/api/scan \
  -H "Content-Type: application/json" \
  -d '{"root_path": "/tmp/test_project"}'

# 3. Check images
curl http://localhost:8080/api/projects/1/images

# Expected: JSON response with STL preview image
```

### Test Case 2: Smart Caching

```bash
# 1. Initial scan
curl -X POST http://localhost:8080/api/scan \
  -d '{"root_path": "/tmp/test_project"}'

# 2. Check logs (should show "Generated STL preview")

# 3. Rescan without changes
curl -X POST http://localhost:8080/api/rescan \
  -d '{"root_path": "/tmp/test_project"}'

# 4. Check logs (should show "Using cached STL preview")

# 5. Modify STL file
touch /tmp/test_project/test.stl

# 6. Rescan again
curl -X POST http://localhost:8080/api/rescan \
  -d '{"root_path": "/tmp/test_project"}'

# 7. Check logs (should show "STL file modified, regenerating preview")
```

### Test Case 3: Error Handling

```bash
# 1. Create corrupted STL file
echo "not a valid stl" > /tmp/test_project/corrupted.stl

# 2. Scan
curl -X POST http://localhost:8080/api/scan \
  -d '{"root_path": "/tmp/test_project"}'

# 3. Check logs (should show warning, not error)
# 4. Verify scan completed successfully despite STL error
```

---

## Troubleshooting

### Issue: STL Previews Not Generating

**Check**:
1. Is stl-thumb library available? `ls ../stl-thumb/target/release/stl-thumb`
2. Are STL files valid? Try opening in 3D viewer
3. Check logs for error messages
4. Verify cache directory writable: `ls -la cache/stl-previews/`

**Solution**: Ensure stl-thumb binary exists and is executable

---

### Issue: Slow Scan Times

**Check**:
1. How many STL files in project? (>2 should be async)
2. Check STL file sizes (>100MB skipped)
3. Monitor memory usage during generation

**Solution**: 
- First 2 STL files are intentionally synchronous
- Remaining files process in background
- Large projects may take time initially, but rescans are fast

---

### Issue: Previews Not Showing in Gallery

**Check**:
1. Query database: `SELECT * FROM image_files WHERE image_source = 'stl_preview'`
2. Verify priority column exists: `PRAGMA table_info(image_files)`
3. Check image retrieval query sorts by priority

**Solution**: Ensure migration 005 applied and queries use priority sorting

---

## Performance Benchmarks

### Expected Performance

| Operation | Expected Time | Notes |
|-----------|---------------|-------|
| Single STL preview (10MB file) | 3-5 seconds | Depends on model complexity |
| Single STL preview (50MB file) | 10-20 seconds | Complex models take longer |
| Scan with 2 STL files | +6-10 seconds | Synchronous generation |
| Scan with 10 STL files | +6-10 seconds | Only first 2 sync |
| Rescan (no changes) | +0-1 seconds | Smart caching |
| Rescan (1 file changed) | +3-5 seconds | Only changed file regenerated |

### Memory Usage

- Per preview generation: 50-250MB (OpenGL rendering)
- Background worker: 50-250MB (one at a time)
- Total additional: < 500MB

---

## Next Steps

After implementing the quickstart:

1. **Run tests**: `cargo test --package backend`
2. **Manual testing**: Follow test cases above
3. **Code review**: Check contracts compliance
4. **Documentation**: Update API docs if needed
5. **Proceed to Phase 2**: Create detailed task breakdown

---

## Resources

- **Spec**: `specs/002-stl-preview-generation/spec.md`
- **Research**: `specs/002-stl-preview-generation/research.md`
- **Data Model**: `specs/002-stl-preview-generation/data-model.md`
- **API Contracts**: `specs/002-stl-preview-generation/contracts/api-contracts.md`
- **Service Contracts**: `specs/002-stl-preview-generation/contracts/service-contracts.md`

---

## Summary

This quickstart covered:

1. ✅ Database migration (5 minutes)
2. ✅ Service enhancement (10 minutes)
3. ✅ Scanner integration (15 minutes)
4. ✅ Query updates (5 minutes)
5. ✅ Dependency wiring (5 minutes)

**Total implementation time**: ~40 minutes of focused coding

**Testing time**: ~20 minutes for basic verification

The implementation leverages existing infrastructure and requires minimal new code. Focus on smart caching logic and error handling to ensure reliability.
