# Implementation Plan: Browse View Refactoring - File Explorer Style with Image Inheritance

**Branch**: `001-browse-view-refactor` | **Date**: 2025-11-28 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-browse-view-refactor/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Refactor the browse view to provide folder-by-folder navigation like a file explorer, implementing image inheritance from parent folders to all descendant projects, and enhancing STL categorization through substring keyword matching. The system will allow users to navigate one level at a time through the folder hierarchy, seeing project previews with inherited images at each level. Folders matching IGNORED_KEYWORDS substrings will be properly categorized as STL containers rather than separate projects.

## Technical Context

**Language/Version**: 
- Backend: Rust 1.75+ with Axum 0.7, tokio 1.35, rusqlite 0.31
- Frontend: TypeScript 5.0+, React 18, Vite 4

**Primary Dependencies**: 
- Backend: axum, tokio, rusqlite, serde, serde_json
- Frontend: React, React Router, Axios, TailwindCSS

**Storage**: SQLite (rusqlite with bundled feature)

**Testing**: 
- Backend: cargo test
- Frontend: Vitest, React Testing Library
- E2E: Chrome DevTools MCP for manual validation

**Target Platform**: Linux server (Docker + native systemd), web browsers (Chrome, Firefox, Safari)

**Project Type**: Web application (Rust backend + React frontend)

**Performance Goals**: 
- Folder navigation response time < 500ms
- Project preview loading < 1 second
- Image inheritance calculation optimized to avoid N+1 queries
- Support up to 1000 projects in a single folder level without pagination

**Constraints**: 
- Must maintain backward compatibility with existing database schema
- Image inheritance must be performant (avoid loading entire tree)
- Frontend must remain responsive during navigation
- Must not break existing search and tag functionality

**Scale/Scope**: 
- Backend: ~10-15 files affected (scanner, models, API routes)
- Frontend: ~8-10 components affected (browse pages, navigation, project tiles)
- Database: Schema modifications for image inheritance tracking
- Expected change: Medium complexity refactor

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Project Type Assessment**: Glyptotheka is a web application with Rust backend and React frontend. Standard web development principles apply.

**Applicable Development Standards**:
- ✅ Write tests for new navigation logic and image inheritance
- ✅ Maintain existing test suite - no deletions or ignoring tests
- ✅ Keep CI green - all tests passing before and after changes
- ✅ Document breaking changes (if any) and migration steps
- ✅ Update related documentation immediately after code changes
- ✅ Investigate existing code patterns and maintain consistency
- ✅ Minimize breaking changes - additive/enhancement only

**Project-Specific Quality Gates**:
1. **No Data Loss**: Existing projects and images must remain accessible
2. **Test Coverage**: Add tests for image inheritance logic and keyword substring matching
3. **Performance**: Folder navigation must remain under 500ms response time
4. **Documentation**: Update README with new navigation model explanation
5. **UI Responsiveness**: Frontend must handle deep hierarchies gracefully
6. **Backward Compatibility**: Existing URLs and bookmarks should continue working

**Status**: ✅ PASS - This is an enhancement that improves UX without breaking existing functionality. Changes are primarily additive.

## Project Structure

### Documentation (this feature)

```text
specs/001-browse-view-refactor/
├── spec.md              # Feature specification (provided)
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output - navigation patterns research
├── data-model.md        # Phase 1 output - schema changes for image inheritance
├── quickstart.md        # Phase 1 output - updated user guide
└── contracts/           # Phase 1 output - API contracts for new navigation endpoints
```

### Source Code (repository root)

```text
backend/
├── src/
│   ├── api/
│   │   └── browse_routes.rs          # [MAJOR REFACTOR] Add folder navigation endpoints
│   ├── config.rs                      # [MODIFY] Add IGNORED_KEYWORDS configuration
│   ├── db/
│   │   ├── migrations.rs              # [ADD] Add image_inheritance tracking table
│   │   └── queries.rs                 # [MODIFY] Add image inheritance queries
│   ├── models/
│   │   ├── project.rs                 # [MODIFY] Add inherited_images field
│   │   └── folder.rs                  # [ADD] New folder navigation model
│   ├── services/
│   │   ├── scanner.rs                 # [MAJOR REFACTOR] Implement substring keyword matching
│   │   ├── image_service.rs           # [ADD] Image inheritance calculation logic
│   │   └── project_service.rs         # [MODIFY] Update to support folder-level queries
│   └── utils/                         # [NO CHANGE]
├── Cargo.toml                         # [NO CHANGE]
└── tests/
    └── integration/                   # [ADD] Navigation and inheritance tests

frontend/
├── src/
│   ├── pages/
│   │   ├── BrowsePage.tsx             # [MAJOR REFACTOR] Folder-by-folder navigation
│   │   └── FolderView.tsx             # [ADD] New component for folder level view
│   ├── components/
│   │   ├── ProjectPreview.tsx         # [MODIFY] Display inherited images
│   │   ├── Breadcrumb.tsx             # [ADD] Navigation breadcrumb component
│   │   └── FolderTile.tsx             # [ADD] Folder display component
│   ├── api/
│   │   └── client.ts                  # [MODIFY] Add folder navigation API calls
│   └── hooks/
│       └── useImageInheritance.ts     # [ADD] Hook for inherited image handling
└── tests/                             # [ADD] Component tests for new navigation

docker-compose.yml                     # [MODIFY] Add IGNORED_KEYWORDS environment variable
.env.example                           # [MODIFY] Add IGNORED_KEYWORDS configuration
README.md                              # [MODIFY] Update usage section with new navigation model
```

**Structure Decision**: This is a web application following backend/frontend split (Option 2). The changes span both layers:
- **Backend**: Scanner service for keyword matching, new API endpoints for folder navigation, database schema for image inheritance
- **Frontend**: Complete refactor of browse view to support folder-by-folder navigation with breadcrumbs
- **Database**: New tables/columns to track image inheritance efficiently

## Complexity Tracking

> **This section is not needed** - no constitution violations present. This is a feature enhancement that adds new functionality while maintaining backward compatibility.

---

## Phase 0: Research & Investigation

**Goal**: Understand current navigation implementation, database schema, and identify the best approach for image inheritance and keyword substring matching.

**Timeline**: 0.5-1 day

**Blocking Status**: Cannot proceed to Phase 1 until research complete ✋

### Tasks

- [ ] **0.1** Analyze current BrowsePage.tsx implementation
  - Understand current project loading and display logic
  - Identify state management patterns
  - Document current navigation approach
  
- [ ] **0.2** Review database schema for projects and images
  - Examine existing relationships between projects, folders, and images
  - Identify where image paths are stored
  - Assess need for new tables vs. modifying existing schema
  
- [ ] **0.3** Investigate scanner service logic
  - Review current keyword matching implementation (exact match vs. substring)
  - Understand how projects vs. STL containers are currently identified
  - Document folder traversal algorithm
  
- [ ] **0.4** Research optimal image inheritance strategy
  - Evaluate options: DB-level inheritance tracking vs. runtime calculation
  - Consider performance implications of deep hierarchies
  - Determine deduplication strategy for inherited images
  
- [ ] **0.5** Examine example folder structure
  - Manually walk through `example/Miniaturen/The Printing Goes Ever On/Welcome Trove`
  - Verify image placement and expected inheritance behavior
  - Test substring keyword matching with "inch", "mm" examples

**Outputs**:
- `research.md` documenting findings
- Decision on image inheritance implementation approach
- List of required schema changes
- Identified API endpoint modifications

**Success Criteria**: Clear understanding of current architecture, documented inheritance strategy, and identified technical approach for both backend and frontend changes.

---

## Phase 1: Design & Documentation

**Goal**: Create detailed technical design for folder navigation, image inheritance, and keyword substring matching. Define API contracts and database schema changes.

**Timeline**: 1-1.5 days

**Blocking Status**: Cannot proceed to Phase 2 until design approved ✋

### Tasks

- [ ] **1.1** Design database schema for image inheritance
  - Add `inherited_images` JSON column to projects table OR
  - Create separate `image_inheritance` junction table
  - Add `folder_level` tracking to optimize queries
  - Write migration script
  
- [ ] **1.2** Design folder navigation API endpoints
  - `GET /api/browse/:path` - Get folders and projects at current level
  - `GET /api/browse/:path/breadcrumb` - Get navigation breadcrumb trail
  - `GET /api/projects/:id/inherited-images` - Get inherited images for project
  - Document request/response schemas in `contracts/api-contracts.md`
  
- [ ] **1.3** Design image inheritance algorithm
  - Define inheritance chain calculation logic
  - Document deduplication strategy (by filename)
  - Plan caching strategy for frequently accessed inheritance chains
  - Specify when inheritance is calculated (scan-time vs. query-time)
  
- [ ] **1.4** Design substring keyword matching algorithm
  - Case-insensitive substring search implementation
  - String trimming and normalization strategy
  - Performance considerations for keyword list
  - Document edge cases (e.g., keyword "in" matching "Miniaturen")
  
- [ ] **1.5** Design frontend component hierarchy
  - `FolderView` component structure and props
  - `ProjectPreview` modifications for inherited images
  - `Breadcrumb` component design
  - State management for current navigation path
  
- [ ] **1.6** Create data model documentation
  - Document `Folder` entity
  - Document updated `Project` entity with inherited images
  - Document `ImageInheritanceChain` concept
  - Write `data-model.md`
  
- [ ] **1.7** Document configuration changes
  - Update `.env.example` with `IGNORED_KEYWORDS`
  - Document keyword format and examples
  - Explain substring matching behavior

**Outputs**:
- `data-model.md` with complete entity definitions
- `contracts/api-contracts.md` with all endpoint specifications
- `quickstart.md` updated with new navigation model
- Database migration script drafted

**Success Criteria**: Complete technical design that can be implemented without further design decisions. API contracts are clear and frontend/backend teams could work in parallel.

---

## Phase 2: Backend - Database & Scanner Service

**Goal**: Implement database schema changes, update scanner to use substring keyword matching, and build image inheritance calculation logic.

**Timeline**: 2-3 days

**Dependencies**: Phase 1 complete

### Tasks

- [ ] **2.1** Implement database migration
  - Add image inheritance tracking (table or column)
  - Add folder-level indexing for efficient queries
  - Test migration on example database
  - Verify rollback works correctly
  
- [ ] **2.2** Update configuration service
  - Add `IGNORED_KEYWORDS` to `Config` struct
  - Parse comma-separated keyword list from environment
  - Add validation for keyword format
  - Write unit tests for configuration loading
  
- [ ] **2.3** Refactor scanner keyword matching
  - Replace exact match with case-insensitive substring matching
  - Implement string trimming and normalization
  - Update project vs. STL container detection logic
  - Write comprehensive unit tests covering edge cases
  
- [ ] **2.4** Implement image inheritance service
  - Create `ImageService::calculate_inheritance(project_path)` function
  - Walk up folder hierarchy collecting images
  - Implement deduplication by filename
  - Cache inheritance chains for performance
  - Write unit tests for inheritance calculation
  
- [ ] **2.5** Update project model
  - Add `inherited_images: Vec<ImagePath>` field
  - Implement serialization for API responses
  - Update project queries to include inherited images
  - Write tests for model changes
  
- [ ] **2.6** Update scanner to populate inheritance data
  - Calculate and store image inheritance during scan
  - Update existing projects with inheritance information
  - Add progress logging for inheritance calculation
  - Test with example folder structure

**Outputs**:
- Database migration applied
- Scanner service updated with substring matching
- Image inheritance calculation implemented
- Unit tests passing for all new logic

**Success Criteria**: Running a scan on the example folder correctly identifies STL containers via substring matching ("1 inch", "2 inch") and calculates image inheritance for all projects.

---

## Phase 3: Backend - Folder Navigation API

**Goal**: Implement new API endpoints for folder-by-folder navigation and breadcrumb trails.

**Timeline**: 1.5-2 days

**Dependencies**: Phase 2 complete

### Tasks

- [ ] **3.1** Create `FolderService` for navigation logic
  - Implement `get_folder_contents(path)` returning folders + projects
  - Implement `get_breadcrumb_trail(path)` returning parent folders
  - Add pagination support for large folders
  - Write unit tests for service methods
  
- [ ] **3.2** Implement `GET /api/browse/:path` endpoint
  - Parse and validate folder path parameter
  - Call `FolderService::get_folder_contents`
  - Return JSON with folders and projects at current level
  - Handle path traversal security (prevent "../.." attacks)
  - Write integration tests
  
- [ ] **3.3** Implement `GET /api/browse/:path/breadcrumb` endpoint
  - Parse folder path
  - Return breadcrumb trail from root to current folder
  - Include project counts at each level
  - Write integration tests
  
- [ ] **3.4** Update `GET /api/projects/:id` endpoint
  - Include inherited images in response
  - Add `inherited_from_paths` field showing source folders
  - Maintain backward compatibility with existing clients
  - Write tests verifying inherited images included
  
- [ ] **3.5** Add error handling for navigation
  - Handle non-existent folder paths (404)
  - Handle permission errors (403)
  - Handle invalid path characters (400)
  - Return meaningful error messages

**Outputs**:
- Folder navigation API endpoints implemented
- Integration tests passing
- API documentation updated in `contracts/api-contracts.md`

**Success Criteria**: Can navigate through example folder structure via API calls, retrieving folders and projects at each level. Breadcrumb trail is correct at every level.

---

## Phase 4: Frontend - Folder Navigation Components

**Goal**: Build new React components for folder-by-folder navigation and breadcrumb UI.

**Timeline**: 2-3 days

**Dependencies**: Phase 3 complete

### Tasks

- [ ] **4.1** Create `Breadcrumb` component
  - Display clickable folder path (Home > Miniaturen > The Printing Goes Ever On)
  - Handle click navigation to parent folders
  - Style with TailwindCSS to match existing UI
  - Add loading state for navigation
  - Write component tests
  
- [ ] **4.2** Create `FolderTile` component
  - Display folder icon and name
  - Show count of contained projects
  - Handle click to navigate into folder
  - Add hover effects
  - Write component tests
  
- [ ] **4.3** Create `FolderView` component
  - Display folders and projects at current level
  - Arrange in responsive grid layout (matching existing project tiles)
  - Handle loading and error states
  - Implement empty state ("No projects in this folder")
  - Write component tests
  
- [ ] **4.4** Update `ProjectPreview` component
  - Display inherited images in carousel
  - Add badge/indicator showing inherited vs. own images
  - Handle case where all images are inherited
  - Maintain existing image carousel functionality
  - Write component tests
  
- [ ] **4.5** Create `useImageInheritance` hook
  - Fetch inherited images for a project
  - Handle caching to avoid redundant API calls
  - Provide loading and error states
  - Write hook tests

**Outputs**:
- New navigation components implemented
- Component tests passing
- Components integrated into storybook (if applicable)

**Success Criteria**: Components render correctly in isolation. Breadcrumb navigation works. Folder tiles display properly.

---

## Phase 5: Frontend - Browse Page Refactor

**Goal**: Integrate new navigation components into BrowsePage and implement folder-by-folder navigation flow.

**Timeline**: 2-2.5 days

**Dependencies**: Phase 4 complete

### Tasks

- [ ] **5.1** Refactor `BrowsePage.tsx` for folder navigation
  - Replace flat project list with folder-level view
  - Add URL routing for folder paths (`/browse/:folderPath`)
  - Implement navigation state management (current path, history)
  - Handle browser back/forward buttons
  - Preserve scroll position when navigating back
  
- [ ] **5.2** Integrate `Breadcrumb` component
  - Place breadcrumb at top of browse view
  - Connect breadcrumb clicks to navigation state
  - Update breadcrumb on folder navigation
  - Style consistently with page header
  
- [ ] **5.3** Integrate `FolderView` component
  - Render folders and projects at current level
  - Connect folder tile clicks to navigation
  - Connect project tile clicks to project detail page
  - Implement loading states during navigation
  
- [ ] **5.4** Update API client for new endpoints
  - Add `fetchFolderContents(path)` function
  - Add `fetchBreadcrumb(path)` function
  - Add error handling for network failures
  - Implement request cancellation for rapid navigation
  
- [ ] **5.5** Update routing configuration
  - Add `/browse/:folderPath*` route supporting nested paths
  - Handle root browse view (`/browse`) as special case
  - Redirect old `/browse` URLs if needed
  - Update navigation menu links
  
- [ ] **5.6** Add keyboard navigation support
  - Implement arrow key navigation for folder tiles
  - Add keyboard shortcut for breadcrumb navigation (e.g., Alt+Up)
  - Ensure focus management for accessibility
  - Write tests for keyboard interactions

**Outputs**:
- BrowsePage fully refactored with folder navigation
- URL routing working for nested folder paths
- All navigation flows functional
- Component integration tests passing

**Success Criteria**: Can navigate through entire example folder structure in browser. Breadcrumb updates correctly. URL reflects current folder path. Browser back/forward work correctly.

---

## Phase 6: Testing & Validation

**Goal**: Comprehensive testing of navigation, image inheritance, and keyword matching using real data and Chrome DevTools.

**Timeline**: 1.5-2 days

**Dependencies**: Phase 5 complete

### Tasks

- [ ] **6.1** End-to-end testing with example folder
  - Navigate through `example/Miniaturen/The Printing Goes Ever On/Welcome Trove`
  - Verify "heroes fighting.jpg" appears in descendant projects
  - Verify "1 inch" and "2 inch" folders are STL categories, not projects
  - Test deduplication of inherited images
  - Use Chrome DevTools MCP to validate UI behavior
  
- [ ] **6.2** Test substring keyword matching
  - Add test keywords to `IGNORED_KEYWORDS`: "inch", "mm", "STL"
  - Verify "1 inch", "2 inch", "40 mm" folders treated as STL containers
  - Verify "PRESUPPORTED_STL" matches "STL" substring
  - Test case-insensitive matching
  - Test trimming of folder names
  
- [ ] **6.3** Performance testing
  - Test navigation in folder with 100+ projects
  - Measure response time for folder navigation (target: <500ms)
  - Test image inheritance calculation performance
  - Verify no N+1 query issues
  - Profile frontend rendering performance
  
- [ ] **6.4** Edge case testing
  - Test folder with no images (verify placeholder shown)
  - Test very deep hierarchy (10+ levels)
  - Test folder names with special characters
  - Test corrupt or missing image files
  - Test circular symlinks (if supported by OS)
  
- [ ] **6.5** Accessibility testing
  - Test keyboard navigation
  - Verify screen reader compatibility
  - Check color contrast ratios
  - Test with browser zoom (200%)
  - Validate ARIA labels and roles
  
- [ ] **6.6** Cross-browser testing
  - Test in Chrome, Firefox, Safari
  - Verify image carousel works in all browsers
  - Test responsive layout on mobile devices
  - Check touch navigation on tablets

**Outputs**:
- All acceptance scenarios from spec.md verified
- Performance benchmarks documented
- Accessibility report
- Bug fixes for issues found during testing

**Success Criteria**: All user stories pass acceptance scenarios. Performance targets met. No accessibility violations. Works across supported browsers.

---

## Phase 7: Documentation & Deployment

**Goal**: Update all documentation, create user guide, and prepare for production deployment.

**Timeline**: 1 day

**Dependencies**: Phase 6 complete

### Tasks

- [ ] **7.1** Update README.md
  - Document new folder navigation model
  - Add screenshots of new browse view
  - Update configuration section with `IGNORED_KEYWORDS`
  - Add troubleshooting section for navigation issues
  
- [ ] **7.2** Update deployment documentation
  - Add `IGNORED_KEYWORDS` to environment variable examples
  - Document database migration steps
  - Update Docker Compose configuration
  - Add rollback procedure if needed
  
- [ ] **7.3** Create user guide
  - Write `quickstart.md` for new navigation
  - Add examples of folder-by-folder navigation
  - Explain image inheritance behavior
  - Document breadcrumb navigation
  
- [ ] **7.4** Update API documentation
  - Document new folder navigation endpoints
  - Add example requests and responses
  - Update OpenAPI/Swagger spec (if applicable)
  - Document breaking changes (if any)
  
- [ ] **7.5** Create release notes
  - Summarize new navigation features
  - List breaking changes (if any)
  - Provide upgrade instructions
  - Include screenshots/GIFs of new UI
  
- [ ] **7.6** Prepare deployment
  - Create database backup
  - Test migration on staging environment
  - Verify Docker build succeeds
  - Create deployment checklist
  - Plan rollback strategy if issues arise

**Outputs**:
- All documentation updated and committed
- User guide published
- API documentation current
- Release notes drafted
- Deployment plan ready

**Success Criteria**: Documentation is complete and accurate. Team members can understand new navigation model from docs alone. Deployment plan is clear and tested on staging.

---

## Phase 8: Production Deployment & Monitoring

**Goal**: Deploy to production, monitor for issues, and gather user feedback.

**Timeline**: 0.5-1 day + ongoing monitoring

**Dependencies**: Phase 7 complete

### Tasks

- [ ] **8.1** Deploy to production
  - Execute database migration
  - Deploy backend with new navigation API
  - Deploy frontend with new browse view
  - Verify environment variables configured correctly
  - Monitor logs for errors during startup
  
- [ ] **8.2** Smoke testing in production
  - Test folder navigation on production data
  - Verify image inheritance working correctly
  - Check performance metrics (response times)
  - Test breadcrumb navigation
  - Verify STL categorization with real data
  
- [ ] **8.3** Monitor application metrics
  - Track folder navigation API response times
  - Monitor error rates for new endpoints
  - Check database query performance
  - Watch for memory leaks or resource issues
  - Set up alerts for anomalies
  
- [ ] **8.4** Gather user feedback
  - Collect feedback on new navigation model
  - Monitor support channels for issues
  - Track user engagement metrics (navigation patterns)
  - Identify pain points or confusion
  
- [ ] **8.5** Address production issues
  - Fix any critical bugs discovered
  - Optimize slow queries if found
  - Adjust UI based on feedback
  - Document known issues and workarounds
  
- [ ] **8.6** Post-deployment review
  - Measure success against success criteria from spec.md
  - Document lessons learned
  - Identify future improvements
  - Update backlog with enhancement ideas

**Outputs**:
- Application deployed to production
- Monitoring dashboards configured
- User feedback collected
- Post-deployment report written

**Success Criteria**: Application running smoothly in production. No critical bugs. Performance targets met. Users successfully navigating folder structure.

---

## Timeline Summary

| Phase | Duration | Dependencies |
|-------|----------|--------------|
| Phase 0: Research | 0.5-1 day | None |
| Phase 1: Design | 1-1.5 days | Phase 0 |
| Phase 2: Backend - DB & Scanner | 2-3 days | Phase 1 |
| Phase 3: Backend - API | 1.5-2 days | Phase 2 |
| Phase 4: Frontend - Components | 2-3 days | Phase 3 |
| Phase 5: Frontend - Integration | 2-2.5 days | Phase 4 |
| Phase 6: Testing | 1.5-2 days | Phase 5 |
| Phase 7: Documentation | 1 day | Phase 6 |
| Phase 8: Deployment | 0.5-1 day | Phase 7 |
| **Total** | **12.5-18.5 days** | Sequential |

**Assumptions**:
- Single developer working full-time
- No major architectural surprises during research
- Example folder structure representative of production data
- CI/CD pipeline handles automated testing and deployment

**Risks**:
- Performance issues with deep folder hierarchies (mitigate: implement pagination, caching)
- Substring keyword matching too aggressive (mitigate: require minimum keyword length)
- Image inheritance too slow (mitigate: cache inheritance chains, calculate at scan-time)
- UI complexity overwhelming users (mitigate: progressive disclosure, onboarding tooltips)

---

## Key Decisions & Tradeoffs

### Decision 1: Image Inheritance Calculation Timing
**Options**:
A. Calculate at scan-time and store in database
B. Calculate at query-time on demand
C. Hybrid: pre-calculate common cases, compute on-demand for rare cases

**Decision**: Option A (scan-time calculation)

**Rationale**: 
- Folder structures change infrequently
- Query-time performance is critical for UX
- Slightly longer scan time acceptable trade-off
- Simplifies API layer (no complex inheritance logic)

**Trade-offs**: 
- (+) Fast query performance
- (+) Simpler API implementation
- (-) Longer scan times
- (-) Requires migration to recalculate existing data

### Decision 2: Substring Keyword Matching Strategy
**Options**:
A. Exact match only (current behavior)
B. Case-insensitive substring match
C. Regex pattern matching
D. Fuzzy matching (Levenshtein distance)

**Decision**: Option B (case-insensitive substring match)

**Rationale**: 
- Spec explicitly requests substring matching
- Balances flexibility with performance
- Regex adds unnecessary complexity
- Fuzzy matching could cause false positives

**Trade-offs**: 
- (+) Flexible keyword configuration
- (+) Handles variations like "1 inch", "2 inch"
- (-) Risk of overly broad matches (e.g., "in" matching "Miniaturen")
- (mitigate) Document recommended keyword length (3+ characters)

### Decision 3: Folder Navigation State Management
**Options**:
A. URL-based (folder path in route)
B. React state only (no URL changes)
C. URL + browser history API for back/forward

**Decision**: Option C (URL + history API)

**Rationale**: 
- Enables bookmarking specific folder views
- Browser back/forward work naturally
- Shareable links for specific folders
- Consistent with web navigation patterns

**Trade-offs**: 
- (+) Bookmarkable folder views
- (+) Browser back/forward work correctly
- (+) Shareable URLs
- (-) More complex routing logic
- (-) Must handle URL encoding for special characters

### Decision 4: Folder vs. Project Display
**Options**:
A. Mixed view (folders and projects side-by-side)
B. Folders first, then projects
C. Separate tabs for folders vs. projects

**Decision**: Option A (mixed view)

**Rationale**: 
- Consistent with file explorer UX
- Users can see all contents at once
- Simpler navigation (no tab switching)
- Grid layout accommodates both entity types

**Trade-offs**: 
- (+) Familiar file explorer pattern
- (+) No cognitive overhead of tabs
- (-) Potentially cluttered for folders with many items
- (mitigate) Implement sorting/filtering options

---

## Success Metrics

### Technical Metrics
- Folder navigation API response time: < 500ms (p95)
- Image inheritance calculation: < 100ms per project (during scan)
- Frontend render time: < 200ms for folder view with 50 projects
- Zero N+1 query issues in navigation API
- Test coverage: > 80% for new navigation logic

### User Experience Metrics
- Time to find specific project: < 30 seconds (down from current baseline)
- User satisfaction score: > 4/5 for new navigation
- Error rate: < 1% for folder navigation requests
- Breadcrumb usage: > 50% of users navigate via breadcrumb at least once
- Mobile usability: Navigation works smoothly on tablets

### Feature Adoption Metrics
- % of users using folder navigation: > 90% within first week
- Average depth of navigation: 2-4 levels
- Folder view bounces: < 10% (users leaving immediately)
- Image inheritance satisfaction: Verify images appear where expected (qualitative feedback)

---

## Rollback Plan

**If critical issues arise in production**:

1. **Immediate Rollback** (< 15 minutes):
   - Revert Docker containers to previous version
   - Verify old version operational
   - Communicate rollback to users

2. **Database Rollback** (if needed):
   - If migration causes data issues, restore from backup
   - Re-run rollback migration script
   - Verify data integrity

3. **Partial Rollback** (if only frontend issue):
   - Rollback frontend only
   - Keep backend changes (API remains backward compatible)
   - Fix frontend issues in development

4. **Post-Rollback**:
   - Document root cause of failure
   - Create hotfix branch
   - Test hotfix thoroughly on staging
   - Redeploy when fixed and verified

**Rollback Triggers**:
- Error rate > 5% for navigation requests
- Response time > 2 seconds (p95)
- Data corruption detected
- Critical bug blocking navigation
- User feedback indicates unusable UI

---

## Notes

- Coordinate with frontend and backend developers if working in parallel after Phase 1
- Consider creating a feature flag for gradual rollout if user base is large
- Image inheritance could be extended in future to support other file types (PDFs, etc.)
- Substring keyword matching may need refinement based on production data patterns
- Consider adding undo/redo navigation in future enhancement
- Monitor database growth due to image inheritance tracking

