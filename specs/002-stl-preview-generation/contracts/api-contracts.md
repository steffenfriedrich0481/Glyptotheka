# API Contracts: STL Preview Image Generation

**Feature**: STL Preview Image Generation During Scanning  
**Date**: 2025-11-18  
**Phase**: 1 - Design & Contracts

## Overview

This document defines the API contract changes for the STL preview generation feature. The changes are **backward compatible** - no new endpoints are added, and existing endpoints return STL preview images transparently through the existing response schema.

---

## API Changes Summary

### No New Endpoints

All functionality is exposed through existing API endpoints with **no breaking changes** to request/response schemas.

### Modified Endpoints

| Endpoint | Method | Change | Backward Compatible |
|----------|--------|--------|---------------------|
| `/api/projects/{id}/images` | GET | Returns STL preview images in results | ✅ Yes - same schema |
| `/api/projects/{id}/preview` | GET | May use STL previews in composite | ✅ Yes - same schema |
| `/api/scan` | POST | Generates STL previews during scan | ✅ Yes - internal only |
| `/api/rescan` | POST | Regenerates STL previews if needed | ✅ Yes - internal only |

---

## Endpoint Details

### 1. Get Project Images

**Endpoint**: `GET /api/projects/{id}/images`

**Description**: Retrieve all images for a project, including regular images and STL preview images, sorted by priority.

**Request**:
```http
GET /api/projects/123/images HTTP/1.1
Accept: application/json
```

**Path Parameters**:
- `id` (integer, required): Project ID

**Query Parameters**: None

**Response Schema** (unchanged):

```json
{
  "project_id": 123,
  "images": [
    {
      "id": 456,
      "filename": "photo1.jpg",
      "file_path": "/path/to/project/photo1.jpg",
      "file_size": 2048000,
      "source_type": "direct",
      "display_order": 0,
      "created_at": 1700000000,
      "updated_at": 1700000000
    },
    {
      "id": 457,
      "filename": "model.stl.png",
      "file_path": "cache/stl-previews/abc123.png",
      "file_size": 125000,
      "source_type": "direct",
      "display_order": 0,
      "created_at": 1700000100,
      "updated_at": 1700000100
    }
  ]
}
```

**Response Notes**:
- `images` array now includes STL preview images
- Regular images appear first (priority 100)
- STL preview images appear after regular images (priority 50)
- Sorting is handled internally via `ORDER BY image_priority DESC, display_order ASC`
- Frontend receives pre-sorted list, no changes needed

**Status Codes**:
- `200 OK`: Success
- `404 Not Found`: Project not found
- `500 Internal Server Error`: Database or filesystem error

**Backward Compatibility**: ✅ **Fully compatible**
- Existing clients see STL previews as additional images (same schema)
- No breaking changes to response structure
- Frontend can display STL previews without modifications

---

### 2. Get Project Composite Preview

**Endpoint**: `GET /api/projects/{id}/preview`

**Description**: Retrieve or generate a composite preview image for a project. May use STL preview images if fewer than 4 regular images are available.

**Request**:
```http
GET /api/projects/123/preview HTTP/1.1
Accept: image/png
```

**Path Parameters**:
- `id` (integer, required): Project ID

**Query Parameters**: None

**Response**: Binary PNG image (800×800 pixels)

**Response Headers**:
```http
Content-Type: image/png
Content-Length: 245678
Cache-Control: public, max-age=3600
```

**Behavior Changes**:
- If project has 4+ regular images: Uses regular images only (unchanged behavior)
- If project has 0-3 regular images: Fills remaining slots with STL preview images (new behavior)
- If project has 0 regular images: Uses up to 4 STL preview images (new behavior)
- If project has 0 images total: Returns 404 (unchanged behavior)

**Status Codes**:
- `200 OK`: Composite preview returned (may be cached)
- `404 Not Found`: Project not found or has no images
- `500 Internal Server Error`: Image generation failed

**Backward Compatibility**: ✅ **Fully compatible**
- Response format unchanged (PNG image)
- Clients receive composite preview as before
- Internal composition logic enhanced to include STL previews

---

### 3. Scan Projects

**Endpoint**: `POST /api/scan`

**Description**: Scan a directory for projects and STL files, generating STL preview images.

**Request**:
```http
POST /api/scan HTTP/1.1
Content-Type: application/json

{
  "root_path": "/path/to/projects"
}
```

**Request Schema** (unchanged):
```json
{
  "root_path": "string (required)"
}
```

**Response Schema** (unchanged):
```json
{
  "projects_found": 42,
  "files_processed": 156,
  "errors": []
}
```

**Behavior Changes**:
- Generates STL previews for first 2 STL files synchronously
- Queues remaining STL files for async background generation
- Adds STL preview images to `image_files` table
- Logs warnings for failed preview generations (non-blocking)

**Response Notes**:
- `files_processed` includes STL files
- `errors` array may include STL preview generation failures (warnings, not errors)
- Scan completes without waiting for async preview generation

**Status Codes**:
- `200 OK`: Scan completed (even if some STL previews failed)
- `400 Bad Request`: Invalid root_path
- `500 Internal Server Error`: Critical scan failure

**Backward Compatibility**: ✅ **Fully compatible**
- Request/response schemas unchanged
- Scan behavior enhanced with STL preview generation
- Non-blocking errors (graceful degradation)

---

### 4. Rescan Projects

**Endpoint**: `POST /api/rescan`

**Description**: Rescan existing projects, detecting changes and regenerating STL previews if needed.

**Request**:
```http
POST /api/rescan HTTP/1.1
Content-Type: application/json

{
  "root_path": "/path/to/projects"
}
```

**Request Schema** (unchanged):
```json
{
  "root_path": "string (required)"
}
```

**Response Schema** (unchanged):
```json
{
  "projects_found": 42,
  "projects_added": 2,
  "projects_updated": 5,
  "projects_removed": 1,
  "files_processed": 156,
  "files_added": 10,
  "files_updated": 8,
  "files_removed": 3,
  "errors": []
}
```

**Behavior Changes**:
- Checks STL file modification times against preview generation times
- Regenerates STL previews only if STL file modified (smart caching)
- Removes STL preview images if STL files deleted
- Follows same sync/async pattern as initial scan (first 2 sync, rest async)

**Response Notes**:
- `files_updated` may include STL files with regenerated previews
- `files_removed` includes deleted STL files (previews also removed)
- Smart caching reduces regeneration (90%+ cache hit rate expected)

**Status Codes**:
- `200 OK`: Rescan completed
- `400 Bad Request`: Invalid root_path
- `500 Internal Server Error`: Critical rescan failure

**Backward Compatibility**: ✅ **Fully compatible**
- Request/response schemas unchanged
- Rescan behavior enhanced with smart STL preview regeneration

---

## Internal Service Interfaces

These are not public API endpoints but internal service method signatures.

### StlPreviewService Interface

```rust
pub trait StlPreviewServiceInterface {
    /// Generate preview with smart caching (checks file mtime)
    async fn generate_preview_with_smart_cache(
        &self, 
        stl_path: &str
    ) -> Result<PathBuf, AppError>;
    
    /// Check if preview exists and is up-to-date
    async fn is_preview_valid(
        &self, 
        stl_path: &str
    ) -> Result<bool, AppError>;
    
    /// Force regeneration (ignore cache)
    async fn regenerate_preview(
        &self, 
        stl_path: &str
    ) -> Result<PathBuf, AppError>;
}
```

### ScannerService Interface

```rust
pub trait ScannerServiceInterface {
    /// Scan directory with STL preview generation
    fn scan_with_previews(
        &self, 
        root_path: &str,
        preview_service: &StlPreviewService
    ) -> Result<ScanResult, AppError>;
}
```

### RescanService Interface

```rust
pub trait RescanServiceInterface {
    /// Rescan directory with smart STL preview regeneration
    fn rescan_with_previews(
        &self, 
        root_path: &str,
        preview_service: &StlPreviewService
    ) -> Result<RescanResult, AppError>;
}
```

---

## Error Handling

### Error Types

All endpoints continue to use the existing `AppError` enum. No new error types added.

**Relevant Error Variants**:
- `AppError::NotFound`: Project or file not found
- `AppError::ValidationError`: Invalid input parameters
- `AppError::InternalServer`: STL preview generation failed (logged as warning, non-blocking)

### Error Response Schema

```json
{
  "error": {
    "code": "INTERNAL_SERVER_ERROR",
    "message": "Failed to generate STL preview: OpenGL initialization failed"
  }
}
```

**Note**: STL preview generation failures do not cause scan/rescan to fail. They are logged as warnings and included in the `errors` array of scan/rescan responses.

---

## Backwards Compatibility Analysis

### Frontend Impact: **NONE**

- ✅ No frontend code changes required
- ✅ STL preview images appear automatically in galleries
- ✅ Composite previews may include STL previews (transparent to frontend)
- ✅ All API responses use existing schemas

### Client Impact: **NONE**

- ✅ No breaking changes to any API endpoint
- ✅ Response schemas unchanged
- ✅ New image sources appear as regular images (same fields)
- ✅ Sorting happens server-side (clients receive pre-sorted lists)

### Database Impact: **MINIMAL**

- ✅ Schema migration adds columns with defaults (non-breaking)
- ✅ Existing queries continue to work (new columns have sensible defaults)
- ✅ New index improves performance (no functional changes)

---

## Performance Considerations

### Response Time Impact

| Endpoint | Before | After | Impact |
|----------|--------|-------|--------|
| GET /api/projects/{id}/images | 50ms | 55ms | +10% (additional rows, index optimizes) |
| GET /api/projects/{id}/preview | 200ms | 250ms | +25% (may load more images) |
| POST /api/scan | 5s | 12s | +140% (first 2 STL previews sync, but acceptable) |
| POST /api/rescan | 3s | 4s | +33% (smart caching minimizes impact) |

**Mitigation**:
- Async preview generation keeps scan time reasonable
- Smart caching reduces rescan overhead
- New index on `image_priority` optimizes queries

### Payload Size Impact

| Response Type | Before | After | Impact |
|---------------|--------|-------|--------|
| Image list (10 images) | 2KB | 2.5KB | +25% (more images) |
| Composite preview | 200KB | 200KB | 0% (same image size) |
| Scan result | 0.5KB | 0.6KB | +20% (more files processed) |

**Impact Assessment**: Negligible - JSON payloads are small.

---

## Testing Strategy

### Contract Tests

**Test 1: Image Priority Sorting**
```http
GET /api/projects/{id}/images
```
**Expected**: Regular images before STL previews in response

**Test 2: Composite Preview with STL Previews**
```http
GET /api/projects/{id_with_1_regular_3_stl}/preview
```
**Expected**: Composite uses 1 regular + 3 STL previews

**Test 3: Scan with STL Files**
```http
POST /api/scan
{
  "root_path": "/test/project/with/stls"
}
```
**Expected**: `files_processed` includes STL files, STL previews created

**Test 4: Rescan with Smart Caching**
```http
POST /api/rescan
{
  "root_path": "/test/project/unchanged"
}
```
**Expected**: STL previews not regenerated (cache hit)

### Integration Tests

1. **End-to-end scan with STL files**
   - Create test project with 5 STL files
   - Scan and verify 5 preview images created
   - Verify first 2 completed before scan finished
   - Verify all 5 eventually appear in background

2. **Smart caching on rescan**
   - Scan project with STL files
   - Rescan without modifying files
   - Verify previews not regenerated
   - Modify one STL file
   - Rescan and verify only that preview regenerated

3. **Error handling**
   - Scan project with corrupted STL file
   - Verify scan completes successfully
   - Verify error logged in `errors` array
   - Verify other STL previews generated

---

## Summary

The API contract changes are **100% backward compatible**:

1. ✅ No new endpoints
2. ✅ No schema changes
3. ✅ Existing clients work without modifications
4. ✅ STL preview images transparently integrated
5. ✅ Graceful error handling (non-breaking failures)

The feature enhances existing endpoints without breaking existing functionality. Frontend receives STL preview images as additional images in the same schema, sorted by priority server-side.
