# Component API Contracts

**Feature**: Modern Tile-Based UI  
**Date**: 2025-11-17  
**Version**: 1.0.0

## Overview

This document defines the public API contracts for all components in the UI modernization feature. These contracts serve as the interface specification for component integration and testing.

## 1. NavBar Component

**File**: `frontend/src/components/common/NavBar.tsx`

### API

```typescript
export interface NavBarProps {
  // No props - self-contained component
}

export function NavBar(): JSX.Element;
```

### Behavior

**Responsibilities**:
- Display application logo/title
- Render search bar (centered on desktop)
- Render navigation links (Home, Browse)
- Render scan button (top-right)
- Handle responsive layout (mobile/desktop)

**State Management**:
- Internal: Mobile menu open/closed state
- External: None (stateless from parent perspective)

### Events

| Event | Trigger | Effect |
|-------|---------|--------|
| Logo click | User clicks logo | Navigate to "/" |
| Browse link click | User clicks Browse | Navigate to "/browse" |
| Scan button click | User clicks Rescan | Trigger library scan (via ScanButton) |

### Styling Contract

**Desktop (≥768px)**:
- Height: 64px (h-16)
- Layout: Horizontal flex (logo | search | links + scan)
- Max width: 1280px (max-w-7xl)
- Background: white with shadow

**Mobile (<768px)**:
- Height: Auto (stacked)
- Layout: Vertical (logo + nav | search)
- Search bar: Full width below header

### Testing Contract

**Unit Tests Required**:
- [ ] Renders logo with correct link
- [ ] Renders search bar
- [ ] Renders browse link
- [ ] Renders scan button
- [ ] Mobile menu toggles on small screens
- [ ] Responsive layout switches at breakpoint

**Accessibility Tests**:
- [ ] Logo link has accessible text
- [ ] Nav links have accessible labels
- [ ] Keyboard navigation works (Tab through links)

---

## 2. ScanButton Component

**File**: `frontend/src/components/scan/ScanButton.tsx`

### API

```typescript
export interface ScanButtonProps {
  variant?: 'primary' | 'secondary';  // Default: 'primary'
  size?: 'sm' | 'md' | 'lg';          // Default: 'md'
  onScanStart?: () => void;           // Callback when scan starts
  onScanComplete?: () => void;        // Callback when scan completes
  className?: string;                 // Additional CSS classes
}

export function ScanButton(props: ScanButtonProps): JSX.Element;
```

### Behavior

**Responsibilities**:
- Trigger library scan via API
- Display scan progress (% or spinner)
- Disable button during scan
- Show error state on failure

**State Management**:
- Internal: `isScanning`, `progress`, `error`
- External: Optional callbacks for lifecycle events

### Events

| Event | Trigger | Effect |
|-------|---------|--------|
| Click (idle) | User clicks button | Start scan, call `onScanStart` |
| Click (scanning) | User clicks while scanning | No-op (disabled) |
| Scan complete | Scan finishes successfully | Call `onScanComplete`, reset state |
| Scan error | Scan fails | Show error message, enable retry |

### States

```typescript
type ScanButtonState = 
  | { status: 'idle' }
  | { status: 'scanning', progress: number }  // 0-100
  | { status: 'error', message: string }
  | { status: 'success' }
```

### Styling Contract

**Variants**:
- `primary`: Blue background (bg-blue-600 hover:bg-blue-700)
- `secondary`: White background with blue border

**Sizes**:
- `sm`: px-3 py-1.5 text-sm
- `md`: px-4 py-2 text-base (default)
- `lg`: px-6 py-3 text-lg

**States**:
- Idle: Normal colors, cursor-pointer
- Scanning: Gray background, cursor-not-allowed, spinner icon
- Error: Red border/text, retry icon
- Success: Green checkmark (brief), then idle

### Testing Contract

**Unit Tests Required**:
- [ ] Renders with correct variant and size
- [ ] Triggers scan on click (idle state)
- [ ] Disables during scan
- [ ] Shows progress indicator
- [ ] Handles scan error
- [ ] Calls lifecycle callbacks

**Integration Tests**:
- [ ] API call triggered with correct endpoint
- [ ] Progress updates received from scan API
- [ ] Error state displays API error message

---

## 3. ProjectTile Component

**File**: `frontend/src/components/project/ProjectTile.tsx`

### API

```typescript
export interface ProjectTileProps {
  project: Project;                   // Project data
  metadata: TileMetadata;             // Calculated metadata
  onClick: () => void;                // Click handler
  loading?: boolean;                  // Show skeleton (default: false)
  lazyLoad?: boolean;                 // Lazy load image (default: true)
  className?: string;                 // Additional CSS classes
}

export function ProjectTile(props: ProjectTileProps): JSX.Element;
```

### Behavior

**Responsibilities**:
- Display project/folder as card
- Show preview image or icon
- Display name, file count, size
- Handle click and keyboard navigation
- Apply hover effects

**State Management**:
- Internal: Hover state, image loaded state
- External: None (controlled component)

### Events

| Event | Trigger | Effect |
|-------|---------|--------|
| Click | User clicks tile | Call `onClick()` |
| Enter/Space | User presses key | Call `onClick()` |
| Hover | Mouse enters tile | Apply hover styles |
| Image load | Preview loads | Fade in image |

### Visual States

```typescript
type TileVisualState =
  | 'idle'        // Default state
  | 'hover'       // Mouse over
  | 'focus'       // Keyboard focus
  | 'loading'     // Skeleton mode
  | 'error'       // Image failed to load
```

### Styling Contract

**Card Structure**:
```
┌──────────────────────────┐
│   Preview Image (h-48)   │ <- Image or icon
├──────────────────────────┤
│ Title (text-lg)          │
│ Metadata (text-sm)       │
│ [Folder/Project badge]   │
└──────────────────────────┘
```

**Dimensions**:
- Min height: 320px
- Image height: 192px (h-48)
- Padding: 16px (p-4)
- Border radius: 8px (rounded-lg)

**Colors**:
- Background: white
- Shadow: md (default), lg (hover)
- Focus ring: 2px blue-500
- Folder badge: blue-100 bg, blue-700 text
- Project badge: green-100 bg, green-700 text

### Testing Contract

**Unit Tests Required**:
- [ ] Renders project name
- [ ] Renders metadata (file count, size)
- [ ] Shows folder icon when is_leaf=false
- [ ] Shows project icon when is_leaf=true
- [ ] Displays preview image when available
- [ ] Lazy loads image when lazyLoad=true
- [ ] Calls onClick when clicked
- [ ] Calls onClick when Enter pressed
- [ ] Shows skeleton when loading=true
- [ ] Applies hover styles
- [ ] Shows focus indicator

**Accessibility Tests**:
- [ ] role="button"
- [ ] tabindex="0"
- [ ] aria-label includes project name + type
- [ ] Focus visible (ring-2)
- [ ] Keyboard navigable (Enter/Space)

---

## 4. ProjectGrid Component

**File**: `frontend/src/components/project/ProjectGrid.tsx`

### API

```typescript
export interface ProjectGridProps {
  projects: Project[];                // Projects to display
  onProjectClick: (id: number) => void;  // Tile click handler
  loading?: boolean;                  // Show skeletons (default: false)
  emptyMessage?: string;              // Custom empty state message
  columns?: {                         // Column count by breakpoint
    mobile?: number;                  // Default: 1
    tablet?: number;                  // Default: 2
    desktop?: number;                 // Default: 4
  };
  className?: string;                 // Additional CSS classes
}

export function ProjectGrid(props: ProjectGridProps): JSX.Element;
```

### Behavior

**Responsibilities**:
- Render projects in responsive grid
- Handle keyboard navigation (arrows, Tab)
- Show loading skeletons
- Display empty state
- Manage focus state
- Calculate tile metadata

**State Management**:
- Internal: `focusedIndex` for keyboard nav
- External: None (controlled component)

### Events

| Event | Trigger | Effect |
|-------|---------|--------|
| Tile click | User clicks tile | Call `onProjectClick(id)` |
| Arrow keys | User navigates grid | Move focus to adjacent tile |
| Tab | User presses Tab | Move focus to next tile |
| Enter/Space | User activates tile | Call `onProjectClick(focusedId)` |
| Home/End | User presses Home/End | Focus first/last tile |

### Keyboard Navigation

**Key Bindings**:
- `Tab`: Next tile (natural tab order)
- `Shift+Tab`: Previous tile
- `ArrowRight`: Next tile in row
- `ArrowLeft`: Previous tile in row
- `ArrowDown`: Tile in next row (same column)
- `ArrowUp`: Tile in previous row (same column)
- `Home`: First tile
- `End`: Last tile
- `Enter` or `Space`: Activate focused tile

**Focus Management**:
- Roving tabindex pattern
- Only focused tile has `tabindex="0"`
- Others have `tabindex="-1"`
- Focus indicator: `ring-2 ring-blue-500`

### Responsive Grid

**Breakpoints**:
```css
/* Mobile (default) */
grid-cols-1

/* Tablet (≥640px) */
sm:grid-cols-2

/* Desktop (≥768px) */
md:grid-cols-3

/* Large Desktop (≥1024px) */
lg:grid-cols-4
```

**Customizable via props**:
```tsx
<ProjectGrid
  projects={projects}
  onProjectClick={handleClick}
  columns={{ mobile: 1, tablet: 3, desktop: 5 }}
/>
```

### Testing Contract

**Unit Tests Required**:
- [ ] Renders correct number of tiles
- [ ] Shows skeletons when loading=true
- [ ] Shows empty state when projects=[]
- [ ] Calls onProjectClick with correct id
- [ ] Arrow key navigation works
- [ ] Tab navigation works
- [ ] Home/End keys work
- [ ] Focus indicator visible
- [ ] Calculates metadata for each tile
- [ ] Responsive columns render correctly

**Integration Tests**:
- [ ] Keyboard nav with 100 tiles
- [ ] Lazy loading triggers for images
- [ ] Focus persists on state update

**Accessibility Tests**:
- [ ] role="grid"
- [ ] aria-label="Projects grid"
- [ ] Keyboard navigation meets ARIA grid pattern
- [ ] Focus always visible

---

## 5. Breadcrumb Component

**File**: `frontend/src/components/common/Breadcrumb.tsx`

### API

```typescript
export interface BreadcrumbItem {
  id: number | null;  // null = root
  name: string;       // Display name
  path?: string;      // Optional full path (for debugging)
}

export interface BreadcrumbProps {
  items: BreadcrumbItem[];           // Breadcrumb trail
  onNavigate: (index: number) => void;  // Click handler
  maxVisible?: number;               // Truncate after N (default: 5)
  separator?: React.ReactNode;       // Custom separator (default: chevron)
  className?: string;                // Additional CSS classes
}

export function Breadcrumb(props: BreadcrumbProps): JSX.Element;
```

### Behavior

**Responsibilities**:
- Display navigation trail
- Handle breadcrumb clicks
- Truncate long paths
- Style current page differently

**State Management**:
- Internal: None (stateless)
- External: Controlled by parent (BrowsePage)

### Events

| Event | Trigger | Effect |
|-------|---------|--------|
| Ancestor click | User clicks non-current item | Call `onNavigate(index)` |
| Current click | User clicks current item | No-op (not clickable) |

### Truncation Logic

**Rules**:
- If items.length ≤ maxVisible: Show all
- If items.length > maxVisible: Show first, "...", last 2

**Example**:
```
Full: Home > Miniatures > Fantasy > Orcs > Characters
Truncated (max 5): Home > ... > Orcs > Characters
```

### Styling Contract

**Structure**:
```html
<nav aria-label="Breadcrumb">
  <ol class="flex items-center gap-2">
    <li>Home</li>
    <li><separator /></li>
    <li>Miniatures</li>
    <li><separator /></li>
    <li aria-current="page">Fantasy</li>
  </ol>
</nav>
```

**Colors**:
- Ancestor: text-gray-600 hover:text-blue-600
- Current: text-gray-900 font-semibold (not clickable)
- Separator: text-gray-400

### Testing Contract

**Unit Tests Required**:
- [ ] Renders all items
- [ ] Truncates when > maxVisible
- [ ] Current item not clickable
- [ ] Calls onNavigate with correct index
- [ ] Shows custom separator if provided
- [ ] Ellipsis renders for truncated paths

**Accessibility Tests**:
- [ ] nav has aria-label="Breadcrumb"
- [ ] Current item has aria-current="page"
- [ ] Ancestor items are buttons (keyboard accessible)

---

## 6. BrowsePage Component

**File**: `frontend/src/pages/BrowsePage.tsx`

### API

```typescript
export interface BrowsePageProps {
  // No props - uses React Router for URL state
}

export function BrowsePage(): JSX.Element;
```

### Behavior

**Responsibilities**:
- Load all projects from API
- Filter projects by current folder
- Manage breadcrumb trail
- Handle tile clicks (navigate or drill down)
- Display loading/error/empty states

**State Management**:
- Internal: 
  - `projects: Project[]` - All projects
  - `currentFolderId: number | null` - Current folder filter
  - `breadcrumbs: BreadcrumbItem[]` - Navigation trail
  - `loading: boolean` - API fetch state
  - `error: string | null` - API error
- External: React Router for navigation

### Events

| Event | Trigger | Effect |
|-------|---------|--------|
| Page load | Component mounts | Fetch all projects |
| Tile click (folder) | User clicks folder tile | Update currentFolderId, add breadcrumb |
| Tile click (project) | User clicks project tile | Navigate to `/project/:id` |
| Breadcrumb click | User clicks ancestor | Update currentFolderId, trim breadcrumbs |

### URL State

**Current Design**: No URL params (stateful navigation)

**Future Enhancement** (optional):
```typescript
// URL: /browse?folder=12
const [searchParams, setSearchParams] = useSearchParams();
const currentFolderId = searchParams.get('folder') 
  ? parseInt(searchParams.get('folder')) 
  : null;
```

### Data Flow

```
1. Load Projects
   └─> API: GET /api/projects
       └─> setState: projects = [...]

2. User clicks folder tile (id: 5)
   └─> if is_leaf: navigate(/project/5)
   └─> else:
       ├─> setCurrentFolderId(5)
       └─> setBreadcrumbs([...prev, { id: 5, name: "..." }])

3. Render grid
   └─> visibleProjects = projects.filter(p => p.parent_id === currentFolderId)
```

### Testing Contract

**Unit Tests Required**:
- [ ] Fetches projects on mount
- [ ] Filters projects by currentFolderId
- [ ] Navigates to project detail on leaf click
- [ ] Updates currentFolderId on folder click
- [ ] Updates breadcrumbs on folder click
- [ ] Resets breadcrumbs on breadcrumb click
- [ ] Shows loading state while fetching
- [ ] Shows error state on API failure
- [ ] Shows empty state when no projects

**Integration Tests**:
- [ ] Full navigation flow (root → folder → project)
- [ ] Back navigation via breadcrumbs
- [ ] Multiple folder levels (3+)
- [ ] Empty folder display

---

## 7. Contract Versioning

**Version**: 1.0.0  
**Date**: 2025-11-17

### Breaking Changes Policy

**Major Version** (X.0.0): Breaking API changes
- Removing props
- Changing prop types (non-compatible)
- Removing components
- Changing event signatures

**Minor Version** (0.X.0): New features
- Adding optional props
- New components
- New events
- Enhanced behavior (backward compatible)

**Patch Version** (0.0.X): Bug fixes
- Fixing incorrect behavior
- Performance improvements
- Styling fixes

### Change Log

**1.0.0** (2025-11-17):
- Initial contract definitions
- NavBar, ScanButton, ProjectTile, ProjectGrid, Breadcrumb, BrowsePage

---

## 8. Integration Examples

### Example 1: Using ProjectGrid

```typescript
import { ProjectGrid } from '@/components/project/ProjectGrid';

function MyPage() {
  const [projects, setProjects] = useState<Project[]>([]);
  const navigate = useNavigate();
  
  const handleProjectClick = (id: number) => {
    navigate(`/project/${id}`);
  };
  
  return (
    <ProjectGrid
      projects={projects}
      onProjectClick={handleProjectClick}
      loading={false}
      emptyMessage="No projects found"
    />
  );
}
```

### Example 2: Using Breadcrumb

```typescript
import { Breadcrumb, BreadcrumbItem } from '@/components/common/Breadcrumb';

function MyPage() {
  const [breadcrumbs, setBreadcrumbs] = useState<BreadcrumbItem[]>([
    { id: null, name: "Home" },
    { id: 5, name: "Miniatures" },
  ]);
  
  const handleNavigate = (index: number) => {
    const newBreadcrumbs = breadcrumbs.slice(0, index + 1);
    setBreadcrumbs(newBreadcrumbs);
    // Update view based on breadcrumbs[index].id
  };
  
  return (
    <Breadcrumb
      items={breadcrumbs}
      onNavigate={handleNavigate}
      maxVisible={5}
    />
  );
}
```

### Example 3: Complete BrowsePage Integration

```typescript
function BrowsePage() {
  const [projects, setProjects] = useState<Project[]>([]);
  const [currentFolderId, setCurrentFolderId] = useState<number | null>(null);
  const [breadcrumbs, setBreadcrumbs] = useState<BreadcrumbItem[]>([
    { id: null, name: "All Projects" }
  ]);
  const navigate = useNavigate();
  
  const visibleProjects = projects.filter(p => p.parent_id === currentFolderId);
  
  const handleTileClick = (id: number) => {
    const project = projects.find(p => p.id === id);
    if (!project) return;
    
    if (project.is_leaf) {
      navigate(`/project/${id}`);
    } else {
      setCurrentFolderId(id);
      setBreadcrumbs([...breadcrumbs, { id, name: project.name }]);
    }
  };
  
  const handleBreadcrumbClick = (index: number) => {
    setBreadcrumbs(breadcrumbs.slice(0, index + 1));
    setCurrentFolderId(breadcrumbs[index].id);
  };
  
  return (
    <div>
      <Breadcrumb items={breadcrumbs} onNavigate={handleBreadcrumbClick} />
      <ProjectGrid projects={visibleProjects} onProjectClick={handleTileClick} />
    </div>
  );
}
```

---

**Contract Version**: 1.0.0  
**Last Updated**: 2025-11-17  
**Status**: Approved for implementation
