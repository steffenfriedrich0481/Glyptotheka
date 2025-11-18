# Implementation Plan: STL Preview Image Generation

**Branch**: `002-stl-preview-generation` | **Date**: 2025-11-18 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/002-stl-preview-generation/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Add automatic STL preview image generation to the scanning workflow, integrating the existing StlPreviewService with scanner and rescan operations. The feature implements a hybrid sync/async generation approach (first 2 STL files synchronously, remainder asynchronously), smart caching based on file modification timestamps, graceful error handling, and image prioritization where regular images rank higher than STL previews. STL previews will be included in composite preview generation and image API responses.

## Technical Context

**Language/Version**: Rust 1.75+ (stable channel)  
**Primary Dependencies**: 
- `stl-thumb` crate v0.5 (STL rendering to PNG)
- `tokio` v1.35 (async runtime)
- `rusqlite` v0.31 (database)
- `image` crate (composite preview generation)
- `walkdir` v2.4 (filesystem traversal)

**Storage**: SQLite database (existing schema requires extension)  
**Testing**: `cargo test` (unit + integration tests)  
**Target Platform**: Linux/macOS/Windows desktop (local application)  
**Project Type**: Web application (Axum backend + frontend)  
**Performance Goals**: 
- First 2 STL previews: < 10 seconds combined (synchronous)
- Additional STL previews: background processing, ~30 seconds per file
- Scan time increase: < 10% compared to non-STL scans
- Smart cache hit rate: > 90% on rescan (unchanged files)

**Constraints**: 
- Non-blocking: STL preview failures must not block scans
- Memory: < 500MB additional during async processing
- File size: STL preview images ~50-500KB each
- Preview dimensions: 512x512 pixels (configurable in StlPreviewService)

**Scale/Scope**: 
- Typical project: 5-50 STL files per folder
- Large project: 100+ STL files
- Total projects: 100-1000+ projects in library
- Concurrent scans: 1 active scan at a time

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Note**: The Glyptotheka Constitution is not applicable to this project. Glyptotheka is a Rust-based web application (Axum + SQLite backend), not an actor-runtime or embedded system. The constitution principles (no_std, actor patterns, protoactor-go/Pekko references) are specific to the fraktor-rs project and do not apply here.

**Applicable Best Practices**:
- ✅ **Test Integrity**: All new features start with tests; maintain CI green status
- ✅ **Code Organization**: Maintain consistent module structure and naming conventions
- ✅ **Error Handling**: Graceful degradation on failures (log warnings, continue operations)
- ✅ **Documentation**: Update relevant documentation for API and service changes

**No Constitution Violations**: N/A - Constitution does not apply to this project type.

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Documentation (this feature)

```text
specs/002-stl-preview-generation/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
# Web application structure (backend + frontend)
backend/
├── src/
│   ├── models/          # Data models (project.rs, file.rs, etc.)
│   ├── services/        # Business logic services
│   │   ├── scanner.rs           # [MODIFY] Add STL preview integration
│   │   ├── rescan.rs            # [MODIFY] Add STL preview integration
│   │   ├── stl_preview.rs       # [MODIFY] Enhance with timestamp checking
│   │   ├── composite_preview.rs # [MODIFY] Add STL preview support
│   │   └── image_cache.rs       # [EXISTS] Used by stl_preview.rs
│   ├── api/
│   │   └── handlers/
│   │       └── projects.rs      # [MODIFY] Update image retrieval logic
│   └── db/
│       ├── repositories/        # Database access layer
│       │   ├── file_repo.rs     # [MODIFY] Add STL preview queries
│       │   └── preview_repo.rs  # [EXISTS] May need updates
│       └── migrations/          # Database schema changes
│           └── 005_stl_preview_priority.sql  # [NEW] Add image priority
└── tests/
    ├── integration/             # [NEW] End-to-end scan tests
    └── unit/                    # [NEW] Service unit tests

cache/
└── stl-previews/                # [NEW] STL preview cache directory

frontend/
├── src/
│   ├── components/              # No changes needed
│   ├── pages/                   # No changes needed
│   └── services/                # API calls (transparent to frontend)
└── tests/                       # No changes needed

../stl-thumb/                    # [EXTERNAL] Binary dependency (already exists)
```

**Structure Decision**: This is a web application with separate backend and frontend. All implementation changes are backend-only. The frontend requires no modifications as STL preview images are returned through existing image API endpoints. The key integration points are:
1. Scanner/Rescan services - trigger preview generation
2. StlPreviewService - core preview generation logic (already exists)
3. Database schema - track image priority/type
4. Image retrieval APIs - prioritize regular images over STL previews
5. CompositePreviewService - consider STL previews as candidates

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

N/A - No constitution violations. No complex patterns required beyond existing architecture.

---

## Phase 0-1 Completion Summary

### Phase 0: Research - ✅ COMPLETED

All technical questions resolved. Key decisions documented in [research.md](./research.md):
- STL preview library: stl-thumb crate v0.5
- Hybrid sync/async pattern: First 2 sync, rest async
- Smart caching: File mtime comparison
- Image priority: 100 (regular), 50 (STL preview), 25 (composite)
- Error handling: Graceful degradation

### Phase 1: Design - ✅ COMPLETED

All design artifacts created:
- [data-model.md](./data-model.md) - Database schema, entities, relationships
- [contracts/api-contracts.md](./contracts/api-contracts.md) - API specifications (100% backward compatible)
- [contracts/service-contracts.md](./contracts/service-contracts.md) - Service interfaces and contracts
- [quickstart.md](./quickstart.md) - Developer implementation guide

### Agent Context Updated

GitHub Copilot instructions updated with:
- Language: Rust 1.75+ (stable channel)
- Database: SQLite database (existing schema requires extension)
- Project type: Web application (Axum backend + frontend)

### Key Implementation Points

**Database**: Migration 005 adds `image_priority` and `image_source` columns to `image_files` table.

**Services**: 
- `StlPreviewService` enhanced with smart caching
- `ScannerService` and `RescanService` integrate preview generation
- `PreviewQueue` handles async background processing (already exists)

**API**: Zero breaking changes - STL previews returned through existing endpoints.

**Estimated Implementation Time**: 1-2 days including testing.

---

## Next Phase

**Phase 2: Task Breakdown** - Run `/speckit.tasks` command to generate detailed implementation tasks.

This plan stops at Phase 1 as specified by the agent instructions.
