# Tasks: Modern Tile-Based UI

**Input**: Design documents from `/specs/001-ui-modernization/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/component-api.md, quickstart.md

**Tests**: Component tests included with Vitest. Chrome-devtools-mcp used for UI validation.

**Organization**: Tasks grouped by user story to enable independent implementation and testing.

## Format: `- [ ] [ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2, US3, US4, US5, US6)
- Include exact file paths in descriptions

## Path Conventions

- **Web app structure**: `frontend/src/` for React components, `frontend/` for config
- All paths relative to repository root

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Install Tailwind CSS and create foundational configuration

- [X] T001 Install Tailwind CSS dependencies in frontend/package.json (tailwindcss ^3.4, postcss ^8.4, autoprefixer ^10.4)
- [X] T002 Create Tailwind configuration in frontend/tailwind.config.js with custom theme (primary colors, animations)
- [X] T003 [P] Create PostCSS configuration in frontend/postcss.config.js
- [X] T004 Update CSS entry point frontend/src/index.css with Tailwind directives and custom base styles
- [X] T005 Create design tokens CSS layer in frontend/src/index.css (@layer components for btn-primary, card classes)
- [ ] T006 [P] Take baseline screenshots with chrome-devtools-mcp (current-home.png, current-browse.png)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Create shared UI components and types that all user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T007 Create BreadcrumbItem interface in frontend/src/types/breadcrumb.ts
- [X] T008 Create TileMetadata interface in frontend/src/types/tile.ts
- [X] T009 Create helper function calculateTileMetadata in frontend/src/utils/tileMetadata.ts
- [X] T010 Create helper function formatBytes in frontend/src/utils/formatBytes.ts
- [X] T011 [P] Create SkeletonTile component in frontend/src/components/project/SkeletonTile.tsx
- [X] T012 [P] Create EmptyState component enhancement in frontend/src/components/common/EmptyState.tsx

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Browse Root Project Folders (Priority: P1) 🎯 MVP

**Goal**: Display all root-level projects and folders as modern tiles in a responsive grid layout with preview images, names, and metadata

**Independent Test**: Launch application, verify root folders appear as tiles with preview images/icons, names, file counts, and sizes. Grid should be responsive from mobile to desktop.

### Implementation for User Story 1

- [X] T013 [P] [US1] Create NavBar component in frontend/src/components/common/NavBar.tsx (top navigation with logo, search, links)
- [X] T014 [P] [US1] Create ScanButton component in frontend/src/components/scan/ScanButton.tsx (extracted from HomePage)
- [X] T015 [US1] Refactor ProjectTile component in frontend/src/components/project/ProjectTile.tsx (card design with shadows, hover effects)
- [X] T016 [US1] Create ProjectTile styles in frontend/src/components/project/ProjectTile.css (card styling with Tailwind classes)
- [X] T017 [US1] Enhance ProjectGrid component in frontend/src/components/project/ProjectGrid.tsx (responsive grid with proper spacing)
- [X] T018 [US1] Create ProjectGrid styles in frontend/src/components/project/ProjectGrid.css (responsive breakpoints)
- [X] T019 [US1] Update App.tsx in frontend/src/App.tsx to integrate NavBar component
- [X] T020 [US1] Update HomePage in frontend/src/pages/HomePage.tsx (remove scan button, integrate with NavBar)
- [X] T021 [P] [US1] Add lazy loading for preview images in ProjectTile (loading="lazy" attribute)
- [X] T022 [P] [US1] Component test for ProjectTile in frontend/src/components/project/ProjectTile.test.tsx
- [X] T023 [P] [US1] Component test for ProjectGrid in frontend/src/components/project/ProjectGrid.test.tsx
- [ ] T024 [US1] Take screenshots with chrome-devtools-mcp for US1 validation (tiles-grid.png, tiles-hover.png)
- [ ] T025 [US1] Validate responsive behavior with chrome-devtools-mcp at breakpoints (320px, 768px, 1024px, 1920px)

**Checkpoint**: At this point, User Story 1 should be fully functional - users can browse root folders in a modern tile grid

---

## Phase 4: User Story 2 - Navigate Hierarchical Project Structure (Priority: P1)

**Goal**: Enable clicking folder tiles to view children and navigate back using breadcrumbs for intuitive hierarchical navigation

**Independent Test**: Click any parent folder tile, verify child projects display as tiles. Click breadcrumb links to navigate back up hierarchy. Breadcrumb trail should show current location.

### Implementation for User Story 2

- [X] T026 [P] [US2] Create Breadcrumb component in frontend/src/components/common/Breadcrumb.tsx (navigation trail with click handlers)
- [X] T027 [P] [US2] Create Breadcrumb styles in frontend/src/components/common/Breadcrumb.css (modern styling with Tailwind)
- [X] T028 [US2] Refactor BrowsePage component in frontend/src/pages/BrowsePage.tsx (hierarchical navigation with parent_id filtering)
- [X] T029 [US2] Add breadcrumb state management in BrowsePage (currentFolderId, breadcrumbs array)
- [X] T030 [US2] Implement folder navigation logic in BrowsePage (handleTileClick for folders vs projects)
- [X] T031 [US2] Implement breadcrumb click handler in BrowsePage (handleBreadcrumbClick)
- [X] T032 [US2] Add folder vs project visual distinction in ProjectTile (folder icon vs project icon)
- [X] T033 [P] [US2] Component test for Breadcrumb in frontend/src/components/common/Breadcrumb.test.tsx
- [ ] T034 [P] [US2] Integration test for BrowsePage navigation in frontend/src/pages/BrowsePage.test.tsx
- [ ] T035 [US2] Take screenshots with chrome-devtools-mcp for US2 validation (breadcrumb-nav.png, folder-children.png)
- [ ] T036 [US2] Test navigation flow with chrome-devtools-mcp (root → folder → children → back)

**Checkpoint**: At this point, User Stories 1 AND 2 should both work - users can browse root AND navigate into folders

---

## Phase 5: User Story 3 - Access Library Management Tools (Priority: P2)

**Goal**: Provide "Rescan Library" button in top-right navigation bar for easy library refresh without cluttering main browsing area

**Independent Test**: Locate Rescan button in top-right of navigation bar on any page. Click button and verify library scan initiates with progress feedback.

### Implementation for User Story 3

- [X] T037 [US3] Integrate ScanButton into NavBar component in frontend/src/components/common/NavBar.tsx (top-right positioning)
- [X] T038 [US3] Add scan progress state to ScanButton in frontend/src/components/scan/ScanButton.tsx (isScanning, progress)
- [X] T039 [US3] Implement scan API integration in ScanButton (scanAPI.start, progress updates)
- [X] T040 [US3] Add loading spinner animation to ScanButton during scan
- [X] T041 [US3] Add error handling to ScanButton (display error message, retry button)
- [X] T042 [US3] Update HomePage in frontend/src/pages/HomePage.tsx (remove old scan controls if any remain)
- [X] T043 [P] [US3] Component test for ScanButton in frontend/src/components/scan/ScanButton.test.tsx
- [X] T044 [US3] Take screenshots with chrome-devtools-mcp for US3 validation (navbar-scan-button.png, scan-in-progress.png)
- [X] T045 [US3] Validate scan button placement with chrome-devtools-mcp on mobile and desktop layouts

**Checkpoint**: All core navigation (US1, US2) and management tools (US3) are functional

---

## Phase 6: User Story 4 - Experience Modern Visual Design (Priority: P2)

**Goal**: Apply professional modern appearance with card-based tiles, proper spacing, visual hierarchy, shadows, and clean typography

**Independent Test**: Visual inspection of tile design, spacing, shadows, colors, and typography. Tiles should have distinct card appearance with proper hover effects and clear hierarchy.

### Implementation for User Story 4

- [X] T046 [P] [US4] Enhance ProjectTile card styling in frontend/src/components/project/ProjectTile.css (shadows, rounded corners, borders)
- [X] T047 [P] [US4] Add hover effects to ProjectTile in frontend/src/components/project/ProjectTile.css (shadow-lg on hover, overlay transition)
- [X] T048 [P] [US4] Enhance typography hierarchy in frontend/src/index.css (heading sizes, weights, colors)
- [X] T049 [US4] Add metadata display to ProjectTile in frontend/src/components/project/ProjectTile.tsx (file count with icon, size)
- [X] T050 [US4] Add type badge to ProjectTile in frontend/src/components/project/ProjectTile.tsx (Folder/Project badge with colors)
- [X] T051 [US4] Create distinct icons for folders vs projects in ProjectTile (FolderIcon vs CubeIcon SVG)
- [X] T052 [US4] Add loading skeleton states to ProjectGrid in frontend/src/components/project/ProjectGrid.tsx (SkeletonTile during fetch)
- [X] T053 [US4] Enhance empty state in ProjectGrid in frontend/src/components/project/ProjectGrid.tsx (icon, message, action)
- [X] T054 [P] [US4] Update Breadcrumb visual styling in frontend/src/components/common/Breadcrumb.css (colors, hover states, separators)
- [X] T055 [P] [US4] Update SearchBar visual styling in frontend/src/components/common/SearchBar.tsx (modern input design)
- [X] T056 [US4] Add consistent spacing throughout grid in frontend/src/components/project/ProjectGrid.css (gap-4 mobile, gap-6 desktop)
- [ ] T057 [US4] Take comparison screenshots with chrome-devtools-mcp (before-after-tiles.png, visual-hierarchy.png)
- [ ] T058 [US4] Validate color contrast with chrome-devtools-mcp accessibility tools (WCAG AA compliance)

**Checkpoint**: Visual design is complete and professional across all components

---

## Phase 7: User Story 5 - Navigate With Keyboard (Priority: P3)

**Goal**: Enable full keyboard navigation through tiles using Tab and Enter keys for efficient mouse-free usage

**Independent Test**: Tab through tiles on browse page, verify focus indicators are visible. Press Enter on focused tile and verify navigation occurs. Arrow keys should navigate grid.

### Implementation for User Story 5

- [X] T059 [US5] Add keyboard event handlers to ProjectTile in frontend/src/components/project/ProjectTile.tsx (onKeyDown for Enter/Space)
- [X] T060 [US5] Add ARIA roles to ProjectTile in frontend/src/components/project/ProjectTile.tsx (role="button", tabindex="0")
- [X] T061 [US5] Enhance focus indicators in ProjectTile in frontend/src/components/project/ProjectTile.css (ring-2 ring-blue-500)
- [X] T062 [US5] Verify arrow key navigation in ProjectGrid (existing keyboard nav should work)
- [X] T063 [US5] Add ARIA labels to ProjectGrid in frontend/src/components/project/ProjectGrid.tsx (role="grid", aria-label)
- [X] T064 [US5] Add keyboard navigation to Breadcrumb in frontend/src/components/common/Breadcrumb.tsx (Tab through links, Enter to navigate)
- [X] T065 [US5] Add skip-to-content link in NavBar in frontend/src/components/common/NavBar.tsx
- [ ] T066 [P] [US5] Accessibility test with chrome-devtools-mcp (keyboard navigation flow, focus indicators)
- [ ] T067 [US5] Test screen reader announcements with chrome-devtools-mcp a11y tree

**Checkpoint**: Full keyboard navigation is functional and accessible

---

## Phase 8: User Story 6 - Browse Large Collections Efficiently (Priority: P3)

**Goal**: Implement lazy loading for images and optimize grid performance for smooth scrolling with 100+ projects

**Independent Test**: Load large project collection (100+ projects), scroll through grid and measure scroll performance (should be 60 FPS). Images should load progressively as they enter viewport.

### Implementation for User Story 6

- [X] T068 [US6] Implement Intersection Observer for lazy loading in ProjectTile in frontend/src/components/project/ProjectTile.tsx
- [X] T069 [US6] Add image fade-in animation in ProjectTile in frontend/src/components/project/ProjectTile.css (animate-fade-in)
- [X] T070 [US6] Add loading placeholder for images in ProjectTile (gray bg during load)
- [X] T071 [US6] Optimize useMemo for visibleProjects filtering in BrowsePage in frontend/src/pages/BrowsePage.tsx
- [X] T072 [US6] Optimize calculateTileMetadata memoization in ProjectGrid in frontend/src/components/project/ProjectGrid.tsx
- [X] T073 [US6] Add viewport optimization to ProjectGrid (only render visible + 1 row preload)
- [ ] T074 [P] [US6] Performance test with chrome-devtools-mcp Performance panel (measure FPS, LCP, FCP)
- [ ] T075 [US6] Validate lazy loading with chrome-devtools-mcp Network panel (images load on scroll)
- [ ] T076 [US6] Test with 500+ project collection in chrome-devtools-mcp (scroll performance validation)

**Checkpoint**: Performance is optimized for large collections

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Final refinements, documentation, and cross-cutting improvements

- [X] T077 [P] Add animation transitions in frontend/src/index.css (@keyframes for fadeIn, slide effects)
- [X] T078 [P] Optimize Tailwind build in frontend/tailwind.config.js (purge unused CSS)
- [X] T079 [P] Update README.md with screenshots of new UI
- [X] T080 [P] Update CHANGELOG.md with UI modernization entry
- [ ] T081 Add mobile touch gesture support in ProjectTile (touch events for mobile swipe)
- [ ] T082 Add loading states for slow network in ProjectGrid (connection-aware loading)
- [X] T083 [P] Run ESLint on all modified files (npm run lint)
- [X] T084 [P] Run TypeScript compilation check (npm run build)
- [ ] T085 Run full Vitest test suite (npm run test)
- [ ] T086 Validate all user stories with quickstart.md test checklist
- [ ] T087 Take final comparison screenshots with chrome-devtools-mcp (final-home.png, final-browse.png, final-mobile.png)
- [ ] T088 Run accessibility audit with chrome-devtools-mcp (contrast, keyboard nav, screen reader)
- [ ] T089 Performance validation with chrome-devtools-mcp (< 2s initial load, 60 FPS scroll)
- [ ] T090 Cross-browser testing with chrome-devtools-mcp emulation (Chrome, Firefox, Safari, Edge)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-8)**: All depend on Foundational phase completion
  - User stories can proceed in parallel (if staffed)
  - Or sequentially in priority order (US1 → US2 → US3 → US4 → US5 → US6)
- **Polish (Phase 9)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Depends on User Story 1 components (ProjectTile, ProjectGrid) - integrates with US1
- **User Story 3 (P2)**: Depends on NavBar from US1 - integrates ScanButton into existing navigation
- **User Story 4 (P2)**: Depends on US1 and US2 components - enhances existing visual design
- **User Story 5 (P3)**: Depends on US1 and US2 components - adds keyboard navigation to existing UI
- **User Story 6 (P3)**: Depends on US1 components - optimizes existing grid and tiles

### Within Each User Story

- Component creation before usage
- Styles after component structure
- Tests can run in parallel with implementation
- Chrome-devtools validation after implementation
- Core functionality before polish

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Within each user story:
  - Component tests can run in parallel
  - Multiple component files can be created in parallel if no dependencies
- User Stories 1 and 3 can partially overlap (NavBar in US1, ScanButton integration in US3)
- User Stories 4, 5, 6 can start once US1 and US2 foundations are complete

---

## Parallel Example: User Story 1

```bash
# Launch component creation in parallel:
Task: "Create NavBar component in frontend/src/components/common/NavBar.tsx"
Task: "Create ScanButton component in frontend/src/components/scan/ScanButton.tsx"
Task: "Create SkeletonTile component in frontend/src/components/project/SkeletonTile.tsx"

# Launch tests in parallel after components exist:
Task: "Component test for ProjectTile in frontend/src/components/project/ProjectTile.test.tsx"
Task: "Component test for ProjectGrid in frontend/src/components/project/ProjectGrid.test.tsx"

# Launch visual enhancements in parallel:
Task: "Add lazy loading for preview images in ProjectTile"
Task: "Create ProjectTile styles in frontend/src/components/project/ProjectTile.css"
Task: "Create ProjectGrid styles in frontend/src/components/project/ProjectGrid.css"
```

---

## Implementation Strategy

### MVP First (User Stories 1 & 2 Only)

1. Complete Phase 1: Setup (Tailwind installation)
2. Complete Phase 2: Foundational (shared types, helpers, components)
3. Complete Phase 3: User Story 1 (Browse root folders)
4. Complete Phase 4: User Story 2 (Hierarchical navigation)
5. **STOP and VALIDATE**: Test US1 and US2 independently with chrome-devtools-mcp
6. Deploy/demo if ready - core browsing experience is complete

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP - Root browsing!)
3. Add User Story 2 → Test independently → Deploy/Demo (Hierarchical navigation!)
4. Add User Story 3 → Test independently → Deploy/Demo (Library management!)
5. Add User Story 4 → Test independently → Deploy/Demo (Modern design!)
6. Add User Story 5 → Test independently → Deploy/Demo (Keyboard nav!)
7. Add User Story 6 → Test independently → Deploy/Demo (Performance!)
8. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (Browse root) + User Story 3 (Scan button)
   - Developer B: User Story 2 (Navigation) - starts after US1 components ready
   - Developer C: User Story 4 (Visual design) - starts after US1/US2 components ready
3. After core stories (US1-US4) complete:
   - Developer D: User Story 5 (Keyboard nav)
   - Developer E: User Story 6 (Performance)
4. Polish phase - all developers

---

## Task Estimates

**Total Tasks**: 90
- Setup: 6 tasks (~2 hours)
- Foundational: 6 tasks (~3 hours)
- User Story 1: 13 tasks (~8 hours)
- User Story 2: 11 tasks (~6 hours)
- User Story 3: 9 tasks (~4 hours)
- User Story 4: 13 tasks (~6 hours)
- User Story 5: 9 tasks (~4 hours)
- User Story 6: 9 tasks (~5 hours)
- Polish: 14 tasks (~4 hours)

**Estimated Timeline**:
- **MVP (US1+US2)**: 2-3 days (Setup + Foundational + US1 + US2)
- **Full P1+P2 Features (US1-US4)**: 4-5 days
- **All Features (US1-US6 + Polish)**: 6-8 days

---

## Chrome-devtools-mcp Validation Tasks

**Validation Tasks Using chrome-devtools-mcp**:

1. **Baseline Capture** (T006):
   - Take screenshot of current home page
   - Take screenshot of current browse page

2. **User Story 1 Validation** (T024, T025):
   - Take screenshot of new tile grid
   - Take screenshot of hover effects
   - Resize page to 320px, 768px, 1024px, 1920px and validate responsive layout

3. **User Story 2 Validation** (T035, T036):
   - Take screenshot of breadcrumb navigation
   - Take screenshot of folder children view
   - Test navigation flow: root → folder → children → back

4. **User Story 3 Validation** (T044, T045):
   - Take screenshot of navbar with scan button
   - Take screenshot of scan in progress
   - Validate button placement on mobile and desktop

5. **User Story 4 Validation** (T057, T058):
   - Take comparison screenshots (before/after)
   - Validate color contrast (WCAG AA)
   - Validate visual hierarchy

6. **User Story 5 Validation** (T066, T067):
   - Test keyboard navigation flow
   - Verify focus indicators
   - Test screen reader announcements

7. **User Story 6 Validation** (T074, T075, T076):
   - Measure FPS with Performance panel
   - Validate lazy loading with Network panel
   - Test with 500+ project collection

8. **Final Validation** (T087, T088, T089, T090):
   - Take final comparison screenshots
   - Run accessibility audit
   - Validate performance metrics
   - Cross-browser testing with emulation

---

## Notes

- [P] tasks = different files, no dependencies - can run in parallel
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Use chrome-devtools-mcp for all UI validation and screenshot capture
- Component tests use Vitest framework
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- All paths assume frontend/ directory structure (React web app)
- Tailwind CSS is the primary styling framework
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
