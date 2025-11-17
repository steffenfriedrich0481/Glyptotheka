# Implementation Plan: Modern Tile-Based UI

**Branch**: `001-ui-modernization` | **Date**: 2025-11-17 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-ui-modernization/spec.md`

## Summary

Modernize the Glyptotheka 3D Print Library UI to provide a professional, tile-based browsing experience inspired by contemporary 3D model platforms. The implementation focuses on:
- Modern card-based tile layout with responsive grid
- Hierarchical folder navigation with breadcrumbs
- Relocated scan button to top navigation bar
- Professional visual design with proper spacing, shadows, and typography
- Enhanced accessibility and keyboard navigation
- Performance optimizations for large collections

Technical approach uses Tailwind CSS for rapid modern UI development, refactoring existing React components (ProjectTile, ProjectGrid, BrowsePage) while maintaining backward compatibility with existing API contracts.

## Technical Context

**Language/Version**: TypeScript 5.9.3 / JavaScript ES2020  
**Primary Dependencies**: React 18.2, React Router 6.30, Vite 5.4, Tailwind CSS (to be added)  
**Storage**: Backend API (existing) - no frontend storage changes  
**Testing**: Vitest 1.6.1 + chrome-devtools-mcp for UI validation  
**Target Platform**: Modern web browsers (Chrome, Firefox, Safari, Edge) - Desktop primary, Mobile responsive  
**Project Type**: Web application (frontend only - backend unchanged)  
**Performance Goals**: 
  - Initial render < 2 seconds
  - 60 FPS scroll performance with 500+ projects
  - Lazy loading for images as they enter viewport
  - First Contentful Paint < 1.5s
**Constraints**: 
  - Must maintain backward compatibility with existing backend API contracts
  - No breaking changes to existing data models or API responses
  - Preserve all existing functionality (search, tags, file management)
  - Support screen sizes from 320px to 2560px width
**Scale/Scope**: 
  - Refactor 5 key components (NavBar, ProjectTile, ProjectGrid, BrowsePage, HomePage)
  - Support libraries with 10-500 projects typically
  - ~15 component files to modify/create
  - Estimated 3-4 screens affected

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Analysis**: This feature is a frontend-only UI modernization. The Glyptotheka constitution principles (focused on Rust no_std actor runtime development) do not directly apply to this TypeScript/React web application project. Key observations:

1. **no_std Core**: Not applicable - this is a web frontend
2. **Test Integrity**: Applies - must maintain green tests, add UI validation tests
3. **Reference-Consistent Design**: Partially applies - reference modern UI patterns from Tailwind UI, Material Design
4. **Module Structure**: Applies to TypeScript - maintain clean component structure, one component per file
5. **Aggressive Design Evolution**: Applies - pre-release allows breaking UI changes
6. **Inductively Consistency-Driven Design**: Applies - follow existing React/TypeScript patterns in codebase
7. **Lifetime-First Design**: Not applicable - JavaScript/React memory model

**Constitution Compliance**: ✅ PASS
- No violations of core principles
- Testing requirements will be met via Vitest + chrome-devtools validation
- Module structure follows existing frontend conventions (one component per file)
- Design references modern UI best practices (Tailwind UI, responsive grids)
- Maintains backward compatibility with backend API

**Re-evaluation After Phase 1**: To be performed after design artifacts are created

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

### Source Code (repository root)

```text
frontend/
├── src/
│   ├── components/
│   │   ├── common/
│   │   │   ├── Breadcrumb.tsx          # MODIFY - enhance styling
│   │   │   ├── Breadcrumb.css          # MODIFY - update styles
│   │   │   ├── LoadingSpinner.tsx      # MODIFY - modern skeleton
│   │   │   ├── SearchBar.tsx           # MODIFY - visual update
│   │   │   └── NavBar.tsx              # CREATE - new navigation component
│   │   ├── project/
│   │   │   ├── ProjectTile.tsx         # MODIFY - card design
│   │   │   ├── ProjectTile.css         # CREATE - tile styles
│   │   │   ├── ProjectGrid.tsx         # MODIFY - responsive grid
│   │   │   └── ProjectGrid.css         # CREATE - grid styles
│   │   └── scan/
│   │       └── ScanButton.tsx          # CREATE - extracted from HomePage
│   ├── pages/
│   │   ├── HomePage.tsx                # MODIFY - modern layout
│   │   ├── BrowsePage.tsx              # MODIFY - hierarchical nav
│   │   └── ProjectPage.tsx             # MODIFY - visual consistency
│   ├── App.tsx                         # MODIFY - integrate NavBar
│   ├── index.css                       # MODIFY - Tailwind integration
│   └── types/
│       └── project.ts                  # NO CHANGE - types remain same
├── package.json                         # MODIFY - add Tailwind CSS
├── tailwind.config.js                  # CREATE - Tailwind configuration
└── postcss.config.js                   # CREATE - PostCSS for Tailwind

specs/001-ui-modernization/
├── plan.md              # This file
├── research.md          # Phase 0 - design research findings
├── data-model.md        # Phase 1 - component structure
├── quickstart.md        # Phase 1 - developer guide
├── contracts/           # Phase 1 - component interfaces
│   └── component-api.md
└── screenshots/         # UI validation snapshots
    ├── current-home.png
    ├── current-browse.png
    └── [implementation snapshots]
```

**Structure Decision**: Web application structure (Option 2 variant) - frontend-only changes. Backend API remains unchanged, maintaining existing contracts. All modifications are scoped to the `frontend/` directory with new Tailwind CSS integration.

## Complexity Tracking

**No violations requiring justification** - this feature stays within standard web development practices with no architectural complexity concerns.

## Implementation Phases

### Phase 0: Research & Design Analysis ✅ COMPLETE

**Status**: Complete (2025-11-17)

**Deliverables**:
- ✅ `research.md` - Design pattern analysis, Tailwind UI research, component architecture decisions
- ✅ Screenshots captured (current-home.png, current-browse.png)
- ✅ CSS framework decision: Tailwind CSS
- ✅ Navigation architecture defined
- ✅ Performance optimization strategy documented

**Key Findings**:
- Tailwind CSS selected for rapid modern UI development
- Client-side hierarchical navigation (no API changes)
- Native lazy loading with Intersection Observer fallback
- Design tokens extracted from Tailwind UI examples
- Progressive enhancement strategy defined

### Phase 1: Design & Contracts ✅ COMPLETE

**Status**: Complete (2025-11-17)

**Deliverables**:
- ✅ `data-model.md` - Component interfaces, props, and data structures
- ✅ `contracts/component-api.md` - Public API contracts for all components
- ✅ `quickstart.md` - Developer setup and implementation guide
- ✅ Agent context updated (Tailwind CSS, Vitest, chrome-devtools-mcp)

**Artifacts Created**:
- BreadcrumbItem interface (navigation state)
- TileMetadata interface (display data)
- Component contracts for NavBar, ScanButton, ProjectTile, ProjectGrid, Breadcrumb, BrowsePage
- Integration examples and testing requirements
- Setup instructions and code samples

**Re-evaluation of Constitution Check**: ✅ PASS
- All design artifacts respect existing patterns
- No violations introduced
- Testing requirements clearly defined
- Module structure follows frontend conventions

### Phase 2: Planning Complete 🎯 READY FOR IMPLEMENTATION

**Status**: Ready for `/speckit.tasks` command

**Next Command**: 
```bash
/speckit.tasks
```

This will generate `tasks.md` with detailed implementation tasks broken down by component.

**Estimated Implementation Timeline**:
- Week 1: Foundation (Tailwind setup, NavBar, ScanButton) - 2-3 days
- Week 2: Core Components (ProjectTile, ProjectGrid refactor) - 3-4 days
- Week 3: Pages & Navigation (BrowsePage hierarchy, Breadcrumb) - 2-3 days
- Week 4: Polish & Testing (lazy loading, animations, validation) - 2-3 days
- **Total**: 9-13 days for full implementation

**Implementation Order** (recommended):
1. Install Tailwind CSS dependencies
2. Create NavBar component
3. Extract ScanButton component
4. Refactor ProjectTile (card design)
5. Enhance ProjectGrid (responsive + keyboard nav)
6. Update Breadcrumb (styling + truncation)
7. Refactor BrowsePage (hierarchical navigation)
8. Update HomePage & ProjectPage (visual consistency)
9. Add lazy loading for images
10. Add loading skeletons
11. Add animations and polish
12. UI validation with chrome-devtools-mcp
13. Vitest component tests
14. Accessibility audit

## Success Metrics

### Completion Criteria

**Must Have (P1)**:
- [ ] Tailwind CSS installed and configured
- [ ] NavBar component with rescan button in top-right
- [ ] ProjectTile displays as modern card with metadata
- [ ] ProjectGrid responsive (1/2/3/4 columns)
- [ ] Hierarchical navigation (root → folders → children)
- [ ] Breadcrumb trail with click navigation
- [ ] Keyboard navigation maintained
- [ ] All existing tests passing
- [ ] No API contract changes

**Should Have (P2)**:
- [ ] Lazy loading for images
- [ ] Loading skeleton states
- [ ] Hover effects and animations
- [ ] Empty state improvements
- [ ] Visual design polished (shadows, spacing, typography)

**Nice to Have (P3)**:
- [ ] Virtual scrolling for 1000+ projects
- [ ] Image fade-in animations
- [ ] Touch gestures on mobile
- [ ] Dark mode support

### Quality Gates

**Before Merge**:
1. All Vitest tests passing (`npm run test`)
2. ESLint clean (`npm run lint`)
3. TypeScript compilation clean (`npm run build`)
4. UI validation screenshots captured with chrome-devtools-mcp
5. Manual testing checklist complete (see quickstart.md)
6. Accessibility audit passed (keyboard nav, screen reader, contrast)
7. Performance validated (< 2s initial load, 60 FPS scroll)

## Risk Assessment

### Technical Risks

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Tailwind increases bundle size | Medium | Low | Tree-shaking removes unused CSS (~10KB final) |
| Lazy loading breaks on old browsers | Low | Low | Intersection Observer fallback provided |
| Performance degradation with 500+ projects | Medium | Medium | Tested up to 500 tiles, virtual scroll as fallback |
| Breaking existing keyboard navigation | High | Low | Preserve existing ProjectGrid keyboard logic |
| Visual regression on mobile | Medium | Low | Responsive testing at 320px, 768px, 1024px |

### Project Risks

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Timeline overrun | Medium | Medium | Progressive enhancement - deliver in phases |
| Scope creep (dark mode, etc.) | Medium | Medium | Strict adherence to P1/P2/P3 prioritization |
| Backend API changes needed | High | Very Low | Design ensures zero API changes |
| Incompatibility with existing code | Medium | Low | Incremental refactoring, preserve interfaces |

## Rollback Plan

**If issues are discovered post-deployment**:

1. **Immediate Rollback** (< 5 minutes):
   ```bash
   git revert <merge-commit>
   npm run build
   # Deploy previous build
   ```

2. **Partial Rollback** (if only specific component broken):
   - Revert individual component file
   - Keep Tailwind CSS setup
   - Disable broken feature with feature flag

3. **Data Safety**:
   - No database changes = no data migration needed
   - API contracts unchanged = backend unaffected
   - Frontend-only = easy rollback

## Documentation Updates

**Files to Update**:
- [ ] `README.md` - Update screenshots if UI shown
- [ ] `CHANGELOG.md` - Add entry for UI modernization
- [ ] `docs/user-guide.md` - Update navigation instructions (if exists)
- [ ] `.github/agents/copilot-instructions.md` - ✅ Already updated

**New Documentation**:
- ✅ `specs/001-ui-modernization/plan.md` (this file)
- ✅ `specs/001-ui-modernization/research.md`
- ✅ `specs/001-ui-modernization/data-model.md`
- ✅ `specs/001-ui-modernization/quickstart.md`
- ✅ `specs/001-ui-modernization/contracts/component-api.md`
- [ ] `specs/001-ui-modernization/tasks.md` (generated by `/speckit.tasks`)

## Approval & Sign-off

**Plan Created**: 2025-11-17  
**Phase 0 Complete**: 2025-11-17  
**Phase 1 Complete**: 2025-11-17  
**Status**: Ready for implementation (`/speckit.tasks`)

**Artifacts Summary**:
- Research document: 18KB (design analysis, decisions, best practices)
- Data model: 19KB (component architecture, interfaces)
- Component contracts: 17KB (API specifications, testing requirements)
- Quickstart guide: 17KB (setup, code examples, troubleshooting)
- Total documentation: ~71KB

**Branch**: `001-ui-modernization`  
**Implementation Plan Path**: `/home/stefffri/Workspace/Glyptotheka/specs/001-ui-modernization/plan.md`

---

**Next Step**: Run `/speckit.tasks` to generate detailed implementation tasks
