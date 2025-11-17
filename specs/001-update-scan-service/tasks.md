---
description: "Task list for Image Inheritance Feature implementation"
---

# Tasks: Image Inheritance from Parent to Child Projects

**Input**: Design documents from `/specs/001-update-scan-service/`
**Prerequisites**: plan.md (required)

**Tests**: Not explicitly requested - implementation focused on core functionality with manual testing plan provided

**Organization**: Tasks are organized by implementation phase to ensure proper dependency ordering and testability

## Format: `[ID] [P?] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Path Conventions

- Web app architecture: `backend/src/`, `frontend/src/`
- Backend is Rust-based with services in `backend/src/services/`

---

## Phase 1: Setup & Analysis

**Purpose**: Understand current implementation and prepare for changes

- [ ] T001 Review current scanner.rs implementation in backend/src/services/scanner.rs
- [ ] T002 Review current rescan.rs implementation in backend/src/services/rescan.rs
- [ ] T003 Review file_repo.rs add_image_file method signature in backend/src/db/repositories/file_repo.rs
- [ ] T004 Verify database schema supports source_type and source_project_id in image_files table

**Checkpoint**: Foundation understood - implementation can begin

---

## Phase 2: Core Implementation - Scanner Service

**Purpose**: Implement downward image inheritance in scanner service

**Goal**: Add image inheritance capability so child projects inherit all images from parent folders

**Independent Test**: Create test folder structure with images at multiple levels, scan it, verify child folders show inherited images with correct source_project_id

### Helper Methods

- [ ] T005 [P] Add ensure_project_exists() helper method in backend/src/services/scanner.rs
  - Purpose: Create project entry for parent folders that may not have STL files
  - Parameters: folder: &Path, root: &Path, path_to_id: &HashMap<PathBuf, i64>
  - Returns: Result<i64, AppError> with project ID
  - Logic: Check path_to_id cache, query database, create if needed
  - Location: Add after create_project_hierarchy() method

- [ ] T006 [P] Add inherit_images_from_parents() method in backend/src/services/scanner.rs
  - Purpose: Walk up folder tree and inherit images from all ancestors
  - Parameters: project_id: i64, folder: &Path, root: &Path, path_to_id: &HashMap<PathBuf, i64>
  - Returns: Result<(), AppError>
  - Logic: Walk up tree, scan each parent folder for images, collect with source_project_id
  - Add inherited images using file_repo.add_image_file() with source_type="inherited"
  - Location: Add after add_images_for_project() method

### Integration into Scan Flow

- [ ] T007 Add second pass for image inheritance in scan() method in backend/src/services/scanner.rs
  - Location: After main scan loop (after line 136)
  - Add logging: "Propagating images from parent folders to children"
  - Iterate through project_folders calling inherit_images_from_parents()
  - Collect errors but continue processing (warn on failures)
  - Add error messages to errors vector

**Checkpoint**: Scanner service now propagates images downward from parents to children

---

## Phase 3: Core Implementation - Rescan Service

**Purpose**: Update rescan service to handle inherited images correctly

**Goal**: Ensure rescans clear inherited images before rebuilding inheritance, allowing image changes to propagate

**Independent Test**: Add image to parent folder, rescan, verify it appears in children; remove image, rescan, verify it's gone from children

### Clear Inherited Images

- [ ] T008 Add inherited image cleanup in rescan() method in backend/src/services/rescan.rs
  - Location: At start of rescan() method, before filesystem walk (around line 48)
  - Execute SQL: "DELETE FROM image_files WHERE source_type = 'inherited'"
  - Log operation: "Clearing inherited images for rebuild"
  - Handle errors gracefully and add to result.errors if cleanup fails

### Update Image Processing

- [ ] T009 Update get_existing_image_files() query in backend/src/services/rescan.rs
  - Current query (line 403): Already filters by source_type = 'direct'
  - Verify this query only returns direct images (not inherited)
  - No changes needed if query is correct

### Add Second Pass for Inheritance

- [ ] T010 Add image inheritance second pass in rescan() method in backend/src/services/rescan.rs
  - Location: After main processing loop, before cleanup_orphaned() (around line 196)
  - Add logging: "Propagating images from parent folders to children"
  - Iterate through all processed projects calling inherit_images_from_parents()
  - Note: Reuse scanner.inherit_images_from_parents() method OR
  - Alternative: Duplicate implementation since RescanService doesn't have access to ScannerService
  - Choose implementation approach: shared trait, code duplication, or service composition

**Checkpoint**: Rescan service properly rebuilds image inheritance on each scan

---

## Phase 4: Testing & Validation

**Purpose**: Manual testing to verify implementation meets requirements

**Goal**: Validate all test scenarios from plan.md work correctly

**Independent Test**: Run all test scenarios and verify expected behavior

- [ ] T011 Test Scenario 1: Simple Inheritance
  - Create parent folder with header_image.jpg
  - Create child folder with STL files
  - Run scan
  - Verify child project shows inherited image
  - Verify source_type = "inherited" in database
  - Verify source_project_id points to parent project

- [ ] T012 Test Scenario 2: Multi-Level Inheritance
  - Create 3-level folder structure: grandparent/parent/child
  - Place different images at each level
  - Run scan
  - Verify child inherits from both parent and grandparent
  - Verify each inherited image has correct source_project_id
  - Verify direct images remain marked as "direct"

- [ ] T013 Test Scenario 3: Rescan with New Images
  - Run initial scan
  - Add new image to parent folder
  - Run rescan
  - Verify new image appears in all child projects
  - Verify old inherited images still present

- [ ] T014 Test Scenario 4: Rescan with Removed Images
  - Run initial scan with parent images
  - Remove image from parent folder
  - Run rescan
  - Verify removed image no longer shows in child projects
  - Verify other inherited images remain

- [ ] T015 Test Scenario 5: Mixed Direct and Inherited Images
  - Create parent with image A
  - Create child with direct image B and STL files
  - Run scan
  - Verify child shows both image A (inherited) and image B (direct)
  - Verify source_type correctly set for each

- [ ] T016 Verify API endpoints return inherited images correctly
  - Test GET /api/projects/{id}/images endpoint
  - Verify response includes both direct and inherited images
  - Verify source_project_id is included in response for inherited images

**Checkpoint**: All test scenarios pass, feature is working as designed

---

## Phase 5: Documentation & Polish

**Purpose**: Document the feature and ensure code quality

- [ ] T017 Add code comments explaining inheritance logic in backend/src/services/scanner.rs
  - Document ensure_project_exists() method purpose
  - Document inherit_images_from_parents() method algorithm
  - Document second pass rationale in scan() method

- [ ] T018 Add code comments explaining inheritance handling in backend/src/services/rescan.rs
  - Document why inherited images are cleared
  - Document rebuild process

- [ ] T019 [P] Update CHANGELOG.md with feature description
  - Add entry for image inheritance feature
  - Describe downward propagation behavior
  - Note benefits for users

- [ ] T020 [P] Consider adding metrics/logging for inheritance operations
  - Log count of inherited images per project
  - Log total inheritance operations in scan summary
  - Add to ScanResult if helpful for monitoring

**Checkpoint**: Feature is documented and production-ready

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup & Analysis)**: No dependencies - can start immediately
- **Phase 2 (Scanner Service)**: Depends on Phase 1 completion
  - T005 and T006 can run in parallel (different methods)
  - T007 depends on T005 and T006 (integrates them)
- **Phase 3 (Rescan Service)**: Depends on Phase 2 completion (needs scanner methods available)
  - T008 and T009 can run in parallel (different areas)
  - T010 depends on Phase 2 methods being available
- **Phase 4 (Testing)**: Depends on Phases 2 and 3 completion
  - T011-T016 should run sequentially (each builds on understanding from previous)
- **Phase 5 (Documentation)**: Can start after Phase 2/3, parallel with testing
  - T017-T020 can run in parallel

### Key Decision Point

**T010 Implementation Approach** requires architectural decision:

**Option A: Code Duplication**
- Duplicate inherit_images_from_parents() logic in rescan.rs
- Pros: No dependencies, simple, each service independent
- Cons: Code duplication, maintenance burden

**Option B: Shared Trait/Module**
- Extract inheritance logic to separate module/trait
- Both services depend on shared module
- Pros: DRY principle, single source of truth
- Cons: More complex, requires refactoring

**Option C: Service Composition**
- RescanService uses ScannerService instance
- Pros: Reuses existing implementation
- Cons: Service dependency, potential circular issues

**Recommendation**: Start with Option A (duplication) for MVP, refactor to Option B if inheritance logic becomes more complex

### Critical Path

1. T001-T004 (Setup) → 
2. T005-T006 (Helper methods) → 
3. T007 (Scanner integration) → 
4. T008-T010 (Rescan updates) → 
5. T011-T016 (Testing)

Parallel opportunities within phases as marked with [P]

---

## Implementation Strategy

### MVP Approach (Minimum Viable Product)

1. **Phase 1**: Setup & Analysis (T001-T004)
2. **Phase 2**: Scanner Service (T005-T007)
3. **Basic Testing**: Test Scenario 1 only (T011)
4. **STOP and VALIDATE**: Does simple inheritance work?
5. If yes, continue to Phase 3
6. If no, debug Phase 2 before proceeding

### Full Implementation

1. Complete Phases 1-3 (Core implementation)
2. Run all test scenarios (Phase 4)
3. Fix any issues found
4. Complete documentation (Phase 5)

### Incremental Testing Strategy

- After each phase, run relevant test scenarios
- Don't wait until end to test
- Fix issues immediately before moving forward

**Example**:
- Complete T007 → Immediately run T011 (simple inheritance test)
- Complete T010 → Immediately run T013 (rescan test)
- This catches issues early when context is fresh

---

## Implementation Notes

### Performance Considerations

- Image inheritance scanned once during initial scan
- Minimal overhead: just reading parent directories (no file copying)
- No duplicate files: only database references
- Query performance maintained with existing indexes

### Database Impact

- Uses existing source_type and source_project_id columns
- No schema changes required
- Inherited images marked with source_type='inherited'
- Each inherited image tracks source_project_id for traceability

### Error Handling

- Inheritance errors logged but don't fail entire scan
- Missing parent folders handled gracefully
- Filesystem errors caught and added to error list
- Scan continues even if some projects fail inheritance

### Edge Cases to Consider

- Parent folder with no project record (ensure_project_exists handles this)
- Circular symlinks (WalkDir already handles with follow_links(false))
- Very deep folder hierarchies (performance may degrade, but functional)
- Images with same name at different levels (both inherited, display_order distinguishes)
- Rescanning after filesystem changes (inherited images cleared and rebuilt)

---

## Summary

**Total Tasks**: 20 tasks across 5 phases

**Task Distribution**:
- Phase 1 (Setup): 4 tasks
- Phase 2 (Scanner): 3 tasks (2 helper methods + integration)
- Phase 3 (Rescan): 3 tasks (cleanup + update + second pass)
- Phase 4 (Testing): 6 tasks (test scenarios)
- Phase 5 (Documentation): 4 tasks

**Parallel Opportunities**: 8 tasks marked [P] can run in parallel within their phases

**MVP Scope** (Minimum viable product):
- Phase 1: Setup (T001-T004)
- Phase 2: Scanner Service (T005-T007)
- Basic Test: T011 only
- **Estimated**: 8 tasks for basic working feature

**Full Feature Scope**: All 20 tasks for production-ready implementation

**Critical Decisions**:
- T010: Choose implementation approach (duplication vs shared module)
- Recommendation: Start with duplication, refactor if needed

**Testing Approach**: Manual testing with provided test scenarios, no automated tests requested

**Ready for Implementation**: Yes - clear implementation path with detailed acceptance criteria per task
