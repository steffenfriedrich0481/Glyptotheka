c---
description: "Task breakdown for Search View Refinement feature"
---

# Tasks: Search View Refinement

**Feature**: 001-search-refinement  
**Branch**: `001-search-refinement`  
**Input**: Design documents from `/specs/001-search-refinement/`

**Prerequisites**: 
- ✅ plan.md - Implementation plan with architecture
- ✅ spec.md - User stories with priorities
- ✅ research.md - Design decisions
- ✅ data-model.md - Entity definitions
- ✅ contracts/search-api.yaml - OpenAPI specification
- ✅ quickstart.md - Step-by-step implementation guide

**Tests**: Testing tasks are included per feature requirements

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `- [ ] [ID] [P?] [Story?] Description`

- **Checkbox**: `- [ ]` - Markdown checkbox for tracking
- **[ID]**: Sequential task identifier (T001, T002, T003...)
- **[P]**: Parallelizable task (different files, no dependencies on incomplete tasks)
- **[Story]**: User story label (US1, US2, US3) - REQUIRED for user story phase tasks
- Include exact file paths in descriptions

## Path Conventions

**Web Application Structure**:
- Backend: `backend/src/` (Rust + Axum)
- Frontend: `frontend/src/` (React + TypeScript)
- Tests: `backend/tests/`, `frontend/tests/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Verify development environment and branch setup

- [x] T001 Verify backend builds successfully with `cd backend && cargo test`
- [x] T002 Verify frontend builds successfully with `cd frontend && npm test`
- [x] T003 [P] Verify development database has sample projects with STL files and images
- [x] T004 [P] Create test data fixtures for hierarchical project structures (parent folders with sub-projects)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T005 Add `leaf_only` field to `SearchParams` struct in backend/src/services/search.rs
- [x] T006 [P] Create `ImagePreview` response struct in backend/src/models/project.rs
- [x] T007 [P] Create `SearchResultProject` response struct in backend/src/models/project.rs
- [x] T008 Add `get_aggregated_images()` method to FileRepository in backend/src/db/repositories/file_repo.rs
- [x] T009 Add helper method to get parent chain using recursive CTE in backend/src/db/repositories/project_repo.rs

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Filter Search to Leaf Projects Only (Priority: P0) 🎯 MVP

**Goal**: Enable users to search only projects containing STL files (leaf projects), excluding parent folders

**Independent Test**: Perform searches with known hierarchical structures (parent folders + sub-projects). Verify only leaf projects with STL files appear in results. Test with empty query (should return all leaf projects) and specific query matching both parents and leaves (should return only leaves).

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T010 [P] [US1] Add unit test for leaf filtering in `search_fts()` method in backend/tests/unit/search_service_test.rs
- [x] T011 [P] [US1] Add unit test for leaf filtering in `search_by_tags()` method in backend/tests/unit/search_service_test.rs
- [x] T012 [P] [US1] Add integration test for leaf-only search with hierarchical data in backend/tests/integration/search_tests.rs
- [x] T013 [P] [US1] Add integration test for empty search results (no leaf projects match) in backend/tests/integration/search_tests.rs

### Implementation for User Story 1

- [x] T014 [P] [US1] Update `search_fts()` method to add `WHERE is_leaf = 1` when `leaf_only = true` in backend/src/services/search.rs
- [x] T015 [P] [US1] Update `search_by_tags()` method to add `WHERE is_leaf = 1` when `leaf_only = true` in backend/src/services/search.rs
- [x] T016 [P] [US1] Update `search_combined()` method to add `WHERE is_leaf = 1` when `leaf_only = true` in backend/src/services/search.rs
- [x] T017 [US1] Update `search_all()` method to respect `leaf_only` parameter in backend/src/services/search.rs
- [x] T018 [US1] Add `leaf_only` query parameter with default `true` to `SearchQuery` struct in backend/src/api/handlers/search.rs
- [x] T019 [US1] Update `search_projects()` handler to pass `leaf_only` to SearchParams in backend/src/api/handlers/search.rs
- [x] T020 [US1] Add `stl_count` field to response JSON in backend/src/api/handlers/search.rs
- [x] T021 [US1] Update frontend `SearchParams` interface to include `leaf_only` field in frontend/src/api/projects.ts
- [x] T022 [US1] Update `searchProjects()` function to pass `leaf_only` parameter in query string in frontend/src/api/projects.ts
- [x] T023 [US1] Update `SearchResultProject` TypeScript interface to extend Project with `stl_count` in frontend/src/types/project.ts
- [x] T024 [US1] Update SearchPage to pass `leaf_only: true` to searchProjects() in frontend/src/pages/SearchPage.tsx
- [x] T025 [US1] Add empty state message "No projects with STL files found" when results are empty in frontend/src/pages/SearchPage.tsx
- [x] T026 [US1] Update ProjectTile to display STL count with icon in frontend/src/components/project/ProjectTile.tsx

**Checkpoint**: At this point, search results show only leaf projects with STL files. Test independently by searching with known hierarchical data.

---

## Phase 4: User Story 2 - Visual Preview with Image Carousel (Priority: P1)

**Goal**: Enable users to see image carousels on search result tiles showing inherited images and STL previews for quick visual assessment

**Independent Test**: View search results for projects with various image configurations (some with images, some without, some with many images). Verify carousels display correctly with navigation controls, images load properly with lazy loading, and placeholders appear for projects without images.

### Tests for User Story 2

- [x] T027 [P] [US2] Add unit test for `get_aggregated_images()` with simple project (no parents) in backend/tests/unit/file_repo_test.rs
- [x] T028 [P] [US2] Add unit test for `get_aggregated_images()` with 3-level parent hierarchy in backend/tests/unit/file_repo_test.rs
- [x] T029 [P] [US2] Add unit test verifying 15-image limit in `get_aggregated_images()` in backend/tests/unit/file_repo_test.rs
- [x] T030 [P] [US2] Add unit test for image priority sorting (regular images before STL previews) in backend/tests/unit/file_repo_test.rs
- [x] T031 [P] [US2] Add integration test for search response including image data in backend/tests/integration/search_tests.rs
- [x] T032 [P] [US2] Add component test for SearchTileCarousel with multiple images in frontend/tests/components/SearchTileCarousel.test.tsx
- [x] T033 [P] [US2] Add component test for SearchTileCarousel with no images (placeholder) in frontend/tests/components/SearchTileCarousel.test.tsx
- [x] T034 [P] [US2] Add component test for SearchTileCarousel navigation controls in frontend/tests/components/SearchTileCarousel.test.tsx

### Implementation for User Story 2

- [x] T035 [US2] Implement recursive CTE query for parent chain in `get_aggregated_images()` method in backend/src/db/repositories/file_repo.rs
- [x] T036 [US2] Implement image fetching with priority sorting and 15-image limit in backend/src/db/repositories/file_repo.rs
- [x] T037 [US2] Update `search_projects()` handler to call `get_aggregated_images()` for each result in backend/src/api/handlers/search.rs
- [x] T038 [US2] Add `images` array with ImagePreview data to response JSON in backend/src/api/handlers/search.rs
- [x] T039 [US2] Add `image_count` field to response JSON in backend/src/api/handlers/search.rs
- [x] T040 [US2] Add `ImagePreview` TypeScript interface in frontend/src/types/project.ts
- [x] T041 [US2] Update `SearchResultProject` interface to include `image_count` and `images` array in frontend/src/types/project.ts
- [x] T042 [US2] Create SearchTileCarousel component with image display and loading skeleton in frontend/src/components/project/SearchTileCarousel.tsx
- [x] T043 [US2] Add navigation controls (prev/next arrows) to SearchTileCarousel in frontend/src/components/project/SearchTileCarousel.tsx
- [x] T044 [US2] Add dot indicators showing current position to SearchTileCarousel in frontend/src/components/project/SearchTileCarousel.tsx
- [x] T045 [US2] Implement lazy loading with `loading="lazy"` attribute in SearchTileCarousel in frontend/src/components/project/SearchTileCarousel.tsx
- [x] T046 [US2] Add placeholder UI for projects with no images in SearchTileCarousel in frontend/src/components/project/SearchTileCarousel.tsx
- [x] T047 [US2] Add image source badges (STL Preview, Inherited) to SearchTileCarousel in frontend/src/components/project/SearchTileCarousel.tsx
- [x] T048 [US2] Hide navigation controls when only single image in SearchTileCarousel in frontend/src/components/project/SearchTileCarousel.tsx
- [x] T049 [US2] Update ProjectTile to use SearchTileCarousel instead of static icon in frontend/src/components/project/ProjectTile.tsx
- [x] T050 [US2] Add image count display to ProjectTile metadata section in frontend/src/components/project/ProjectTile.tsx

**Checkpoint**: At this point, search result tiles show image carousels with manual navigation. Test independently by viewing search results with various image configurations.

---

## Phase 5: User Story 3 - Image Carousel Auto-Advance (Priority: P2)

**Goal**: Enable carousels to automatically advance through images for passive visual browsing experience

**Independent Test**: Load search results and observe carousel behavior over time without interaction. Verify carousels advance automatically at appropriate intervals, pause on hover, pause after manual navigation, and advance at staggered intervals across multiple tiles.

### Tests for User Story 3

- [x] T051 [P] [US3] Add component test for auto-advance functionality in SearchTileCarousel in frontend/tests/components/SearchTileCarousel.test.tsx
- [x] T052 [P] [US3] Add component test for pause on hover in SearchTileCarousel in frontend/tests/components/SearchTileCarousel.test.tsx
- [x] T053 [P] [US3] Add component test for pause after manual navigation in SearchTileCarousel in frontend/tests/components/SearchTileCarousel.test.tsx

### Implementation for User Story 3

- [x] T054 [US3] Add `autoAdvance` prop to SearchTileCarousel component in frontend/src/components/project/SearchTileCarousel.tsx
- [x] T055 [US3] Implement auto-advance logic with staggered intervals (3-5 seconds) using setInterval in frontend/src/components/project/SearchTileCarousel.tsx
- [x] T056 [US3] Implement pause on hover by tracking `isHovered` state in frontend/src/components/project/SearchTileCarousel.tsx
- [x] T057 [US3] Implement pause after manual navigation with 10-second delay in frontend/src/components/project/SearchTileCarousel.tsx
- [x] T058 [US3] Add cleanup logic to clear interval on component unmount in frontend/src/components/project/SearchTileCarousel.tsx
- [x] T059 [US3] Update ProjectTile to pass `autoAdvance={true}` to SearchTileCarousel in frontend/src/components/project/ProjectTile.tsx

**Checkpoint**: All user stories should now be independently functional. Test auto-advance behavior with multiple search result tiles.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T060 [P] Add accessibility: keyboard navigation (arrow keys) for carousel in frontend/src/components/project/SearchTileCarousel.tsx
- [x] T061 [P] Add accessibility: ARIA labels for carousel controls and images in frontend/src/components/project/SearchTileCarousel.tsx
- [x] T062 [P] Add Intersection Observer for viewport-based lazy loading in frontend/src/components/project/SearchTileCarousel.tsx
- [x] T063 [P] Optimize performance: Add React.memo() to SearchTileCarousel component in frontend/src/components/project/SearchTileCarousel.tsx
- [x] T064 [P] Optimize performance: Add CSS containment (`contain: layout style paint`) to carousel in frontend/src/components/project/SearchTileCarousel.tsx
- [x] T065 [P] Batch image queries for all search results in single database call in backend/src/api/handlers/search.rs
- [ ] T066 [P] Add performance benchmark test for search query with 50 results in backend/tests/performance/search_bench.rs
- [x] T067 [P] Add error handling for failed image loads with retry option in frontend/src/components/project/SearchTileCarousel.tsx
- [x] T068 [P] Add loading skeleton animation with smooth transitions in frontend/src/components/project/SearchTileCarousel.tsx
- [ ] T069 [P] Update API documentation to reflect new search parameters and response structure in docs/api.md
- [ ] T070 [P] Add frontend component documentation for SearchTileCarousel in frontend/src/components/project/SearchTileCarousel.tsx
- [x] T071 Run all backend tests with `cargo test` and verify passing
- [x] T072 Run all frontend tests with `npm test` and verify passing
- [ ] T073 Run quickstart.md validation end-to-end with real database
- [ ] T074 Perform manual testing of all acceptance scenarios from spec.md
- [ ] T075 Run Lighthouse audit on search page and verify performance score > 90

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-5)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P0 → P1 → P2)
- **Polish (Phase 6)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P0)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - Independent but enhances US1 output
- **User Story 3 (P2)**: Depends on User Story 2 completion (builds on carousel component)

### Within Each User Story

- Tests MUST be written and FAIL before implementation
- Backend changes before frontend changes (API contract first)
- Models before services
- Services before handlers
- API client updates before UI component updates
- Core implementation before polish/optimization
- Story complete before moving to next priority

### Parallel Opportunities

#### Phase 1 (Setup)
- T001-T004: All setup tasks can run in parallel

#### Phase 2 (Foundational)
- T006, T007: Response structs can be created in parallel
- After T005, T008, T009 complete: Foundation is ready

#### Phase 3 (User Story 1)
- T010, T011, T012, T013: All tests can be written in parallel
- T014, T015, T016: Search method updates can be done in parallel (different methods)
- T021, T022, T023: Frontend type/API updates can be done together

#### Phase 4 (User Story 2)
- T027-T034: All tests can be written in parallel
- T040, T041: TypeScript interfaces can be created together

#### Phase 5 (User Story 3)
- T051, T052, T053: All tests can be written in parallel

#### Phase 6 (Polish)
- T060-T070: Most polish tasks can be done in parallel (different files/concerns)

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task T010: "Add unit test for leaf filtering in search_fts() method"
Task T011: "Add unit test for leaf filtering in search_by_tags() method"
Task T012: "Add integration test for leaf-only search with hierarchical data"
Task T013: "Add integration test for empty search results"

# Launch all search method updates together:
Task T014: "Update search_fts() method to add WHERE is_leaf = 1"
Task T015: "Update search_by_tags() method to add WHERE is_leaf = 1"
Task T016: "Update search_combined() method to add WHERE is_leaf = 1"

# Launch all frontend type updates together:
Task T021: "Update SearchParams interface to include leaf_only field"
Task T022: "Update searchProjects() function to pass leaf_only parameter"
Task T023: "Update SearchResultProject TypeScript interface"
```

---

## Parallel Example: User Story 2

```bash
# Launch all tests together:
Task T027: "Add unit test for get_aggregated_images() with simple project"
Task T028: "Add unit test for get_aggregated_images() with 3-level hierarchy"
Task T029: "Add unit test verifying 15-image limit"
Task T030: "Add unit test for image priority sorting"
Task T031: "Add integration test for search response with images"
Task T032: "Add component test for SearchTileCarousel with multiple images"
Task T033: "Add component test for SearchTileCarousel with no images"
Task T034: "Add component test for SearchTileCarousel navigation controls"

# Launch frontend type updates together:
Task T040: "Add ImagePreview TypeScript interface"
Task T041: "Update SearchResultProject interface to include images array"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
   - Search with hierarchical data shows only leaf projects
   - Empty searches work correctly
   - STL counts display properly
5. Deploy/demo if ready (basic functional search)

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP - functional leaf filtering!)
3. Add User Story 2 → Test independently → Deploy/Demo (Visual search with carousels!)
4. Add User Story 3 → Test independently → Deploy/Demo (Animated carousels!)
5. Add Polish → Final validation → Production deployment
6. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (Backend leaf filtering + Frontend integration)
   - Developer B: User Story 2 (Backend image aggregation + Frontend carousel - can start in parallel if API contract is clear)
   - Developer C: Can work on Phase 6 polish tasks or prepare test data
3. User Story 3 starts after User Story 2 completes (depends on carousel component)
4. Stories integrate smoothly due to independent design

---

## Performance Targets

| Metric | Target | How to Validate |
|--------|--------|----------------|
| Search query time (backend) | <100ms | Backend profiling with 10,000+ projects |
| Search page render (50 results) | <2s | Lighthouse audit |
| Carousel interaction | <16ms per frame | Chrome DevTools Performance tab |
| API response size per page | <500KB | Network tab monitoring |
| Image load time | <500ms per image | Browser timing API |

---

## Notes

- **[P] tasks** = Parallelizable (different files, no dependencies on incomplete tasks)
- **[Story] labels** = Map task to specific user story for traceability (US1, US2, US3)
- Each user story should be independently completable and testable
- Verify tests fail before implementing (TDD approach)
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Backend API contract must be stable before frontend implementation begins
- Follow quickstart.md for detailed implementation guidance
- Reference research.md for architectural decisions
- Reference data-model.md for entity structures
- Reference contracts/search-api.yaml for API specifications

---

## Summary

- **Total Tasks**: 75 tasks
- **User Story 1 (P0)**: 17 tasks (10 tests + 13 implementation + 4 foundational dependencies)
- **User Story 2 (P1)**: 24 tasks (8 tests + 16 implementation)
- **User Story 3 (P2)**: 9 tasks (3 tests + 6 implementation)
- **Setup & Foundation**: 9 tasks
- **Polish & Validation**: 16 tasks
- **Parallel Opportunities**: 40+ tasks can be parallelized
- **MVP Scope**: Phase 1 + Phase 2 + Phase 3 (26 tasks total)
- **Independent Testing**: Each user story has clear test criteria and can be validated independently

**Suggested Delivery**: 
- **Week 1**: MVP (Setup + Foundation + US1) - 26 tasks - Functional leaf filtering
- **Week 2**: Enhanced (Add US2) - 24 tasks - Visual search with carousels
- **Week 3**: Polish (Add US3 + Polish) - 25 tasks - Auto-advance + optimization + documentation
