# Data Model: STL Preview Image Generation

**Feature**: STL Preview Image Generation During Scanning  
**Date**: 2025-11-18  
**Phase**: 1 - Design & Contracts

## Overview

This document defines the data model changes required to support STL preview image generation, including database schema modifications, entity relationships, and validation rules.

## Database Schema Changes

### Modified Table: `image_files`

The existing `image_files` table will be extended to support image prioritization and source type tracking.

**New Columns**:

```sql
-- Add priority for sorting images (higher = more important)
image_priority INTEGER NOT NULL DEFAULT 100

-- Track image source type for filtering and display
image_source TEXT NOT NULL DEFAULT 'regular' 
  CHECK (image_source IN ('regular', 'stl_preview', 'composite'))
```

**Complete Modified Schema**:

```sql
CREATE TABLE image_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    filename TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    source_type TEXT NOT NULL DEFAULT 'direct',  -- existing: 'direct' or 'inherited'
    source_project_id INTEGER,                    -- existing: inherited from which project
    display_order INTEGER NOT NULL DEFAULT 0,     -- existing: order within same priority
    
    -- NEW COLUMNS
    image_priority INTEGER NOT NULL DEFAULT 100,  -- 100=regular, 50=stl_preview, 25=composite
    image_source TEXT NOT NULL DEFAULT 'regular', -- 'regular', 'stl_preview', 'composite'
    
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (source_project_id) REFERENCES projects(id) ON DELETE CASCADE,
    
    CHECK (length(filename) > 0),
    CHECK (length(file_path) > 0),
    CHECK (file_size >= 0),
    CHECK (source_type IN ('direct', 'inherited')),
    CHECK (image_source IN ('regular', 'stl_preview', 'composite')),
    CHECK (created_at > 0),
    CHECK (updated_at >= created_at),
    CHECK (
        (source_type = 'direct' AND source_project_id IS NULL) OR
        (source_type = 'inherited' AND source_project_id IS NOT NULL)
    )
);
```

**New Index**:

```sql
-- Optimize image retrieval sorted by priority then display order
CREATE INDEX idx_image_files_priority 
  ON image_files(project_id, image_priority DESC, display_order ASC);
```

**Migration Strategy**: 
- Add columns with defaults (100, 'regular') so existing rows get correct values
- Existing images automatically become priority=100, source='regular'
- No data migration needed

### Modified Table: `stl_files` (No Changes)

The existing `stl_files` table already has the necessary columns for tracking preview generation:

```sql
CREATE TABLE stl_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    filename TEXT NOT NULL,
    file_path TEXT NOT NULL UNIQUE,
    file_size INTEGER NOT NULL,
    preview_path TEXT,                    -- Path to generated preview image
    preview_generated_at INTEGER,         -- Timestamp of preview generation
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    ...
);
```

**Usage**: 
- `preview_path`: Stores cache path to generated PNG
- `preview_generated_at`: Used for smart caching (compare with file mtime)

### Modified Table: `cached_files` (No Changes)

The existing `cached_files` table already supports STL previews:

```sql
CREATE TABLE cached_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    original_path TEXT NOT NULL UNIQUE,   -- STL file path
    cache_path TEXT NOT NULL UNIQUE,      -- Generated preview path
    file_type TEXT NOT NULL,              -- 'preview' for STL previews
    file_size INTEGER NOT NULL,
    checksum TEXT,
    cached_at INTEGER NOT NULL,
    accessed_at INTEGER NOT NULL,
    
    CHECK (file_type IN ('image', 'preview')),
    ...
);
```

**Usage**: STL previews use `file_type='preview'` to distinguish from regular image caching.

---

## Entity Definitions

### Entity: STL Preview Image

**Description**: A generated preview image representing a 3D STL file.

**Attributes**:

| Attribute | Type | Description | Constraints |
|-----------|------|-------------|-------------|
| id | INTEGER | Primary key | Auto-increment |
| project_id | INTEGER | Parent project | FK to projects.id, NOT NULL |
| filename | TEXT | Preview filename | NOT NULL, e.g., "part1.stl.png" |
| file_path | TEXT | Full path to preview PNG | NOT NULL, e.g., "cache/stl-previews/abc123.png" |
| file_size | INTEGER | Preview file size in bytes | >= 0, typically 50-500KB |
| source_type | TEXT | Inheritance status | 'direct' or 'inherited' |
| source_project_id | INTEGER | Source if inherited | FK to projects.id, NULL if direct |
| display_order | INTEGER | Order within priority group | >= 0, default 0 |
| image_priority | INTEGER | Priority ranking | 50 for STL previews |
| image_source | TEXT | Image source type | 'stl_preview' |
| created_at | INTEGER | Creation timestamp | Unix timestamp |
| updated_at | INTEGER | Last update timestamp | Unix timestamp |

**Relationships**:
- Belongs to: `Project` (via project_id)
- Source from: `Project` (via source_project_id if inherited)
- Generated from: `STLFile` (1:1 relationship)

**Validation Rules**:
- Priority must be 50 for all STL previews
- Source must be 'stl_preview'
- File path must point to PNG file in cache directory
- If source_type='inherited', source_project_id must be set
- If source_type='direct', source_project_id must be NULL

**State Transitions**:
- **Created**: STL preview generated and added to image_files
- **Inherited**: Copied to child project as inherited image
- **Stale**: STL file modified after preview_generated_at (requires regeneration)
- **Deleted**: STL file deleted, preview removed from image_files and cache

---

### Entity: Regular Image

**Description**: A regular image file (JPG, PNG, etc.) found in project folder.

**Changes from Existing**:
- Add `image_priority = 100` (higher than STL previews)
- Add `image_source = 'regular'`

**Priority Rule**: Regular images ALWAYS rank higher than STL previews in all contexts (galleries, composite previews, inheritance).

---

### Entity: Composite Preview Image

**Description**: A generated composite image combining multiple source images.

**Future Support** (not implemented in this feature):
- Add `image_priority = 25` (lower than STL previews)
- Add `image_source = 'composite'`
- Composite previews use STL previews as candidates if available

---

## Relationships

### STL File → STL Preview Image (1:1)

```
stl_files (1) ──generates→ (0..1) image_files
  via: stl_files.preview_path == image_files.file_path
```

**Cardinality**: Each STL file has at most one preview image. Preview images are regenerated (same row updated) if STL file changes.

### Project → Images (1:Many)

```
projects (1) ──contains→ (*) image_files
  via: image_files.project_id
```

**Includes**: Both regular images and STL preview images. Sorted by priority for retrieval.

### Project → Inherited Images (1:Many)

```
projects (parent) (1) ──inherited_by→ (*) image_files
  via: image_files.source_project_id
```

**Inheritance Rules**:
- Child projects inherit both regular images and STL previews
- Inherited images maintain original priority
- Regular inherited images rank higher than direct STL previews in child project

---

## Query Patterns

### Get All Images for Project (Priority-Sorted)

```sql
SELECT 
  id, filename, file_path, image_priority, image_source, 
  source_type, display_order
FROM image_files
WHERE project_id = ?
ORDER BY 
  image_priority DESC,  -- Regular (100) before STL previews (50)
  display_order ASC,    -- Within same priority, sort by order
  created_at ASC        -- Tie-breaker: older first
```

**Result**: Regular images first, then STL previews, then composite previews (future).

### Get Images for Composite Preview (Top 4)

```sql
SELECT file_path
FROM image_files
WHERE project_id = ?
ORDER BY image_priority DESC, display_order ASC
LIMIT 4
```

**Result**: Up to 4 images, prioritizing regular images, filling remaining slots with STL previews.

### Get STL Preview for Specific STL File

```sql
SELECT i.file_path, i.created_at as preview_generated_at
FROM image_files i
JOIN stl_files s ON s.preview_path = i.file_path
WHERE s.file_path = ?
  AND i.image_source = 'stl_preview'
```

**Usage**: Check if preview exists for smart caching logic.

### Find STL Files Needing Preview Regeneration

```sql
SELECT s.file_path, s.preview_generated_at
FROM stl_files s
LEFT JOIN image_files i ON s.preview_path = i.file_path
WHERE s.project_id = ?
  AND (
    -- No preview exists
    i.id IS NULL 
    OR 
    -- STL modified after preview generated (requires filesystem mtime check)
    s.preview_generated_at < ? 
  )
```

**Usage**: During rescan, identify which STL files need regeneration.

---

## Validation Rules

### Image Priority Validation

**Rule**: Image priority must match image source:

| image_source | Required image_priority |
|--------------|-------------------------|
| regular | 100 |
| stl_preview | 50 |
| composite | 25 |

**Enforcement**: Application-level validation in service layer. Database constraint not added to allow future flexibility.

### STL Preview File Path Validation

**Rule**: STL preview file paths must:
1. Start with "cache/stl-previews/"
2. End with ".png"
3. Use hash-based naming (e.g., "abc123def456.png")

**Enforcement**: Application-level in StlPreviewService.

### Smart Cache Validation

**Rule**: Preview is valid for reuse if:
```
STL_file_mtime <= preview_generated_at
```

**Enforcement**: Application-level in generate_preview_with_smart_cache() method.

---

## State Machines

### STL Preview Lifecycle

```
[STL File Discovered]
       ↓
  ┌────────────┐
  │ No Preview │ ← (Initial state)
  └────────────┘
       ↓ generate_preview()
  ┌────────────┐
  │ Generating │ (sync or async)
  └────────────┘
       ↓ success
  ┌────────────┐
  │   Valid    │ ← (preview exists, up-to-date)
  └────────────┘
       ↓ STL file modified
  ┌────────────┐
  │   Stale    │ (mtime > preview_generated_at)
  └────────────┘
       ↓ regenerate
  [Generating] → [Valid]
       
  ┌────────────┐
  │   Failed   │ ← (generation error, logged)
  └────────────┘
       ↓ retry (on next scan)
  [No Preview]
```

**Transitions**:
- **No Preview → Generating**: Triggered by scan/rescan discovering new STL file
- **Generating → Valid**: Preview successfully generated and cached
- **Generating → Failed**: Generation error (logged, can retry later)
- **Valid → Stale**: STL file modified (detected during rescan)
- **Stale → Generating**: Rescan triggers regeneration
- **Valid → No Preview**: Preview cache file deleted (rare, manual intervention)

---

## Data Constraints Summary

### Database Constraints (Enforced by Schema)

```sql
-- image_files table
CHECK (image_source IN ('regular', 'stl_preview', 'composite'))
CHECK (source_type IN ('direct', 'inherited'))
CHECK (file_size >= 0)
CHECK (created_at > 0)
CHECK (updated_at >= created_at)
```

### Application Constraints (Enforced by Services)

1. **Priority-Source Matching**: image_priority matches image_source
2. **File Path Format**: STL previews in cache/stl-previews/*.png
3. **Smart Cache Logic**: Regenerate if mtime > preview_generated_at
4. **File Size Limit**: Skip STL files > 100MB
5. **Duplicate Prevention**: One preview per STL file (upsert on conflict)

---

## Migration Plan

### Migration File: `005_stl_preview_priority.sql`

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

**Rollback** (if needed):
```sql
-- SQLite doesn't support DROP COLUMN, so rollback requires recreation
-- (Not recommended; forward-only migrations preferred)
```

**Impact**: 
- Existing data preserved
- All existing images become priority=100, source='regular'
- No application downtime required
- New index improves query performance

---

## Summary

The data model changes are minimal and non-breaking:

1. **Two new columns** added to `image_files` table with sensible defaults
2. **One new index** for query optimization
3. **No changes** to other tables (stl_files, cached_files already support the feature)
4. **Backward compatible**: Existing data and queries continue to work
5. **Forward compatible**: Design accommodates future image types (composite previews)

The priority-based system cleanly separates regular images (100), STL previews (50), and future composite previews (25), ensuring correct ranking in all contexts.
