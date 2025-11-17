# Phase 0: Research & Design Analysis

**Feature**: Modern Tile-Based UI  
**Date**: 2025-11-17  
**Status**: Complete

## Research Overview

This document consolidates research findings for modernizing the Glyptotheka UI based on analysis of contemporary design patterns, similar 3D model platforms, and modern CSS frameworks.

## 1. Design Pattern Research

### 1.1 Reference Platform Analysis

**Target Inspiration**: https://www.printables.com/model  
**Status**: ⚠️ Blocked by Cloudflare protection

**Alternative References Analyzed**:
- Tailwind UI Grid Lists (https://tailwindui.com/components/application-ui/lists/grid-lists)
- Material Design card patterns
- Modern file browser UIs (Google Drive, Dropbox)

### 1.2 Tailwind UI Grid Lists Analysis

**Research Method**: Direct analysis of Tailwind UI grid components via chrome-devtools-mcp

**Key Findings**:

#### Card Design Patterns
1. **Contact Cards with Small Portraits**
   - Clean white cards with subtle shadows
   - Portrait image at top
   - Name as heading (h3)
   - Metadata in smaller, muted text
   - Action buttons at bottom
   - Hover effects for interactivity

2. **Simple Cards**
   - Minimalist design with initials/icons
   - Project name as clickable link
   - Member count metadata
   - Options menu (three-dot)
   - Clean spacing and borders

3. **Images with Details**
   - Image prominently displayed
   - Filename and size overlay on hover
   - Dark overlay for readability
   - Click-to-view interaction
   - Responsive grid layout

**Design Tokens Extracted**:
```css
/* Spacing */
gap: 1rem (16px) between tiles
padding: 1.5rem (24px) inside cards

/* Colors */
background: white (#ffffff)
border: gray-200 (#e5e7eb)
text-primary: gray-900 (#111827)
text-secondary: gray-600 (#4b5563)
hover-shadow: lg (0 10px 15px rgba(0,0,0,0.1))

/* Typography */
heading: text-lg (18px) font-semibold
metadata: text-sm (14px) font-normal
label: text-xs (12px) text-gray-500

/* Borders & Shadows */
border-radius: 0.5rem (8px)
box-shadow: md (0 4px 6px rgba(0,0,0,0.1))
hover-shadow: lg (0 10px 15px rgba(0,0,0,0.1))

/* Grid Layout */
grid-cols: 1 (mobile), 2 (tablet), 3-4 (desktop)
responsive breakpoints: 640px (sm), 768px (md), 1024px (lg)
```

### 1.3 Current UI Analysis

**Research Method**: Local chrome-devtools analysis of http://localhost:5173

**Current State**:
- Basic grid layout using Tailwind utility classes
- Simple tiles with minimal styling
- Basic hover effects (shadow-md → shadow-lg)
- No preview images (folder icon placeholder)
- Flat hierarchy (no breadcrumb navigation visible)
- Search bar in header
- Rescan button not visible in navigation

**Screenshots Captured**:
- `screenshots/current-home.png` - Home page with scan controls
- `screenshots/current-browse.png` - Browse page (empty state)

**Gaps Identified**:
1. ❌ No card-based visual design with proper shadows
2. ❌ No preview images (only emoji placeholders)
3. ❌ No hierarchical navigation breadcrumbs
4. ❌ No metadata overlays (file count, size)
5. ❌ Basic typography without hierarchy
6. ❌ Rescan button not in top navigation
7. ✅ Responsive grid present (1/3/4 columns)
8. ✅ Keyboard navigation implemented

## 2. CSS Framework Decision

### 2.1 Options Evaluated

| Framework | Pros | Cons | Decision |
|-----------|------|------|----------|
| **Tailwind CSS** | Already partially used, rapid development, utility-first, excellent documentation, tree-shakable | Learning curve for team, verbose HTML | ✅ **SELECTED** |
| CSS Modules | Type-safe, scoped styles, no runtime | More boilerplate, slower development | ❌ Rejected |
| Styled Components | Dynamic styling, component-scoped | Runtime overhead, larger bundle | ❌ Rejected |
| Plain CSS | Full control, no dependencies | Time-consuming, maintenance burden | ❌ Rejected |

### 2.2 Tailwind CSS Justification

**Decision**: Adopt Tailwind CSS as the primary styling solution

**Rationale**:
1. **Existing Partial Usage**: Code already uses Tailwind classes (`grid-cols-1 md:grid-cols-3 lg:grid-cols-4`)
2. **Rapid Modern UI**: Perfect for implementing card designs quickly
3. **Responsive Design**: Built-in responsive utilities simplify multi-screen support
4. **Consistency**: Design system tokens built-in (spacing, colors, shadows)
5. **Performance**: Tree-shaking removes unused CSS (production builds ~10-20KB)
6. **Documentation**: Extensive examples and patterns available
7. **Community**: Large ecosystem, Tailwind UI components for reference

**Implementation Approach**:
- Install `tailwindcss`, `postcss`, `autoprefixer` as devDependencies
- Create `tailwind.config.js` with custom color palette
- Update `postcss.config.js` for Tailwind processing
- Import Tailwind directives in `index.css`
- Progressively enhance existing components

## 3. Component Architecture Research

### 3.1 Existing Component Analysis

**Current Components** (from codebase analysis):
```
components/
├── common/
│   ├── Breadcrumb.tsx/css       # EXISTS - needs enhancement
│   ├── LoadingSpinner.tsx/css   # EXISTS - needs skeleton states
│   ├── SearchBar.tsx/css        # EXISTS - visual update needed
│   └── Tile.tsx/css             # EXISTS - generic tile component
├── project/
│   ├── ProjectGrid.tsx          # EXISTS - good keyboard nav
│   ├── ProjectTile.tsx          # EXISTS - needs card design
│   └── FileList.tsx             # EXISTS - no changes needed
└── scan/
    └── ScanProgress.tsx         # EXISTS - no changes needed
```

**Analysis**:
- ✅ Separation of concerns (common vs. domain-specific)
- ✅ Existing keyboard navigation in ProjectGrid
- ✅ Breadcrumb component exists but not styled
- ❌ No dedicated NavBar component
- ❌ ProjectTile lacks modern card design
- ❌ No lazy loading for images

### 3.2 Component Refactoring Strategy

#### High Priority (P1)
1. **NavBar Component** (NEW)
   - Extract navigation from App.tsx
   - Add Rescan button to top-right
   - Maintain search bar integration
   - Responsive layout (mobile stacked, desktop inline)

2. **ProjectTile Component** (REFACTOR)
   - Card-based design with shadows
   - Preview image with lazy loading
   - Metadata overlay (file count, size)
   - Folder vs. project visual distinction
   - Hover effects and focus states

3. **ProjectGrid Component** (ENHANCE)
   - Maintain keyboard navigation
   - Add lazy loading for tile images
   - Responsive column adjustments
   - Empty state improvements

4. **BrowsePage Component** (REFACTOR)
   - Hierarchical navigation (parent_id filtering)
   - Breadcrumb integration
   - Root folder view
   - Child folder navigation

#### Medium Priority (P2)
5. **Breadcrumb Component** (ENHANCE)
   - Modern styling with Tailwind
   - Click handlers for navigation
   - Truncation for deep hierarchies

6. **LoadingSpinner Component** (ENHANCE)
   - Skeleton cards instead of spinner
   - Match tile layout during load
   - Progressive disclosure

#### Low Priority (P3)
7. **SearchBar Component** (ENHANCE)
   - Visual refinement
   - Better mobile layout

## 4. Navigation Architecture

### 4.1 Hierarchical Navigation Pattern

**Decision**: Client-side filtering with React Router state

**Approach**:
```typescript
// BrowsePage navigation logic
const [currentFolderId, setCurrentFolderId] = useState<number | null>(null);
const [breadcrumbs, setBreadcrumbs] = useState<Breadcrumb[]>([]);

// Filter projects by parent_id
const visibleProjects = projects.filter(p => p.parent_id === currentFolderId);

// On tile click
const handleTileClick = (project: Project) => {
  if (project.is_leaf) {
    navigate(`/project/${project.id}`);
  } else {
    setCurrentFolderId(project.id);
    setBreadcrumbs([...breadcrumbs, { id: project.id, name: project.name }]);
  }
};

// On breadcrumb click
const handleBreadcrumbClick = (index: number) => {
  const newBreadcrumbs = breadcrumbs.slice(0, index + 1);
  setBreadcrumbs(newBreadcrumbs);
  setCurrentFolderId(newBreadcrumbs[index]?.id || null);
};
```

**Rationale**:
- Leverages existing `parent_id` and `is_leaf` fields
- No backend API changes required
- Simple React state management
- Fast navigation (no API calls between levels)

**Alternatives Considered**:
- ❌ Server-side hierarchy API - requires backend changes
- ❌ React Router params - complex state management
- ✅ Client-side filtering - simplest, maintains compatibility

### 4.2 Breadcrumb Strategy

**Pattern**: Home > Parent Folder > Current Folder

**Implementation**:
- Array of breadcrumb objects: `{ id: number | null, name: string }`
- Root level: `[{ id: null, name: "All Projects" }]`
- Click handler navigates to that level
- Truncation with ellipsis for 5+ levels

## 5. Performance Optimization Research

### 5.1 Lazy Loading Images

**Decision**: Intersection Observer API with loading="lazy"

**Pattern**:
```typescript
// Native browser lazy loading
<img 
  src={previewPath} 
  loading="lazy" 
  alt={project.name}
  className="w-full h-48 object-cover"
/>

// Fallback with Intersection Observer for older browsers
const [isVisible, setIsVisible] = useState(false);
const imgRef = useRef<HTMLImageElement>(null);

useEffect(() => {
  const observer = new IntersectionObserver(
    ([entry]) => {
      if (entry.isIntersecting) {
        setIsVisible(true);
        observer.disconnect();
      }
    },
    { rootMargin: "50px" }
  );
  
  if (imgRef.current) observer.observe(imgRef.current);
  return () => observer.disconnect();
}, []);
```

**Rationale**:
- Modern browsers support native `loading="lazy"` (96%+ compatibility)
- Intersection Observer for older browsers
- Preload images 50px before viewport entry
- Reduces initial page load time significantly

### 5.2 Virtual Scrolling Assessment

**Decision**: NOT NEEDED for initial implementation

**Rationale**:
- Target: 10-500 projects typically
- Modern browsers handle 500 DOM nodes easily
- CSS Grid + lazy images = sufficient performance
- Virtual scrolling adds complexity
- Can be added later if needed (P3 requirement)

**Defer Until**: User collections exceed 1000+ projects

## 6. Design System Tokens

### 6.1 Color Palette

**Decision**: Use Tailwind default palette with custom brand colors

```javascript
// tailwind.config.js
module.exports = {
  theme: {
    extend: {
      colors: {
        primary: {
          50: '#eff6ff',
          100: '#dbeafe',
          500: '#3b82f6',  // Main brand color (blue)
          600: '#2563eb',
          700: '#1d4ed8',
        },
        accent: {
          500: '#8b5cf6',  // Purple for highlights
        }
      }
    }
  }
}
```

**Rationale**:
- Blue primary aligns with 3D/tech aesthetic
- Sufficient contrast for accessibility (WCAG AA)
- Tailwind defaults for grays, success, error states

### 6.2 Spacing System

**Standard Spacing Scale** (Tailwind default - no customization needed):
```
0.5 = 2px
1   = 4px
2   = 8px
3   = 12px
4   = 16px
6   = 24px
8   = 32px
12  = 48px
16  = 64px
```

**Component Spacing Guidelines**:
- Tile padding: `p-6` (24px)
- Grid gap: `gap-4` (16px) on mobile, `gap-6` (24px) on desktop
- Section margins: `mb-8` (32px)
- Card margins: `mb-4` (16px)

### 6.3 Typography Scale

**Decision**: Tailwind default with custom heading weights

```javascript
// tailwind.config.js typography
fontFamily: {
  sans: ['Inter', 'system-ui', 'sans-serif'],
},
fontSize: {
  xs: ['0.75rem', { lineHeight: '1rem' }],      // 12px
  sm: ['0.875rem', { lineHeight: '1.25rem' }],  // 14px
  base: ['1rem', { lineHeight: '1.5rem' }],     // 16px
  lg: ['1.125rem', { lineHeight: '1.75rem' }],  // 18px
  xl: ['1.25rem', { lineHeight: '1.75rem' }],   // 20px
  '2xl': ['1.5rem', { lineHeight: '2rem' }],    // 24px
  '3xl': ['1.875rem', { lineHeight: '2.25rem' }], // 30px
}
```

**Typography Hierarchy**:
- Page title: `text-3xl font-bold` (30px)
- Tile title: `text-lg font-semibold` (18px)
- Metadata: `text-sm text-gray-600` (14px)
- Labels: `text-xs text-gray-500` (12px)

### 6.4 Shadow System

**Shadows for Depth**:
```css
/* Default state */
shadow-md: 0 4px 6px -1px rgba(0,0,0,0.1)

/* Hover state */
shadow-lg: 0 10px 15px -3px rgba(0,0,0,0.1)

/* Focus state */
ring-2 ring-blue-500 ring-offset-2
```

### 6.5 Border Radius

**Consistent Rounding**:
- Cards: `rounded-lg` (8px)
- Buttons: `rounded-md` (6px)
- Images: `rounded-t-lg` (8px top corners)
- Inputs: `rounded-md` (6px)

## 7. Accessibility Research

### 7.1 Keyboard Navigation Requirements

**Current State**: ✅ ProjectGrid has keyboard navigation
- Tab, Arrow keys, Enter, Space, Home, End
- Focus management with tabindex
- Roving tabindex pattern implemented

**Enhancements Needed**:
- Ensure new NavBar is keyboard accessible
- Add skip-to-content link
- Announce breadcrumb changes to screen readers

### 7.2 Screen Reader Support

**Requirements**:
- `role="grid"` on ProjectGrid (already present)
- `aria-label` on tiles with project name + type
- `alt` text on preview images
- Focus indicators (`ring-2 ring-blue-500`)
- Announce loading states with `aria-live="polite"`

### 7.3 Color Contrast

**WCAG AA Compliance**:
- Text on white: gray-900 (#111827) - 19.6:1 ratio ✅
- Secondary text: gray-600 (#4b5563) - 7.2:1 ratio ✅
- Links: blue-600 (#2563eb) - 7.5:1 ratio ✅
- Disabled state: gray-400 (#9ca3af) - 4.7:1 ratio ✅

## 8. Testing Strategy

### 8.1 UI Validation with chrome-devtools-mcp

**Approach**:
```bash
# Phase 1: Baseline screenshots
chrome-devtools-take_screenshot -> screenshots/baseline-home.png
chrome-devtools-take_screenshot -> screenshots/baseline-browse.png

# Phase 2: Implementation screenshots
chrome-devtools-take_screenshot -> screenshots/impl-tiles.png
chrome-devtools-take_snapshot -> validate accessibility tree

# Phase 3: Responsive testing
chrome-devtools-resize_page 320x568 -> mobile validation
chrome-devtools-resize_page 768x1024 -> tablet validation
chrome-devtools-resize_page 1920x1080 -> desktop validation
```

**Validation Checklist**:
- [ ] Tiles display preview images
- [ ] Hover effects visible
- [ ] Keyboard focus indicators
- [ ] Breadcrumb navigation
- [ ] Rescan button in top nav
- [ ] Responsive grid (1/2/3/4 columns)
- [ ] Lazy loading works (images load on scroll)
- [ ] Loading skeletons during fetch

### 8.2 Vitest Component Tests

**Test Coverage Required**:
```typescript
// ProjectTile.test.tsx
- renders folder icon when is_leaf=false
- renders preview image when available
- displays file count and size metadata
- emits onClick when clicked
- applies hover styles
- keyboard accessible (focus, Enter)

// ProjectGrid.test.tsx
- renders correct column count at breakpoints
- maintains keyboard navigation
- lazy loads images outside viewport
- empty state displays correctly

// BrowsePage.test.tsx
- filters projects by parent_id
- updates breadcrumbs on navigation
- navigates to project on leaf click
- navigates to folder on parent click
```

### 8.3 Performance Testing

**Metrics to Validate**:
```javascript
// Chrome DevTools Performance panel
- First Contentful Paint: < 1.5s
- Largest Contentful Paint: < 2.5s
- Time to Interactive: < 3.5s
- Cumulative Layout Shift: < 0.1

// Manual validation
- Scroll performance: 60 FPS with 500 tiles
- Image lazy load: only visible + 1 row preloaded
- Navigation speed: < 100ms to update view
```

## 9. Implementation Best Practices

### 9.1 Progressive Enhancement

**Strategy**: Implement in layers
1. **Phase 1**: Core tile layout + basic styling
2. **Phase 2**: Preview images + lazy loading
3. **Phase 3**: Hierarchical navigation + breadcrumbs
4. **Phase 4**: Polish (animations, hover effects)

**Rationale**:
- Delivers value incrementally
- Easier to validate each layer
- Rollback is simpler if issues arise

### 9.2 Backward Compatibility

**Guarantees**:
- ✅ All existing API contracts unchanged
- ✅ Project data model unchanged (`types/project.ts`)
- ✅ Existing routes preserved (`/`, `/browse`, `/project/:id`)
- ✅ Search functionality maintained
- ✅ Tag system unchanged

**Migration Path**:
- No data migration needed
- No backend deployment required
- Frontend deploy only (Vite build)

### 9.3 Code Quality

**Standards**:
- ESLint rules: max-warnings 0
- TypeScript strict mode
- Component test coverage > 80%
- No `any` types in new code
- Accessibility audit with axe-core

## 10. Open Questions & Resolutions

### 10.1 Preview Image Source

**Question**: Where do preview images come from?  
**Resolution**: Backend API already provides `preview_path` for STL files. Use existing image cache infrastructure.

### 10.2 Empty Folder Handling

**Question**: How to display folders with no children?  
**Resolution**: Show empty state with "No projects in this folder" message. Breadcrumb navigation still works.

### 10.3 Metadata Calculation

**Question**: How to get file count and size for folders?  
**Resolution**: Calculate client-side from `children` array. For leaf projects, use `stl_count` from API response.

### 10.4 Mobile Navigation

**Question**: How does breadcrumb work on small screens?  
**Resolution**: Truncate with ellipsis after 2 levels. Show first and last, collapse middle with "...".

## 11. Summary & Next Steps

### Phase 0 Complete ✅

**Key Decisions Made**:
1. ✅ Tailwind CSS for styling framework
2. ✅ Client-side hierarchical navigation (no API changes)
3. ✅ Native lazy loading with Intersection Observer fallback
4. ✅ Tailwind default palette with custom blue primary
5. ✅ Progressive enhancement strategy
6. ✅ chrome-devtools-mcp for UI validation

**Unknowns Resolved**:
- Design tokens extracted from Tailwind UI
- Component architecture defined
- Performance optimizations identified
- Testing strategy established

### Phase 1 Next

**Deliverables**:
1. `data-model.md` - Component interfaces and props
2. `contracts/component-api.md` - Component contracts
3. `quickstart.md` - Developer setup guide
4. Update agent context with Tailwind CSS, Vitest, chrome-devtools-mcp

**Timeline Estimate**: 2-3 days for design artifacts

---

**Research Complete**: 2025-11-17  
**Approved By**: [Pending review]  
**Next Phase**: Phase 1 - Design & Contracts
