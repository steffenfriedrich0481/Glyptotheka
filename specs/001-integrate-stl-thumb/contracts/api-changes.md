# API Contract Changes

**Feature**: Integrate stl-thumb as Rust Library  
**Date**: 2025-11-17  
**Phase**: Design (Phase 1)

## Overview

This document describes API contract changes for the stl-thumb library integration. Most API endpoints remain unchanged - only configuration-related endpoints are affected.

---

## Affected Endpoints

### GET /api/config

**Description**: Retrieve current application configuration

#### Current Response
```json
{
  "id": 1,
  "root_path": "/projects",
  "last_scan_at": 1700000000,
  "stl_thumb_path": "/usr/local/bin/stl-thumb",
  "cache_max_size_mb": 1000,
  "images_per_page": 50,
  "created_at": 1699000000,
  "updated_at": 1700000000
}
```

#### New Response
```json
{
  "id": 1,
  "root_path": "/projects",
  "last_scan_at": 1700000000,
  "cache_max_size_mb": 1000,
  "images_per_page": 50,
  "created_at": 1699000000,
  "updated_at": 1700000000
}
```

**Change**: `stl_thumb_path` field removed

**Breaking Change**: Yes - field removed from response
**Mitigation**: Frontend may need to remove any UI elements that display/edit this field

---

### POST /api/config

**Description**: Update application configuration

#### Current Request Body
```json
{
  "root_path": "/new/projects/path",
  "stl_thumb_path": "/custom/stl-thumb",
  "cache_max_size_mb": 2000,
  "images_per_page": 100
}
```

#### New Request Body
```json
{
  "root_path": "/new/projects/path",
  "cache_max_size_mb": 2000,
  "images_per_page": 100
}
```

**Change**: `stl_thumb_path` field no longer accepted

**Breaking Change**: Yes - field no longer accepted
**Mitigation**: Sending `stl_thumb_path` will be ignored (not cause error)

#### Response

Same as GET /api/config response (see above)

---

## Unaffected Endpoints

The following endpoints have **NO CHANGES**:

### Preview Generation

- **GET /api/stl/:id/preview** - Unchanged
  - Still returns 512x512 PNG image
  - Same caching behavior
  - Same error responses
  
### File Operations

- **GET /api/files/:id** - Unchanged
- **GET /api/files/:id/download** - Unchanged
- **GET /api/projects/:id/download** - Unchanged

### Search & Browse

- **GET /api/projects** - Unchanged
- **GET /api/projects/:id** - Unchanged
- **GET /api/search** - Unchanged
- **GET /api/tags** - Unchanged

### Scan Operations

- **POST /api/scan/start** - Unchanged
- **GET /api/scan/status** - Unchanged

---

## Error Response Changes

### Preview Generation Errors

#### Current Error Response
```json
{
  "error": "stl-thumb failed: ERROR: Failed to load mesh from /path/to/file.stl\n"
}
```

#### New Error Response
```json
{
  "error": "STL rendering failed: Failed to load mesh from /path/to/file.stl: Invalid STL format"
}
```

**Improvement**: Cleaner error messages without stderr artifacts

### Configuration Errors

#### Removed Error

**Current**: "stl-thumb is not configured"  
**New**: N/A - this error no longer occurs

No configuration is needed for preview generation with library integration.

---

## TypeScript Interface Changes

### Frontend Type Updates Required

#### Before (frontend/src/types/config.ts)
```typescript
export interface AppConfig {
  id: number;
  root_path: string | null;
  last_scan_at: number | null;
  stl_thumb_path: string | null;  // ❌ REMOVE
  cache_max_size_mb: number;
  images_per_page: number;
  created_at: number;
  updated_at: number;
}

export interface UpdateConfigRequest {
  root_path?: string;
  stl_thumb_path?: string;  // ❌ REMOVE
  cache_max_size_mb?: number;
  images_per_page?: number;
}
```

#### After (frontend/src/types/config.ts)
```typescript
export interface AppConfig {
  id: number;
  root_path: string | null;
  last_scan_at: number | null;
  // stl_thumb_path removed
  cache_max_size_mb: number;
  images_per_page: number;
  created_at: number;
  updated_at: number;
}

export interface UpdateConfigRequest {
  root_path?: string;
  // stl_thumb_path removed
  cache_max_size_mb?: number;
  images_per_page?: number;
}
```

---

## OpenAPI Schema (Partial)

### Configuration Schemas

```yaml
components:
  schemas:
    AppConfig:
      type: object
      required:
        - id
        - cache_max_size_mb
        - images_per_page
        - created_at
        - updated_at
      properties:
        id:
          type: integer
          example: 1
        root_path:
          type: string
          nullable: true
          example: "/projects"
        last_scan_at:
          type: integer
          nullable: true
          example: 1700000000
        # stl_thumb_path removed
        cache_max_size_mb:
          type: integer
          minimum: 100
          example: 1000
        images_per_page:
          type: integer
          minimum: 10
          maximum: 200
          example: 50
        created_at:
          type: integer
          example: 1699000000
        updated_at:
          type: integer
          example: 1700000000
    
    UpdateConfigRequest:
      type: object
      properties:
        root_path:
          type: string
          example: "/new/path"
        # stl_thumb_path removed
        cache_max_size_mb:
          type: integer
          minimum: 100
          example: 2000
        images_per_page:
          type: integer
          minimum: 10
          maximum: 200
          example: 100
```

---

## Backward Compatibility

### API Version Consideration

**Current Version**: No explicit versioning

**Options**:

1. **Breaking change without version bump** (Recommended)
   - Justification: Internal application, not public API
   - Justification: Change simplifies deployment (removes dependency)
   - Frontend and backend deployed together

2. **Add API versioning** (Not recommended for this change)
   - Overkill for single field removal
   - Adds complexity for minimal benefit

**Decision**: Accept breaking change, update frontend simultaneously

### Migration Path

1. Deploy backend with library integration
2. Deploy frontend with updated types
3. Existing cached previews remain valid (no regeneration needed)
4. Database migration runs automatically on startup

---

## Testing Contract Changes

### API Tests to Update

1. **Configuration Endpoint Tests**
   ```rust
   #[tokio::test]
   async fn test_get_config_no_stl_thumb_path() {
       let response = api.get("/api/config").await?;
       let config: AppConfig = response.json()?;
       
       // Assert stl_thumb_path is not present
       assert!(serde_json::to_value(&config)?
           .get("stl_thumb_path")
           .is_none());
   }
   ```

2. **Update Config Tests**
   ```rust
   #[tokio::test]
   async fn test_update_config_ignores_stl_thumb_path() {
       let request = json!({
           "root_path": "/test",
           "stl_thumb_path": "/should/be/ignored"
       });
       
       let response = api.post("/api/config")
           .json(&request)
           .await?;
       
       let config: AppConfig = response.json()?;
       // No error, but stl_thumb_path not in response
       assert!(serde_json::to_value(&config)?
           .get("stl_thumb_path")
           .is_none());
   }
   ```

---

## Frontend Impact

### Components Potentially Affected

1. **Settings/Configuration Page**
   - Remove stl_thumb_path input field
   - Remove any validation related to stl_thumb_path
   - Update form submission logic

2. **Setup/Installation UI** (if exists)
   - Remove stl-thumb installation instructions
   - Simplify setup flow

3. **Error Display**
   - May receive improved error messages
   - No changes needed - just better UX

### Recommended Frontend Changes

```typescript
// Before
const configSchema = z.object({
  root_path: z.string().optional(),
  stl_thumb_path: z.string().optional(),  // ❌ REMOVE
  cache_max_size_mb: z.number().min(100).optional(),
  images_per_page: z.number().min(10).max(200).optional(),
});

// After
const configSchema = z.object({
  root_path: z.string().optional(),
  // stl_thumb_path removed
  cache_max_size_mb: z.number().min(100).optional(),
  images_per_page: z.number().min(10).max(200).optional(),
});
```

---

## Summary

### Changes

- ✅ Configuration endpoints: Remove `stl_thumb_path` field
- ✅ Error messages: Improved clarity
- ✅ All other endpoints: Unchanged

### Impact

- **Breaking Change**: Yes, but minimal (one configuration field)
- **Data Migration**: Database schema change (automatic)
- **Cache Migration**: None needed (existing previews valid)
- **Frontend Update**: Required (remove one form field)

### Benefits

- Simpler API contract
- Fewer configuration options to manage
- Better error messages
- No external dependencies to configure
