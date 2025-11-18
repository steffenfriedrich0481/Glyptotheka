# Service Contracts: STL Preview Image Generation

**Feature**: STL Preview Image Generation During Scanning  
**Date**: 2025-11-18  
**Phase**: 1 - Design & Contracts

## Overview

This document defines the internal service contracts and interfaces for the STL preview generation feature. These contracts govern how services interact with each other.

---

## Service Architecture

```
┌─────────────────┐
│  Scanner/Rescan │  (Entry point: scan operations)
│    Services     │
└────────┬────────┘
         │ uses
         ↓
┌─────────────────┐
│  StlPreview     │  (Core: preview generation)
│    Service      │
└────────┬────────┘
         │ uses
         ↓
┌─────────────────┐     ┌─────────────────┐
│  ImageCache     │     │  PreviewQueue   │  (Async: background processing)
│    Service      │     │                 │
└─────────────────┘     └─────────────────┘
         │                       │
         └───────────┬───────────┘
                     │ updates
                     ↓
              ┌─────────────┐
              │  FileRepo   │  (Data: database access)
              │             │
              └─────────────┘
```

---

## StlPreviewService Contract

### Interface Definition

```rust
pub struct StlPreviewService {
    image_cache: ImageCacheService,
    pool: DbPool,
}

impl StlPreviewService {
    /// Create new service instance
    pub fn new(image_cache: ImageCacheService, pool: DbPool) -> Self;
    
    /// Generate preview with smart caching (primary method)
    pub async fn generate_preview_with_smart_cache(
        &self, 
        stl_path: &str
    ) -> Result<PreviewResult, AppError>;
    
    /// Check if preview is valid (exists and up-to-date)
    pub async fn is_preview_valid(
        &self, 
        stl_path: &str
    ) -> Result<bool, AppError>;
    
    /// Force regeneration (ignore cache)
    pub async fn regenerate_preview(
        &self, 
        stl_path: &str
    ) -> Result<PathBuf, AppError>;
    
    /// Get existing preview path (if valid)
    pub fn get_preview_path(
        &self, 
        stl_path: &str
    ) -> Result<Option<PathBuf>, AppError>;
}

/// Result of preview generation
pub struct PreviewResult {
    pub preview_path: PathBuf,
    pub was_cached: bool,
    pub generation_time_ms: u64,
}
```

### Method Specifications

#### `generate_preview_with_smart_cache`

**Purpose**: Generate or retrieve STL preview image with smart caching logic.

**Algorithm**:
```
1. Check if preview exists in cache
2. If exists:
   a. Get STL file modification time (mtime)
   b. Get preview generation timestamp from DB
   c. If stl_mtime <= preview_generated_at:
      - Return cached preview (CACHE HIT)
   d. Else:
      - Continue to step 3 (CACHE MISS - stale)
3. If not exists or stale:
   a. Check STL file size (skip if > 100MB)
   b. Render STL to PNG using stl-thumb library
   c. Save PNG to cache directory
   d. Update stl_files table with preview_path and preview_generated_at
   e. Insert into image_files table with priority=50, source='stl_preview'
   f. Return new preview path
```

**Parameters**:
- `stl_path: &str` - Full path to STL file

**Returns**:
- `Ok(PreviewResult)` - Preview generated or retrieved from cache
- `Err(AppError::NotFound)` - STL file not found
- `Err(AppError::ValidationError)` - STL file too large (>100MB)
- `Err(AppError::InternalServer)` - Rendering failed

**Side Effects**:
- Writes PNG file to cache directory
- Updates `stl_files` table
- Inserts/updates `image_files` table
- Inserts/updates `cached_files` table

**Performance**:
- Cache hit: < 10ms (file system lookup only)
- Cache miss: 3-30 seconds (depends on STL complexity)
- Timeout: 30 seconds (cancels generation, returns error)

---

#### `is_preview_valid`

**Purpose**: Check if a valid, up-to-date preview exists without generating one.

**Algorithm**:
```
1. Query stl_files table for preview_path and preview_generated_at
2. If no preview_path: return false
3. Check if cache file exists on filesystem
4. If not exists: return false
5. Get STL file modification time
6. Compare: stl_mtime <= preview_generated_at
7. Return comparison result
```

**Parameters**:
- `stl_path: &str` - Full path to STL file

**Returns**:
- `Ok(true)` - Valid preview exists
- `Ok(false)` - No preview or preview is stale
- `Err(AppError::NotFound)` - STL file not found

**Side Effects**: None (read-only)

**Performance**: < 5ms (database + filesystem checks)

---

#### `regenerate_preview`

**Purpose**: Force preview regeneration, ignoring cache.

**Use Case**: Manual refresh or error recovery.

**Algorithm**:
```
1. Delete existing preview from cache (if exists)
2. Delete image_files entry (if exists)
3. Call generate_preview_with_smart_cache
   (which will see no cache and regenerate)
```

**Parameters**:
- `stl_path: &str` - Full path to STL file

**Returns**:
- `Ok(PathBuf)` - New preview path
- `Err(AppError)` - Generation failed

**Side Effects**: Same as `generate_preview_with_smart_cache` plus deletion of old preview

**Performance**: 3-30 seconds (full regeneration)

---

## PreviewQueue Contract

### Interface Definition

```rust
pub struct PreviewQueue {
    sender: mpsc::Sender<String>,
}

impl PreviewQueue {
    /// Create new queue with background worker
    pub fn new(
        preview_service: StlPreviewService, 
        queue_size: usize
    ) -> Self;
    
    /// Queue STL file for background preview generation
    pub async fn queue_preview(
        &self, 
        stl_path: String
    ) -> Result<(), AppError>;
    
    /// Get queue depth (for monitoring)
    pub fn queue_depth(&self) -> usize;
}
```

### Method Specifications

#### `new`

**Purpose**: Create preview queue and spawn background worker.

**Algorithm**:
```
1. Create bounded mpsc channel with specified capacity
2. Spawn background tokio task:
   While channel open:
     a. Receive stl_path from channel
     b. Call preview_service.generate_preview_with_smart_cache(stl_path)
     c. Log result (success or warning)
3. Return PreviewQueue with sender
```

**Parameters**:
- `preview_service: StlPreviewService` - Service for generating previews
- `queue_size: usize` - Maximum queued items (recommended: 100)

**Returns**: `PreviewQueue` instance

**Side Effects**: Spawns background task that runs until channel closed

**Lifecycle**: Background worker runs for application lifetime or until queue dropped

---

#### `queue_preview`

**Purpose**: Add STL file to background processing queue.

**Algorithm**:
```
1. Send stl_path through mpsc channel
2. If channel full, wait until space available
3. If channel closed, return error
```

**Parameters**:
- `stl_path: String` - Full path to STL file

**Returns**:
- `Ok(())` - Queued successfully
- `Err(AppError::InternalServer)` - Queue closed or send failed

**Side Effects**: None immediately (background worker processes asynchronously)

**Performance**: < 1ms (channel send operation)

---

## ScannerService Contract

### Interface Extension

```rust
impl ScannerService {
    /// Existing constructor
    pub fn new(pool: DbPool) -> Self;
    
    /// NEW: Scan with STL preview generation
    pub fn scan_with_previews(
        &self,
        root_path: &str,
        preview_service: &StlPreviewService,
        preview_queue: &PreviewQueue,
    ) -> Result<ScanResult, AppError>;
}
```

### Method Specification: `scan_with_previews`

**Purpose**: Scan directories and generate STL previews (first 2 sync, rest async).

**Algorithm**:
```
1. Walk directory tree and collect STL files (existing logic)
2. Create projects and file records (existing logic)
3. For each project:
   a. Get list of STL files
   b. Split: first 2 for sync, rest for async
   c. For first 2 STL files:
      - Call preview_service.generate_preview_with_smart_cache(stl_path)
      - Log result (warn on error, continue)
   d. For remaining STL files:
      - Call preview_queue.queue_preview(stl_path)
      - Log if queue failed
4. Generate composite previews (existing logic, now includes STL previews)
5. Return ScanResult with counts and errors
```

**Parameters**:
- `root_path: &str` - Root directory to scan
- `preview_service: &StlPreviewService` - Service for sync generation
- `preview_queue: &PreviewQueue` - Queue for async generation

**Returns**:
- `Ok(ScanResult)` - Scan completed (may include STL preview warnings)
- `Err(AppError)` - Critical scan failure

**Error Handling**:
- STL preview errors logged as warnings, added to `ScanResult.errors`
- Scan continues even if all STL previews fail
- Non-blocking: preview failures don't stop scan

**Performance**:
- Adds 6-10 seconds for typical project (2 STL previews sync)
- Async processing continues after scan completes
- No scan time increase if no STL files present

---

## RescanService Contract

### Interface Extension

```rust
impl RescanService {
    /// Existing constructor
    pub fn new(pool: DbPool) -> Self;
    
    /// NEW: Rescan with smart STL preview regeneration
    pub fn rescan_with_previews(
        &self,
        root_path: &str,
        preview_service: &StlPreviewService,
        preview_queue: &PreviewQueue,
    ) -> Result<RescanResult, AppError>;
}
```

### Method Specification: `rescan_with_previews`

**Purpose**: Rescan projects and regenerate STL previews only if needed (smart caching).

**Algorithm**:
```
1. Scan filesystem and compare with database (existing logic)
2. Detect added/modified/deleted STL files (existing logic)
3. For each project:
   a. Get list of STL files
   b. For each STL file:
      - Check if preview is valid (stl_mtime <= preview_generated_at)
      - If valid: skip (CACHE HIT)
      - If invalid: add to regeneration list
   c. Split regeneration list: first 2 for sync, rest for async
   d. Regenerate sync previews (same as scanner)
   e. Queue async previews
4. Remove previews for deleted STL files
5. Regenerate composite previews if image set changed
6. Return RescanResult with detailed counts
```

**Parameters**:
- `root_path: &str` - Root directory to rescan
- `preview_service: &StlPreviewService` - Service for sync generation
- `preview_queue: &PreviewQueue` - Queue for async generation

**Returns**:
- `Ok(RescanResult)` - Rescan completed
- `Err(AppError)` - Critical rescan failure

**Smart Caching Logic**:
```rust
let stl_mtime = fs::metadata(&stl_path)?.modified()?;
let preview_generated_at = get_preview_timestamp_from_db(&stl_path)?;

if stl_mtime <= preview_generated_at {
    info!("STL preview valid, skipping: {}", stl_path);
    continue; // CACHE HIT
}

// CACHE MISS - regenerate
warn!("STL modified, regenerating preview: {}", stl_path);
preview_service.generate_preview_with_smart_cache(&stl_path).await?;
```

**Performance**:
- 90%+ cache hit rate on typical rescans
- Minimal time increase for unchanged projects
- Only modified STL files regenerated

---

## FileRepository Contract Extension

### Interface Extension

```rust
impl FileRepository {
    /// NEW: Insert STL preview image
    pub fn insert_stl_preview_image(
        &self,
        project_id: i64,
        stl_path: &str,
        preview_path: &str,
    ) -> Result<i64, AppError>;
    
    /// NEW: Get images sorted by priority
    pub fn get_images_by_priority(
        &self,
        project_id: i64,
    ) -> Result<Vec<ImageFile>, AppError>;
    
    /// NEW: Delete STL preview image
    pub fn delete_stl_preview_image(
        &self,
        stl_path: &str,
    ) -> Result<(), AppError>;
    
    /// NEW: Update image priority (for future use)
    pub fn update_image_priority(
        &self,
        image_id: i64,
        priority: i32,
    ) -> Result<(), AppError>;
}
```

### Method Specifications

#### `insert_stl_preview_image`

**SQL**:
```sql
INSERT INTO image_files (
    project_id, filename, file_path, file_size,
    source_type, image_priority, image_source,
    display_order, created_at, updated_at
) VALUES (
    ?, ?, ?, ?,
    'direct', 50, 'stl_preview',
    0, strftime('%s', 'now'), strftime('%s', 'now')
)
ON CONFLICT(file_path) DO UPDATE SET
    updated_at = strftime('%s', 'now')
```

**Purpose**: Add STL preview to image_files table with correct priority.

**Returns**: Image file ID

---

#### `get_images_by_priority`

**SQL**:
```sql
SELECT * FROM image_files
WHERE project_id = ?
ORDER BY image_priority DESC, display_order ASC, created_at ASC
```

**Purpose**: Retrieve all images sorted by priority (regular before STL previews).

**Returns**: List of ImageFile structs

---

## Error Handling Contracts

### Error Propagation Rules

1. **Critical Errors** (propagate up, stop operation):
   - Database connection failures
   - Filesystem permission errors (can't write to cache)
   - Invalid scan root path

2. **Non-Critical Errors** (log, continue):
   - STL preview generation failures
   - Individual file processing errors
   - stl-thumb rendering errors

3. **Warning Conditions** (log, don't error):
   - stl-thumb binary not found (disable feature)
   - STL file too large (skip)
   - Timeout during rendering (skip)

### Error Logging Format

```rust
// Critical error (propagate)
error!("Failed to connect to database: {}", e);
return Err(AppError::Database(e));

// Non-critical error (log, continue)
warn!("Failed to generate STL preview for {}: {}", stl_path, e);
scan_result.errors.push(format!("STL preview failed: {}", stl_path));
// Continue processing other files

// Warning condition (log, skip)
info!("STL file too large ({}MB), skipping preview: {}", size_mb, stl_path);
// No error recorded, feature gracefully degraded
```

---

## Performance Contracts

### Service SLAs

| Service Method | Target Latency | Max Latency | Notes |
|---------------|----------------|-------------|-------|
| `is_preview_valid` | < 5ms | 100ms | Read-only, fast check |
| `generate_preview_with_smart_cache` (cache hit) | < 10ms | 100ms | File lookup only |
| `generate_preview_with_smart_cache` (cache miss) | 3-10s | 30s | Depends on STL complexity |
| `queue_preview` | < 1ms | 10ms | Channel send |
| `scan_with_previews` (2 STLs) | +6-10s | +30s | Sync preview generation |
| `rescan_with_previews` (no changes) | +0-1s | +5s | Smart caching |

### Memory Contracts

| Operation | Expected Memory | Max Memory | Notes |
|-----------|-----------------|------------|-------|
| Single preview generation | 50-250MB | 500MB | OpenGL rendering |
| Preview queue | ~100KB | 1MB | Queue of filenames |
| Background worker | 50-250MB | 500MB | One preview at a time |
| Total additional memory | 100-300MB | 500MB | Per spec constraint |

### Concurrency Contracts

- **Scan concurrency**: 1 active scan at a time (enforced by application)
- **Preview generation**: Sequential (stl-thumb uses GPU, parallel doesn't help)
- **Queue workers**: 1 background worker (sufficient throughput)
- **Database connections**: Use existing connection pool

---

## Testing Contracts

### Unit Test Coverage

Each service method must have:
1. **Happy path test**: Normal operation succeeds
2. **Cache hit test**: Smart caching works
3. **Error handling test**: Graceful degradation on failure
4. **Edge case tests**: Empty input, large files, missing files

### Integration Test Coverage

1. **End-to-end scan**: STL files discovered and previews generated
2. **Smart caching**: Rescan reuses valid previews
3. **Async processing**: Background queue processes remaining files
4. **Error resilience**: Scan completes even with STL preview failures
5. **Priority sorting**: Images returned in correct order

### Performance Test Requirements

1. **Latency**: Measure sync preview generation time (should be < 10s for 2 files)
2. **Memory**: Monitor memory usage during generation (should be < 500MB)
3. **Cache hit rate**: Measure rescan cache hits (should be > 90%)
4. **Throughput**: Async worker processes ~2 files/minute

---

## Summary

The service contracts define clear responsibilities and interfaces:

1. **StlPreviewService**: Core preview generation with smart caching
2. **PreviewQueue**: Async background processing
3. **Scanner/RescanService**: Integration points for preview generation
4. **FileRepository**: Database access for image metadata

All contracts emphasize:
- ✅ Graceful error handling (non-blocking failures)
- ✅ Smart caching (efficiency on rescans)
- ✅ Clear performance expectations
- ✅ Testable interfaces
- ✅ Backward compatibility
