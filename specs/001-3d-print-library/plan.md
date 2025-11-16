# Implementation Plan: 3D Print Model Library

**Branch**: `001-3d-print-library` | **Date**: 2025-11-16 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-3d-print-library/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

A web-based 3D print model library application that scans a local file system for STL files and associated images, presents them in a hierarchical tile-based interface with preview images, supports search and tagging, and enables file downloads. Built with Rust/Axum backend, React/TypeScript frontend, and SQLite database for local-first operation.

## Technical Context

**Language/Version**: Rust 1.75+ (backend), TypeScript 5.0+ (frontend)
**Primary Dependencies**: 
  - Backend: Axum (web framework), rusqlite (SQLite), tokio (async runtime), tower-http (middleware)
  - Frontend: React 18+, TypeScript 5.0+, Vite (build tool)
  - STL Processing: stl-thumb (external git repo: git@github.com:unlimitedbacon/stl-thumb.git)
**Storage**: SQLite (local file-based database via rusqlite)
**Testing**: cargo test (backend), vitest (frontend)
**Target Platform**: Desktop web application (Linux/macOS/Windows), modern browsers (Chrome 90+, Firefox 88+, Safari 14+)
**Project Type**: Web (separate backend and frontend)
**Performance Goals**: 
  - Scan and index 100+ projects per minute
  - Search results in <1 second for 10k projects
  - Tile navigation displays in <2 seconds
  - ZIP generation starts in <10 seconds for 50-file projects
**Constraints**: 
  - Local-first: all data stored locally, no cloud requirements
  - Single-user: no authentication or multi-user concurrency needed
  - Streaming ZIP downloads to avoid browser memory limits
  - Image pagination at 20 images per page
**Scale/Scope**: 
  - Support 100-10,000 projects
  - Recursive folder scanning
  - Multiple file types (STL, JPG, PNG, GIF, WebP)
  - Hierarchical folder structures up to reasonable depth

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Analysis**: The Glyptotheka constitution appears to be specific to the Fraktor-RS actor runtime system with no_std requirements, embedded targets, and strict module structure. This 3D Print Library feature is a separate web application and does not appear to be part of the Fraktor-RS project.

**Evaluation**:
- ✅ **I. no_std Core**: Not applicable - this is a standalone web application, not part of Fraktor-RS
- ✅ **II. Test Integrity**: Applicable - will maintain green CI checks with cargo test and vitest
- ❌ **III. Reference-Consistent Design**: Not applicable - no reference to protoactor-go or Apache Pekko needed
- ❌ **IV. Module Structure**: Not applicable - different project with web app structure conventions
- ✅ **V. Design Evolution**: Applicable - pre-release feature, breaking changes acceptable with documentation
- ❌ **VI. Inductive Consistency**: Not applicable - this is a new feature in a separate domain
- ❌ **VII. Lifetime-First Design**: Not applicable - web application context where std is available

**Conclusion**: This appears to be a separate application from the Fraktor-RS actor system. The constitution's embedded/no_std requirements and actor-specific design principles do not apply. We will follow:
- Test-driven development with green CI
- Clear documentation
- Standard Rust/web best practices

If this feature is intended to be part of Fraktor-RS modules, please clarify the integration points.

## Project Structure

### Documentation (this feature)

```text
specs/001-3d-print-library/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
│   ├── openapi.yaml     # REST API specification
│   └── schemas/         # JSON schemas for data models
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
backend/
├── Cargo.toml
├── src/
│   ├── main.rs              # Application entry point
│   ├── config.rs            # Configuration management
│   ├── models/              # Database models and entities
│   │   ├── mod.rs
│   │   ├── project.rs       # Project entity
│   │   ├── stl_file.rs      # STL file entity
│   │   ├── image_file.rs    # Image file entity
│   │   └── tag.rs           # Tag entity
│   ├── db/                  # Database layer
│   │   ├── mod.rs
│   │   ├── connection.rs    # SQLite connection management
│   │   ├── migrations.rs    # Schema migrations
│   │   └── repositories/    # Data access layer
│   │       ├── mod.rs
│   │       ├── project_repo.rs
│   │       ├── file_repo.rs
│   │       └── tag_repo.rs
│   ├── services/            # Business logic
│   │   ├── mod.rs
│   │   ├── scanner.rs       # File system scanning
│   │   ├── image_cache.rs   # Image caching management
│   │   ├── stl_preview.rs   # STL preview generation (stl-thumb integration)
│   │   ├── search.rs        # Search functionality
│   │   ├── download.rs      # File download and ZIP creation
│   │   └── rescan.rs        # Rescan operations
│   ├── api/                 # HTTP API handlers
│   │   ├── mod.rs
│   │   ├── routes.rs        # Route definitions
│   │   ├── handlers/        # Request handlers
│   │   │   ├── mod.rs
│   │   │   ├── projects.rs  # Project CRUD and navigation
│   │   │   ├── search.rs    # Search endpoints
│   │   │   ├── tags.rs      # Tag management
│   │   │   ├── files.rs     # File serving and downloads
│   │   │   └── scan.rs      # Scanning operations
│   │   └── middleware/      # Custom middleware
│   │       ├── mod.rs
│   │       ├── cors.rs
│   │       └── error.rs     # Error handling
│   └── utils/               # Utilities
│       ├── mod.rs
│       ├── error.rs         # Error types
│       └── pagination.rs    # Pagination helpers
├── tests/
│   ├── integration/         # Integration tests
│   │   ├── api_tests.rs
│   │   └── scan_tests.rs
│   └── fixtures/            # Test data
│       └── sample_stls/
├── cache/                   # Image cache directory (gitignored)
└── glyptotheka.db           # SQLite database (gitignored)

frontend/
├── package.json
├── vite.config.ts
├── tsconfig.json
├── index.html
├── src/
│   ├── main.tsx             # Application entry point
│   ├── App.tsx              # Root component
│   ├── types/               # TypeScript types
│   │   ├── index.ts
│   │   ├── project.ts
│   │   ├── file.ts
│   │   └── api.ts
│   ├── api/                 # API client
│   │   ├── client.ts        # Axios/fetch configuration
│   │   ├── projects.ts      # Project API calls
│   │   ├── search.ts        # Search API calls
│   │   ├── tags.ts          # Tag API calls
│   │   └── files.ts         # File API calls
│   ├── components/          # Reusable components
│   │   ├── common/          # Generic components
│   │   │   ├── Tile.tsx
│   │   │   ├── Breadcrumb.tsx
│   │   │   ├── SearchBar.tsx
│   │   │   ├── TagInput.tsx
│   │   │   ├── Pagination.tsx
│   │   │   └── LoadingSpinner.tsx
│   │   └── project/         # Project-specific components
│   │       ├── ProjectGrid.tsx
│   │       ├── ProjectTile.tsx
│   │       ├── ProjectDetail.tsx
│   │       ├── FileList.tsx
│   │       └── ImageGallery.tsx
│   ├── pages/               # Page components
│   │   ├── HomePage.tsx     # Root/setup page
│   │   ├── BrowsePage.tsx   # Browsing interface
│   │   ├── SearchPage.tsx   # Search results
│   │   └── ProjectPage.tsx  # Project detail page
│   ├── hooks/               # Custom React hooks
│   │   ├── useProjects.ts
│   │   ├── useSearch.ts
│   │   ├── useTags.ts
│   │   └── useNavigation.ts
│   ├── store/               # State management (Context API or Zustand)
│   │   ├── index.ts
│   │   ├── navigationContext.ts
│   │   └── searchContext.ts
│   └── utils/               # Utility functions
│       ├── formatters.ts
│       └── validators.ts
└── tests/
    ├── unit/                # Component unit tests
    └── e2e/                 # End-to-end tests (optional)

.github/
└── workflows/
    └── ci.yml               # CI/CD pipeline
```

**Structure Decision**: Web application structure with separate backend and frontend. Backend uses standard Rust web service organization with clear separation of concerns: models (data structures), db (persistence), services (business logic), and api (HTTP layer). Frontend follows React best practices with components, pages, hooks, and API client separation. This structure supports independent development and testing of each tier.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

N/A - Constitution does not apply to this standalone web application.

---

## Architecture Overview

### System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         Browser (Client)                        │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │              React Application (Port 5173)                │  │
│  │  - Component-based UI (tiles, galleries, forms)          │  │
│  │  - React Router for navigation                           │  │
│  │  - Zustand for state management                          │  │
│  │  - Axios for HTTP client                                 │  │
│  └───────────────────────┬──────────────────────────────────┘  │
└────────────────────────────┼───────────────────────────────────┘
                             │ REST API (JSON)
                             │ HTTP/HTTPS
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Axum Web Server (Port 3000)                  │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                   API Layer (Handlers)                   │   │
│  │  GET /api/projects, /api/search, /api/files, etc.       │   │
│  └───────────────────────┬─────────────────────────────────┘   │
│                          │                                      │
│  ┌───────────────────────┴─────────────────────────────────┐   │
│  │                  Services Layer (Business Logic)         │   │
│  │  ┌──────────┐  ┌───────────┐  ┌─────────┐  ┌─────────┐ │   │
│  │  │ Scanner  │  │STL Preview│  │ Search  │  │Download │ │   │
│  │  │ Service  │  │  Service  │  │ Service │  │ Service │ │   │
│  │  └──────────┘  └───────────┘  └─────────┘  └─────────┘ │   │
│  └───────────────────────┬─────────────────────────────────┘   │
│                          │                                      │
│  ┌───────────────────────┴─────────────────────────────────┐   │
│  │              Repository Layer (Data Access)              │   │
│  │  ProjectRepo, FileRepo, TagRepo, CacheRepo               │   │
│  └───────────────────────┬─────────────────────────────────┘   │
└──────────────────────────┼──────────────────────────────────────┘
                           │
        ┌──────────────────┼──────────────────┐
        ▼                  ▼                  ▼
   ┌─────────┐      ┌──────────┐      ┌──────────────┐
   │ SQLite  │      │  Cache   │      │  User Files  │
   │Database │      │Directory │      │  (Read-Only) │
   │         │      │(Images & │      │  (STL, PNG,  │
   │Projects │      │Previews) │      │   JPG, etc.) │
   │Files    │      └──────────┘      └──────────────┘
   │Tags     │             │
   │Config   │             │ stl-thumb (external process)
   └─────────┘             └──────────────┐
                                         ▼
                                  ┌──────────────┐
                                  │  stl-thumb   │
                                  │  (CLI tool)  │
                                  │  STL → PNG   │
                                  └──────────────┘
```

### Key Design Decisions

#### 1. **Frontend: React + Vite (Not Next.js)**

**Decision**: Use React with Vite instead of Next.js

**Rationale**:
- **Local-First Application**: No SSR/SSG benefits (not a public website)
- **Simpler Architecture**: Direct client-server model matches use case perfectly
- **Better DX**: Vite provides faster HMR than Next.js
- **Smaller Bundle**: SPA architecture without Next.js overhead
- **No Need for API Routes**: Backend handles all APIs

**Trade-offs**:
- ✅ Gains: Faster builds, simpler mental model, smaller bundle
- ❌ Loses: SSR capabilities (not needed), built-in routing conventions (use React Router)

#### 2. **Backend: Rust + Axum**

**Decision**: Use Axum web framework with Rust

**Rationale**:
- **Performance**: Rust's zero-cost abstractions ideal for file I/O intensive operations
- **Async by Default**: Tokio runtime handles concurrent requests efficiently
- **Type Safety**: Compile-time guarantees prevent common bugs
- **Axum Benefits**: 
  - Built on Tower (middleware ecosystem)
  - Ergonomic extractors for requests
  - Native support for streaming responses (critical for ZIP downloads)

**Trade-offs**:
- ✅ Gains: High performance, memory safety, excellent async support
- ❌ Loses: Slower compile times, steeper learning curve than Node.js/Python

#### 3. **Database: SQLite**

**Decision**: Use SQLite with rusqlite (not PostgreSQL or other)

**Rationale**:
- **Zero Configuration**: File-based, no server process needed
- **Perfect for Local-First**: Single file, easy backup/restore
- **Sufficient Performance**: Handles 10k+ projects with proper indexing
- **FTS5 Built-In**: Full-text search without external dependencies
- **ACID Compliance**: Reliable even with crashes

**Configuration for Performance**:
```rust
PRAGMA journal_mode = WAL;       // Write-Ahead Logging for concurrency
PRAGMA synchronous = NORMAL;     // Balance safety and speed
PRAGMA cache_size = -64000;      // 64MB cache
PRAGMA mmap_size = 30000000000;  // 30GB memory-mapped I/O
```

**Trade-offs**:
- ✅ Gains: Simplicity, portability, zero configuration, built-in FTS
- ❌ Loses: Limited concurrent writes (not an issue for single-user), no network access

#### 4. **STL Preview: stl-thumb Integration**

**Decision**: Use external stl-thumb CLI tool via process spawning

**Rationale**:
- **Proven Solution**: Mature, widely-used tool for STL rendering
- **Avoid Complexity**: 3D rendering requires OpenGL/graphics libraries
- **Configurable**: Supports various output formats, sizes, angles
- **Separation of Concerns**: Keeps graphics logic out of our codebase

**Integration Pattern**:
```rust
std::process::Command::new("stl-thumb")
    .arg(stl_path)
    .arg(output_path)
    .arg("--size=512")
    .output()
```

**Trade-offs**:
- ✅ Gains: Simple integration, reliable rendering, configurable
- ❌ Loses: External dependency (must be installed), process spawn overhead

#### 5. **Image Caching Strategy**

**Decision**: File system cache with database metadata

**Rationale**:
- **Performance**: File system access faster than database BLOBs
- **Scalability**: No database bloat with large images
- **Flexibility**: Easy to clear, backup, or migrate cache separately

**Structure**:
```
cache/
├── images/    # User images (hashed by path)
└── previews/  # Generated STL previews (hashed by STL path)
```

**Trade-offs**:
- ✅ Gains: Fast access, scalable, easy maintenance
- ❌ Loses: Two storage locations (DB + files), cache invalidation complexity

#### 6. **Streaming ZIP Downloads**

**Decision**: Use async-zip with streaming responses

**Rationale**:
- **Memory Efficiency**: Stream chunks instead of loading entire ZIP in memory
- **Large File Support**: No browser memory limits
- **Better UX**: Download starts immediately

**Implementation**:
```rust
use async_zip::write::ZipFileWriter;
use axum::body::StreamBody;

// Stream ZIP as it's created
let stream = create_zip_stream(files).await;
StreamBody::new(stream)
```

**Trade-offs**:
- ✅ Gains: Supports any file size, better memory usage, faster start
- ❌ Loses: More complex than synchronous ZIP creation

#### 7. **Hierarchical Data Model**

**Decision**: Adjacency list with path caching

**Rationale**:
- **Simple to Implement**: Parent-child via foreign key
- **Efficient for Common Queries**: Most navigation is one level at a time
- **Path Caching**: Store full path for breadcrumbs and search

**Schema**:
```sql
CREATE TABLE projects (
    id INTEGER PRIMARY KEY,
    name TEXT,
    full_path TEXT UNIQUE,
    parent_id INTEGER REFERENCES projects(id)
);
```

**Recursive Queries for Ancestors**:
```sql
WITH RECURSIVE ancestors AS (
    SELECT id, name, parent_id FROM projects WHERE id = ?
    UNION ALL
    SELECT p.id, p.name, p.parent_id 
    FROM projects p JOIN ancestors a ON p.id = a.parent_id
)
SELECT * FROM ancestors;
```

**Trade-offs**:
- ✅ Gains: Simple, efficient for common cases, flexible depth
- ❌ Loses: Recursive queries needed for full tree operations

#### 8. **Tag System**

**Decision**: Many-to-many with FTS5 for search

**Rationale**:
- **Flexibility**: Projects can have multiple tags, tags reusable
- **Fast Search**: SQLite FTS5 provides sub-second search
- **Autocomplete**: Easy to query existing tags

**Schema**:
```sql
CREATE TABLE tags (id, name UNIQUE);
CREATE TABLE project_tags (project_id, tag_id, PRIMARY KEY);
CREATE VIRTUAL TABLE projects_fts USING fts5(project_id, name);
```

**Trade-offs**:
- ✅ Gains: Fast search, flexible tagging, good UX
- ❌ Loses: More complex queries than simple string tags

### Technology Stack Summary

| Layer | Technology | Version | Purpose |
|-------|-----------|---------|---------|
| **Frontend** | React | 18+ | UI framework |
| | TypeScript | 5.0+ | Type safety |
| | Vite | 5.0+ | Build tool & dev server |
| | React Router | 6.20+ | Client-side routing |
| | Zustand | 4.4+ | State management |
| | Axios | 1.6+ | HTTP client |
| **Backend** | Rust | 1.75+ | Systems programming |
| | Axum | 0.7 | Web framework |
| | Tokio | 1.35+ | Async runtime |
| | Tower | 0.4 | Middleware |
| | rusqlite | 0.31+ | SQLite bindings |
| | r2d2 | 0.8 | Connection pooling |
| | async-zip | 0.0.16 | Streaming ZIP |
| | walkdir | 2.4 | Directory traversal |
| **Database** | SQLite | 3.40+ | Local database |
| **External** | stl-thumb | Latest | STL preview generation |

### Data Flow

#### 1. Initial Setup Flow
```
User → Frontend → POST /api/config {rootPath}
                → Backend validates path
                → Saves to config table
                → Returns success
```

#### 2. Scanning Flow
```
User → Frontend → POST /api/scan
                → Backend starts async scan
                → walkdir traverses file system
                → For each directory with STL files:
                    - Insert project into DB
                    - Insert STL files
                    - Copy images to cache
                    - Queue STL preview generation
                → Scan completes
                → Frontend polls /api/scan/status
```

#### 3. Navigation Flow
```
User clicks tile → Frontend → GET /api/projects/{id}
                            → Backend queries DB for project
                            → Joins with files, tags
                            → Returns JSON
                            → Frontend renders detail page
```

#### 4. Search Flow
```
User types query → Frontend → GET /api/search?q=dragon&tags=fantasy
                            → Backend queries FTS5 table
                            → Filters by tags if provided
                            → Returns matching projects
                            → Frontend renders results grid
```

#### 5. Download Flow
```
User clicks download → Frontend → GET /api/projects/{id}/download
                               → Backend streams ZIP creation
                               → For each file:
                                   - Read from disk
                                   - Write to ZIP stream
                                   - Send chunk to client
                               → Frontend receives and saves
```

### API Design Principles

1. **RESTful**: Standard HTTP methods and resource paths
2. **JSON Responses**: Consistent format with `{data, meta}` structure
3. **Pagination**: All list endpoints support `page` and `perPage`
4. **Error Handling**: Structured errors with codes and messages
5. **Streaming**: Large responses (images, ZIPs) use streaming

### Security Considerations

**Local Application Context**: Running on user's machine, no network exposure

- **No Authentication**: Single-user local app
- **Path Traversal**: Validate all file paths within root directory
- **SQL Injection**: Use parameterized queries (rusqlite handles this)
- **XSS**: React escapes by default, no `dangerouslySetInnerHTML`
- **File Access**: Read-only access to user files, write only to cache

### Performance Optimizations

#### Database
- WAL mode for concurrent reads during scans
- Appropriate indexes on all foreign keys and search columns
- FTS5 for full-text search
- Connection pooling (r2d2)

#### Caching
- Image cache prevents repeated file reads
- STL preview cache (expensive to regenerate)
- LRU eviction based on `accessed_at` timestamp

#### Concurrency
- Async I/O throughout (tokio)
- Parallel directory scanning (buffer_unordered)
- Streaming responses for large files

#### Frontend
- Pagination (20 images/page)
- Lazy loading images
- Debounced search input
- Optimistic UI updates

### Testing Strategy

#### Backend Tests
```rust
// Unit tests
#[cfg(test)]
mod tests {
    #[test]
    fn test_project_repository() { ... }
}

// Integration tests
#[tokio::test]
async fn test_scan_endpoint() { ... }
```

#### Frontend Tests
```typescript
// Component tests (vitest)
describe('ProjectTile', () => {
  it('renders project name', () => { ... });
});

// E2E tests (optional, Playwright)
test('can scan and browse projects', async ({ page }) => { ... });
```

### Deployment

**Not applicable** - Local application, no deployment infrastructure needed.

Users run:
```bash
# Backend
cd backend && cargo run --release

# Frontend
cd frontend && npm run build && serve dist
```

Or package as:
- Linux: AppImage or .deb
- macOS: .app bundle
- Windows: .exe installer

### Future Considerations

**Phase 2+ Features** (out of scope for initial implementation):

1. **File Watching**: Auto-rescan on file system changes (notify crate)
2. **Interactive 3D Viewer**: Three.js for STL viewing in browser
3. **Batch Operations**: Multi-select for tagging, deleting
4. **Export/Import**: Database export for backup/sharing
5. **Multi-User**: Authentication, permissions (requires architecture changes)
6. **Cloud Sync**: Optional cloud backup (Google Drive, Dropbox integration)
7. **Mobile Apps**: Native iOS/Android apps
8. **Print Tracking**: Log which models have been printed

---

## Implementation Phases

### Phase 0: Research ✅ COMPLETED
- ✅ Technology evaluation (Next.js vs React+Vite)
- ✅ stl-thumb integration research
- ✅ Database schema design
- ✅ API endpoint design
- ✅ Caching strategy
- **Output**: research.md

### Phase 1: Design & Contracts ✅ COMPLETED
- ✅ Data model with SQLite schema
- ✅ API contracts (OpenAPI spec)
- ✅ Quick start guide
- ✅ Agent context updated
- **Output**: data-model.md, contracts/openapi.yaml, quickstart.md

### Phase 2: Project Setup (Week 1)
- Create backend Rust project with Cargo
- Create frontend React project with Vite
- Set up CI/CD pipeline (GitHub Actions)
- Configure development environment
- Initialize SQLite database with schema
- Set up testing frameworks

**Deliverable**: Empty projects with boilerplate, running locally

### Phase 3: Core Backend Infrastructure (Week 1-2)
- Implement database connection pooling
- Create migration system
- Implement repository pattern for data access
- Set up Axum server with basic routes
- Add CORS middleware
- Implement error handling

**Deliverable**: Backend API skeleton with health check endpoint

### Phase 4: File System Scanning (Week 2-3)
- Implement recursive directory scanner
- STL file detection logic
- Image file detection and caching
- Project hierarchy construction
- Database persistence
- Scan session tracking
- Progress reporting

**Deliverable**: Functional scan that populates database

### Phase 5: STL Preview Generation (Week 3)
- Integrate stl-thumb via CLI
- Implement async preview generation queue
- Cache preview images
- Fallback to placeholder images
- Error handling for missing stl-thumb

**Deliverable**: STL files have preview images

### Phase 6: Project API Endpoints (Week 3-4)
- List projects endpoint
- Get project details endpoint
- Get children endpoint
- Breadcrumb trail endpoint
- Get project files endpoint (paginated)

**Deliverable**: Complete project browsing API

### Phase 7: Search & Tags API (Week 4)
- Implement FTS5 search
- Tag CRUD endpoints
- Add/remove tags from projects
- Tag autocomplete
- Combined search (name + tags)

**Deliverable**: Functional search and tagging API

### Phase 8: File Download API (Week 4-5)
- Individual file download endpoint
- Streaming ZIP generation
- Image serving with cache
- Preview image serving
- Progress indication

**Deliverable**: Complete file download functionality

### Phase 9: Frontend Core UI (Week 5-6)
- Set up React Router
- Create layout components
- Implement navigation state management
- Create reusable tile component
- Implement breadcrumb component
- Add loading and error states

**Deliverable**: Basic navigation shell

### Phase 10: Frontend Browse Pages (Week 6-7)
- Home/setup page with config form
- Browse page with project grid
- Project detail page
- Image gallery with pagination
- File list components

**Deliverable**: Complete browsing experience

### Phase 11: Frontend Search & Tags (Week 7)
- Search bar component
- Search results page
- Tag input with autocomplete
- Tag management UI
- Filter controls

**Deliverable**: Working search and tag UI

### Phase 12: Frontend Downloads (Week 7-8)
- Download buttons
- Progress indicators
- Error handling
- Toast notifications

**Deliverable**: Complete download UX

### Phase 13: Polish & UX Refinements (Week 8)
- Responsive design
- Loading skeletons
- Empty states
- Keyboard navigation
- Accessibility improvements

**Deliverable**: Production-ready UI

### Phase 14: Testing & Documentation (Week 8)
- Backend integration tests
- Frontend component tests
- Manual testing checklist
- User documentation
- Developer documentation
- README updates

**Deliverable**: Tested and documented application

### Phase 15: Performance Optimization (Week 9)
- Database query optimization
- Image loading optimization
- Bundle size reduction
- Caching improvements
- Benchmark suite

**Deliverable**: Performance metrics meeting success criteria

### Milestone Summary

| Milestone | Week | Deliverable |
|-----------|------|-------------|
| M1: Research Complete | 0 | research.md, data-model.md, contracts |
| M2: Project Setup | 1 | Running skeleton apps |
| M3: Core Backend | 2 | API server with DB |
| M4: Scanning Works | 3 | Database populated from file system |
| M5: API Complete | 4-5 | All endpoints functional |
| M6: UI Shell | 5-6 | Navigation and layout |
| M7: Feature Complete | 7-8 | All user stories implemented |
| M8: Production Ready | 8-9 | Tested, documented, optimized |

### Success Metrics

From spec.md success criteria, to be validated:

- ✅ **SC-002**: Scan 100+ projects/minute
- ✅ **SC-004**: Search results in <1 second (10k projects)
- ✅ **SC-007**: Tile display in <2 seconds
- ✅ **SC-009**: ZIP generation starts in <10 seconds (50 files)
- ✅ **SC-011**: 95% search success rate with tags

---

## Conclusion

This implementation plan provides a comprehensive roadmap for building the 3D Print Model Library with:
- ✅ Clear technology choices with documented rationale
- ✅ Well-designed data model optimized for hierarchical data
- ✅ RESTful API with complete OpenAPI specification
- ✅ Detailed implementation phases with deliverables
- ✅ Performance considerations for large collections
- ✅ Testing strategy for quality assurance

**Next Step**: Phase 2 - Begin implementation with `/speckit.tasks` command to generate detailed task breakdown.
