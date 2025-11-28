# Tasks: Browse View Refactoring - File Explorer Style with Image Inheritance

**Input**: Design documents from `/specs/001-browse-view-refactor/`
**Prerequisites**: plan.md, spec.md
**Branch**: `001-browse-view-refactor`
**Generated**: 2025-11-28

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `- [ ] [ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4)
- Include exact file paths in descriptions

## Path Conventions

This is a web application with:
- **Backend**: `backend/src/`
- **Frontend**: `frontend/src/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and configuration for browse view refactoring

- [x] T001 Review current BrowsePage.tsx implementation in frontend/src/pages/BrowsePage.tsx
- [x] T002 Review current project/image database schema in backend/src/db/
- [x] T003 Review scanner service implementation in backend/src/services/scanner.rs
- [x] T004 [P] Add IGNORED_KEYWORDS configuration to backend/src/config.rs
- [x] T005 [P] Update docker-compose.yml with IGNORED_KEYWORDS environment variable
- [x] T006 [P] Update .env.example with IGNORED_KEYWORDS configuration example

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T007 Create database migration for image inheritance tracking in backend/src/db/migrations.rs
- [x] T008 Add folder_level column/tracking to projects table in migration
- [x] T009 [P] Create Folder model in backend/src/models/folder.rs
- [x] T010 [P] Update Project model to include inherited_images field in backend/src/models/project.rs
- [x] T011 Add image inheritance queries to backend/src/db/queries.rs
- [x] T012 Apply database migration and verify schema changes

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Folder-by-Folder Navigation (Priority: P1) 🎯 MVP

**Goal**: Users can navigate through the project folder hierarchy one level at a time, similar to a file explorer, seeing contained folders and projects at each level before descending deeper.

**Independent Test**: Navigate through folder structure (e.g., starting at "Miniaturen", clicking into "The Printing Goes Ever On", then "Welcome Trove") and verify that each level shows only the immediate children folders/projects without automatically expanding the entire tree.

### Implementation for User Story 1

#### Backend Navigation API

- [ ] T013 [P] [US1] Create FolderService for navigation logic in backend/src/services/folder_service.rs
- [ ] T014 [US1] Implement get_folder_contents(path) method in FolderService
- [ ] T015 [US1] Implement get_breadcrumb_trail(path) method in FolderService
- [ ] T016 [US1] Add pagination support for large folders in FolderService
- [ ] T017 [US1] Create browse_routes.rs with folder navigation endpoints in backend/src/api/browse_routes.rs
- [ ] T018 [US1] Implement GET /api/browse/:path endpoint in browse_routes.rs
- [ ] T019 [US1] Implement GET /api/browse/:path/breadcrumb endpoint in browse_routes.rs
- [ ] T020 [US1] Add path traversal security validation in browse_routes.rs
- [ ] T021 [US1] Add error handling for non-existent paths in browse_routes.rs
- [ ] T022 [P] [US1] Update project_service.rs to support folder-level queries in backend/src/services/project_service.rs

#### Frontend Navigation Components

- [ ] T023 [P] [US1] Create Breadcrumb component in frontend/src/components/Breadcrumb.tsx
- [ ] T024 [P] [US1] Create FolderTile component in frontend/src/components/FolderTile.tsx
- [ ] T025 [US1] Create FolderView component in frontend/src/pages/FolderView.tsx
- [ ] T026 [US1] Add folder navigation API calls to frontend/src/api/client.ts
- [ ] T027 [US1] Implement fetchFolderContents(path) in API client
- [ ] T028 [US1] Implement fetchBreadcrumb(path) in API client
- [ ] T029 [US1] Add request cancellation for rapid navigation in API client

#### Frontend Routing & Integration

- [ ] T030 [US1] Update React Router with /browse/:folderPath* route in frontend/src/App.tsx
- [ ] T031 [US1] Refactor BrowsePage.tsx for folder-level navigation in frontend/src/pages/BrowsePage.tsx
- [ ] T032 [US1] Integrate Breadcrumb component into BrowsePage.tsx
- [ ] T033 [US1] Integrate FolderView component into BrowsePage.tsx
- [ ] T034 [US1] Implement browser back/forward history support in BrowsePage.tsx
- [ ] T035 [US1] Add keyboard navigation support for folder tiles in FolderView.tsx
- [ ] T036 [US1] Test navigation through example/Miniaturen folder structure

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently. Users can navigate folder-by-folder through the entire hierarchy with breadcrumb navigation.

---

## Phase 4: User Story 2 - Project Preview Display (Priority: P2)

**Goal**: Users can see visual previews of projects contained within the current folder level, allowing them to identify interesting projects before clicking through.

**Independent Test**: View any folder that contains projects and verify that each project displays a preview image (either its own images or inherited ones).

### Implementation for User Story 2

#### Backend Project Preview Support

- [x] T037 [US2] Update GET /api/projects/:id endpoint to include images in backend/src/api/project_routes.rs
- [x] T038 [US2] Add project preview metadata to folder contents response in FolderService
- [x] T039 [US2] Optimize image path queries for folder-level display in backend/src/db/queries.rs
- [ ] T040 [US2] Add caching for frequently accessed project previews in FolderService

#### Frontend Project Preview Components

- [x] T041 [US2] Update ProjectPreview component to display images in frontend/src/components/ProjectPreview.tsx
- [x] T042 [US2] Add image carousel functionality to ProjectPreview component
- [x] T043 [US2] Implement placeholder/default icon for projects without images in ProjectPreview.tsx
- [ ] T044 [US2] Add responsive grid layout for projects in FolderView.tsx
- [ ] T045 [US2] Handle loading and error states for project previews in FolderView.tsx
- [ ] T046 [US2] Test project preview display with various image scenarios

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently. Navigation works and project previews display correctly with their own images.

---

## Phase 5: User Story 3 - Image Inheritance Down Hierarchy (Priority: P3)

**Goal**: Images found at any level in the folder structure are inherited and displayed by all projects below them in the hierarchy, providing visual context throughout nested structures.

**Independent Test**: Examine the "Welcome Trove/heroes fighting.jpg" image and verify it appears in all descendant projects like "Welcome-Trove-Remastered" and "Samuel" subdirectories.

### Implementation for User Story 3

#### Backend Image Inheritance Logic

- [ ] T047 [US3] Create ImageService for inheritance calculation in backend/src/services/image_service.rs
- [ ] T048 [US3] Implement calculate_inheritance(project_path) method in ImageService
- [ ] T049 [US3] Implement deduplication logic by filename in ImageService
- [ ] T050 [US3] Add caching for inheritance chains in ImageService
- [ ] T051 [US3] Update scanner to populate inheritance data during scan in backend/src/services/scanner.rs
- [ ] T052 [US3] Add progress logging for inheritance calculation in scanner.rs
- [ ] T053 [US3] Update GET /api/projects/:id to include inherited_images in backend/src/api/project_routes.rs
- [ ] T054 [US3] Add inherited_from_paths field to project response in project_routes.rs

#### Frontend Image Inheritance Display

- [ ] T055 [US3] Create useImageInheritance hook in frontend/src/hooks/useImageInheritance.ts
- [ ] T056 [US3] Update ProjectPreview to display inherited images in frontend/src/components/ProjectPreview.tsx
- [ ] T057 [US3] Add badge/indicator showing inherited vs. own images in ProjectPreview.tsx
- [ ] T058 [US3] Handle case where all images are inherited in ProjectPreview.tsx
- [ ] T059 [US3] Test image inheritance with example/Miniaturen/The Printing Goes Ever On/Welcome Trove
- [ ] T060 [US3] Verify "heroes fighting.jpg" appears in descendant projects

**Checkpoint**: All user stories 1, 2, and 3 should now be independently functional. Image inheritance works throughout the hierarchy.

---

## Phase 6: User Story 4 - Substring Keyword Matching for STL Categorization (Priority: P4)

**Goal**: The system uses substring matching (case-insensitive) for ignored keywords to correctly identify STL container folders versus actual projects, enabling proper categorization of STL files.

**Independent Test**: Verify that folders like "1 inch" and "2 inch" are treated as STL categories under the "Desert" project when "inch" is in the IGNORED_KEYWORDS list.

### Implementation for User Story 4

#### Backend Substring Keyword Matching

- [ ] T061 [US4] Refactor scanner keyword matching to use substring matching in backend/src/services/scanner.rs
- [ ] T062 [US4] Implement case-insensitive substring comparison in scanner.rs
- [ ] T063 [US4] Add string trimming and normalization in scanner.rs
- [ ] T064 [US4] Update project vs. STL container detection logic in scanner.rs
- [ ] T065 [US4] Update FolderService to handle STL category folders in backend/src/services/folder_service.rs
- [ ] T066 [US4] Add STL file grouping by category to project response in backend/src/services/project_service.rs

#### Frontend STL Category Display

- [ ] T067 [US4] Update ProjectPreview to display STL categories in frontend/src/components/ProjectPreview.tsx
- [ ] T068 [US4] Add UI for grouped STL files by category in ProjectPreview.tsx
- [ ] T069 [US4] Test with IGNORED_KEYWORDS containing "inch", "mm", "STL"
- [ ] T070 [US4] Verify "1 inch", "2 inch", "PRESUPPORTED_STL" folders categorized correctly
- [ ] T071 [US4] Test edge cases with overlapping keywords

**Checkpoint**: All user stories should now be independently functional. STL categorization works correctly with substring matching.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

### Performance & Optimization

- [ ] T072 [P] Optimize folder navigation query performance in backend/src/db/queries.rs
- [ ] T073 [P] Add database indexing for folder-level queries in backend/src/db/migrations.rs
- [ ] T074 [P] Profile and optimize image inheritance calculation performance
- [ ] T075 [P] Test navigation with folders containing 100+ projects
- [ ] T076 [P] Verify response times meet <500ms target for folder navigation

### Error Handling & Edge Cases

- [ ] T077 [P] Add comprehensive error messages for navigation failures
- [ ] T078 [P] Handle corrupt or missing image files gracefully
- [ ] T079 [P] Test deep hierarchy navigation (10+ levels)
- [ ] T080 [P] Test folder names with special characters
- [ ] T081 [P] Add empty state UI for folders with no content

### Accessibility & Cross-Browser

- [ ] T082 [P] Add ARIA labels to navigation components
- [ ] T083 [P] Test keyboard navigation with screen readers
- [ ] T084 [P] Verify color contrast ratios meet WCAG standards
- [ ] T085 [P] Test in Chrome, Firefox, and Safari
- [ ] T086 [P] Test responsive layout on mobile devices

### Documentation

- [ ] T087 [P] Update README.md with new navigation model explanation
- [ ] T088 [P] Add screenshots of folder navigation to documentation
- [ ] T089 [P] Document IGNORED_KEYWORDS configuration in README.md
- [ ] T090 [P] Update API documentation with new endpoints in specs/001-browse-view-refactor/contracts/
- [ ] T091 [P] Create user guide for folder navigation in docs/

### Deployment Preparation

- [ ] T092 Create database backup before migration
- [ ] T093 Test migration on staging environment
- [ ] T094 Verify Docker build succeeds with new configuration
- [ ] T095 Create rollback plan documentation
- [ ] T096 Prepare release notes with feature summary

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational phase completion
- **User Story 2 (Phase 4)**: Depends on User Story 1 completion (needs navigation working)
- **User Story 3 (Phase 5)**: Depends on User Story 2 completion (needs preview display working)
- **User Story 4 (Phase 6)**: Can start after Foundational phase - independent of US1/US2/US3
- **Polish (Phase 7)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - Foundation for all navigation
- **User Story 2 (P2)**: Depends on User Story 1 - needs folder navigation to display previews
- **User Story 3 (P3)**: Depends on User Story 2 - enhances preview display with inheritance
- **User Story 4 (P4)**: Can start after Foundational (Phase 2) - Independent of US1/US2/US3, focuses on scanner logic

**Note**: User Stories 1, 2, and 3 form a dependent chain (navigation → preview → inheritance). User Story 4 (substring matching) is independent and can be developed in parallel with the navigation chain after foundational work is complete.

### Within Each User Story

- Backend API before Frontend components
- Models before services
- Services before API endpoints
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Within User Story 1: Backend API tasks (T013-T022) and Frontend components (T023-T029) can partially overlap
- Within User Story 2: Backend support (T037-T040) and Frontend components (T041-T046) can partially overlap
- Within User Story 3: Backend logic (T047-T054) and Frontend display (T055-T060) can partially overlap
- Within User Story 4: Backend matching (T061-T066) and Frontend display (T067-T071) can partially overlap
- User Story 4 can be developed in parallel with US1/US2/US3 by a separate developer
- All Polish phase tasks marked [P] can run in parallel

---

## Parallel Example: User Story 1 Backend

```bash
# Launch all backend model creation together:
Task: "Create Folder model in backend/src/models/folder.rs"
Task: "Update Project model to include inherited_images field in backend/src/models/project.rs"

# Launch backend components after models complete:
Task: "Create FolderService for navigation logic in backend/src/services/folder_service.rs"
Task: "Update project_service.rs to support folder-level queries in backend/src/services/project_service.rs"
```

## Parallel Example: User Story 1 Frontend

```bash
# Launch all frontend components together:
Task: "Create Breadcrumb component in frontend/src/components/Breadcrumb.tsx"
Task: "Create FolderTile component in frontend/src/components/FolderTile.tsx"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T006)
2. Complete Phase 2: Foundational (T007-T012) - CRITICAL - blocks all stories
3. Complete Phase 3: User Story 1 (T013-T036)
4. **STOP and VALIDATE**: Test User Story 1 independently through folder navigation
5. Deploy/demo if ready - Users can now navigate folder-by-folder

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!) - Folder navigation works
3. Add User Story 2 → Test independently → Deploy/Demo - Project previews appear
4. Add User Story 3 → Test independently → Deploy/Demo - Image inheritance complete
5. Add User Story 4 → Test independently → Deploy/Demo - STL categorization refined
6. Complete Polish phase → Final production deployment
7. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together (T001-T012)
2. Once Foundational is done:
   - **Developer A**: User Stories 1, 2, 3 (sequential - T013-T060) - Navigation chain
   - **Developer B**: User Story 4 (T061-T071) - Substring matching
   - **Developer C**: Polish tasks (T072-T096) - Can start incrementally as stories complete
3. Stories complete and integrate independently

**Recommended**: Single developer should work sequentially through US1 → US2 → US3 → US4 due to the dependency chain in navigation features.

---

## Success Criteria Summary

### User Story 1 Success

- ✅ Can navigate through folder hierarchy one level at a time
- ✅ Breadcrumb navigation works correctly
- ✅ URL reflects current folder path
- ✅ Browser back/forward buttons work
- ✅ Response time < 500ms for folder navigation

### User Story 2 Success

- ✅ Project previews display at least one image per project
- ✅ Projects with images show their images
- ✅ Projects without images show placeholder
- ✅ Preview carousel works smoothly
- ✅ Responsive grid layout displays correctly

### User Story 3 Success

- ✅ Images inherit from parent folders to all descendants
- ✅ "heroes fighting.jpg" appears in Welcome Trove descendants
- ✅ Duplicate images are deduplicated by filename
- ✅ Both inherited and own images display together
- ✅ Inheritance source paths are tracked

### User Story 4 Success

- ✅ Folders matching IGNORED_KEYWORDS substrings are STL categories
- ✅ "1 inch", "2 inch" treated as STL containers (not projects)
- ✅ "PRESUPPORTED_STL" matches "STL" substring
- ✅ Case-insensitive matching works correctly
- ✅ STL files are grouped by category folder names

### Overall Success

- ✅ All acceptance scenarios from spec.md pass
- ✅ Performance targets met (navigation <500ms, render <200ms)
- ✅ Works across Chrome, Firefox, Safari
- ✅ Keyboard navigation and accessibility working
- ✅ No data loss or breaking changes to existing functionality

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Run existing tests frequently to catch regressions early
- Monitor performance during development, not just at the end
- Document decisions and trade-offs as you implement
- Test with example folder structure throughout development
