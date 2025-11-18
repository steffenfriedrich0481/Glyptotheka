---
description: "Task breakdown for STL Preview Image Generation feature"
---

# Tasks: STL Preview Image Generation

**Feature Branch**: `002-stl-preview-generation`  
**Input**: Design documents from `/specs/002-stl-preview-generation/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Tests are NOT required by the specification - implementation and manual testing only.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Database schema and basic infrastructure for STL preview support

- [X] T001 Create database migration file backend/src/db/migrations/005_stl_preview_priority.sql
- [X] T002 Apply migration to add image_priority and image_source columns to image_files table
- [ ] T003 Create cache directory structure cache/stl-previews/ with appropriate permissions
- [ ] T004 Verify stl-thumb binary exists at ../stl-thumb and is executable

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core preview generation service and infrastructure that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T005 Enhance StlPreviewService with generate_preview_with_smart_cache method in backend/src/services/stl_preview.rs
- [X] T006 [P] Add is_preview_valid method to StlPreviewService in backend/src/services/stl_preview.rs
- [X] T007 [P] Add get_preview_timestamp helper method to StlPreviewService in backend/src/services/stl_preview.rs
- [X] T008 [P] Implement file size validation (100MB limit) in StlPreviewService in backend/src/services/stl_preview.rs
- [X] T009 Add PreviewResult struct to backend/src/services/stl_preview.rs
- [X] T010 Implement smart caching logic (mtime comparison) in generate_preview_with_smart_cache in backend/src/services/stl_preview.rs
- [X] T011 Add timeout handling (30 second limit) for preview generation in backend/src/services/stl_preview.rs
- [X] T012 Implement graceful error handling with warning logs in StlPreviewService in backend/src/services/stl_preview.rs
- [X] T013 Update FileRepository with insert_stl_preview_image method in backend/src/db/repositories/file_repo.rs
- [X] T014 [P] Update FileRepository with get_images_by_priority method in backend/src/db/repositories/file_repo.rs
- [X] T015 [P] Update FileRepository with delete_stl_preview_image method in backend/src/db/repositories/file_repo.rs
- [X] T016 Create or verify PreviewQueue exists with background worker in backend/src/services/preview_queue.rs
- [X] T017 Add queue_preview method to PreviewQueue for async processing in backend/src/services/preview_queue.rs

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Automatic STL Preview Generation on Scan (Priority: P1) 🎯 MVP

**Goal**: Automatically generate preview images for STL files during scanning, with first 2 synchronous and remainder asynchronous

**Independent Test**: Scan a folder with STL files and verify preview images are generated and accessible via image API

### Implementation for User Story 1

- [X] T018 [P] [US1] Add scan_with_previews method signature to ScannerService in backend/src/services/scanner.rs
- [X] T019 [US1] Implement STL file detection and collection logic in scan_with_previews in backend/src/services/scanner.rs
- [X] T020 [US1] Add logic to split STL files into first 2 (sync) and remainder (async) in backend/src/services/scanner.rs
- [X] T021 [US1] Implement synchronous preview generation for first 2 STL files in backend/src/services/scanner.rs
- [X] T022 [US1] Implement async queue submission for remaining STL files in backend/src/services/scanner.rs
- [X] T023 [US1] Add add_stl_preview_to_db helper method to ScannerService in backend/src/services/scanner.rs
- [X] T024 [US1] Integrate STL preview generation into main scan workflow in backend/src/services/scanner.rs
- [X] T025 [US1] Add error handling for preview generation failures (log warnings, continue scan) in backend/src/services/scanner.rs
- [X] T026 [US1] Update ScanResult to include STL preview generation stats in backend/src/services/scanner.rs
- [X] T027 [US1] Wire up StlPreviewService and PreviewQueue dependencies in scanner service initialization in backend/src/main.rs

**Checkpoint**: User Story 1 complete - STL previews are generated during scan operations

---

## Phase 4: User Story 2 - STL Previews in Project Image Gallery (Priority: P2)

**Goal**: STL preview images appear in project image galleries with correct priority (after regular images)

**Independent Test**: View a project's image gallery and verify STL previews appear with lower ranking than regular images

### Implementation for User Story 2

- [X] T028 [US2] Update image retrieval query to sort by image_priority DESC in get_images_by_priority in backend/src/db/repositories/file_repo.rs
- [X] T029 [US2] Add ImageFile struct fields for image_priority and image_source in backend/src/models/file.rs
- [X] T030 [US2] Update projects API handler to use get_images_by_priority method in backend/src/api/handlers/projects.rs
- [X] T031 [US2] Update image inheritance logic to preserve priority for inherited STL previews in backend/src/services/scanner.rs
- [X] T032 [US2] Verify API response includes STL preview images in correct order in backend/src/api/handlers/projects.rs

**Checkpoint**: User Story 2 complete - STL previews appear in image galleries with correct ranking

---

## Phase 5: User Story 3 - STL Previews in Composite Previews (Priority: P3)

**Goal**: Composite preview generation considers STL previews as candidates, prioritizing regular images first

**Independent Test**: Generate composite previews for projects with various combinations of regular images and STL previews

### Implementation for User Story 3

- [X] T033 [US3] Update composite preview service to use priority-sorted image list in backend/src/services/composite_preview.rs
- [X] T034 [US3] Verify composite preview generation uses first 4 images by priority in backend/src/services/composite_preview.rs
- [X] T035 [US3] Test composite generation with 0 regular images (STL previews only) in backend/src/services/composite_preview.rs
- [X] T036 [US3] Test composite generation with 2 regular + 2 STL previews in backend/src/services/composite_preview.rs
- [X] T037 [US3] Update composite preview API endpoint to use priority-sorted images in backend/src/api/handlers/projects.rs

**Checkpoint**: User Story 3 complete - Composite previews include STL previews when appropriate

---

## Phase 6: User Story 4 - Rescan Integration with Smart Caching (Priority: P2)

**Goal**: Rescan operations regenerate STL previews only when STL files have been modified, using smart caching

**Independent Test**: Rescan projects with unchanged STL files and verify previews are not regenerated (cache hit)

### Implementation for User Story 4

- [ ] T038 [P] [US4] Add rescan_with_previews method signature to RescanService in backend/src/services/rescan.rs
- [ ] T039 [US4] Implement STL file modification detection in rescan_with_previews in backend/src/services/rescan.rs
- [ ] T040 [US4] Add logic to check preview validity using is_preview_valid in backend/src/services/rescan.rs
- [ ] T041 [US4] Implement smart cache logic (skip valid previews, regenerate stale ones) in backend/src/services/rescan.rs
- [ ] T042 [US4] Add preview regeneration for modified STL files (first 2 sync, rest async) in backend/src/services/rescan.rs
- [ ] T043 [US4] Implement orphaned preview cleanup (remove previews for deleted STL files) in backend/src/services/rescan.rs
- [ ] T044 [US4] Update RescanResult to include STL preview regeneration stats in backend/src/services/rescan.rs
- [ ] T045 [US4] Wire up StlPreviewService and PreviewQueue dependencies in rescan service initialization in backend/src/main.rs

**Checkpoint**: User Story 4 complete - Rescan operations use smart caching for STL previews

---

## Phase 7: Graceful Error Handling & Edge Cases (Cross-Story Polish)

**Purpose**: Ensure robustness across all user stories

- [ ] T046 [P] Add startup check for stl-thumb binary availability in backend/src/services/stl_preview.rs
- [ ] T047 [P] Implement feature flag to disable STL previews if stl-thumb unavailable in backend/src/services/stl_preview.rs
- [ ] T048 Add logging for all STL preview operations (info, warn, error) in backend/src/services/stl_preview.rs
- [ ] T049 Add error recovery for corrupted STL files in backend/src/services/stl_preview.rs
- [ ] T050 Implement disk space check before preview generation in backend/src/services/stl_preview.rs
- [ ] T051 Add memory usage monitoring for preview generation operations in backend/src/services/stl_preview.rs

---

## Phase 8: Testing & Validation

**Purpose**: Manual testing and validation of all user stories

- [ ] T052 [P] Create test fixtures directory tests/fixtures/ with sample STL files
- [ ] T053 Manual test: Scan folder with 5 STL files, verify first 2 generated synchronously
- [ ] T054 Manual test: Verify async preview generation completes for remaining 3 STL files
- [ ] T055 Manual test: Rescan same folder without changes, verify cache hits logged
- [ ] T056 Manual test: Modify one STL file, rescan, verify only that preview regenerated
- [ ] T057 Manual test: View project images API, verify STL previews appear after regular images
- [ ] T058 Manual test: Generate composite preview with mixed regular and STL images
- [ ] T059 Manual test: Scan with corrupted STL file, verify scan continues with warning
- [ ] T060 Performance test: Measure scan time increase with 2 STL files (should be <10 seconds)
- [ ] T061 Performance test: Measure memory usage during preview generation (should be <500MB)
- [ ] T062 Performance test: Verify cache hit rate on rescan (should be >90%)

---

## Phase 9: Documentation & Polish

**Purpose**: Final documentation and cleanup

- [ ] T063 [P] Update API documentation with STL preview behavior in docs/ (if API docs exist)
- [ ] T064 [P] Update README.md with STL preview feature description
- [ ] T065 [P] Update CHANGELOG.md with feature summary
- [ ] T066 Add inline code comments for complex smart caching logic
- [ ] T067 Run quickstart.md validation steps from specs/002-stl-preview-generation/quickstart.md
- [ ] T068 Code review and refactoring pass

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-6)**: All depend on Foundational phase completion
  - User Story 1 (Scan): Can start after Foundational
  - User Story 2 (Gallery): Can start after Foundational (can work in parallel with US1)
  - User Story 3 (Composite): Depends on US2 (needs priority sorting)
  - User Story 4 (Rescan): Can start after Foundational (similar to US1)
- **Error Handling (Phase 7)**: Can work in parallel with user stories
- **Testing (Phase 8)**: Depends on all user stories being complete
- **Documentation (Phase 9)**: Depends on testing completion

### User Story Dependencies

- **User Story 1 (P1 - Scan)**: Can start after Foundational - No dependencies on other stories ✅ MVP
- **User Story 2 (P2 - Gallery)**: Can start after Foundational - Works independently
- **User Story 3 (P3 - Composite)**: Depends on US2 (needs priority sorting to be implemented)
- **User Story 4 (P2 - Rescan)**: Can start after Foundational - Similar pattern to US1

### Within Each User Story

- Setup must complete before Foundational
- Foundational must complete before any user story
- Within US1: Service enhancements → Scanner integration → Dependency wiring
- Within US2: Model updates → Repository query updates → API handler updates
- Within US3: Depends on US2 completion → Update composite service
- Within US4: Similar flow to US1 for RescanService

### Parallel Opportunities

**Setup Phase**: T001, T003, T004 can run in parallel (different files)

**Foundational Phase**: 
- T006, T007, T008 can run in parallel (different methods in StlPreviewService)
- T013, T014, T015 can run in parallel (different methods in FileRepository)

**User Stories**:
- US1 and US2 can start in parallel after Foundational
- US1 and US4 can be worked on in parallel (similar patterns, different files)
- US2 tasks T028, T029 can run in parallel (different files)

**Error Handling Phase**: T046, T047 can run in parallel

**Documentation Phase**: T063, T064, T065 can run in parallel (different files)

---

## Parallel Example: Foundational Phase

```bash
# These tasks can run simultaneously (different methods/files):
Task T006: "Add is_preview_valid method to StlPreviewService"
Task T007: "Add get_preview_timestamp helper method to StlPreviewService"
Task T008: "Implement file size validation in StlPreviewService"

# These tasks can also run in parallel:
Task T013: "Update FileRepository with insert_stl_preview_image method"
Task T014: "Update FileRepository with get_images_by_priority method"
Task T015: "Update FileRepository with delete_stl_preview_image method"
```

---

## Parallel Example: User Story Implementation

```bash
# After Foundational is complete, these stories can start in parallel:
Team Member A: Phase 3 (User Story 1 - Scan)
Team Member B: Phase 4 (User Story 2 - Gallery)
Team Member C: Phase 6 (User Story 4 - Rescan)

# Once US2 is complete:
Team Member D: Phase 5 (User Story 3 - Composite)
```

---

## Implementation Strategy

### MVP First (User Story 1 Only) - Recommended

1. Complete Phase 1: Setup (4 tasks, ~15 minutes)
2. Complete Phase 2: Foundational (13 tasks, ~2 hours)
3. Complete Phase 3: User Story 1 - Scan (10 tasks, ~1 hour)
4. **STOP and VALIDATE**: Test scanning with STL files
5. Deploy/demo if ready

**MVP Deliverable**: Users can scan projects with STL files and see preview images generated automatically.

### Incremental Delivery (Recommended Order)

1. **MVP** (Setup + Foundational + US1) → Test → Deploy
2. **Phase 2** (Add US4 - Rescan) → Test independently → Deploy
3. **Phase 3** (Add US2 - Gallery) → Test independently → Deploy
4. **Phase 4** (Add US3 - Composite) → Test independently → Deploy
5. **Polish** (Error Handling + Testing + Docs) → Final validation → Deploy

Each phase adds value without breaking previous functionality.

### Full Feature Strategy

For complete feature implementation in one go:

1. Phase 1: Setup → Phase 2: Foundational (BLOCKS everything)
2. Phase 3-6: User Stories (can work in parallel if multiple developers)
3. Phase 7: Error Handling (can overlap with user stories)
4. Phase 8: Testing & Validation
5. Phase 9: Documentation & Polish

**Total Estimated Time**: 
- Core implementation: ~6-8 hours (Setup + Foundational + All User Stories)
- Testing & validation: ~2-3 hours
- Documentation: ~1 hour
- **Total: 1-2 days** as specified in plan.md

---

## Task Summary

- **Total Tasks**: 68
- **Setup Phase**: 4 tasks
- **Foundational Phase**: 13 tasks (BLOCKING)
- **User Story 1 (Scan)**: 10 tasks
- **User Story 2 (Gallery)**: 5 tasks
- **User Story 3 (Composite)**: 5 tasks
- **User Story 4 (Rescan)**: 8 tasks
- **Error Handling**: 6 tasks
- **Testing**: 11 tasks
- **Documentation**: 6 tasks

**Parallel Opportunities Identified**: 15+ tasks can run in parallel (marked with [P])

**Independent Test Criteria**:
- US1: Scan generates STL previews (first 2 sync, rest async)
- US2: Images API returns STL previews after regular images
- US3: Composite previews include STL previews when needed
- US4: Rescan uses smart caching (90%+ cache hit rate)

**Suggested MVP Scope**: Setup + Foundational + User Story 1 (27 tasks, ~4 hours)

---

## Notes

- [P] tasks = different files/methods, no dependencies, can run in parallel
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Tests are optional per specification - focus on implementation and manual testing
- All file paths are relative to repository root
- Foundational phase is CRITICAL - must complete before any user story work
