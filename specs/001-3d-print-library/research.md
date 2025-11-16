# Research: 3D Print Model Library

**Date**: 2025-11-16  
**Feature**: 001-3d-print-library  
**Phase**: 0 - Outline & Research

## Executive Summary

This document consolidates research findings for key technology choices and integration patterns for the 3D Print Model Library application. All unknowns from the Technical Context have been investigated, and decisions have been made with documented rationale.

## 1. Frontend Framework: Next.js vs React + Vite

### Decision
**Use React + Vite** (not Next.js)

### Rationale
1. **No SSR/SSG Benefits**: The application is a local-first desktop web app communicating with a local Rust backend. Server-side rendering provides no value since:
   - No SEO requirements (not a public website)
   - No initial load performance gains (local API is already fast)
   - No static content generation needs

2. **Simpler Architecture**: Plain React + Vite avoids unnecessary complexity:
   - Direct client-server model matches the use case
   - No need for Next.js API routes (Rust backend handles all APIs)
   - Fewer configuration layers and build steps

3. **Better Development Experience**: Vite provides:
   - Faster HMR (Hot Module Replacement) than Next.js
   - Simpler configuration
   - Smaller bundle sizes for SPA architecture

4. **Local-First Nature**: The application runs entirely locally with no deployment complexity, making Next.js's deployment features irrelevant

### Alternatives Considered
- **Next.js**: Rejected due to overhead of features not needed (SSR, SSG, API routes, routing complexity)
- **Create React App**: Rejected due to slower build times compared to Vite
- **Plain React with custom build**: Rejected - Vite provides all needed tooling

## 2. STL Preview Generation with stl-thumb

### Decision
**Integrate stl-thumb as external command-line tool** via Rust `std::process::Command`

### Rationale
1. **Proven Solution**: stl-thumb is mature, widely used, and specifically designed for STL preview generation
2. **Clean Separation**: Keeps complex 3D rendering logic outside our codebase
3. **Configurability**: stl-thumb supports various output formats, sizes, and rendering options
4. **No Rust Bindings Needed**: While parsing STL in Rust is possible, rendering requires OpenGL/graphics libraries that are complex to integrate

### Implementation Pattern
```rust
// services/stl_preview.rs
use std::process::Command;
use std::path::{Path, PathBuf};

pub struct StlPreviewGenerator {
    stl_thumb_path: PathBuf,
    cache_dir: PathBuf,
}

impl StlPreviewGenerator {
    pub async fn generate_preview(&self, stl_path: &Path) -> Result<PathBuf> {
        let output_path = self.cache_dir.join(format!("{}.png", hash(stl_path)));
        
        let output = Command::new(&self.stl_thumb_path)
            .arg(stl_path)
            .arg(&output_path)
            .arg("--size=512")
            .output()?;
        
        if output.status.success() {
            Ok(output_path)
        } else {
            Err(PreviewError::GenerationFailed)
        }
    }
}
```

### Integration Requirements
- stl-thumb must be installed on the system (document in README)
- Fallback to placeholder image if stl-thumb not found
- Cache generated previews to avoid regeneration
- Consider async processing for large files

### Alternatives Considered
- **Pure Rust STL rendering**: Rejected due to complexity of implementing 3D rendering (OpenGL/Vulkan bindings, camera positioning, lighting)
- **JavaScript-based rendering (Three.js)**: Rejected - frontend would need to request raw STL files, increasing bandwidth and complexity
- **Pre-generated previews**: Rejected - users may not have previews for their existing collections

## 3. Image Caching Strategy

### Decision
**File system-based cache with database metadata tracking**

### Rationale
1. **Performance**: Direct file system access is faster than database BLOB storage
2. **Storage Efficiency**: Images remain on disk; no duplication into database
3. **Scalability**: Can handle large image collections without bloating database
4. **Flexibility**: Easy to clear cache, backup separately, or migrate storage

### Architecture
```
cache/
├── images/              # Discovered user images
│   └── {hash}.{ext}    # Hash of original path + extension
└── previews/            # Generated STL previews
    └── {hash}.png      # Hash of STL file path
```

### Database Schema Addition
```sql
CREATE TABLE cached_files (
    id INTEGER PRIMARY KEY,
    original_path TEXT NOT NULL UNIQUE,
    cache_path TEXT NOT NULL,
    file_type TEXT NOT NULL,  -- 'image' or 'preview'
    cached_at INTEGER NOT NULL,
    file_size INTEGER NOT NULL,
    checksum TEXT
);

CREATE INDEX idx_cached_files_original ON cached_files(original_path);
```

### Cache Management
- **On Scan**: Copy images to cache, record in database
- **On Access**: Serve from cache_path
- **On Rescan**: Verify original files still exist, update cache
- **Cleanup**: Remove cache entries for deleted source files

### Alternatives Considered
- **Database BLOBs**: Rejected - poor performance for large images, database bloat
- **No cache (serve directly)**: Rejected - file system access on every request is slower, no STL preview storage
- **CDN/Object Storage**: Rejected - unnecessary for local application

## 4. Streaming ZIP Downloads for Large Projects

### Decision
**Use async-zip with Axum streaming responses**

### Rationale
1. **Memory Efficiency**: Streaming prevents loading entire ZIP into memory
2. **Large File Support**: No browser memory limits, supports multi-GB projects
3. **Better UX**: Download starts immediately, progress visible to user
4. **Rust Ecosystem**: Well-supported with tokio async streams

### Implementation Pattern
```rust
use async_zip::write::ZipFileWriter;
use axum::body::StreamBody;
use tokio::io::AsyncWriteExt;
use futures::stream::StreamExt;

pub async fn download_project_zip(
    project_id: i64,
) -> Result<StreamBody<impl Stream<Item = Result<Bytes>>>> {
    let files = fetch_project_files(project_id).await?;
    
    let (tx, rx) = tokio::sync::mpsc::channel(100);
    
    tokio::spawn(async move {
        let mut zip_writer = ZipFileWriter::new(/* ... */);
        
        for file in files {
            let data = tokio::fs::read(&file.path).await?;
            zip_writer.write_entry_whole(
                file.name.as_str().into(),
                &data
            ).await?;
            
            tx.send(Ok(/* chunk */)).await?;
        }
        
        zip_writer.close().await?;
    });
    
    Ok(StreamBody::new(ReceiverStream::new(rx)))
}
```

### Crate Dependencies
- `async-zip = "0.0.16"` - Async ZIP creation
- `tokio = { version = "1.35", features = ["fs", "sync"] }` - Async file I/O
- `futures = "0.3"` - Stream utilities

### Alternatives Considered
- **zip crate (synchronous)**: Rejected - blocks thread, poor for large files
- **Pre-generate ZIP files**: Rejected - wastes disk space, stale data issues
- **Client-side ZIP generation**: Rejected - requires sending all files to browser first, defeats purpose

## 5. Hierarchical Project Data Model

### Decision
**Adjacency list with path-based queries**

### Rationale
1. **Simple to Implement**: Parent-child relationships via parent_id foreign key
2. **Efficient for Common Queries**: Most navigation is one level at a time
3. **Flexible Depth**: No arbitrary depth limits
4. **Path Caching**: Store full path for breadcrumb and search optimization

### Database Schema
```sql
CREATE TABLE projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    full_path TEXT NOT NULL UNIQUE,
    parent_id INTEGER,
    is_leaf BOOLEAN NOT NULL DEFAULT 0,  -- Has STL files
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (parent_id) REFERENCES projects(id) ON DELETE CASCADE
);

CREATE INDEX idx_projects_parent ON projects(parent_id);
CREATE INDEX idx_projects_path ON projects(full_path);
CREATE INDEX idx_projects_leaf ON projects(is_leaf);
```

### Query Patterns
```rust
// Get children of a folder
SELECT * FROM projects WHERE parent_id = ? ORDER BY name;

// Get breadcrumb trail
WITH RECURSIVE ancestors AS (
    SELECT id, name, parent_id FROM projects WHERE id = ?
    UNION ALL
    SELECT p.id, p.name, p.parent_id 
    FROM projects p JOIN ancestors a ON p.id = a.parent_id
)
SELECT * FROM ancestors ORDER BY id;

// Get all descendants (for deletion)
WITH RECURSIVE descendants AS (
    SELECT id FROM projects WHERE id = ?
    UNION ALL
    SELECT p.id FROM projects p JOIN descendants d ON p.parent_id = d.id
)
SELECT * FROM descendants;
```

### Alternatives Considered
- **Nested Sets**: Rejected - complex to maintain, overkill for simple navigation
- **Path Enumeration**: Rejected as primary - used as secondary index (full_path) for search
- **Closure Table**: Rejected - adds complexity with separate table for all ancestor-descendant pairs

## 6. Tag System and Search Functionality

### Decision
**Many-to-many relationship with full-text search via SQLite FTS5**

### Rationale
1. **Flexible Tagging**: Projects can have multiple tags, tags can apply to multiple projects
2. **Fast Search**: SQLite FTS5 provides efficient full-text search
3. **Simple Schema**: Standard junction table pattern
4. **Tag Suggestions**: Easy to query existing tags for autocomplete

### Database Schema
```sql
CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    created_at INTEGER NOT NULL
);

CREATE TABLE project_tags (
    project_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    PRIMARY KEY (project_id, tag_id),
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

-- Full-text search on project names
CREATE VIRTUAL TABLE projects_fts USING fts5(
    project_id UNINDEXED,
    name,
    tokenize='porter unicode61'
);

CREATE INDEX idx_project_tags_project ON project_tags(project_id);
CREATE INDEX idx_project_tags_tag ON project_tags(tag_id);
CREATE INDEX idx_tags_name ON tags(name);
```

### Search Queries
```rust
// Search by name
SELECT p.* FROM projects p
JOIN projects_fts fts ON p.id = fts.project_id
WHERE projects_fts MATCH ?
ORDER BY rank;

// Search by tag
SELECT DISTINCT p.* FROM projects p
JOIN project_tags pt ON p.id = pt.project_id
JOIN tags t ON pt.tag_id = t.id
WHERE t.name = ?;

// Search by name AND tags (combined)
SELECT DISTINCT p.* FROM projects p
JOIN projects_fts fts ON p.id = fts.project_id
JOIN project_tags pt ON p.id = pt.project_id
JOIN tags t ON pt.tag_id = t.id
WHERE projects_fts MATCH ? AND t.name IN (?, ?, ?)
ORDER BY rank;

// Tag autocomplete
SELECT name FROM tags 
WHERE name LIKE ? || '%' 
ORDER BY name LIMIT 10;
```

### Alternatives Considered
- **Single tags column (comma-separated)**: Rejected - poor query performance, no referential integrity
- **External search engine (Elasticsearch/MeiliSearch)**: Rejected - overkill for local app, adds deployment complexity
- **Separate tag table per project**: Rejected - violates normalization

## 7. API Design for Frontend-Backend Communication

### Decision
**RESTful JSON API with Axum**

### Rationale
1. **Standard and Simple**: REST is well-understood, easy to document
2. **Stateless**: Each request contains all needed information
3. **Axum Features**: Built-in JSON serialization, extractors, middleware
4. **TypeScript Integration**: Easy to generate types from OpenAPI spec

### API Structure
```
GET    /api/config                    # Get current root path
POST   /api/config                    # Set root path
POST   /api/scan                      # Trigger scan
GET    /api/scan/status               # Get scan progress

GET    /api/projects                  # List root projects
GET    /api/projects/:id              # Get project details
GET    /api/projects/:id/children     # Get child projects
GET    /api/projects/:id/files        # Get project files (paginated)

GET    /api/search?q=...&tags=...     # Search projects
GET    /api/tags                      # List all tags
POST   /api/projects/:id/tags         # Add tag to project
DELETE /api/projects/:id/tags/:tag    # Remove tag from project

GET    /api/files/:id                 # Download individual file
GET    /api/projects/:id/download     # Download project as ZIP

GET    /api/images/:hash              # Serve cached image
GET    /api/previews/:hash            # Serve STL preview
```

### Response Format
```typescript
// Success
{
  "data": { /* payload */ },
  "meta": { /* pagination, etc */ }
}

// Error
{
  "error": {
    "code": "PROJECT_NOT_FOUND",
    "message": "Project with id 123 not found",
    "details": { /* optional */ }
  }
}
```

### Alternatives Considered
- **GraphQL**: Rejected - overkill for simple CRUD operations, adds complexity
- **gRPC**: Rejected - not web-friendly, requires code generation
- **WebSocket**: Rejected as primary (used only for scan progress updates)

## 8. File System Scanning Best Practices in Rust

### Decision
**Async scanning with tokio::fs and walkdir**

### Rationale
1. **Performance**: Async I/O prevents blocking on slow file systems
2. **Cancellability**: Can abort long-running scans
3. **Progress Reporting**: Natural fit with async streams
4. **Error Handling**: Continue on permission errors, collect for reporting

### Implementation Pattern
```rust
use walkdir::WalkDir;
use tokio::fs;
use futures::stream::{self, StreamExt};

pub async fn scan_directory(
    root_path: &Path,
    progress_tx: mpsc::Sender<ScanProgress>,
) -> Result<Vec<ProjectMetadata>> {
    let entries: Vec<_> = WalkDir::new(root_path)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .collect();
    
    let projects = stream::iter(entries)
        .filter_map(|entry| async move {
            match entry {
                Ok(e) if e.file_type().is_dir() => {
                    Some(analyze_directory(e.path()).await)
                },
                Err(e) => {
                    warn!("Scan error: {}", e);
                    None
                }
                _ => None
            }
        })
        .buffer_unordered(10) // Process 10 dirs concurrently
        .collect()
        .await;
    
    Ok(projects)
}

async fn analyze_directory(path: &Path) -> Option<ProjectMetadata> {
    let mut stl_files = Vec::new();
    let mut images = Vec::new();
    
    let mut dir = fs::read_dir(path).await.ok()?;
    while let Some(entry) = dir.next_entry().await.ok()? {
        let path = entry.path();
        match path.extension()?.to_str()? {
            "stl" => stl_files.push(path),
            "jpg" | "jpeg" | "png" | "gif" | "webp" => images.push(path),
            _ => {}
        }
    }
    
    if !stl_files.is_empty() {
        Some(ProjectMetadata { stl_files, images, /* ... */ })
    } else {
        None
    }
}
```

### Error Handling Strategy
- Log permission errors, continue scanning
- Skip unreadable files, track count
- Report summary of errors after scan completes
- Don't fail entire scan for individual file errors

### Alternatives Considered
- **Synchronous walkdir**: Rejected - blocks thread during I/O
- **ignore crate**: Considered for .gitignore support, deferred to future
- **notify crate for file watching**: Deferred to future for auto-rescan

## 9. SQLite Optimization for This Use Case

### Decision
**Use rusqlite with connection pooling and WAL mode**

### Rationale
1. **WAL Mode**: Write-Ahead Logging improves concurrent read performance
2. **Connection Pool**: r2d2-sqlite provides thread-safe connections
3. **Pragmas**: Optimize for read-heavy workload
4. **Prepared Statements**: Reuse compiled queries for performance

### Configuration
```rust
use rusqlite::{Connection, OpenFlags};
use r2d2_sqlite::SqliteConnectionManager;

pub fn create_pool(db_path: &Path) -> Result<r2d2::Pool<SqliteConnectionManager>> {
    let manager = SqliteConnectionManager::file(db_path)
        .with_flags(OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE)
        .with_init(|conn| {
            // Enable WAL mode for better concurrency
            conn.execute_batch("
                PRAGMA journal_mode = WAL;
                PRAGMA synchronous = NORMAL;
                PRAGMA cache_size = -64000;  -- 64MB cache
                PRAGMA temp_store = MEMORY;
                PRAGMA mmap_size = 30000000000;  -- 30GB mmap
                PRAGMA page_size = 4096;
                PRAGMA foreign_keys = ON;
            ")?;
            Ok(())
        });
    
    r2d2::Pool::builder()
        .max_size(10)
        .build(manager)
}
```

### Index Strategy
- Index all foreign keys
- Index search columns (name, full_path)
- Index frequently filtered columns (parent_id, is_leaf)
- FTS5 for full-text search

### Alternatives Considered
- **PostgreSQL**: Rejected - overkill, requires separate server process
- **sled**: Rejected - less mature, more complex than SQLite for this use case
- **JSON files**: Rejected - no query capabilities, poor for relationships

## 10. Image Pagination Implementation

### Decision
**Cursor-based pagination with LIMIT/OFFSET**

### Rationale
1. **Simple Implementation**: Standard SQL LIMIT/OFFSET
2. **Sufficient for Use Case**: Image lists are relatively stable
3. **Frontend Compatibility**: Easy to implement with React

### API Response Format
```json
{
  "data": [
    {
      "id": 1,
      "path": "/cache/images/abc123.jpg",
      "filename": "photo1.jpg",
      "size": 2048000,
      "thumbnail_url": "/api/images/abc123?size=thumb"
    }
  ],
  "meta": {
    "page": 1,
    "per_page": 20,
    "total": 150,
    "total_pages": 8
  }
}
```

### Query Pattern
```rust
pub async fn get_project_images(
    project_id: i64,
    page: u32,
    per_page: u32,
) -> Result<(Vec<ImageFile>, PaginationMeta)> {
    let offset = (page - 1) * per_page;
    
    let images = sqlx::query_as!(
        ImageFile,
        "SELECT * FROM image_files 
         WHERE project_id = ? 
         ORDER BY filename 
         LIMIT ? OFFSET ?",
        project_id, per_page, offset
    ).fetch_all(db).await?;
    
    let total: i64 = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM image_files WHERE project_id = ?",
        project_id
    ).fetch_one(db).await?;
    
    Ok((images, PaginationMeta { page, per_page, total }))
}
```

### Alternatives Considered
- **Keyset pagination**: Rejected - more complex, not needed for stable datasets
- **Infinite scroll with cursor**: Considered for future UX improvement
- **Load all images**: Rejected - poor performance for projects with hundreds of images

## Summary of Key Dependencies

### Backend (Cargo.toml)
```toml
[dependencies]
# Web framework
axum = "0.7"
tokio = { version = "1.35", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["fs", "cors", "trace"] }

# Database
rusqlite = { version = "0.31", features = ["bundled"] }
r2d2 = "0.8"
r2d2_sqlite = "0.24"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Async & streaming
futures = "0.3"
async-zip = "0.0.16"
bytes = "1.5"

# File system
walkdir = "2.4"
sha2 = "0.10"  # For file hashing

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

[dev-dependencies]
tempfile = "3.8"
```

### Frontend (package.json)
```json
{
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "react-router-dom": "^6.20.0",
    "axios": "^1.6.0",
    "zustand": "^4.4.0"
  },
  "devDependencies": {
    "@types/react": "^18.2.0",
    "@types/react-dom": "^18.2.0",
    "@vitejs/plugin-react": "^4.2.0",
    "typescript": "^5.3.0",
    "vite": "^5.0.0",
    "vitest": "^1.0.0"
  }
}
```

## Implementation Phases

### Phase 1: Core Infrastructure (Week 1-2)
- Set up project structure (backend/frontend)
- Database schema and migrations
- Basic Axum server with CORS
- React app with Vite
- Basic routing and navigation

### Phase 2: File Scanning (Week 2-3)
- Implement recursive directory scanning
- STL file detection
- Image file detection and caching
- Database persistence of project hierarchy
- Progress reporting

### Phase 3: STL Preview Generation (Week 3)
- Integrate stl-thumb
- Preview caching
- Fallback to placeholder images
- Async generation queue

### Phase 4: Frontend UI (Week 4-5)
- Tile-based project grid
- Breadcrumb navigation
- Project detail page
- Image gallery with pagination
- Responsive layout

### Phase 5: Search & Tags (Week 5-6)
- Tag CRUD operations
- Search by name (FTS5)
- Search by tags
- Tag autocomplete
- Search results page

### Phase 6: Downloads (Week 6)
- Individual file downloads
- Streaming ZIP generation
- Progress indicators
- Error handling

### Phase 7: Rescan & Maintenance (Week 7)
- Rescan functionality
- Tag preservation
- Cache cleanup
- Error reporting

### Phase 8: Polish & Testing (Week 8)
- Error handling improvements
- Loading states
- Empty states
- Integration tests
- Documentation

## Open Questions

1. **stl-thumb installation**: Should we bundle stl-thumb or require users to install it separately?
   - **Recommendation**: Document manual installation initially, consider bundling in future releases

2. **Image thumbnail generation**: Should we generate thumbnails for faster loading?
   - **Recommendation**: Yes, add thumbnail generation in Phase 4, store at 200x200px

3. **Configuration UI**: Where should users configure the root path?
   - **Recommendation**: Simple settings page accessible from home, persist in SQLite

4. **Maximum folder depth**: Should we limit recursion depth?
   - **Recommendation**: Set soft limit of 10 levels with warning, hard limit of 20

5. **Progress WebSocket vs Polling**: How to report scan progress?
   - **Recommendation**: Start with polling (simpler), migrate to WebSocket if performance issues

## Conclusion

All technical unknowns have been resolved. The technology stack is well-suited for a local-first web application with the following highlights:
- React + Vite provides optimal frontend development experience
- Rust + Axum delivers high-performance backend
- SQLite is perfect for local data storage
- stl-thumb integration solves STL preview generation
- Streaming architecture handles large files efficiently

The architecture is clean, maintainable, and meets all functional requirements specified in the feature spec.
