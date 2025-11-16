# Tasks: 3D Print Model Library

**Feature Branch**: `001-3d-print-library`  
**Input**: Design documents from `/specs/001-3d-print-library/`  
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/openapi.yaml ✅

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `- [ ] [ID] [P?] [Story] Description`

- **Checkbox**: `- [ ]` at start (markdown task list)
- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2, US3, US4, US5)
- Include exact file paths in descriptions

## Path Conventions

This is a web application with separate backend (Rust) and frontend (React):
- **Backend**: `backend/src/`, `backend/tests/`
- **Frontend**: `frontend/src/`, `frontend/tests/`
- **Specs**: `specs/001-3d-print-library/`
- **Example folder**: `example/` (for testing and validation)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [X] T001 Create backend Rust project structure in backend/ with Cargo.toml
- [X] T002 Create frontend React project structure in frontend/ with package.json and Vite config
- [X] T003 [P] Configure GitHub Actions CI pipeline in .github/workflows/ci.yml
- [X] T004 [P] Set up example/ folder structure with sample 3D print files for testing
- [X] T005 [P] Configure Rust formatting and linting (rustfmt, clippy) in backend/
- [X] T006 [P] Configure TypeScript linting and formatting (ESLint, Prettier) in frontend/

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### Database & Backend Core

- [X] T007 Create SQLite schema migration in backend/src/db/migrations/001_initial.sql
- [X] T008 [P] Implement database connection pool in backend/src/db/connection.rs
- [X] T009 [P] Create base project model in backend/src/models/project.rs
- [X] T010 [P] Create STL file model in backend/src/models/stl_file.rs
- [X] T011 [P] Create image file model in backend/src/models/image_file.rs
- [X] T012 [P] Create tag model in backend/src/models/tag.rs
- [X] T013 [P] Create cached file model in backend/src/models/cached_file.rs
- [X] T014 Implement project repository with CRUD operations in backend/src/db/repositories/project_repo.rs
- [X] T015 [P] Implement file repository in backend/src/db/repositories/file_repo.rs
- [X] T016 [P] Implement tag repository in backend/src/db/repositories/tag_repo.rs
- [X] T017 Set up Axum server with basic routing in backend/src/main.rs
- [X] T018 [P] Implement CORS middleware in backend/src/api/middleware/cors.rs
- [X] T019 [P] Implement error handling middleware in backend/src/api/middleware/error.rs
- [X] T020 [P] Create common error types in backend/src/utils/error.rs
- [X] T021 [P] Create pagination utilities in backend/src/utils/pagination.rs

### Frontend Core

- [X] T022 Set up React Router with base routes in frontend/src/App.tsx
- [X] T023 [P] Create API client configuration in frontend/src/api/client.ts
- [X] T024 [P] Create TypeScript types for all entities in frontend/src/types/project.ts
- [X] T025 [P] Create TypeScript types for API responses in frontend/src/types/api.ts
- [X] T026 [P] Create navigation context provider in frontend/src/store/navigationContext.tsx
- [X] T027 [P] Implement base Tile component in frontend/src/components/common/Tile.tsx
- [X] T028 [P] Implement Breadcrumb component in frontend/src/components/common/Breadcrumb.tsx
- [X] T029 [P] Implement LoadingSpinner component in frontend/src/components/common/LoadingSpinner.tsx
- [X] T030 [P] Implement Pagination component in frontend/src/components/common/Pagination.tsx

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Browse and Navigate 3D Print Projects (Priority: P1) 🎯 MVP

**Goal**: Users can specify a root folder, scan it to discover projects, and navigate through a tile-based interface with images.

**Independent Test**: Specify example/ folder as root, trigger scan, verify tile display shows projects and images, click through hierarchy, verify breadcrumb navigation works.

### Backend Implementation for User Story 1

- [X] T031 [P] [US1] Implement config service in backend/src/services/config.rs
- [X] T032 [P] [US1] Implement file system scanner service in backend/src/services/scanner.rs
- [X] T033 [US1] Implement project hierarchy builder in backend/src/services/scanner.rs
- [X] T034 [P] [US1] Implement image caching service in backend/src/services/image_cache.rs
- [X] T035 [US1] Implement GET /api/config handler in backend/src/api/handlers/config.rs
- [X] T036 [US1] Implement POST /api/config handler in backend/src/api/handlers/config.rs
- [X] T037 [US1] Implement POST /api/scan handler in backend/src/api/handlers/scan.rs
- [X] T038 [US1] Implement GET /api/scan/status handler in backend/src/api/handlers/scan.rs
- [X] T039 [US1] Implement GET /api/projects (list root) handler in backend/src/api/handlers/projects.rs
- [X] T040 [US1] Implement GET /api/projects/:id handler in backend/src/api/handlers/projects.rs
- [X] T041 [US1] Implement GET /api/projects/:id/children handler in backend/src/api/handlers/projects.rs
- [X] T042 [US1] Implement GET /api/projects/:id/files handler with pagination in backend/src/api/handlers/projects.rs
- [X] T043 [US1] Implement GET /api/images/:hash handler in backend/src/api/handlers/files.rs
- [X] T044 [US1] Wire all US1 routes in backend/src/api/routes.rs

### Frontend Implementation for User Story 1

- [X] T045 [P] [US1] Create config API client in frontend/src/api/config.ts
- [X] T046 [P] [US1] Create scan API client in frontend/src/api/scan.ts
- [X] T047 [P] [US1] Create projects API client in frontend/src/api/projects.ts
- [X] T048 [P] [US1] Implement HomePage component with config form in frontend/src/pages/HomePage.tsx
- [X] T049 [P] [US1] Implement BrowsePage component with project grid in frontend/src/pages/BrowsePage.tsx
- [X] T050 [P] [US1] Implement ProjectPage component with detail view in frontend/src/pages/ProjectPage.tsx
- [X] T051 [P] [US1] Implement ProjectGrid component in frontend/src/components/project/ProjectGrid.tsx
- [X] T052 [P] [US1] Implement ProjectTile component in frontend/src/components/project/ProjectTile.tsx
- [X] T053 [P] [US1] Implement ImageGallery component with pagination in frontend/src/components/project/ImageGallery.tsx
- [X] T054 [P] [US1] Implement FileList component in frontend/src/components/project/FileList.tsx
- [X] T055 [US1] Create useProjects hook in frontend/src/hooks/useProjects.ts
- [X] T056 [US1] Create useNavigation hook in frontend/src/hooks/useNavigation.ts

### US1 Validation with Chrome DevTools

- [X] T057 [US1] Verify HomePage renders config form correctly using chrome-devtools take_snapshot
- [X] T058 [US1] Test scan functionality: fill example/ path, click scan, verify success using chrome-devtools
- [X] T059 [US1] Verify tile-based navigation displays projects using chrome-devtools take_snapshot
- [X] T060 [US1] Test clicking on project tile navigates correctly using chrome-devtools click
- [X] T061 [US1] Verify breadcrumb navigation shows correct path using chrome-devtools take_snapshot
- [X] T062 [US1] Test project detail page displays images with pagination using chrome-devtools
- [X] T063 [US1] Verify image pagination (20 per page) works using chrome-devtools click on pagination controls

**Checkpoint**: User Story 1 should be fully functional - users can scan example/ folder and browse projects with images

---

## Phase 4: User Story 2 - Search and Filter Projects (Priority: P2)

**Goal**: Users can search projects by name or tags to quickly find specific content in large collections.

**Independent Test**: Add some tags to projects in example/ folder, use search bar to find projects by name, verify tag-based search returns correct results.

### Backend Implementation for User Story 2

- [X] T064 [P] [US2] Implement FTS5 search service in backend/src/services/search.rs
- [X] T065 [US2] Implement GET /api/search handler with name and tag filters in backend/src/api/handlers/search.rs
- [X] T066 [US2] Implement GET /api/tags handler (list all tags) in backend/src/api/handlers/tags.rs
- [X] T067 [US2] Implement GET /api/tags/autocomplete handler in backend/src/api/handlers/tags.rs
- [X] T068 [US2] Wire US2 routes in backend/src/api/routes.rs

### Frontend Implementation for User Story 2

- [X] T069 [P] [US2] Create search API client in frontend/src/api/search.ts
- [X] T070 [P] [US2] Create tags API client in frontend/src/api/tags.ts
- [X] T071 [P] [US2] Implement SearchBar component with debouncing in frontend/src/components/common/SearchBar.tsx
- [X] T072 [P] [US2] Implement SearchPage component with results grid in frontend/src/pages/SearchPage.tsx
- [X] T073 [US2] Create useSearch hook in frontend/src/hooks/useSearch.ts
- [X] T074 [US2] Create searchContext for state management in frontend/src/store/searchContext.tsx
- [X] T075 [US2] Integrate SearchBar into main layout in frontend/src/App.tsx

### US2 Validation with Chrome DevTools

- [ ] T076 [US2] Verify SearchBar renders correctly using chrome-devtools take_snapshot
- [ ] T077 [US2] Test search by name: enter project name, verify results using chrome-devtools
- [ ] T078 [US2] Test search by tag: select tag filter, verify filtered results using chrome-devtools
- [ ] T079 [US2] Test search with no results displays message using chrome-devtools take_snapshot
- [ ] T080 [US2] Test clicking on search result navigates to project using chrome-devtools click

**Checkpoint**: User Stories 1 AND 2 should both work - users can browse OR search to find projects

---

## Phase 5: User Story 3 - Download Project Files (Priority: P2)

**Goal**: Users can download individual files or complete project ZIP archives.

**Independent Test**: Navigate to a project in example/ folder, click download button for individual STL file (verify download), click "Download All as ZIP" (verify ZIP contains all files).

### Backend Implementation for User Story 3

- [ ] T081 [P] [US3] Implement async ZIP streaming service in backend/src/services/download.rs
- [ ] T082 [US3] Implement GET /api/files/:id handler (individual file download) in backend/src/api/handlers/files.rs
- [ ] T083 [US3] Implement GET /api/projects/:id/download handler (ZIP stream) in backend/src/api/handlers/files.rs
- [ ] T084 [US3] Wire US3 routes in backend/src/api/routes.rs

### Frontend Implementation for User Story 3

- [ ] T085 [P] [US3] Create files API client in frontend/src/api/files.ts
- [ ] T086 [P] [US3] Implement download utilities in frontend/src/utils/download.ts
- [ ] T087 [US3] Add download buttons to FileList component in frontend/src/components/project/FileList.tsx
- [ ] T088 [US3] Add "Download All as ZIP" button to ProjectPage in frontend/src/pages/ProjectPage.tsx
- [ ] T089 [US3] Implement download progress indicators in frontend/src/components/common/ProgressIndicator.tsx
- [ ] T090 [US3] Add error handling for failed downloads in frontend/src/components/project/ProjectPage.tsx

### US3 Validation with Chrome DevTools

- [ ] T091 [US3] Verify download buttons render on project page using chrome-devtools take_snapshot
- [ ] T092 [US3] Test individual file download: click download button, verify file downloads
- [ ] T093 [US3] Test ZIP download: click "Download All", verify progress indicator appears using chrome-devtools
- [ ] T094 [US3] Test download error handling: simulate error, verify error message using chrome-devtools

**Checkpoint**: User Stories 1, 2, AND 3 should work - complete browse → search → download workflow functional

---

## Phase 6: User Story 4 - Tag and Organize Projects (Priority: P3)

**Goal**: Users can assign custom tags to projects for flexible, cross-cutting organization.

**Independent Test**: Navigate to project in example/ folder, add tags (e.g., "painted", "priority"), verify tags persist after page refresh, verify tags appear in search results.

### Backend Implementation for User Story 4

- [ ] T095 [P] [US4] Implement POST /api/projects/:id/tags handler (add tag) in backend/src/api/handlers/tags.rs
- [ ] T096 [P] [US4] Implement DELETE /api/projects/:id/tags/:tag_id handler (remove tag) in backend/src/api/handlers/tags.rs
- [ ] T097 [P] [US4] Implement POST /api/tags handler (create new tag) in backend/src/api/handlers/tags.rs
- [ ] T098 [US4] Wire US4 routes in backend/src/api/routes.rs

### Frontend Implementation for User Story 4

- [ ] T099 [P] [US4] Implement TagInput component with autocomplete in frontend/src/components/common/TagInput.tsx
- [ ] T100 [P] [US4] Implement TagManager component in frontend/src/components/project/TagManager.tsx
- [ ] T101 [US4] Integrate TagManager into ProjectPage in frontend/src/pages/ProjectPage.tsx
- [ ] T102 [US4] Create useTags hook in frontend/src/hooks/useTags.ts
- [ ] T103 [US4] Add tag filtering to SearchPage in frontend/src/pages/SearchPage.tsx

### US4 Validation with Chrome DevTools

- [ ] T104 [US4] Verify tag input component renders on project page using chrome-devtools take_snapshot
- [ ] T105 [US4] Test adding tag: type tag name, verify autocomplete suggestions using chrome-devtools
- [ ] T106 [US4] Test tag persistence: add tag, refresh page, verify tag still present using chrome-devtools
- [ ] T107 [US4] Test removing tag: click remove button, verify tag deleted using chrome-devtools
- [ ] T108 [US4] Test tag-based search integration: search by tag, verify correct results using chrome-devtools

**Checkpoint**: User Stories 1-4 complete - full browsing, searching, downloading, and tagging functionality

---

## Phase 7: User Story 5 - Rescan and Update Library (Priority: P3)

**Goal**: Users can trigger rescans to update library when files are added/removed, with tag preservation.

**Independent Test**: Add new files to example/ folder, trigger rescan, verify new projects appear. Remove files, rescan, verify projects removed. Verify existing tags persist.

### Backend Implementation for User Story 5

- [ ] T109 [P] [US5] Implement rescan logic with tag preservation in backend/src/services/rescan.rs
- [ ] T110 [P] [US5] Implement cache cleanup for orphaned files in backend/src/services/image_cache.rs
- [ ] T111 [US5] Implement project removal for deleted files in backend/src/services/rescan.rs
- [ ] T112 [US5] Update POST /api/scan handler to support force rescan in backend/src/api/handlers/scan.rs
- [ ] T113 [US5] Add rescan progress reporting to GET /api/scan/status in backend/src/api/handlers/scan.rs

### Frontend Implementation for User Story 5

- [ ] T114 [P] [US5] Add "Rescan" button to HomePage in frontend/src/pages/HomePage.tsx
- [ ] T115 [P] [US5] Implement rescan progress display in frontend/src/components/common/ScanProgress.tsx
- [ ] T116 [US5] Add rescan confirmation dialog in frontend/src/components/common/ConfirmDialog.tsx
- [ ] T117 [US5] Display scan error summary in frontend/src/pages/HomePage.tsx

### US5 Validation with Chrome DevTools

- [ ] T118 [US5] Test rescan with new files: add files to example/, click rescan, verify new projects appear using chrome-devtools
- [ ] T119 [US5] Test rescan with deleted files: remove files from example/, rescan, verify projects removed using chrome-devtools
- [ ] T120 [US5] Test tag preservation: tag project, rescan, verify tags persist using chrome-devtools
- [ ] T121 [US5] Test rescan progress display using chrome-devtools take_snapshot during scan

**Checkpoint**: All 5 user stories complete - full library management functionality

---

## Phase 8: STL Preview Generation

**Purpose**: Generate preview images from STL files for better visual browsing

### Backend Implementation

- [ ] T122 [P] Implement stl-thumb integration service in backend/src/services/stl_preview.rs
- [ ] T123 [P] Implement preview generation queue in backend/src/services/stl_preview.rs
- [ ] T124 Implement GET /api/previews/:hash handler in backend/src/api/handlers/files.rs
- [ ] T125 Add preview generation to scan workflow in backend/src/services/scanner.rs
- [ ] T126 Implement fallback to placeholder images in backend/src/services/stl_preview.rs

### Frontend Implementation

- [ ] T127 Update ProjectTile to display STL preview if available in frontend/src/components/project/ProjectTile.tsx
- [ ] T128 Update FileList to show preview thumbnails in frontend/src/components/project/FileList.tsx
- [ ] T129 Add preview generation status indicator in frontend/src/pages/ProjectPage.tsx

### Validation with Example Folder

- [ ] T130 Test STL preview generation: scan example/ folder with STL files, verify previews generate
- [ ] T131 Test fallback behavior: verify placeholder shows if stl-thumb not installed or generation fails
- [ ] T132 Verify preview caching: rescan, confirm previews not regenerated unnecessarily

**Checkpoint**: STL previews enhance visual browsing experience

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

### UI/UX Polish

- [ ] T133 [P] Implement responsive design for mobile/tablet in frontend/src/App.tsx
- [ ] T134 [P] Add loading skeletons for project tiles in frontend/src/components/common/Skeleton.tsx
- [ ] T135 [P] Implement empty states for no projects/results in frontend/src/components/common/EmptyState.tsx
- [ ] T136 [P] Add keyboard navigation support in frontend/src/components/project/ProjectGrid.tsx
- [ ] T137 [P] Implement toast notifications for user actions in frontend/src/components/common/Toast.tsx
- [ ] T138 [P] Add accessibility improvements (ARIA labels, focus management) across all components

### Performance & Optimization

- [ ] T139 [P] Add database query optimization and indexing in backend/src/db/migrations/
- [ ] T140 [P] Implement lazy loading for images in frontend/src/components/project/ImageGallery.tsx
- [ ] T141 [P] Add image thumbnail generation in backend/src/services/image_cache.rs
- [ ] T142 [P] Optimize bundle size in frontend/vite.config.ts
- [ ] T143 Run performance benchmarks with example/ folder containing 100+ projects

### Testing & Documentation

- [ ] T144 [P] Write integration tests for scan workflow in backend/tests/integration/scan_tests.rs
- [ ] T145 [P] Write integration tests for API endpoints in backend/tests/integration/api_tests.rs
- [ ] T146 [P] Write component tests for key UI components in frontend/tests/unit/
- [ ] T147 [P] Update README.md with installation and setup instructions
- [ ] T148 [P] Update quickstart.md with validation checklist
- [ ] T149 [P] Create user documentation in docs/user-guide.md
- [ ] T150 Run complete quickstart.md validation with example/ folder

### Error Handling & Logging

- [ ] T151 [P] Improve error messages across all API endpoints in backend/src/api/handlers/
- [ ] T152 [P] Add structured logging with tracing in backend/src/main.rs
- [ ] T153 [P] Implement client-side error boundary in frontend/src/components/ErrorBoundary.tsx
- [ ] T154 Add error reporting for scan failures in backend/src/services/scanner.rs

**Checkpoint**: Production-ready application with comprehensive polish and testing

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup (Phase 1) completion - BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational (Phase 2) - Core browsing functionality
- **User Story 2 (Phase 4)**: Depends on Foundational (Phase 2) - Can run parallel with US1 but typically after
- **User Story 3 (Phase 5)**: Depends on US1 (Phase 3) - Needs project navigation to download from
- **User Story 4 (Phase 6)**: Depends on US1, US2 (Phases 3-4) - Enhances search and browse
- **User Story 5 (Phase 7)**: Depends on US1 (Phase 3) - Extends scanning functionality
- **STL Previews (Phase 8)**: Depends on US1 (Phase 3) - Enhances visual browsing
- **Polish (Phase 9)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational - No dependencies on other stories
  - Delivers: Basic scanning and browsing
  - Enables: MVP functionality
  
- **User Story 2 (P2)**: Can start after Foundational - Independent of US1 implementation but uses US1 data
  - Delivers: Search and filtering
  - Enhances: Finding projects in large collections
  
- **User Story 3 (P2)**: Needs US1 complete - Relies on project navigation UI
  - Delivers: File downloads
  - Completes: Browse → view → download workflow
  
- **User Story 4 (P3)**: Needs US1, US2 complete - Enhances both browsing and search
  - Delivers: Custom tagging
  - Enables: Flexible organization
  
- **User Story 5 (P3)**: Needs US1 complete - Extends scanning functionality
  - Delivers: Library updates
  - Enables: Long-term maintenance

### Within Each User Story

- Backend models before services
- Services before API handlers
- API handlers before route wiring
- API client before frontend components
- Basic components before page components
- Pages before hooks
- Implementation before validation
- Story complete before moving to next priority

### Parallel Opportunities

**Phase 1 (Setup)**: T003, T004, T005, T006 can run in parallel

**Phase 2 (Foundational - Backend)**: T008-T013 (models) can run in parallel, then T015-T016 (repos) in parallel, then T018-T021 (middleware/utils) in parallel

**Phase 2 (Foundational - Frontend)**: T023-T025 (API/types) in parallel, then T027-T030 (components) in parallel

**Within User Stories**: All tasks marked [P] can run in parallel

**Across User Stories**: Once Foundational complete, US1 and US2 can start in parallel if team capacity allows

---

## Parallel Example: User Story 1 Backend

```bash
# All models can be created in parallel:
T009: backend/src/models/project.rs
T010: backend/src/models/stl_file.rs
T011: backend/src/models/image_file.rs

# Then repositories in parallel:
T014: backend/src/db/repositories/project_repo.rs
T015: backend/src/db/repositories/file_repo.rs

# Services in parallel:
T031: backend/src/services/config.rs
T032: backend/src/services/scanner.rs
T034: backend/src/services/image_cache.rs
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (browse and navigate)
4. **STOP and VALIDATE**: Test US1 with example/ folder
5. Deploy/demo basic library browsing

**Result**: Users can scan a folder and browse their 3D print collection visually

### Incremental Delivery

1. **Foundation**: Setup + Foundational → Dev environment ready
2. **MVP (US1)**: Add browsing → Test independently with example/ → Deploy
3. **Search (US2)**: Add search → Test with tagged projects → Deploy
4. **Downloads (US3)**: Add downloads → Test file and ZIP downloads → Deploy
5. **Tags (US4)**: Add tagging → Test tag management → Deploy
6. **Rescan (US5)**: Add rescan → Test library updates → Deploy
7. **Polish**: Add STL previews and final polish → Production ready

### Parallel Team Strategy

With multiple developers:

1. **Weeks 1-2**: Team completes Setup + Foundational together
2. **Weeks 3-4**: Once Foundational done:
   - Developer A: User Story 1 (Backend + Frontend)
   - Developer B: User Story 2 (Backend + Frontend)
   - Developer C: Foundational components refinement
3. **Weeks 5-6**: 
   - Developer A: User Story 3
   - Developer B: User Story 4
   - Developer C: User Story 5
4. **Weeks 7-8**: All developers on STL Previews and Polish
5. Stories integrate and validate independently

### Chrome DevTools Validation Pattern

Each user story includes validation tasks using chrome-devtools MCP:
1. Take snapshot to verify UI renders correctly
2. Use fill() to enter test data (paths, search terms, tags)
3. Use click() to test interactive elements
4. Take snapshots to verify state changes
5. Validate with example/ folder containing known test data

---

## Example Folder Structure

The `example/` folder should contain:
```
example/
├── miniatures/
│   ├── fantasy/
│   │   ├── dragon/
│   │   │   ├── dragon.stl
│   │   │   ├── photo1.jpg
│   │   │   └── photo2.jpg
│   │   └── wiking/
│   │       ├── warrior.stl
│   │       └── image.png
│   └── scifi/
│       └── robot.stl
├── terrain/
│   ├── rocks.stl
│   └── hill.stl
└── vehicles/
    ├── car.stl
    └── tank.stl
```

This provides:
- Hierarchical structure (2-3 levels deep)
- Multiple projects with STL files
- Mix of projects with/without images
- Both JPG and PNG image formats
- Suitable for testing all user stories

---

## Notes

- **[P] tasks**: Different files, no dependencies - can run in parallel
- **[Story] label**: Maps task to specific user story for traceability
- **Example folder**: Used throughout for testing and validation
- **Chrome DevTools**: Automated UI validation for each story
- **Independent stories**: Each user story should be completable and testable independently
- **Checkpoint validation**: Stop at any checkpoint to validate story works standalone
- **Commit strategy**: Commit after each task or logical group
- **MVP focus**: User Story 1 delivers immediate value (browsing existing collection)

---

## Success Criteria Validation

After completing all tasks, validate against spec.md success criteria:

- **SC-001**: Setup completes in <5 minutes (validate with quickstart.md)
- **SC-002**: Scan 100+ projects/minute (test with large example/ folder)
- **SC-004**: Search <1 second for 10k projects (load test with generated data)
- **SC-007**: Tile display <2 seconds (test with chrome-devtools)
- **SC-009**: ZIP starts <10 seconds for 50 files (test with example/ project)
- **SC-011**: 95% search success rate (test with various queries)

---

## Total Task Count: 154 tasks

**By User Story**:
- Setup: 6 tasks
- Foundational: 24 tasks (blocks everything)
- User Story 1: 33 tasks (MVP - browse & navigate)
- User Story 2: 17 tasks (search & filter)
- User Story 3: 14 tasks (download files)
- User Story 4: 14 tasks (tag & organize)
- User Story 5: 13 tasks (rescan & update)
- STL Previews: 11 tasks (visual enhancement)
- Polish: 22 tasks (production ready)

**Parallel Opportunities**: 89 tasks marked [P] can run concurrently

**MVP Scope**: Setup + Foundational + US1 = 63 tasks (41% of total)

**Independent Test Criteria**: Each user story includes chrome-devtools validation tasks using example/ folder
