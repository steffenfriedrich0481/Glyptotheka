# Data Model: 3D Print Model Library

**Date**: 2025-11-16  
**Feature**: 001-3d-print-library  
**Phase**: 1 - Design & Contracts

## Overview

This document defines the complete data model for the 3D Print Model Library, including SQLite schema, entity relationships, validation rules, and state transitions.

## Entity-Relationship Diagram

```
┌─────────────┐         ┌──────────────┐         ┌─────────┐
│  projects   │◄──────┐ │ project_tags │◄────────┤  tags   │
│             │       │ │              │         │         │
│ id (PK)     │       │ │ project_id   │         │ id (PK) │
│ name        │       │ │ tag_id       │         │ name    │
│ full_path   │       │ └──────────────┘         └─────────┘
│ parent_id   │       │
│ is_leaf     │       │
└──────┬──────┘       │
       │              │
       │ parent_id    │
       │              │
       └──────────────┘
       
┌─────────────┐         ┌──────────────┐         ┌──────────────┐
│  stl_files  │         │ image_files  │         │ cached_files │
│             │         │              │         │              │
│ id (PK)     │         │ id (PK)      │         │ id (PK)      │
│ project_id  ├────┐    │ project_id   ├────┐    │ original_path│
│ filename    │    │    │ filename     │    │    │ cache_path   │
│ file_path   │    │    │ file_path    │    │    │ file_type    │
│ file_size   │    │    │ file_size    │    │    └──────────────┘
└─────────────┘    │    │ source_type  │    │
                   │    └──────────────┘    │
                   │                        │
                   └────────┬───────────────┘
                            │
                            ▼
                      ┌──────────┐
                      │ projects │
                      └──────────┘

┌────────────────┐
│ scan_sessions  │
│                │
│ id (PK)        │
│ root_path      │
│ status         │
│ started_at     │
│ completed_at   │
│ projects_found │
│ errors_count   │
└────────────────┘
```

## Database Schema

### 1. projects

Represents folders in the hierarchy. A project is "leaf" if it contains STL files.

```sql
CREATE TABLE projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    full_path TEXT NOT NULL UNIQUE,
    parent_id INTEGER,
    is_leaf BOOLEAN NOT NULL DEFAULT 0,
    description TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    
    FOREIGN KEY (parent_id) REFERENCES projects(id) ON DELETE CASCADE,
    
    CHECK (length(name) > 0),
    CHECK (length(full_path) > 0),
    CHECK (created_at > 0),
    CHECK (updated_at >= created_at)
);

CREATE INDEX idx_projects_parent ON projects(parent_id);
CREATE INDEX idx_projects_path ON projects(full_path);
CREATE INDEX idx_projects_leaf ON projects(is_leaf);
CREATE INDEX idx_projects_name ON projects(name);

-- Full-text search virtual table
CREATE VIRTUAL TABLE projects_fts USING fts5(
    project_id UNINDEXED,
    name,
    full_path,
    tokenize='porter unicode61'
);

-- Trigger to keep FTS in sync
CREATE TRIGGER projects_fts_insert AFTER INSERT ON projects
BEGIN
    INSERT INTO projects_fts(project_id, name, full_path)
    VALUES (NEW.id, NEW.name, NEW.full_path);
END;

CREATE TRIGGER projects_fts_update AFTER UPDATE ON projects
BEGIN
    UPDATE projects_fts 
    SET name = NEW.name, full_path = NEW.full_path
    WHERE project_id = NEW.id;
END;

CREATE TRIGGER projects_fts_delete AFTER DELETE ON projects
BEGIN
    DELETE FROM projects_fts WHERE project_id = OLD.id;
END;
```

**Validation Rules**:
- `name` must not be empty
- `full_path` must be absolute and unique
- `parent_id` must reference existing project or be NULL (root)
- `is_leaf` is TRUE when project contains STL files
- `updated_at` must be >= `created_at`

**Business Rules**:
- Root projects have `parent_id = NULL`
- Deleting a project cascades to all children
- A project can be both a container (has children) and a leaf (has STL files)

### 2. stl_files

Represents individual STL files within projects.

```sql
CREATE TABLE stl_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    filename TEXT NOT NULL,
    file_path TEXT NOT NULL UNIQUE,
    file_size INTEGER NOT NULL,
    preview_path TEXT,
    preview_generated_at INTEGER,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    
    CHECK (length(filename) > 0),
    CHECK (length(file_path) > 0),
    CHECK (file_size >= 0),
    CHECK (created_at > 0),
    CHECK (updated_at >= created_at)
);

CREATE INDEX idx_stl_files_project ON stl_files(project_id);
CREATE INDEX idx_stl_files_path ON stl_files(file_path);
```

**Validation Rules**:
- `filename` must end with `.stl` (case-insensitive)
- `file_path` must be absolute and unique
- `file_size` must be >= 0
- `preview_path` is NULL until preview is generated

**Business Rules**:
- Each STL file belongs to exactly one project
- Preview generation is async; `preview_path` populated when complete
- `preview_generated_at` tracks cache freshness

### 3. image_files

Represents image files associated with projects (own or inherited).

```sql
CREATE TABLE image_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    filename TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    source_type TEXT NOT NULL DEFAULT 'direct',
    source_project_id INTEGER,
    display_order INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (source_project_id) REFERENCES projects(id) ON DELETE CASCADE,
    
    CHECK (length(filename) > 0),
    CHECK (length(file_path) > 0),
    CHECK (file_size >= 0),
    CHECK (source_type IN ('direct', 'inherited')),
    CHECK (created_at > 0),
    CHECK (updated_at >= created_at),
    CHECK (
        (source_type = 'direct' AND source_project_id IS NULL) OR
        (source_type = 'inherited' AND source_project_id IS NOT NULL)
    )
);

CREATE INDEX idx_image_files_project ON image_files(project_id);
CREATE INDEX idx_image_files_path ON image_files(file_path);
CREATE INDEX idx_image_files_source ON image_files(source_project_id);
CREATE INDEX idx_image_files_order ON image_files(project_id, display_order);
```

**Validation Rules**:
- `filename` must have valid image extension (jpg, jpeg, png, gif, webp)
- `file_path` must exist in filesystem
- `source_type` must be 'direct' or 'inherited'
- If `source_type = 'inherited'`, `source_project_id` must be set
- If `source_type = 'direct'`, `source_project_id` must be NULL

**Business Rules**:
- `direct` images are found in the project's own folder
- `inherited` images come from parent folders
- `source_project_id` tracks which parent provided the image
- `display_order` controls presentation order (0 = first)

### 4. cached_files

Tracks cached copies of user images and generated STL previews.

```sql
CREATE TABLE cached_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    original_path TEXT NOT NULL UNIQUE,
    cache_path TEXT NOT NULL UNIQUE,
    file_type TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    checksum TEXT,
    cached_at INTEGER NOT NULL,
    accessed_at INTEGER NOT NULL,
    
    CHECK (length(original_path) > 0),
    CHECK (length(cache_path) > 0),
    CHECK (file_type IN ('image', 'preview')),
    CHECK (file_size >= 0),
    CHECK (cached_at > 0),
    CHECK (accessed_at >= cached_at)
);

CREATE INDEX idx_cached_files_original ON cached_files(original_path);
CREATE INDEX idx_cached_files_type ON cached_files(file_type);
CREATE INDEX idx_cached_files_accessed ON cached_files(accessed_at);
```

**Validation Rules**:
- `original_path` must be unique (one cache entry per source file)
- `cache_path` must be unique (no duplicate cache files)
- `file_type` must be 'image' or 'preview'
- `checksum` is SHA256 hash for integrity verification

**Business Rules**:
- Cache entries persist across rescans
- `accessed_at` updated on each access for LRU cache management
- Orphaned cache files (original deleted) cleaned up during rescan

### 5. tags

Represents user-defined labels for organizing projects.

```sql
CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE COLLATE NOCASE,
    color TEXT,
    created_at INTEGER NOT NULL,
    usage_count INTEGER NOT NULL DEFAULT 0,
    
    CHECK (length(name) > 0 AND length(name) <= 50),
    CHECK (usage_count >= 0)
);

CREATE INDEX idx_tags_name ON tags(name COLLATE NOCASE);
CREATE INDEX idx_tags_usage ON tags(usage_count DESC);
```

**Validation Rules**:
- `name` must be 1-50 characters
- `name` is case-insensitive unique (e.g., "Painted" = "painted")
- `color` is optional hex color code (#RRGGBB)

**Business Rules**:
- `usage_count` tracks how many projects use this tag
- Updated via triggers on project_tags insert/delete
- Tags with zero usage are kept (user may reuse later)

### 6. project_tags

Junction table for many-to-many relationship between projects and tags.

```sql
CREATE TABLE project_tags (
    project_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    
    PRIMARY KEY (project_id, tag_id),
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

CREATE INDEX idx_project_tags_project ON project_tags(project_id);
CREATE INDEX idx_project_tags_tag ON project_tags(tag_id);

-- Trigger to maintain tag usage counts
CREATE TRIGGER project_tags_insert AFTER INSERT ON project_tags
BEGIN
    UPDATE tags SET usage_count = usage_count + 1 WHERE id = NEW.tag_id;
END;

CREATE TRIGGER project_tags_delete AFTER DELETE ON project_tags
BEGIN
    UPDATE tags SET usage_count = usage_count - 1 WHERE id = OLD.tag_id;
END;
```

**Validation Rules**:
- A project-tag pair must be unique (composite primary key)
- Both `project_id` and `tag_id` must reference existing records

**Business Rules**:
- Many-to-many: projects can have multiple tags, tags can apply to multiple projects
- Cascade delete: removing project removes its tags associations
- Cascade delete: removing tag removes all associations

### 7. scan_sessions

Tracks file system scanning operations for audit and debugging.

```sql
CREATE TABLE scan_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    root_path TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'running',
    started_at INTEGER NOT NULL,
    completed_at INTEGER,
    projects_found INTEGER DEFAULT 0,
    files_processed INTEGER DEFAULT 0,
    errors_count INTEGER DEFAULT 0,
    error_log TEXT,
    
    CHECK (status IN ('running', 'completed', 'failed', 'cancelled')),
    CHECK (projects_found >= 0),
    CHECK (files_processed >= 0),
    CHECK (errors_count >= 0),
    CHECK (completed_at IS NULL OR completed_at >= started_at)
);

CREATE INDEX idx_scan_sessions_status ON scan_sessions(status);
CREATE INDEX idx_scan_sessions_started ON scan_sessions(started_at DESC);
```

**Validation Rules**:
- `status` must be one of: running, completed, failed, cancelled
- `completed_at` must be NULL (running) or >= `started_at`
- Counts must be non-negative

**Business Rules**:
- New scan creates a session with status='running'
- On completion: status='completed', `completed_at` set
- On error: status='failed', errors logged in `error_log`
- User cancellation: status='cancelled'

### 8. config

Stores application configuration (single row table).

```sql
CREATE TABLE config (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    root_path TEXT,
    last_scan_at INTEGER,
    stl_thumb_path TEXT,
    cache_max_size_mb INTEGER DEFAULT 5000,
    images_per_page INTEGER DEFAULT 20,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Insert default config
INSERT INTO config (id, cache_max_size_mb, images_per_page, created_at, updated_at)
VALUES (1, 5000, 20, strftime('%s', 'now'), strftime('%s', 'now'));
```

**Validation Rules**:
- Only one row allowed (id = 1)
- `cache_max_size_mb` must be > 0
- `images_per_page` must be between 1 and 100

**Business Rules**:
- Single-row table pattern for global config
- `root_path` is NULL until user specifies it
- `last_scan_at` tracks most recent successful scan

## Entity Details

### Project Entity

**Rust Structure**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub full_path: String,
    pub parent_id: Option<i64>,
    pub is_leaf: bool,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectWithRelations {
    #[serde(flatten)]
    pub project: Project,
    pub parent: Option<Box<Project>>,
    pub children: Vec<Project>,
    pub stl_files: Vec<StlFile>,
    pub images: Vec<ImageFile>,
    pub tags: Vec<Tag>,
}
```

**State Transitions**:
1. **Created** → Initial state after discovery during scan
2. **Updated** → File system changes detected during rescan
3. **Deleted** → Source folder no longer exists

### StlFile Entity

**Rust Structure**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StlFile {
    pub id: i64,
    pub project_id: i64,
    pub filename: String,
    pub file_path: String,
    pub file_size: i64,
    pub preview_path: Option<String>,
    pub preview_generated_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

pub enum PreviewStatus {
    NotGenerated,
    Generating,
    Ready(String),  // path
    Failed(String), // error
}
```

**State Transitions**:
1. **Discovered** → STL file found during scan, no preview
2. **Preview Queued** → Added to preview generation queue
3. **Preview Generating** → stl-thumb processing
4. **Preview Ready** → Preview image cached and available
5. **Preview Failed** → Generation error, fallback to placeholder

### ImageFile Entity

**Rust Structure**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageFile {
    pub id: i64,
    pub project_id: i64,
    pub filename: String,
    pub file_path: String,
    pub file_size: i64,
    pub source_type: ImageSourceType,
    pub source_project_id: Option<i64>,
    pub display_order: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageSourceType {
    Direct,
    Inherited,
}
```

**State Transitions**:
1. **Discovered** → Image found during scan
2. **Cached** → Copied to cache directory
3. **Inherited** → Associated with child project
4. **Removed** → Source file deleted

### Tag Entity

**Rust Structure**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub color: Option<String>,
    pub created_at: i64,
    pub usage_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagProjectRequest {
    pub tag_name: String,
}
```

**Validation**:
- Name: 1-50 characters, trimmed, case-insensitive unique
- Color: Optional, must be valid hex (#RRGGBB) if provided

## Relationships

### 1. Project Hierarchy (Self-Referencing)
- **Type**: One-to-Many (parent to children)
- **Implementation**: `parent_id` foreign key
- **Cascade**: Delete parent → delete all children
- **Queries**: Recursive CTEs for ancestors/descendants

### 2. Project ← STL Files
- **Type**: One-to-Many
- **Cascade**: Delete project → delete all STL files
- **Business Rule**: A project with `is_leaf = true` must have at least one STL file

### 3. Project ← Image Files
- **Type**: One-to-Many
- **Cascade**: Delete project → delete all image associations
- **Business Rule**: Images can be direct or inherited from parent

### 4. Project ↔ Tags (Many-to-Many)
- **Type**: Many-to-Many via `project_tags`
- **Cascade**: Delete project → remove tag associations
- **Cascade**: Delete tag → remove all project associations
- **Business Rule**: Tags persist across rescans

### 5. Cached Files ← Original Files
- **Type**: One-to-One (one cache entry per original)
- **Business Rule**: Cache survives rescan; cleaned up if original missing

## Indexes Strategy

### Primary Indexes
- All primary keys are auto-incrementing integers
- Unique constraints on natural keys (paths, names where appropriate)

### Foreign Key Indexes
- Index all foreign keys for join performance
- `idx_stl_files_project`, `idx_image_files_project`, etc.

### Search Indexes
- FTS5 virtual table for full-text search on project names
- `idx_tags_name` for tag autocomplete
- `idx_projects_path` for breadcrumb queries

### Performance Indexes
- `idx_image_files_order` for paginated image queries
- `idx_cached_files_accessed` for LRU cache eviction
- `idx_scan_sessions_started` for recent scan history

## Migration Strategy

### Initial Schema (v1)
```sql
-- Run all CREATE TABLE statements above
-- Run all CREATE INDEX statements
-- Run all CREATE TRIGGER statements
-- Insert default config row
```

### Future Migrations
Migrations will be versioned and applied sequentially:

```rust
pub struct Migration {
    pub version: u32,
    pub description: &'static str,
    pub up: &'static str,
    pub down: &'static str,
}

pub const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        description: "Initial schema",
        up: include_str!("migrations/001_initial.sql"),
        down: include_str!("migrations/001_initial_down.sql"),
    },
    // Future migrations...
];
```

## Validation Rules Summary

| Entity | Field | Rule |
|--------|-------|------|
| projects | name | Not empty, max 255 chars |
| projects | full_path | Absolute path, unique |
| projects | parent_id | NULL or valid project.id |
| stl_files | filename | Ends with .stl |
| stl_files | file_size | >= 0 |
| image_files | source_type | 'direct' or 'inherited' |
| image_files | source_project_id | NULL if direct, NOT NULL if inherited |
| tags | name | 1-50 chars, case-insensitive unique |
| tags | color | NULL or #RRGGBB format |
| cached_files | file_type | 'image' or 'preview' |

## Query Patterns

### 1. Get Project with Children
```sql
SELECT p.*, 
       (SELECT COUNT(*) FROM projects WHERE parent_id = p.id) as child_count,
       (SELECT COUNT(*) FROM stl_files WHERE project_id = p.id) as stl_count
FROM projects p
WHERE p.id = ?;

SELECT * FROM projects WHERE parent_id = ? ORDER BY name;
```

### 2. Get Breadcrumb Trail
```sql
WITH RECURSIVE ancestors AS (
    SELECT id, name, parent_id, 0 as level FROM projects WHERE id = ?
    UNION ALL
    SELECT p.id, p.name, p.parent_id, a.level + 1
    FROM projects p JOIN ancestors a ON p.id = a.parent_id
)
SELECT * FROM ancestors ORDER BY level DESC;
```

### 3. Search Projects by Name
```sql
SELECT p.*, rank
FROM projects p
JOIN projects_fts fts ON p.id = fts.project_id
WHERE projects_fts MATCH ?
ORDER BY rank
LIMIT ? OFFSET ?;
```

### 4. Search Projects by Tag
```sql
SELECT DISTINCT p.*
FROM projects p
JOIN project_tags pt ON p.id = pt.project_id
JOIN tags t ON pt.tag_id = t.id
WHERE t.name IN (?, ?, ?)
ORDER BY p.name
LIMIT ? OFFSET ?;
```

### 5. Get Project Images (Paginated)
```sql
SELECT * FROM image_files
WHERE project_id = ?
ORDER BY display_order, filename
LIMIT ? OFFSET ?;
```

### 6. Get All Project Files for ZIP
```sql
-- STL files
SELECT file_path, filename FROM stl_files WHERE project_id = ?;

-- Direct images
SELECT file_path, filename FROM image_files 
WHERE project_id = ? AND source_type = 'direct';
```

## Performance Considerations

1. **Recursive Queries**: Use `LIMIT` to prevent runaway recursion in deep hierarchies
2. **FTS5 Performance**: Maintain fresh statistics with `ANALYZE` periodically
3. **Pagination**: Use indexed columns in ORDER BY clauses
4. **Cascade Deletes**: May be slow for projects with many children; consider background jobs
5. **Cache Cleanup**: Run periodic maintenance to remove orphaned cache entries

## Data Integrity

### Foreign Key Constraints
- Enabled via `PRAGMA foreign_keys = ON`
- All relationships enforced at database level
- Cascade deletes prevent orphaned records

### Triggers
- FTS sync triggers keep search index current
- Usage count triggers maintain tag statistics
- All triggers are idempotent and transaction-safe

### Checksums
- SHA256 checksums for cached files
- Verify integrity on access
- Detect file system corruption or manual edits

### Transactions
- All multi-table operations wrapped in transactions
- Atomic scans: all-or-nothing project discovery
- Tag operations always update usage counts atomically

## TypeScript Types (Frontend)

```typescript
// types/project.ts
export interface Project {
  id: number;
  name: string;
  fullPath: string;
  parentId: number | null;
  isLeaf: boolean;
  description: string | null;
  createdAt: number;
  updatedAt: number;
}

export interface ProjectWithChildren extends Project {
  children: Project[];
  stlCount: number;
  imageCount: number;
  tags: Tag[];
}

export interface StlFile {
  id: number;
  projectId: number;
  filename: string;
  filePath: string;
  fileSize: number;
  previewPath: string | null;
  previewGeneratedAt: number | null;
}

export interface ImageFile {
  id: number;
  projectId: number;
  filename: string;
  filePath: string;
  fileSize: number;
  sourceType: 'direct' | 'inherited';
  sourceProjectId: number | null;
  displayOrder: number;
}

export interface Tag {
  id: number;
  name: string;
  color: string | null;
  usageCount: number;
}

export interface ScanSession {
  id: number;
  rootPath: string;
  status: 'running' | 'completed' | 'failed' | 'cancelled';
  startedAt: number;
  completedAt: number | null;
  projectsFound: number;
  filesProcessed: number;
  errorsCount: number;
}
```

## Summary

This data model provides:
- ✅ Hierarchical project organization with self-referencing
- ✅ Efficient search with FTS5 and tag indexing
- ✅ Image inheritance from parent folders
- ✅ Robust caching with integrity checks
- ✅ Flexible tagging system
- ✅ Audit trail via scan sessions
- ✅ Clean separation of concerns (projects, files, metadata)
- ✅ Strong referential integrity with cascading deletes
- ✅ Optimized for read-heavy workload
- ✅ Supports all functional requirements from spec.md
