# Data Model: Search View Refinement

**Feature**: 001-search-refinement  
**Date**: 2025-11-21  
**Status**: Complete

## Overview

This document defines the data models, entities, and their relationships for the search view refinement feature. All models build upon the existing database schema and API structures.

---

## Entities

### 1. SearchResultProject (NEW)

**Description**: Extended project model returned by search API, includes aggregated image data for carousel display.

**Purpose**: Provides all necessary data for rendering search result tiles with image carousels in a single API response.

**Fields**:

| Field | Type | Required | Description | Validation |
|-------|------|----------|-------------|------------|
| id | integer | Yes | Unique project identifier | > 0 |
| name | string | Yes | Project name | Length 1-255 |
| full_path | string | Yes | Absolute filesystem path | Length 1-1024 |
| parent_id | integer | No | Parent project ID (null for root) | > 0 or null |
| is_leaf | boolean | Yes | True if project contains STL files | Always true in search results |
| description | string | No | Project description | Max 2048 chars |
| stl_count | integer | Yes | Number of STL files in project | >= 0 |
| image_count | integer | Yes | Total images available (may exceed returned count) | >= 0 |
| images | ImagePreview[] | Yes | Up to 15 images for carousel | 0-15 items |
| tags | Tag[] | Yes | Project tags | 0-N items |
| created_at | integer | Yes | Unix timestamp | > 0 |
| updated_at | integer | Yes | Unix timestamp | >= created_at |

**Relationships**:
- Has many `ImagePreview` (embedded, up to 15)
- Has many `Tag` (embedded)
- References parent `Project` via `parent_id`

**Business Rules**:
- `is_leaf` must be `true` for all search results (per FR-001)
- `images` array limited to 15 items (per FR-010)
- `images` sorted by priority: regular images (100) first, STL previews (50) second
- `stl_count` must be > 0 (implied by `is_leaf = true`)

---

### 2. ImagePreview (NEW)

**Description**: Lightweight image metadata for search result carousels. Subset of full `ImageFile` model.

**Purpose**: Provides minimal data needed for carousel display without transferring unnecessary metadata (file sizes, timestamps, etc.).

**Fields**:

| Field | Type | Required | Description | Validation |
|-------|------|----------|-------------|------------|
| id | integer | Yes | Unique image identifier | > 0 |
| filename | string | Yes | Image filename | Length 1-255 |
| image_source | string | Yes | Image type | One of: 'regular', 'stl_preview', 'composite' |
| source_type | string | Yes | How image relates to project | One of: 'direct', 'inherited' |
| source_project_id | integer | No | Origin project if inherited | > 0 or null |
| image_priority | integer | Yes | Display priority (higher = first) | 25, 50, or 100 |

**Relationships**:
- Belongs to `SearchResultProject` (parent)
- References source `Project` via `source_project_id` (if inherited)

**Business Rules**:
- `source_type = 'inherited'` requires `source_project_id IS NOT NULL`
- `source_type = 'direct'` requires `source_project_id IS NULL`
- `image_priority` values:
  - 100: Regular images (photos, renders)
  - 50: STL preview images
  - 25: Composite preview images
- Images ordered by `image_priority DESC, display_order ASC` within parent array

---

### 3. SearchQueryParams (EXTENDED)

**Description**: Query parameters for search API endpoint. Extends existing `SearchQuery` struct.

**Purpose**: Defines all supported search filters including new `leaf_only` parameter.

**Fields**:

| Field | Type | Required | Default | Description | Validation |
|-------|------|----------|---------|-------------|------------|
| q | string | No | null | Full-text search query | Max 256 chars |
| tags | string | No | null | Comma-separated tag names | Max 512 chars |
| page | integer | No | 1 | Page number for pagination | >= 1 |
| per_page | integer | No | 20 | Results per page | 1-100 |
| leaf_only | boolean | No | true | Filter to leaf projects only | true/false |

**Business Rules**:
- `per_page` capped at 100 (per existing API convention)
- `leaf_only = true` filters to projects where `is_leaf = 1`
- `tags` parsed by splitting on comma and trimming whitespace
- Empty `q` and empty `tags` with `leaf_only = true` returns all leaf projects

---

### 4. SearchResponse (EXTENDED)

**Description**: Response envelope for search API. Contains results and pagination metadata.

**Purpose**: Standard paginated response format with metadata for frontend pagination controls.

**Structure**:

```typescript
interface SearchResponse {
  data: SearchResultProject[];
  meta: SearchMeta;
}

interface SearchMeta {
  total: number;         // Total matching projects
  page: number;          // Current page number
  per_page: number;      // Results per page
  total_pages: number;   // Total pages available
}
```

**Business Rules**:
- `data` contains 0 to `per_page` projects
- `total_pages = ceil(total / per_page)` or 0 if `total = 0`
- Empty results return `{ data: [], meta: { total: 0, page: 1, per_page: 20, total_pages: 0 } }`

---

## State Transitions

### SearchResultProject Lifecycle

```
[Scanner Detects STL] → [is_leaf = true set] → [Available for Search]
                                                       ↓
                                          [User Performs Search]
                                                       ↓
                                          [Matches Query & is_leaf = true]
                                                       ↓
                                          [Images Aggregated]
                                                       ↓
                                          [Returned in SearchResponse]
```

**State Rules**:
- Projects transition to `is_leaf = true` when scanner finds first STL file
- Projects transition to `is_leaf = false` if all STL files are deleted
- Images are aggregated dynamically per search query (not cached)

---

## Database Schema Changes

### No Schema Migrations Required ✅

**Reason**: All necessary columns already exist in the current schema:

**Existing Schema** (already supports feature):
```sql
-- projects table
CREATE TABLE projects (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    full_path TEXT NOT NULL,
    parent_id INTEGER,
    is_leaf BOOLEAN NOT NULL,  -- ✅ Already exists
    -- ... other fields
);

-- image_files table
CREATE TABLE image_files (
    id INTEGER PRIMARY KEY,
    project_id INTEGER NOT NULL,
    filename TEXT NOT NULL,
    file_path TEXT NOT NULL,
    source_type TEXT NOT NULL,      -- ✅ Already exists: 'direct' | 'inherited'
    source_project_id INTEGER,      -- ✅ Already exists
    display_order INTEGER NOT NULL, -- ✅ Already exists
    image_priority INTEGER NOT NULL, -- ✅ Already exists (added in migration 005)
    image_source TEXT NOT NULL,     -- ✅ Already exists: 'regular' | 'stl_preview' | 'composite'
    -- ... other fields
);
```

**Indexes** (already optimized):
```sql
CREATE INDEX idx_projects_leaf ON projects(is_leaf);  -- ✅ Exists
CREATE INDEX idx_image_files_priority ON image_files(
    project_id, image_priority DESC, display_order ASC
);  -- ✅ Exists (added in migration 005)
```

---

## Query Patterns

### 1. Search with Leaf Filtering

**Purpose**: Find leaf projects matching search query

**Pattern**:
```rust
// Add WHERE is_leaf = 1 to all search queries
let query = "
    SELECT p.id, p.name, p.full_path, p.parent_id, p.is_leaf, 
           p.description, p.created_at, p.updated_at
    FROM projects p
    INNER JOIN projects_fts fts ON p.id = fts.project_id
    WHERE projects_fts MATCH ?1
      AND p.is_leaf = 1  -- NEW: Filter to leaf projects
    ORDER BY p.name
    LIMIT ?2 OFFSET ?3
";
```

**Indexes Used**: `idx_projects_leaf` + FTS5 index

---

### 2. Aggregate Images for Project

**Purpose**: Get all images (inherited + STL previews) for a leaf project

**Pattern**:
```rust
// Step 1: Get parent chain
let parent_chain_query = "
    WITH RECURSIVE parent_chain AS (
        SELECT id, parent_id, 0 AS depth
        FROM projects WHERE id = ?1
        UNION ALL
        SELECT p.id, p.parent_id, pc.depth + 1
        FROM projects p
        INNER JOIN parent_chain pc ON p.id = pc.parent_id
        WHERE pc.depth < 5  -- Limit recursion depth
    )
    SELECT id FROM parent_chain
";

// Step 2: Get images from project and all parents
let images_query = "
    SELECT id, filename, image_source, source_type, 
           source_project_id, image_priority
    FROM image_files
    WHERE project_id IN (/* parent_chain */)
    ORDER BY image_priority DESC, display_order ASC
    LIMIT 15
";
```

**Indexes Used**: 
- `idx_projects_parent` (for recursive CTE)
- `idx_image_files_priority` (for image ordering)

**Performance**: O(log N) for parent traversal + O(1) for indexed image lookup

---

### 3. Batch Image Aggregation for Search Results

**Purpose**: Efficiently load images for all projects in search results

**Pattern** (optimization):
```rust
// Instead of N queries (one per project), use single query with IN clause
let project_ids: Vec<i64> = search_results.iter().map(|p| p.id).collect();

// Get images for all projects at once
let images_query = "
    WITH project_images AS (
        -- For each project, get its parent chain
        -- Then get images from project + parents
        -- Limit to 15 per project
    )
    SELECT project_id, id, filename, image_source, source_type
    FROM project_images
    WHERE project_id IN (?, ?, ?, ...)  -- All search result IDs
    ORDER BY project_id, image_priority DESC
";

// Group images by project_id in application code
```

**Trade-off**: More complex query but single database round-trip vs. N simple queries

---

## Validation Rules

### SearchResultProject Validation

**Backend (Rust)**:
```rust
impl SearchResultProject {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if !self.is_leaf {
            return Err(ValidationError::new("is_leaf must be true for search results"));
        }
        if self.images.len() > 15 {
            return Err(ValidationError::new("images array must not exceed 15 items"));
        }
        if self.stl_count == 0 {
            return Err(ValidationError::new("leaf projects must have stl_count > 0"));
        }
        Ok(())
    }
}
```

**Frontend (TypeScript)**:
```typescript
function validateSearchResultProject(project: SearchResultProject): void {
  if (!project.is_leaf) {
    throw new Error('Search results must only contain leaf projects');
  }
  if (project.images.length > 15) {
    console.warn(`Project ${project.id} has ${project.images.length} images (capped at 15)`);
  }
}
```

---

## API Data Flow

### Complete Request/Response Flow

```
┌─────────────┐
│   Client    │
│  (Search)   │
└──────┬──────┘
       │ GET /api/search?q=dwarf&leaf_only=true
       ↓
┌──────────────────────────────────────────────┐
│  Backend: search_projects() Handler          │
│  1. Parse query params                       │
│  2. Create SearchParams with leaf_only=true  │
└──────┬───────────────────────────────────────┘
       │
       ↓
┌──────────────────────────────────────────────┐
│  SearchService.search()                      │
│  1. Execute FTS query with is_leaf filter    │
│  2. Get project IDs from results             │
└──────┬───────────────────────────────────────┘
       │
       ↓
┌──────────────────────────────────────────────┐
│  For each project:                           │
│  1. Get parent chain (recursive CTE)         │
│  2. Get images from project + parents        │
│  3. Sort by priority, limit to 15            │
│  4. Get STL count                            │
│  5. Get tags                                 │
└──────┬───────────────────────────────────────┘
       │
       ↓
┌──────────────────────────────────────────────┐
│  Build SearchResponse                        │
│  {                                           │
│    data: [SearchResultProject, ...],         │
│    meta: { total, page, per_page, ... }      │
│  }                                           │
└──────┬───────────────────────────────────────┘
       │
       │ JSON response
       ↓
┌──────────────────────────────────────────────┐
│  Client: Render SearchPage                   │
│  1. Display ProjectGrid with tiles           │
│  2. Each tile shows SearchTileCarousel       │
│  3. Lazy load images as tiles scroll         │
└──────────────────────────────────────────────┘
```

---

## Summary

### Key Data Entities
1. **SearchResultProject**: Extended project model with embedded images and metadata
2. **ImagePreview**: Lightweight image data for carousel display
3. **SearchQueryParams**: Extended with `leaf_only` parameter
4. **SearchResponse**: Standard paginated response with metadata

### Schema Impact
- ✅ **No migrations required**: All necessary columns exist
- ✅ **Indexes optimized**: `is_leaf` and `image_priority` indexes already in place
- ✅ **Backward compatible**: Existing API clients unaffected

### Validation Rules
- Search results must have `is_leaf = true`
- Images limited to 15 per project
- Leaf projects must have `stl_count > 0`
- Image arrays sorted by priority DESC

### Performance Characteristics
- Single query for leaf filtering: O(log N) with index
- Recursive parent chain: O(log N), max depth 5
- Image aggregation: O(1) with composite index
- Batch loading: Single query for all search results
