# Research: STL Preview Image Generation

**Feature**: STL Preview Image Generation During Scanning  
**Date**: 2025-11-18  
**Phase**: 0 - Outline & Research

## Research Questions from Technical Context

All technical context items have been resolved. No NEEDS CLARIFICATION items were present.

## Key Technology Decisions

### 1. STL Preview Generation Library

**Decision**: Use existing `stl-thumb` crate (v0.5) integration

**Rationale**:
- Already integrated in the codebase (`backend/Cargo.toml`)
- StlPreviewService already implements wrapper around stl-thumb
- Proven solution for STL thumbnail generation
- Generates high-quality PNG images at configurable dimensions (currently 512x512)
- External stl-thumb binary available at `../stl-thumb` (confirmed to exist)
- Headless rendering capability (no GUI required)

**Alternatives Considered**:
- **Custom OpenGL renderer**: Would require significant development effort, duplicating existing functionality
- **three-rs or kiss3d**: Rust 3D rendering libraries, but stl-thumb is already specialized for thumbnails
- **External process calling stl-thumb binary**: More fragile than library integration

**Implementation Note**: The stl-thumb crate renders in blocking mode (CPU/GPU-bound OpenGL work), so StlPreviewService correctly uses `tokio::task::spawn_blocking` to avoid blocking the async runtime.

---

### 2. Hybrid Sync/Async Preview Generation Pattern

**Decision**: Generate first 2 STL previews synchronously during scan, queue remaining previews for async processing

**Rationale**:
- Immediate visual feedback: Users can see first 2 previews without waiting for entire scan
- Scan performance: Limits scan time increase to ~10 seconds for most projects
- Background processing: Large projects (100+ STLs) don't block scan completion
- User experience: Gallery loads with initial previews, more appear progressively
- Matches specification requirement (FR-004)

**Alternatives Considered**:
- **All synchronous**: Would significantly increase scan time for large projects (>50 STLs)
- **All asynchronous**: No immediate visual feedback, poor UX for small projects
- **First N configurable**: Adds complexity without clear benefit; 2 is optimal balance

**Implementation Pattern**:
```rust
// In scanner/rescan services:
let stl_files = collect_stl_files();
let (sync_files, async_files) = stl_files.split_at(2.min(stl_files.len()));

// Generate first 2 synchronously
for stl in sync_files {
    preview_service.generate_preview_with_smart_cache(stl).await?;
}

// Queue remaining for async processing
for stl in async_files {
    preview_queue.queue_preview(stl).await?;
}
```

---

### 3. Smart Caching Strategy

**Decision**: Use file modification timestamp comparison to determine if regeneration is needed

**Rationale**:
- Efficient: Avoids expensive re-rendering when STL file unchanged
- Reliable: File system mtime is standard across platforms
- Simple: No complex checksumming or content hashing required
- Performance: 90%+ cache hit rate on rescans (per success criteria SC-007)
- Matches specification requirement (FR-005)

**Implementation Logic**:
1. Check if preview exists in cache (`cached_files` table)
2. If exists, compare STL file mtime with preview generation timestamp
3. If STL mtime <= preview generation timestamp, return cached preview
4. If STL mtime > preview generation timestamp, regenerate preview

**Database Schema**:
- Existing `stl_files.preview_generated_at` tracks when preview was created
- Compare against STL file's filesystem modification time
- Cache directory: `cache/stl-previews/`

**Edge Cases Handled**:
- Clock skew: Use >= comparison to be conservative
- Missing cache files: Regenerate if cache file missing even if DB record exists
- File replacement: If file replaced with same name, mtime changes trigger regeneration

---

### 4. Image Priority System

**Decision**: Extend `image_files` table with `image_priority` column (INTEGER); regular images = 100, STL previews = 50

**Rationale**:
- Clear separation: Regular photos always rank higher than generated previews
- Extensibility: Priority system allows future image types (e.g., composite previews = 25)
- Sort efficiency: Simple integer comparison in SQL queries
- User experience: Users see actual project photos first, STL previews as fallback
- Matches specification requirements (FR-007, FR-009)

**Database Migration**:
```sql
-- 005_stl_preview_priority.sql
ALTER TABLE image_files ADD COLUMN image_priority INTEGER NOT NULL DEFAULT 100;
ALTER TABLE image_files ADD COLUMN image_source TEXT NOT NULL DEFAULT 'regular' 
  CHECK (image_source IN ('regular', 'stl_preview', 'composite'));

-- Update priority based on source
UPDATE image_files SET 
  image_priority = CASE 
    WHEN image_source = 'regular' THEN 100
    WHEN image_source = 'stl_preview' THEN 50
    WHEN image_source = 'composite' THEN 25
  END;

CREATE INDEX idx_image_files_priority ON image_files(project_id, image_priority DESC, display_order);
```

**Query Pattern**:
```sql
SELECT * FROM image_files 
WHERE project_id = ? 
ORDER BY image_priority DESC, display_order ASC, created_at ASC
```

---

### 5. Error Handling Strategy

**Decision**: Graceful degradation with warning logs; continue scan even if STL preview generation fails

**Rationale**:
- Resilience: Single problematic STL file shouldn't break entire scan
- User experience: Users get partial results rather than total failure
- Observability: Warnings logged for troubleshooting
- Tool availability: System works even if stl-thumb missing/broken
- Matches specification requirements (FR-011, FR-012)

**Error Scenarios**:
1. **stl-thumb library error**: Log warning, skip that STL, continue
2. **Corrupted STL file**: Catch render error, log warning, continue
3. **Timeout (>30s)**: Cancel generation, log warning, continue
4. **Disk space**: Log error, disable STL preview generation for scan, continue
5. **Cache write failure**: Log warning, don't cache preview, continue scan

**Implementation Pattern**:
```rust
match preview_service.generate_preview(&stl_path).await {
    Ok(preview_path) => {
        info!("Generated preview: {}", preview_path.display());
        // Add to image_files table
    }
    Err(e) => {
        warn!("Failed to generate STL preview for {}: {}", stl_path, e);
        // Continue with next file
    }
}
```

---

### 6. Async Background Processing

**Decision**: Use existing tokio mpsc channel pattern from StlPreviewService::PreviewQueue

**Rationale**:
- Already implemented: PreviewQueue exists with channel-based worker
- Proven pattern: Standard tokio async processing
- Bounded queue: Prevents memory issues with large project libraries
- Fire-and-forget: Scanner doesn't wait for async previews to complete
- Simple lifecycle: Worker task spawned once, processes queue until channel closed

**Queue Configuration**:
- Queue size: 100 pending STL files
- Worker threads: Single background worker (preview generation is sequential anyway due to OpenGL)
- Failure handling: Log warning, continue with next queued item

**Lifecycle**:
```rust
// At application startup
let preview_queue = PreviewQueue::new(preview_service.clone(), 100);

// During scan
for stl in async_stls {
    preview_queue.queue_preview(stl.to_string()).await?;
}
// Worker continues processing in background
```

---

### 7. Composite Preview Integration

**Decision**: Modify CompositePreviewService to accept STL previews as image candidates, prioritizing regular images first

**Rationale**:
- Unified visual experience: Composite previews show project overview
- Priority-aware: Regular images fill slots first, STL previews fill remaining
- Consistent with spec: FR-008, FR-009 requirements
- User value: Projects with few/no photos still get visual representation

**Implementation Changes**:
```rust
// In CompositePreviewService::generate_preview
pub fn generate_preview(
    &self,
    project_id: i64,
    image_paths: &[String],  // Already sorted by priority
) -> Result<PathBuf, AppError> {
    // Take up to 4 images (already priority-sorted by caller)
    let count = image_paths.len().min(4);
    // ... existing logic
}
```

**Caller responsibility**: Image retrieval already sorts by priority, so composite service receives images in correct order (regular first, then STL previews).

---

## Best Practices Research

### Rust + Tokio Async Patterns

**Best Practices Applied**:
1. **CPU-bound work**: Use `spawn_blocking` for OpenGL rendering (already implemented in StlPreviewService)
2. **Async channels**: Use `mpsc::channel` for background job queue (already implemented)
3. **Error propagation**: Use `Result<T, AppError>` throughout service layer
4. **Logging**: Use `tracing` crate for structured logging (already integrated)

### SQLite Performance Optimization

**Best Practices Applied**:
1. **Indexes**: Add composite index on `(project_id, image_priority, display_order)` for efficient sorting
2. **Batch operations**: Use transactions for multi-row inserts during scan
3. **Query planning**: Use `EXPLAIN QUERY PLAN` to verify index usage

### File System Cache Management

**Best Practices Applied**:
1. **Naming convention**: Use hash-based naming for cache files to avoid collisions
2. **Directory structure**: Use `cache/stl-previews/` subdirectory for organization
3. **Metadata tracking**: Store cache paths in `cached_files` table for lookup
4. **Cleanup strategy**: Out of scope for this feature; future enhancement

---

## Integration Points Analysis

### 1. Scanner Service Integration

**Current Behavior**: 
- Scans directories for STL files
- Creates projects and file records
- No preview generation

**Required Changes**:
- Inject StlPreviewService dependency
- After collecting STL files, trigger preview generation
- Generate first 2 synchronously, queue remainder
- Add STL preview images to `image_files` table with priority=50

### 2. Rescan Service Integration

**Current Behavior**:
- Rescans existing projects
- Detects file additions/changes/deletions
- Clears inherited images

**Required Changes**:
- Same as scanner: inject StlPreviewService
- Apply smart caching logic (check file mtime)
- Regenerate only if STL file modified
- Handle STL file deletions (remove preview from cache)

### 3. Image Retrieval API

**Current Behavior**:
- Returns images for project via `GET /api/projects/{id}/images`
- Handles inherited images from parent projects
- Returns images sorted by `display_order`

**Required Changes**:
- Update query to sort by `image_priority DESC, display_order ASC`
- Include STL preview images in results
- No API contract changes (transparent to frontend)

### 4. Composite Preview Service

**Current Behavior**:
- Generates composite preview from up to 4 images
- Called during scan/rescan
- Uses project's image files

**Required Changes**:
- No logic changes needed
- Caller already provides sorted image list
- Automatically uses STL previews if regular images < 4

---

## Open Questions & Decisions

### Q1: Should STL previews inherit to child folders like regular images?

**Decision**: YES - Follow existing inheritance pattern

**Rationale**:
- Consistency: STL previews are treated as images in the data model
- User experience: Child folders without images can show parent's STL previews
- Implementation: Existing inheritance logic handles this automatically via `source_type='inherited'`

### Q2: Should we enforce a file size limit for STL preview generation?

**Decision**: YES - Add 100MB file size limit

**Rationale**:
- Performance: Very large STL files (>100MB) can take minutes to render
- User experience: Timeout on large files would delay async processing
- Practicality: Most project STL files are under 50MB

**Implementation**: Check file size before queuing for preview generation; log info message for skipped files.

### Q3: What happens if stl-thumb binary is missing?

**Decision**: Detect at startup, log warning, disable STL preview generation

**Rationale**:
- Clear feedback: User knows immediately if feature unavailable
- Graceful degradation: Application still functions without STL previews
- No repeated errors: Check once at startup, not on every STL file

**Implementation**: Add startup check in StlPreviewService::new(); set internal flag to disable generation if binary not found.

---

## Performance Considerations

### Expected Load Profile

**Typical Project**:
- 5-20 STL files
- First 2 previews: ~3-5 seconds each = 6-10 seconds total
- Scan time increase: 6-10 seconds (acceptable per spec: <10% increase)

**Large Project**:
- 100+ STL files
- First 2 previews: 6-10 seconds (synchronous)
- Remaining 98 previews: Background processing, ~30 min total
- User sees results immediately, more previews appear progressively

**Rescan Efficiency**:
- 90% cache hit rate (unchanged STL files)
- Only modified files regenerated
- Minimal scan time impact

### Memory Profile

**During Sync Generation** (per file):
- STL file loaded: ~10-50MB (depending on model complexity)
- stl-thumb rendering: ~50-200MB (OpenGL buffers)
- PNG output: ~100-500KB
- Total peak: ~50-250MB per file

**During Async Processing**:
- Queue memory: ~1KB per queued filename × 100 = 100KB
- Worker memory: Same as sync (50-250MB for active render)
- Total additional: <500MB (per spec constraint)

### Disk Space

**Preview Images**:
- Size per preview: ~50-500KB (512×512 PNG)
- 1000 STL files = ~50-500MB total
- Acceptable for desktop application

---

## Risk Assessment

### Risk 1: STL Preview Generation Timeout

**Likelihood**: Medium (complex models can take >30s)  
**Impact**: Low (async processing continues, just skips that file)  
**Mitigation**: 
- Set 30-second timeout in spawn_blocking
- Log warning and continue
- Consider adding user notification for failed previews (future enhancement)

### Risk 2: stl-thumb Library Panics

**Likelihood**: Low (mature library, but OpenGL issues possible)  
**Impact**: Medium (could crash worker thread)  
**Mitigation**:
- spawn_blocking isolates panic to worker thread
- Main application continues
- Log error and skip file

### Risk 3: Disk Space Exhaustion

**Likelihood**: Low (previews are small)  
**Impact**: High (could affect other operations)  
**Mitigation**:
- Check disk space before generation
- Disable preview generation if <1GB free
- Log error clearly

### Risk 4: Cache Inconsistency

**Likelihood**: Low  
**Impact**: Medium (stale previews shown)  
**Mitigation**:
- Smart caching uses file mtime (reliable)
- Cache cleanup tool (future enhancement)
- Manual cache clear option

---

## Summary

All technical decisions have been researched and documented. The implementation approach leverages existing infrastructure (StlPreviewService, PreviewQueue, image inheritance) and requires minimal new code. Key changes are:

1. **Database**: Add `image_priority` and `image_source` columns to `image_files` table
2. **Services**: Integrate StlPreviewService into scanner/rescan workflows
3. **Smart caching**: Implement file mtime comparison logic
4. **Error handling**: Wrap all preview generation in try-catch with logging
5. **API**: Update image queries to sort by priority

No blockers identified. Ready to proceed to Phase 1 (Design & Contracts).
