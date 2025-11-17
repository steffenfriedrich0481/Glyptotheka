# Data Model & Service Architecture: STL Preview Integration

**Feature**: Integrate stl-thumb as Rust Library  
**Date**: 2025-11-17  
**Phase**: Design (Phase 1)

---

## Database Schema Changes

### Current Schema (Affected Tables)

#### config table
```sql
CREATE TABLE config (
    id INTEGER PRIMARY KEY,
    root_path TEXT,
    last_scan_at INTEGER,
    stl_thumb_path TEXT,              -- ❌ TO BE REMOVED
    cache_max_size_mb INTEGER NOT NULL DEFAULT 1000,
    images_per_page INTEGER NOT NULL DEFAULT 50,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
```

#### stl_files table (NO CHANGES)
```sql
CREATE TABLE stl_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    file_path TEXT UNIQUE NOT NULL,
    filename TEXT NOT NULL,
    size_bytes INTEGER NOT NULL,
    preview_path TEXT,                -- ✅ UNCHANGED - cache path
    preview_generated_at INTEGER,     -- ✅ UNCHANGED - timestamp
    created_at INTEGER NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);
```

#### cached_files table (NO CHANGES)
```sql
CREATE TABLE cached_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_path TEXT UNIQUE NOT NULL,
    cache_path TEXT NOT NULL,
    cache_type TEXT NOT NULL CHECK(cache_type IN ('image', 'preview')),
    size_bytes INTEGER NOT NULL,
    last_accessed INTEGER NOT NULL,
    created_at INTEGER NOT NULL
);
```

### Migration Required

#### Migration: Remove stl_thumb_path

**File**: `backend/src/db/migrations.rs` (or new migration file)

**SQLite Note**: SQLite doesn't support `ALTER TABLE DROP COLUMN` directly before version 3.35.0. Must recreate table.

**Migration Up**:
```sql
-- Migration: 004_remove_stl_thumb_path.sql
-- Remove stl_thumb_path from config table

BEGIN TRANSACTION;

-- Create new table without stl_thumb_path
CREATE TABLE config_new (
    id INTEGER PRIMARY KEY,
    root_path TEXT,
    last_scan_at INTEGER,
    cache_max_size_mb INTEGER NOT NULL DEFAULT 1000,
    images_per_page INTEGER NOT NULL DEFAULT 50,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Copy data (excluding stl_thumb_path)
INSERT INTO config_new (id, root_path, last_scan_at, cache_max_size_mb, 
                        images_per_page, created_at, updated_at)
SELECT id, root_path, last_scan_at, cache_max_size_mb, 
       images_per_page, created_at, updated_at
FROM config;

-- Drop old table
DROP TABLE config;

-- Rename new table
ALTER TABLE config_new RENAME TO config;

COMMIT;
```

**Migration Down** (for rollback):
```sql
-- Restore stl_thumb_path column with NULL default
BEGIN TRANSACTION;

CREATE TABLE config_new (
    id INTEGER PRIMARY KEY,
    root_path TEXT,
    last_scan_at INTEGER,
    stl_thumb_path TEXT,
    cache_max_size_mb INTEGER NOT NULL DEFAULT 1000,
    images_per_page INTEGER NOT NULL DEFAULT 50,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

INSERT INTO config_new (id, root_path, last_scan_at, stl_thumb_path, 
                        cache_max_size_mb, images_per_page, created_at, updated_at)
SELECT id, root_path, last_scan_at, NULL as stl_thumb_path,
       cache_max_size_mb, images_per_page, created_at, updated_at
FROM config;

DROP TABLE config;
ALTER TABLE config_new RENAME TO config;

COMMIT;
```

**Impact**: 
- Existing deployments will lose `stl_thumb_path` configuration
- **Acceptable** - this is the intended change (removing external dependency)
- No data loss for preview-related fields

---

## Data Model Changes

### Configuration Models

#### Config Struct (backend/src/config.rs)

**Current**:
```rust
#[derive(Debug, Clone)]
pub struct Config {
    pub database_path: String,
    pub cache_dir: String,
    pub stl_thumb_path: Option<String>,  // ❌ REMOVE
}
```

**New**:
```rust
#[derive(Debug, Clone)]
pub struct Config {
    pub database_path: String,
    pub cache_dir: String,
    // stl_thumb_path removed - no longer needed
}
```

**Default Implementation**:
```rust
impl Default for Config {
    fn default() -> Self {
        Self {
            database_path: "glyptotheka.db".to_string(),
            cache_dir: "cache".to_string(),
            // No stl_thumb_path
        }
    }
}
```

#### AppConfig Struct (backend/src/config.rs)

**Current**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub id: i64,
    pub root_path: Option<String>,
    pub last_scan_at: Option<i64>,
    pub stl_thumb_path: Option<String>,  // ❌ REMOVE
    pub cache_max_size_mb: i64,
    pub images_per_page: i64,
    pub created_at: i64,
    pub updated_at: i64,
}
```

**New**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub id: i64,
    pub root_path: Option<String>,
    pub last_scan_at: Option<i64>,
    // stl_thumb_path removed
    pub cache_max_size_mb: i64,
    pub images_per_page: i64,
    pub created_at: i64,
    pub updated_at: i64,
}
```

#### UpdateConfigRequest Struct

**Current**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfigRequest {
    pub root_path: Option<String>,
    pub stl_thumb_path: Option<String>,  // ❌ REMOVE
    pub cache_max_size_mb: Option<i64>,
    pub images_per_page: Option<i64>,
}
```

**New**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfigRequest {
    pub root_path: Option<String>,
    // stl_thumb_path removed
    pub cache_max_size_mb: Option<i64>,
    pub images_per_page: Option<i64>,
}
```

### No Changes Required

- ✅ `stl_files` table schema unchanged
- ✅ `cached_files` table schema unchanged
- ✅ Preview cache structure unchanged (PNG files in cache/previews/)
- ✅ Preview generation results identical (512x512 PNG)

---

## Service Architecture

### StlPreviewService Refactoring

#### Current Implementation

**File**: `backend/src/services/stl_preview.rs`

**Current Structure**:
```rust
#[derive(Clone)]
pub struct StlPreviewService {
    stl_thumb_path: Option<PathBuf>,    // ❌ REMOVE
    image_cache: ImageCacheService,      // ✅ KEEP
    pool: DbPool,                        // ✅ KEEP
}

impl StlPreviewService {
    pub fn new(
        stl_thumb_path: Option<PathBuf>, // ❌ REMOVE PARAMETER
        image_cache: ImageCacheService,
        pool: DbPool
    ) -> Self { ... }
    
    async fn run_stl_thumb(&self, stl_path: &Path, stl_thumb_path: &Path) 
        -> Result<Vec<u8>, AppError> 
    {
        // ❌ REMOVE - subprocess execution
        Command::new(stl_thumb_path)
            .arg(stl_path)
            .arg("-")
            .arg("-s").arg("512")
            .output()?
    }
}
```

#### New Implementation

**File**: `backend/src/services/stl_preview.rs`

**New Structure**:
```rust
use stl_thumb::{Config as StlConfig}; // ✅ ADD

#[derive(Clone)]
pub struct StlPreviewService {
    // stl_thumb_path removed - not needed
    image_cache: ImageCacheService,
    pool: DbPool,
}

impl StlPreviewService {
    pub fn new(
        image_cache: ImageCacheService,
        pool: DbPool
    ) -> Self {
        Self {
            image_cache,
            pool,
        }
    }
    
    /// Generate a preview for an STL file
    pub async fn generate_preview(&self, stl_path: &str) -> Result<PathBuf, AppError> {
        // Check cache first (unchanged)
        if let Some(cached_path) = self.image_cache.get_cached_preview(stl_path)? {
            info!("Using cached preview for {}", stl_path);
            return Ok(cached_path);
        }
        
        let stl_path_buf = PathBuf::from(stl_path);
        if !stl_path_buf.exists() {
            return Err(AppError::NotFound(format!("STL file not found: {}", stl_path)));
        }
        
        // Generate preview using library (NEW)
        let preview_path = self.render_stl_preview(&stl_path_buf).await?;
        
        // Update database (unchanged)
        self.update_stl_preview_info(stl_path, preview_path.to_str().unwrap())?;
        
        info!("Generated preview for {}", stl_path);
        Ok(preview_path)
    }
    
    /// Render STL file to PNG using stl-thumb library
    async fn render_stl_preview(&self, stl_path: &Path) -> Result<PathBuf, AppError> {
        let stl_path = stl_path.to_path_buf();
        
        // Generate output path from cache service
        let cache_path = self.image_cache.generate_preview_path(&stl_path)?;
        let output_path = cache_path.clone();
        
        // Render in blocking thread (CPU-bound OpenGL work)
        task::spawn_blocking(move || {
            let config = StlConfig {
                size: 512,
                ..Default::default()
            };
            
            stl_thumb::render_to_file(&stl_path, &config, &output_path)
                .map_err(|e| AppError::InternalServer(
                    format!("STL rendering failed: {}", e)
                ))?;
            
            Ok(output_path)
        })
        .await
        .map_err(|e| AppError::InternalServer(format!("Task join error: {}", e)))?
    }
    
    // update_stl_preview_info, has_preview, get_preview unchanged
}
```

**Key Changes**:
- ❌ Remove `stl_thumb_path` field
- ❌ Remove `run_stl_thumb()` subprocess function
- ✅ Add `render_stl_preview()` library function
- ✅ Keep `spawn_blocking` pattern (CPU-bound work)
- ✅ Improve error messages (no stderr parsing)
- ✅ Maintain identical public API

### API State Initialization

#### Current (backend/src/api/routes.rs)

```rust
pub struct AppState {
    pub pool: DbPool,
    pub config: Arc<Config>,
    pub preview_service: StlPreviewService,
    pub scanner_service: ScannerService,
    // ...
}

// Initialization
let preview_service = StlPreviewService::new(
    config.stl_thumb_path.as_ref().map(PathBuf::from),  // ❌ REMOVE
    image_cache.clone(),
    pool.clone()
);
```

#### New (backend/src/api/routes.rs)

```rust
pub struct AppState {
    pub pool: DbPool,
    pub config: Arc<Config>,
    pub preview_service: StlPreviewService,
    pub scanner_service: ScannerService,
    // ... unchanged
}

// Initialization
let preview_service = StlPreviewService::new(
    image_cache.clone(),
    pool.clone()
);
```

---

## Configuration Service Changes

**File**: `backend/src/config.rs` - `ConfigService`

### Current Methods Affected

```rust
pub fn get_config(&self) -> Result<AppConfig, AppError> {
    // Query needs to remove stl_thumb_path from SELECT
    let mut stmt = conn.prepare(
        "SELECT id, root_path, last_scan_at, 
         cache_max_size_mb, images_per_page, created_at, updated_at  -- removed stl_thumb_path
         FROM config WHERE id = 1"
    )?;
    
    let config = stmt.query_row([], |row| {
        Ok(AppConfig {
            id: row.get(0)?,
            root_path: row.get(1)?,
            last_scan_at: row.get(2)?,
            // skip stl_thumb_path
            cache_max_size_mb: row.get(3)?,  // index shifted
            images_per_page: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    })?;
}

pub fn update_config(&self, updates: &UpdateConfigRequest) -> Result<AppConfig, AppError> {
    // Remove entire stl_thumb_path update block
    // if let Some(ref stl_thumb_path) = updates.stl_thumb_path { ... }  -- DELETE
    
    // Keep root_path, cache_max_size_mb, images_per_page updates
}
```

---

## Error Handling Architecture

### Current Error Flow

```
User uploads STL
  → API endpoint
  → StlPreviewService::generate_preview()
  → run_stl_thumb() subprocess
  → Command::new(stl_thumb_path)
    → Process spawn fails → Err("Failed to run stl-thumb: {io_error}")
    → Process exits non-zero → Err("stl-thumb failed: {stderr}")
  → Parse stderr string for error details
  → Map to AppError::InternalServer
```

### New Error Flow

```
User uploads STL
  → API endpoint
  → StlPreviewService::generate_preview()
  → render_stl_preview() library call
  → spawn_blocking
    → stl_thumb::render_to_file()
      → Library error → Err(stl_thumb::Error)
  → Map directly to AppError::InternalServer with error Display impl
  → Return clear error message to client
```

### Error Types

#### Current Errors
- Subprocess spawn failures (OS/process issues)
- Non-zero exit codes (parsing stderr)
- Missing binary (file not found)
- Configuration errors (no stl_thumb_path set)

#### New Errors
- File I/O errors (can't read STL file)
- Parse errors (invalid STL format)
- Rendering errors (OpenGL issues, memory)
- Library errors (direct error types from stl-thumb)

### Error Message Improvements

**Before**:
```
"stl-thumb failed: ERROR: Failed to load mesh from /path/to/file.stl\n"
```

**After**:
```
"STL rendering failed: Failed to load mesh from /path/to/file.stl: Invalid STL format"
```

**Benefits**:
- No stderr parsing
- Structured error types
- Better error context
- Rust error chaining

---

## Cache Compatibility

### Preview Cache Format

**✅ NO CHANGES REQUIRED**

Current cache structure:
```
cache/
└── previews/
    ├── <hash1>.png
    ├── <hash2>.png
    └── ...
```

Library output:
- Same PNG format
- Same 512x512 resolution
- Same file-based caching
- Same hash-based filenames

**Migration**: None needed - existing cached previews remain valid

### Cache Service Integration

**File**: `backend/src/services/image_cache.rs`

**✅ NO CHANGES** - ImageCacheService API unchanged:
- `get_cached_preview(stl_path)` - check cache
- `cache_preview(stl_path, data)` - store preview (if using byte buffer)
- `generate_preview_path(stl_path)` - compute cache path

The service doesn't care whether preview came from subprocess or library.

---

## Performance Considerations

### Expected Improvements

1. **No subprocess spawn overhead**: ~50-100ms per preview eliminated
2. **Direct memory access**: No IPC between processes
3. **In-process rendering**: Shared memory for image data

### Thread Pool Usage

- **Rendering is CPU-bound**: 100% CPU usage during rendering
- **Use spawn_blocking**: Offload to blocking thread pool
- **Default pool size**: 512 threads (tokio default)
- **Typical render time**: 2-5 seconds per STL
- **Concurrent capacity**: Limited by PreviewQueue (existing mechanism)

### Benchmarking Plan

Test with 50 diverse STL files:
- Small files (<1MB)
- Medium files (1-10MB)
- Large files (>10MB)
- ASCII vs binary format
- Simple vs complex geometry

Measure:
- Total generation time
- Per-file time distribution
- Memory usage
- CPU utilization

**Success**: Within 10% of current performance (likely faster)

---

## Testing Strategy

### Unit Tests

1. **Test preview generation with library**
   ```rust
   #[tokio::test]
   async fn test_generate_preview_library() {
       let service = StlPreviewService::new(cache, pool);
       let preview = service.generate_preview("test.stl").await?;
       assert!(preview.exists());
       assert_eq!(image_size(&preview)?, (512, 512));
   }
   ```

2. **Test error handling**
   ```rust
   #[tokio::test]
   async fn test_invalid_stl_error() {
       let service = StlPreviewService::new(cache, pool);
       let result = service.generate_preview("invalid.stl").await;
       assert!(result.is_err());
   }
   ```

### Integration Tests

1. **End-to-end preview generation**
2. **Cache hit/miss scenarios**
3. **Concurrent preview generation**
4. **Various STL file formats**

### Compatibility Tests

1. **Existing cache compatibility**: Old previews still load
2. **Database migration**: Schema update succeeds
3. **API response format**: Config endpoint matches expectations

---

## Summary of Changes

### Files Modified

1. ✅ **backend/Cargo.toml** - Add stl-thumb dependency
2. ✅ **backend/src/services/stl_preview.rs** - Replace subprocess with library
3. ✅ **backend/src/config.rs** - Remove stl_thumb_path fields
4. ✅ **backend/src/api/routes.rs** - Update AppState initialization
5. ✅ **backend/src/db/migrations.rs** - Add migration to drop column

### Files Not Modified

- ❌ `backend/src/services/image_cache.rs` - No changes
- ❌ `backend/src/models/*.rs` - No model changes
- ❌ `frontend/**/*` - Frontend unchanged
- ❌ Any API endpoint logic (except config response)

### Configuration Files Modified

1. ✅ **.env.example** - Remove STL_THUMB_PATH
2. ✅ **docker-compose.yml** - Remove STL_THUMB_PATH env var
3. ✅ **backend/Dockerfile** - Add OpenGL libraries
4. ✅ **README.md** - Update installation instructions

---

## Next Steps

1. ✅ **Phase 0 Complete**: Research findings documented
2. ✅ **Phase 1 Complete**: Data model and architecture designed
3. ➡️ **Create contracts**: Document API changes
4. ➡️ **Create quickstart**: Update deployment guide
5. ➡️ **Update agent context**: Document design decisions
6. ➡️ **Proceed to Phase 2**: Generate implementation tasks
