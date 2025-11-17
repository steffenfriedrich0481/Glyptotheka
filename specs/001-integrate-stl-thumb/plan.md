# Implementation Plan: Integrate STL Preview Generation

**Branch**: `001-integrate-stl-thumb` | **Date**: 2025-11-17 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-integrate-stl-thumb/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Integrate stl-thumb directly as a Rust library dependency instead of calling it as an external command-line tool. This eliminates the need for manual installation of external binaries, simplifies deployment (Docker, systemd), improves error handling by removing subprocess overhead, and removes STL_THUMB_PATH configuration requirements. Preview generation will occur in-process using the library API while maintaining identical visual output quality and performance.

## Technical Context

**Language/Version**: Rust 1.75+  
**Primary Dependencies**: Axum 0.7, tokio 1.35, rusqlite 0.31, NEEDS CLARIFICATION (stl-thumb library availability and API)  
**Storage**: SQLite (rusqlite with bundled feature)  
**Testing**: cargo test, integration tests  
**Target Platform**: Linux server (Docker + native systemd), potentially cross-platform  
**Project Type**: Web (Rust backend + React frontend)  
**Performance Goals**: Preview generation within 10% of current performance (<5 seconds for typical STL files), maintain sub-second API response times  
**Constraints**: Must maintain 512x512 PNG output quality, support ASCII and binary STL formats, zero external binary dependencies, maintain existing cache compatibility  
**Scale/Scope**: Existing 3D print library management system with ~10-20 affected files in backend, single service refactor

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Project Type Assessment**: Glyptotheka is a web application with Rust backend and React frontend. The constitution.md file found contains rules for fraktor-rs, an actor-based embedded system with no_std requirements. These rules do not apply to this web application project.

**Applicable Development Standards**:
- ✅ Write tests first when adding new functionality (Test Integrity principle)
- ✅ Maintain existing test suite - no deletions or ignoring tests
- ✅ Keep CI green - all tests passing before and after changes
- ✅ Document breaking changes and migration steps
- ✅ Update related documentation immediately after code changes
- ✅ Investigate existing code patterns and maintain consistency (Inductively Consistency-Driven Design)
- ✅ Minimize breaking changes where possible; this change is additive/replacement only

**Project-Specific Quality Gates**:
1. **No Breaking Changes**: Existing cache must continue to work
2. **Test Coverage**: Maintain or improve test coverage for preview generation
3. **Performance**: Preview generation time must not regress by more than 10%
4. **Documentation**: Update all deployment docs to remove external dependency steps
5. **Error Handling**: Improve error messages by removing subprocess layer

**Status**: ✅ PASS - This is a straightforward dependency integration with no violations. The change simplifies the architecture by removing external dependencies.

## Project Structure

### Documentation (this feature)

```text
specs/001-integrate-stl-thumb/
├── spec.md              # Feature specification (provided)
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output - stl-thumb library research
├── data-model.md        # Phase 1 output - data model changes (if any)
├── quickstart.md        # Phase 1 output - updated deployment guide
└── contracts/           # Phase 1 output - API contracts (minimal changes expected)
```

### Source Code (repository root)

```text
backend/
├── src/
│   ├── api/
│   │   └── routes.rs           # [MODIFY] Remove stl_thumb_path from AppState
│   ├── config.rs                # [MODIFY] Remove stl_thumb_path field and logic
│   ├── db/
│   │   └── migrations.rs        # [MODIFY] Add migration to remove stl_thumb_path from config table
│   ├── models/                  # [NO CHANGE] Data models unchanged
│   ├── services/
│   │   ├── stl_preview.rs       # [MAJOR REFACTOR] Replace subprocess with library calls
│   │   └── image_cache.rs       # [NO CHANGE] Cache logic unchanged
│   └── utils/                   # [NO CHANGE]
├── Cargo.toml                   # [MODIFY] Add stl-thumb library dependency
├── Dockerfile                   # [MODIFY] Remove stl-thumb binary installation
└── tests/
    └── integration/             # [ADD/MODIFY] Add preview generation tests

frontend/                        # [NO CHANGE] Frontend unchanged

docker-compose.yml               # [MODIFY] Remove STL_THUMB_PATH environment variable
.env.example                     # [MODIFY] Remove STL_THUMB_PATH configuration
README.md                        # [MODIFY] Update installation and deployment sections
docs/                            # [MODIFY] Update any deployment documentation
```

**Structure Decision**: This is a web application following Option 2 (backend/frontend split). The primary changes are concentrated in the backend, specifically in the services layer where STL preview generation occurs. Configuration and deployment files require updates to remove external dependency references. The frontend is completely unaffected as the API interface remains identical.

## Complexity Tracking

> **This section is not needed** - no constitution violations present. This is a straightforward dependency integration that simplifies the architecture by removing external process calls.


---

## Phase 0: Research & Investigation

**Objective**: Resolve all NEEDS CLARIFICATION items and determine technical approach.

### Research Tasks

#### R1: Verify stl-thumb Library Availability
- **Question**: Is stl-thumb published on crates.io as a library (not just binary)?
- **Investigation**:
  - Check crates.io for stl-thumb or stl_thumb
  - Review GitHub repository (https://github.com/unlimitedbacon/stl-thumb) for library API
  - Check if stl-thumb exposes public functions for in-process thumbnail generation
  - Identify alternative Rust STL rendering libraries if stl-thumb is CLI-only
- **Deliverable**: Library availability report with API documentation links

#### R2: Analyze stl-thumb API Surface
- **Question**: What is the library API for generating previews?
- **Investigation**:
  - Review library documentation and examples
  - Identify function signatures for STL to PNG conversion
  - Determine input formats (file path vs byte array vs reader)
  - Check async/sync interface and tokio compatibility
  - Identify required dependencies and feature flags
- **Deliverable**: API integration approach with code examples

#### R3: Evaluate stl-thumb Dependencies
- **Question**: What dependencies does stl-thumb bring and are they compatible?
- **Investigation**:
  - Review stl-thumb's Cargo.toml for dependency tree
  - Check for potential conflicts with existing dependencies (axum, tokio versions)
  - Evaluate binary size impact
  - Check licensing compatibility (MIT required per spec)
  - Review rendering backend (likely glium or wgpu)
- **Deliverable**: Dependency compatibility report

#### R4: Alternative STL Rendering Libraries
- **Question**: If stl-thumb is not usable as library, what are alternatives?
- **Investigation**:
  - Research stl-io for STL parsing
  - Research image rendering libraries (image crate, resvg, tiny-skia)
  - Research 3D rendering libraries (glium, wgpu, softbuffer)
  - Evaluate effort to build custom renderer vs patching stl-thumb
- **Deliverable**: Alternative approach decision with justification

#### R5: Integration Pattern Analysis
- **Question**: How should the library be integrated in async context?
- **Investigation**:
  - Determine if stl-thumb rendering is CPU-bound
  - Test tokio::task::spawn_blocking vs async rendering
  - Evaluate thread pool sizing for preview generation
  - Analyze impact on overall application performance
- **Deliverable**: Integration pattern with performance considerations

### Research Consolidation

All findings will be documented in `research.md` with the following structure:
- **Decision**: Chosen integration approach
- **Rationale**: Why this approach was selected
- **Alternatives Considered**: Other options evaluated and why rejected
- **Implementation Notes**: Key technical details for Phase 1

**Exit Criteria**: All NEEDS CLARIFICATION resolved, clear path forward identified

---

## Phase 1: Design & Contracts

**Prerequisites**: research.md completed, library approach confirmed

### Design Tasks

#### D1: Data Model Changes

**File**: `data-model.md`

Current state:
- `config` table has `stl_thumb_path` field (nullable)
- `stl_files` table has `preview_path` and `preview_generated_at`
- `cached_files` table tracks preview cache

Required changes:
- **REMOVE**: `stl_thumb_path` from `config` table
- **MAINTAIN**: All preview-related fields in `stl_files` and `cached_files`
- **DECISION**: Cache format remains identical (no migration needed)

Migration strategy:
```sql
-- Migration: Remove stl_thumb_path configuration
ALTER TABLE config DROP COLUMN stl_thumb_path;
-- Note: SQLite requires table recreation for DROP COLUMN
```

**Deliverable**: Updated data model showing removed configuration field

#### D2: Service Layer Design

**File**: `data-model.md` (Service Architecture section)

Current architecture:
```
StlPreviewService
  ├─ stl_thumb_path: Option<PathBuf>
  ├─ generate_preview() -> subprocess call
  └─ run_stl_thumb() -> Command::new()
```

New architecture:
```
StlPreviewService
  ├─ [no external path needed]
  ├─ generate_preview() -> library call
  └─ render_stl_thumbnail() -> in-process rendering
```

Key changes:
- Remove `stl_thumb_path` field from service struct
- Replace `run_stl_thumb()` subprocess function with library function
- Maintain identical error handling and return types
- Keep async wrapper with `spawn_blocking` for CPU-bound work

**Deliverable**: Service architecture diagram with API changes

#### D3: Configuration Contract Changes

**File**: `contracts/config-api.yaml` (or similar)

API endpoints affected:
- `GET /api/config` - Response schema changes (remove stl_thumb_path)
- `POST /api/config` - Request schema changes (remove stl_thumb_path)

Cache behavior:
- No changes - existing previews remain valid
- Preview format unchanged (512x512 PNG)

**Deliverable**: Updated API contract for configuration endpoints

#### D4: Error Handling Design

**File**: `data-model.md` (Error Handling section)

Current error types:
- Subprocess spawn failures
- Subprocess exit code failures
- stderr parsing for error messages

New error types:
- Library initialization errors
- Rendering errors (file format, memory, etc.)
- Direct error messages from library

Error mapping strategy:
- Map library errors to existing `AppError::InternalServer`
- Improve error messages (no more "stl-thumb failed: stderr")
- Maintain error propagation patterns

**Deliverable**: Error handling strategy document

#### D5: Build & Deployment Changes

**File**: `quickstart.md`

Changes required:
1. **Cargo.toml**: Add stl-thumb dependency
2. **Dockerfile**: Remove stl-thumb installation steps
3. **docker-compose.yml**: Remove STL_THUMB_PATH env var
4. **.env.example**: Remove STL_THUMB_PATH comment
5. **README.md**: Remove stl-thumb installation instructions

Build process:
- Verify cross-compilation support
- Check for new system dependencies (OpenGL, etc.)
- Document any new runtime requirements

**Deliverable**: Updated deployment guide with simplified steps

### Agent Context Update

After completing design phase, update agent context:

```bash
.specify/scripts/bash/update-agent-context.sh copilot
```

This will:
- Detect that we're using GitHub Copilot
- Update `.github/copilot-context.md` or similar
- Add stl-thumb library integration notes
- Preserve existing manual annotations

**Deliverable**: Updated agent context file

---

## Phase 2: Implementation Plan Generation

**Note**: Phase 2 (task breakdown) is handled by `/speckit.tasks` command, not by this planning phase.

The implementation will involve:

### Code Changes Overview

1. **Dependency Management**
   - Add stl-thumb to Cargo.toml with appropriate features
   - Run cargo update to generate new Cargo.lock
   - Verify build succeeds with new dependency

2. **Service Refactoring**
   - Replace subprocess code in `stl_preview.rs`
   - Remove `stl_thumb_path` parameter from constructor
   - Update `run_stl_thumb()` to use library API
   - Maintain async wrapper pattern

3. **Configuration Updates**
   - Remove `stl_thumb_path` from `Config` struct
   - Remove from `AppConfig` struct
   - Remove from `UpdateConfigRequest`
   - Update ConfigService methods

4. **Database Migration**
   - Create migration to remove `stl_thumb_path` column
   - Handle SQLite column drop (table recreation)
   - Test migration up/down paths

5. **API Updates**
   - Update AppState initialization in `routes.rs`
   - Remove stl_thumb_path parameter passing
   - Verify API responses unchanged (except config)

6. **Deployment Updates**
   - Simplify Dockerfile
   - Update docker-compose.yml
   - Update .env.example
   - Update README.md

7. **Testing**
   - Write integration test for preview generation
   - Test with various STL files (ASCII, binary, complex)
   - Verify existing cache compatibility
   - Performance benchmarking

8. **Documentation**
   - Update installation guide
   - Update troubleshooting section
   - Update Docker deployment docs
   - Update systemd service example

---

## Success Metrics

Based on spec.md success criteria:

1. **SC-001**: Deployment without external tools
   - ✅ Docker build completes without stl-thumb installation
   - ✅ systemd service works without separate binary

2. **SC-002**: Visually identical output
   - ✅ Generate 20 test previews and compare with current output
   - ✅ Validate PNG format, size, and visual quality

3. **SC-003**: Performance within 10%
   - ✅ Benchmark 50 diverse STL files
   - ✅ Compare generation time before/after

4. **SC-004**: Docker build time reduction
   - ✅ Measure build time improvement (target: 20%+ reduction)

5. **SC-005**: Configuration reduction
   - ✅ One fewer config field (STL_THUMB_PATH removed)

6. **SC-006**: Zero deployment failures
   - ✅ Test deployments on clean systems
   - ✅ Verify no "external tool not found" errors

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| stl-thumb not available as library | Medium | High | Research alternatives in Phase 0; consider forking or contributing library API to stl-thumb |
| Performance regression | Low | Medium | Benchmark early; library should be faster than subprocess |
| Dependency conflicts | Low | Low | Check dependency tree early; Rust ecosystem is generally compatible |
| Breaking existing cache | Very Low | Medium | Maintain identical PNG format; test with existing cache |
| New system dependencies required | Medium | Low | Document in quickstart; likely needs OpenGL or similar |
| License incompatibility | Very Low | High | Verify MIT license in Phase 0 research |

---

## Timeline Estimate

- **Phase 0 (Research)**: 2-4 hours
  - Library availability check: 30 min
  - API investigation: 1-2 hours
  - Dependency analysis: 30 min
  - Alternative research: 30-60 min (if needed)

- **Phase 1 (Design)**: 3-4 hours
  - Data model updates: 1 hour
  - Service architecture: 1 hour
  - Contract updates: 30 min
  - Deployment design: 1 hour
  - Documentation: 30 min

- **Phase 2 (Implementation)**: 8-12 hours
  - Dependency integration: 1-2 hours
  - Service refactoring: 3-4 hours
  - Configuration cleanup: 1-2 hours
  - Database migration: 1 hour
  - Testing: 2-3 hours
  - Documentation: 1 hour

**Total Estimate**: 13-20 hours

---

## Open Questions

1. Does stl-thumb library expose a Rust API, or is it CLI-only?
   - **Resolution Required**: Phase 0 research

2. What rendering backend does stl-thumb use (OpenGL, Vulkan, software)?
   - **Impact**: System dependencies for Docker and production
   - **Resolution Required**: Phase 0 research

3. Is the rendering process thread-safe for concurrent preview generation?
   - **Impact**: May need mutex or per-thread renderer instances
   - **Resolution Required**: Phase 0 research

4. Can we generate previews to memory buffer instead of file?
   - **Impact**: Cleaner code, less I/O
   - **Resolution Required**: Phase 0 API analysis

5. Are there any known bugs or limitations in stl-thumb library mode?
   - **Impact**: May need workarounds or patches
   - **Resolution Required**: Phase 0 GitHub issue review

---

## Next Steps

1. **Run research phase**: Execute Phase 0 research tasks
2. **Document findings**: Create `research.md` with all decisions
3. **Proceed to design**: Complete Phase 1 deliverables
4. **Generate tasks**: Run `/speckit.tasks` for Phase 2 breakdown

**Current Status**: ✅ Plan complete, ready for Phase 0 research
