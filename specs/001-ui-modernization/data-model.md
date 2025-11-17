# Phase 1: Data Model & Component Structure

**Feature**: Modern Tile-Based UI  
**Date**: 2025-11-17  
**Status**: Complete

## Overview

This document defines the component architecture, data structures, and interfaces for the modernized tile-based UI. No backend data models are changed - all modifications are frontend-only.

## 1. Component Hierarchy

```
App
├── NavBar (NEW)
│   ├── Logo
│   ├── SearchBar (EXISTING)
│   └── ScanButton (NEW - extracted)
│
├── BrowsePage (REFACTORED)
│   ├── Breadcrumb (ENHANCED)
│   ├── ProjectGrid (ENHANCED)
│   │   └── ProjectTile[] (REFACTORED)
│   └── EmptyState (EXISTING)
│
├── ProjectPage (VISUAL UPDATE)
│   └── [No structural changes]
│
└── HomePage (MODIFIED)
    └── [Scan controls remain]
```

## 2. Core Data Structures

### 2.1 Project (No Changes)

**Source**: `frontend/src/types/project.ts`

```typescript
interface Project {
  id: number;
  name: string;
  full_path: string;
  parent_id: number | null;  // KEY: Used for hierarchy
  is_leaf: boolean;          // KEY: Folder vs. project distinction
  description: string | null;
  created_at: number;
  updated_at: number;
}

interface ProjectWithChildren extends Project {
  children: Project[];       // KEY: For metadata calculation
  stl_count: number;         // KEY: File count display
  image_count: number;
  tags: Tag[];
}
```

**Usage in UI**:
- `parent_id`: Filter projects by folder level
- `is_leaf`: Determine tile icon (folder vs. project)
- `children`: Calculate folder metadata
- `stl_count`: Display "X files" in tile

### 2.2 BreadcrumbItem (NEW)

**Purpose**: Navigation hierarchy state

```typescript
interface BreadcrumbItem {
  id: number | null;  // null = root level
  name: string;       // Display name
  path?: string;      // Optional: full path for debugging
}

// Example usage
const breadcrumbs: BreadcrumbItem[] = [
  { id: null, name: "All Projects" },              // Root
  { id: 5, name: "Miniatures" },                   // Level 1
  { id: 12, name: "Fantasy", path: "/Miniatures/Fantasy" }  // Level 2
];
```

**State Management**:
- Stored in `BrowsePage` component state
- Updated on tile navigation
- Passed to `Breadcrumb` component as prop

### 2.3 TileMetadata (NEW)

**Purpose**: Calculated metadata for tile display

```typescript
interface TileMetadata {
  fileCount: number;      // Number of STL files
  totalSize?: string;     // Human-readable size (e.g., "245 MB")
  imageCount?: number;    // Number of images (for folders)
  previewImage?: string;  // Path to preview image
  icon: 'folder' | 'project';  // Icon type
}

// Calculation helper
function calculateTileMetadata(project: ProjectWithChildren): TileMetadata {
  if (project.is_leaf) {
    return {
      fileCount: project.stl_count,
      totalSize: formatBytes(project.total_size),
      previewImage: project.preview_path,
      icon: 'project'
    };
  } else {
    // Folder: aggregate children
    const fileCount = project.children.reduce((sum, child) => 
      sum + (child.stl_count || 0), 0
    );
    return {
      fileCount,
      imageCount: project.image_count,
      icon: 'folder'
    };
  }
}
```

## 3. Component Interfaces

### 3.1 NavBar (NEW)

**Purpose**: Top navigation with search and scan button

```typescript
interface NavBarProps {
  // No props - handles internal state
}

interface NavBarState {
  isMobileMenuOpen: boolean;
}
```

**Component Structure**:
```tsx
<nav className="bg-white shadow-md">
  <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <div className="flex items-center justify-between h-16">
      {/* Logo */}
      <Link to="/" className="text-xl font-bold text-gray-900">
        Glyptotheka
      </Link>
      
      {/* Search Bar (center, desktop) */}
      <div className="flex-1 max-w-2xl mx-8 hidden md:block">
        <SearchBar />
      </div>
      
      {/* Nav Links + Scan Button (right) */}
      <div className="flex items-center gap-4">
        <Link to="/browse">Browse</Link>
        <ScanButton />
      </div>
    </div>
    
    {/* Mobile Search (stacked below) */}
    <div className="md:hidden py-3">
      <SearchBar />
    </div>
  </div>
</nav>
```

**Styling Classes**:
```css
nav: bg-white shadow-md sticky top-0 z-50
container: max-w-7xl mx-auto px-4
flex-layout: flex items-center justify-between h-16
logo: text-xl font-bold text-gray-900 hover:text-blue-600
nav-link: text-gray-600 hover:text-blue-600 transition-colors
```

### 3.2 ScanButton (NEW - Extracted)

**Purpose**: Trigger library rescan from navigation

```typescript
interface ScanButtonProps {
  variant?: 'primary' | 'secondary';  // Default: 'primary'
  size?: 'sm' | 'md' | 'lg';          // Default: 'md'
}

interface ScanButtonState {
  isScanning: boolean;
  progress?: ScanProgress;
}
```

**Component Structure**:
```tsx
<button
  onClick={handleScanClick}
  disabled={isScanning}
  className={cn(
    "flex items-center gap-2 px-4 py-2 rounded-md font-medium",
    "transition-colors duration-200",
    isScanning 
      ? "bg-gray-300 cursor-not-allowed" 
      : "bg-blue-600 hover:bg-blue-700 text-white"
  )}
>
  {isScanning ? (
    <>
      <Spinner size="sm" />
      <span>Scanning...</span>
    </>
  ) : (
    <>
      <RefreshIcon className="w-5 h-5" />
      <span>Rescan Library</span>
    </>
  )}
</button>
```

### 3.3 ProjectTile (REFACTORED)

**Purpose**: Display individual project/folder as modern card

```typescript
interface ProjectTileProps {
  project: Project;
  metadata: TileMetadata;
  onClick: () => void;
  loading?: boolean;           // Show skeleton
  lazyLoad?: boolean;          // Enable lazy image loading (default: true)
}
```

**Component Structure**:
```tsx
<div
  role="button"
  tabIndex={0}
  onClick={onClick}
  onKeyDown={handleKeyDown}
  className={cn(
    "group relative bg-white rounded-lg shadow-md overflow-hidden",
    "cursor-pointer transition-shadow duration-200",
    "hover:shadow-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
  )}
>
  {/* Preview Image / Icon */}
  <div className="relative h-48 bg-gray-100">
    {metadata.previewImage ? (
      <img
        src={metadata.previewImage}
        alt={project.name}
        loading={lazyLoad ? "lazy" : undefined}
        className="w-full h-full object-cover"
      />
    ) : (
      <div className="flex items-center justify-center h-full">
        {metadata.icon === 'folder' ? (
          <FolderIcon className="w-16 h-16 text-gray-400" />
        ) : (
          <CubeIcon className="w-16 h-16 text-gray-400" />
        )}
      </div>
    )}
    
    {/* Hover Overlay (optional) */}
    <div className="absolute inset-0 bg-black bg-opacity-0 group-hover:bg-opacity-10 transition-opacity" />
  </div>
  
  {/* Content */}
  <div className="p-4">
    {/* Title */}
    <h3 className="text-lg font-semibold text-gray-900 mb-2 line-clamp-2">
      {project.name}
    </h3>
    
    {/* Metadata */}
    <div className="flex items-center gap-4 text-sm text-gray-600">
      <span className="flex items-center gap-1">
        <FileIcon className="w-4 h-4" />
        {metadata.fileCount} file{metadata.fileCount !== 1 ? 's' : ''}
      </span>
      {metadata.totalSize && (
        <span>{metadata.totalSize}</span>
      )}
    </div>
    
    {/* Type Badge */}
    <div className="mt-3">
      <span className={cn(
        "inline-block px-2 py-1 text-xs rounded-full",
        metadata.icon === 'folder' 
          ? "bg-blue-100 text-blue-700" 
          : "bg-green-100 text-green-700"
      )}>
        {metadata.icon === 'folder' ? 'Folder' : 'Project'}
      </span>
    </div>
  </div>
</div>
```

**Styling Classes**:
```css
/* Card */
bg-white rounded-lg shadow-md overflow-hidden
transition-shadow duration-200
hover:shadow-lg

/* Image Container */
relative h-48 bg-gray-100

/* Icon */
w-16 h-16 text-gray-400

/* Title */
text-lg font-semibold text-gray-900 mb-2 line-clamp-2

/* Metadata */
flex items-center gap-4 text-sm text-gray-600

/* Badge */
inline-block px-2 py-1 text-xs rounded-full
bg-blue-100 text-blue-700 (folder)
bg-green-100 text-green-700 (project)
```

### 3.4 ProjectGrid (ENHANCED)

**Purpose**: Responsive grid with keyboard navigation

```typescript
interface ProjectGridProps {
  projects: Project[];
  onProjectClick: (id: number) => void;
  loading?: boolean;           // Show skeletons
  emptyMessage?: string;       // Custom empty state
}

interface ProjectGridState {
  focusedIndex: number;
}
```

**Component Structure**:
```tsx
<div
  ref={gridRef}
  role="grid"
  aria-label="Projects grid"
  className={cn(
    "grid gap-4 sm:gap-6",
    "grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4"
  )}
>
  {loading ? (
    // Skeleton tiles
    Array.from({ length: 8 }).map((_, i) => (
      <SkeletonTile key={i} />
    ))
  ) : projects.length === 0 ? (
    // Empty state
    <div className="col-span-full">
      <EmptyState message={emptyMessage} />
    </div>
  ) : (
    // Project tiles
    projects.map((project, index) => {
      const metadata = calculateTileMetadata(project);
      return (
        <ProjectTile
          key={project.id}
          project={project}
          metadata={metadata}
          onClick={() => onProjectClick(project.id)}
          lazyLoad={true}
        />
      );
    })
  )}
</div>
```

**Responsive Breakpoints**:
```css
/* Mobile (< 640px) */
grid-cols-1

/* Tablet (640px - 768px) */
sm:grid-cols-2

/* Small Desktop (768px - 1024px) */
md:grid-cols-3

/* Large Desktop (1024px+) */
lg:grid-cols-4
```

**Keyboard Navigation** (existing - no changes):
- Tab: Next/previous tile
- Arrow keys: Grid navigation
- Enter/Space: Activate tile
- Home/End: First/last tile

### 3.5 Breadcrumb (ENHANCED)

**Purpose**: Show current location in hierarchy

```typescript
interface BreadcrumbProps {
  items: BreadcrumbItem[];
  onNavigate: (index: number) => void;
  maxVisible?: number;         // Default: 5 (truncate after)
}
```

**Component Structure**:
```tsx
<nav aria-label="Breadcrumb" className="mb-6">
  <ol className="flex items-center gap-2 text-sm">
    {visibleItems.map((item, index) => (
      <li key={item.id || 'root'} className="flex items-center gap-2">
        {index > 0 && (
          <ChevronRightIcon className="w-4 h-4 text-gray-400" />
        )}
        
        {index === items.length - 1 ? (
          // Current page (not clickable)
          <span className="font-semibold text-gray-900">
            {item.name}
          </span>
        ) : (
          // Clickable ancestor
          <button
            onClick={() => onNavigate(index)}
            className="text-gray-600 hover:text-blue-600 transition-colors"
          >
            {item.name}
          </button>
        )}
      </li>
    ))}
  </ol>
</nav>
```

**Truncation Logic**:
```typescript
function truncateBreadcrumbs(
  items: BreadcrumbItem[], 
  maxVisible: number
): BreadcrumbItem[] {
  if (items.length <= maxVisible) return items;
  
  // Show: First, ..., Last 2
  return [
    items[0],
    { id: -1, name: '...' },  // Ellipsis
    ...items.slice(-2)
  ];
}
```

### 3.6 BrowsePage (REFACTORED)

**Purpose**: Root view with hierarchical navigation

```typescript
interface BrowsePageProps {
  // No props - uses React Router
}

interface BrowsePageState {
  projects: Project[];
  currentFolderId: number | null;
  breadcrumbs: BreadcrumbItem[];
  loading: boolean;
  error: string | null;
}
```

**Component Structure**:
```tsx
function BrowsePage() {
  const [projects, setProjects] = useState<Project[]>([]);
  const [currentFolderId, setCurrentFolderId] = useState<number | null>(null);
  const [breadcrumbs, setBreadcrumbs] = useState<BreadcrumbItem[]>([
    { id: null, name: "All Projects" }
  ]);
  const [loading, setLoading] = useState(true);
  
  const navigate = useNavigate();
  
  // Load all projects once
  useEffect(() => {
    loadProjects();
  }, []);
  
  // Filter visible projects by current folder
  const visibleProjects = useMemo(() => {
    return projects.filter(p => p.parent_id === currentFolderId);
  }, [projects, currentFolderId]);
  
  const handleTileClick = (projectId: number) => {
    const project = projects.find(p => p.id === projectId);
    if (!project) return;
    
    if (project.is_leaf) {
      // Navigate to project detail
      navigate(`/project/${projectId}`);
    } else {
      // Navigate into folder
      setCurrentFolderId(projectId);
      setBreadcrumbs([...breadcrumbs, { 
        id: projectId, 
        name: project.name 
      }]);
    }
  };
  
  const handleBreadcrumbClick = (index: number) => {
    const newBreadcrumbs = breadcrumbs.slice(0, index + 1);
    setBreadcrumbs(newBreadcrumbs);
    setCurrentFolderId(newBreadcrumbs[index].id);
  };
  
  return (
    <div className="container mx-auto px-4 py-8">
      <h1 className="text-3xl font-bold mb-6">Browse Projects</h1>
      
      <Breadcrumb 
        items={breadcrumbs}
        onNavigate={handleBreadcrumbClick}
      />
      
      <ProjectGrid
        projects={visibleProjects}
        onProjectClick={handleTileClick}
        loading={loading}
        emptyMessage="No projects in this folder"
      />
    </div>
  );
}
```

**State Flow**:
```
Initial Load:
  currentFolderId = null
  breadcrumbs = [{ id: null, name: "All Projects" }]
  visibleProjects = projects.filter(p => p.parent_id === null)

Click Folder (id: 5, name: "Miniatures"):
  currentFolderId = 5
  breadcrumbs = [...prev, { id: 5, name: "Miniatures" }]
  visibleProjects = projects.filter(p => p.parent_id === 5)

Click Breadcrumb (index: 0):
  currentFolderId = null
  breadcrumbs = [{ id: null, name: "All Projects" }]
  visibleProjects = projects.filter(p => p.parent_id === null)
```

## 4. State Management

### 4.1 Component State (React useState)

**Rationale**: Simple local state sufficient - no global state needed

**State Locations**:
- `BrowsePage`: Navigation state (currentFolderId, breadcrumbs)
- `ProjectGrid`: Focus management (focusedIndex)
- `ProjectTile`: Hover state (internal)
- `NavBar`: Mobile menu state (isMobileMenuOpen)
- `ScanButton`: Scan progress (isScanning, progress)

**No Zustand/Redux needed** - navigation state is page-specific

### 4.2 Data Fetching (React Query)

**Existing**: Uses `@tanstack/react-query` for API calls

```typescript
// Existing pattern (no changes)
import { useQuery } from '@tanstack/react-query';

function BrowsePage() {
  const { data: projects, isLoading, error } = useQuery({
    queryKey: ['projects', 'root'],
    queryFn: () => projectsAPI.listRoot(),
    staleTime: 5 * 60 * 1000,  // 5 minutes
  });
  
  // ... component logic
}
```

**No changes needed** - existing API calls remain

## 5. Validation Rules

### 5.1 Input Validation

**No user input changes** - all validation remains in existing components:
- SearchBar: existing validation
- Tag input: existing validation
- Scan path: existing validation

### 5.2 UI State Validation

**Navigation Constraints**:
```typescript
// Cannot navigate to non-existent folder
function validateNavigationTarget(projectId: number): boolean {
  const project = projects.find(p => p.id === projectId);
  return project !== undefined && !project.is_leaf;
}

// Breadcrumb index must be valid
function validateBreadcrumbIndex(index: number): boolean {
  return index >= 0 && index < breadcrumbs.length;
}
```

### 5.3 Accessibility Validation

**Required ARIA attributes**:
- `role="button"` on clickable tiles
- `aria-label` with project name + type
- `role="grid"` on ProjectGrid
- `aria-label="Breadcrumb"` on breadcrumb nav
- `alt` text on all images
- Focus indicators (`focus:ring-2`)

## 6. Error States

### 6.1 Loading States

**Skeleton Components**:
```tsx
function SkeletonTile() {
  return (
    <div className="bg-white rounded-lg shadow-md overflow-hidden animate-pulse">
      <div className="h-48 bg-gray-200" />
      <div className="p-4 space-y-3">
        <div className="h-4 bg-gray-200 rounded w-3/4" />
        <div className="h-3 bg-gray-200 rounded w-1/2" />
      </div>
    </div>
  );
}
```

### 6.2 Empty States

**No Projects**:
```tsx
<EmptyState
  icon={<FolderIcon className="w-16 h-16 text-gray-400" />}
  title="No projects found"
  message="This folder is empty. Add some projects or run a scan."
  action={
    <Button onClick={handleScan}>
      Scan Library
    </Button>
  }
/>
```

**Empty Folder**:
```tsx
<EmptyState
  icon={<FolderOpenIcon className="w-16 h-16 text-gray-400" />}
  title="Empty folder"
  message="This folder contains no projects yet."
/>
```

### 6.3 Error States

**API Failure**:
```tsx
<ErrorState
  title="Failed to load projects"
  message={error.message}
  action={
    <Button onClick={retry}>
      Try Again
    </Button>
  }
/>
```

## 7. Performance Considerations

### 7.1 Memoization

**Expensive Calculations**:
```typescript
// Memoize filtered projects
const visibleProjects = useMemo(() => {
  return projects.filter(p => p.parent_id === currentFolderId);
}, [projects, currentFolderId]);

// Memoize metadata calculation
const tilesWithMetadata = useMemo(() => {
  return visibleProjects.map(project => ({
    project,
    metadata: calculateTileMetadata(project)
  }));
}, [visibleProjects]);
```

### 7.2 Lazy Loading

**Image Loading Strategy**:
```tsx
<img
  src={previewPath}
  loading="lazy"  // Native browser lazy loading
  alt={project.name}
  className="w-full h-48 object-cover"
  onLoad={handleImageLoad}  // Optional: fade-in animation
/>
```

### 7.3 Virtual Scrolling (NOT IMPLEMENTED)

**Deferred to P3** - simple grid sufficient for 500 projects

## 8. Migration Path

### 8.1 Component Migration Order

**Phase 1**: Foundation
1. Install Tailwind CSS
2. Create `NavBar` component
3. Extract `ScanButton` component

**Phase 2**: Core Components
4. Refactor `ProjectTile` (card design)
5. Enhance `ProjectGrid` (responsive)
6. Update `Breadcrumb` (styling)

**Phase 3**: Pages
7. Refactor `BrowsePage` (hierarchy)
8. Update `HomePage` (visual consistency)
9. Update `ProjectPage` (visual consistency)

**Phase 4**: Polish
10. Add lazy loading
11. Add skeletons
12. Add animations

### 8.2 No Data Migration

**Zero Backend Changes**:
- ✅ All data structures unchanged
- ✅ API contracts unchanged
- ✅ Database schema unchanged
- ✅ Frontend-only deployment

## 9. Summary

### Entities Created

| Entity | Type | Purpose |
|--------|------|---------|
| `BreadcrumbItem` | Interface | Navigation hierarchy state |
| `TileMetadata` | Interface | Calculated tile display data |
| `NavBar` | Component | Top navigation |
| `ScanButton` | Component | Library rescan trigger |
| `ProjectTile` (refactored) | Component | Modern card tile |
| `ProjectGrid` (enhanced) | Component | Responsive grid |
| `Breadcrumb` (enhanced) | Component | Hierarchy navigation |
| `BrowsePage` (refactored) | Component | Root browsing page |
| `SkeletonTile` | Component | Loading placeholder |

### Key Relationships

```
BrowsePage
  ├── manages: breadcrumbs (BreadcrumbItem[])
  ├── manages: currentFolderId (number | null)
  ├── filters: projects by parent_id
  ├── renders: Breadcrumb
  └── renders: ProjectGrid
        └── renders: ProjectTile[]
              └── uses: TileMetadata

NavBar
  ├── renders: SearchBar (existing)
  └── renders: ScanButton (new)
```

### No Backend Changes Required

**API Contracts Preserved**:
- `GET /api/projects` - unchanged
- `GET /api/projects/:id` - unchanged
- `POST /api/scan` - unchanged

**Data Models Preserved**:
- `Project` interface - unchanged
- `ProjectWithChildren` interface - unchanged
- All other types unchanged

---

**Phase 1 Complete**: 2025-11-17  
**Next**: Contracts & Quickstart
