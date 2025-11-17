# Quickstart Guide: UI Modernization

**Feature**: Modern Tile-Based UI  
**Date**: 2025-11-17  
**Audience**: Developers implementing or extending the UI modernization

## Prerequisites

**Required Knowledge**:
- React 18 (hooks, state management)
- TypeScript basics
- Tailwind CSS utility classes
- React Router v6

**Required Tools**:
- Node.js 18+ and npm 9+
- Git
- Code editor (VS Code recommended)

**Recommended Reading**:
- [spec.md](../spec.md) - Feature specification
- [research.md](../research.md) - Design decisions
- [data-model.md](../data-model.md) - Component architecture
- [contracts/component-api.md](../contracts/component-api.md) - API contracts

## Setup

### 1. Install Dependencies

```bash
cd frontend

# Install Tailwind CSS and dependencies
npm install -D tailwindcss@^3.4 postcss@^8.4 autoprefixer@^10.4

# Initialize Tailwind config
npx tailwindcss init -p
```

### 2. Configure Tailwind CSS

**File**: `frontend/tailwind.config.js`

```javascript
/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: {
          50: '#eff6ff',
          100: '#dbeafe',
          500: '#3b82f6',
          600: '#2563eb',
          700: '#1d4ed8',
        },
      },
      animation: {
        'fade-in': 'fadeIn 0.3s ease-in-out',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
      },
    },
  },
  plugins: [],
}
```

**File**: `frontend/postcss.config.js`

```javascript
export default {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
}
```

### 3. Update CSS Entry Point

**File**: `frontend/src/index.css`

```css
/* Tailwind directives */
@tailwind base;
@tailwind components;
@tailwind utilities;

/* Custom base styles */
@layer base {
  body {
    @apply bg-gray-50 text-gray-900;
  }
  
  h1 {
    @apply text-3xl font-bold;
  }
  
  h2 {
    @apply text-2xl font-semibold;
  }
  
  h3 {
    @apply text-lg font-semibold;
  }
}

/* Custom components */
@layer components {
  .btn-primary {
    @apply px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors;
  }
  
  .btn-secondary {
    @apply px-4 py-2 bg-white text-blue-600 border border-blue-600 rounded-md hover:bg-blue-50 transition-colors;
  }
  
  .card {
    @apply bg-white rounded-lg shadow-md hover:shadow-lg transition-shadow;
  }
}

/* Existing styles below (preserve) */
```

### 4. Verify Setup

```bash
# Start dev server
npm run dev

# Open http://localhost:5173
# Tailwind classes should now work
```

## Development Workflow

### Phase 1: Create New Components

#### Step 1: Create NavBar Component

**File**: `frontend/src/components/common/NavBar.tsx`

```tsx
import React from 'react';
import { Link } from 'react-router-dom';
import { SearchBar } from './SearchBar';
import { ScanButton } from '../scan/ScanButton';

export function NavBar() {
  return (
    <nav className="bg-white shadow-md sticky top-0 z-50">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Desktop Layout */}
        <div className="hidden md:flex items-center justify-between h-16">
          <Link to="/" className="text-xl font-bold text-gray-900 hover:text-blue-600">
            Glyptotheka
          </Link>
          
          <div className="flex-1 max-w-2xl mx-8">
            <SearchBar />
          </div>
          
          <div className="flex items-center gap-4">
            <Link to="/browse" className="text-gray-600 hover:text-blue-600">
              Browse
            </Link>
            <ScanButton />
          </div>
        </div>
        
        {/* Mobile Layout */}
        <div className="md:hidden py-3 space-y-3">
          <div className="flex items-center justify-between">
            <Link to="/" className="text-lg font-bold text-gray-900">
              Glyptotheka
            </Link>
            <div className="flex gap-3">
              <Link to="/browse" className="text-sm text-gray-600">
                Browse
              </Link>
              <ScanButton size="sm" />
            </div>
          </div>
          <SearchBar />
        </div>
      </div>
    </nav>
  );
}
```

#### Step 2: Create ScanButton Component

**File**: `frontend/src/components/scan/ScanButton.tsx`

```tsx
import React, { useState } from 'react';
import { scanAPI } from '../../api/scan';

export interface ScanButtonProps {
  variant?: 'primary' | 'secondary';
  size?: 'sm' | 'md' | 'lg';
  onScanStart?: () => void;
  onScanComplete?: () => void;
  className?: string;
}

export function ScanButton({
  variant = 'primary',
  size = 'md',
  onScanStart,
  onScanComplete,
  className = '',
}: ScanButtonProps) {
  const [isScanning, setIsScanning] = useState(false);
  
  const handleClick = async () => {
    if (isScanning) return;
    
    setIsScanning(true);
    onScanStart?.();
    
    try {
      await scanAPI.start();
      onScanComplete?.();
    } catch (error) {
      console.error('Scan failed:', error);
    } finally {
      setIsScanning(false);
    }
  };
  
  const sizeClasses = {
    sm: 'px-3 py-1.5 text-sm',
    md: 'px-4 py-2 text-base',
    lg: 'px-6 py-3 text-lg',
  };
  
  const variantClasses = {
    primary: isScanning 
      ? 'bg-gray-300 cursor-not-allowed' 
      : 'bg-blue-600 hover:bg-blue-700 text-white',
    secondary: isScanning
      ? 'bg-gray-100 border-gray-300 cursor-not-allowed'
      : 'bg-white border-blue-600 text-blue-600 hover:bg-blue-50',
  };
  
  return (
    <button
      onClick={handleClick}
      disabled={isScanning}
      className={`
        flex items-center gap-2 rounded-md font-medium transition-colors
        ${sizeClasses[size]}
        ${variantClasses[variant]}
        ${className}
      `}
    >
      {isScanning ? (
        <>
          <svg className="animate-spin h-5 w-5" viewBox="0 0 24 24">
            <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" fill="none" />
            <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
          </svg>
          <span>Scanning...</span>
        </>
      ) : (
        <>
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
          </svg>
          <span>Rescan</span>
        </>
      )}
    </button>
  );
}
```

### Phase 2: Refactor Existing Components

#### Step 3: Update ProjectTile

**File**: `frontend/src/components/project/ProjectTile.tsx`

```tsx
import React from 'react';
import { Project } from '../../types/project';

export interface TileMetadata {
  fileCount: number;
  totalSize?: string;
  previewImage?: string;
  icon: 'folder' | 'project';
}

export interface ProjectTileProps {
  project: Project;
  metadata: TileMetadata;
  onClick: () => void;
  loading?: boolean;
  lazyLoad?: boolean;
}

export function ProjectTile({
  project,
  metadata,
  onClick,
  loading = false,
  lazyLoad = true,
}: ProjectTileProps) {
  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      onClick();
    }
  };
  
  if (loading) {
    return <SkeletonTile />;
  }
  
  return (
    <div
      role="button"
      tabIndex={0}
      onClick={onClick}
      onKeyDown={handleKeyDown}
      className="group card cursor-pointer overflow-hidden focus:outline-none focus:ring-2 focus:ring-blue-500"
      aria-label={`${project.name}, ${metadata.icon === 'folder' ? 'Folder' : 'Project'}`}
    >
      {/* Preview Image */}
      <div className="relative h-48 bg-gray-100">
        {metadata.previewImage ? (
          <img
            src={metadata.previewImage}
            alt={project.name}
            loading={lazyLoad ? 'lazy' : undefined}
            className="w-full h-full object-cover"
          />
        ) : (
          <div className="flex items-center justify-center h-full">
            {metadata.icon === 'folder' ? (
              <svg className="w-16 h-16 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
              </svg>
            ) : (
              <svg className="w-16 h-16 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
              </svg>
            )}
          </div>
        )}
        <div className="absolute inset-0 bg-black bg-opacity-0 group-hover:bg-opacity-10 transition-opacity" />
      </div>
      
      {/* Content */}
      <div className="p-4">
        <h3 className="text-lg font-semibold text-gray-900 mb-2 line-clamp-2">
          {project.name}
        </h3>
        
        <div className="flex items-center gap-4 text-sm text-gray-600 mb-3">
          <span className="flex items-center gap-1">
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z" />
            </svg>
            {metadata.fileCount} file{metadata.fileCount !== 1 ? 's' : ''}
          </span>
          {metadata.totalSize && (
            <span>{metadata.totalSize}</span>
          )}
        </div>
        
        <span className={`
          inline-block px-2 py-1 text-xs rounded-full
          ${metadata.icon === 'folder' ? 'bg-blue-100 text-blue-700' : 'bg-green-100 text-green-700'}
        `}>
          {metadata.icon === 'folder' ? 'Folder' : 'Project'}
        </span>
      </div>
    </div>
  );
}

function SkeletonTile() {
  return (
    <div className="card overflow-hidden animate-pulse">
      <div className="h-48 bg-gray-200" />
      <div className="p-4 space-y-3">
        <div className="h-4 bg-gray-200 rounded w-3/4" />
        <div className="h-3 bg-gray-200 rounded w-1/2" />
        <div className="h-5 bg-gray-200 rounded w-16" />
      </div>
    </div>
  );
}
```

#### Step 4: Update BrowsePage

**File**: `frontend/src/pages/BrowsePage.tsx`

```tsx
import React, { useState, useEffect, useMemo } from 'react';
import { useNavigate } from 'react-router-dom';
import { projectsAPI } from '../api/projects';
import { Project } from '../types/project';
import { ProjectGrid } from '../components/project/ProjectGrid';
import { Breadcrumb, BreadcrumbItem } from '../components/common/Breadcrumb';
import LoadingSpinner from '../components/common/LoadingSpinner';

export default function BrowsePage() {
  const [projects, setProjects] = useState<Project[]>([]);
  const [currentFolderId, setCurrentFolderId] = useState<number | null>(null);
  const [breadcrumbs, setBreadcrumbs] = useState<BreadcrumbItem[]>([
    { id: null, name: "All Projects" }
  ]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  
  const navigate = useNavigate();
  
  useEffect(() => {
    loadProjects();
  }, []);
  
  const loadProjects = async () => {
    try {
      setLoading(true);
      const data = await projectsAPI.listRoot();
      setProjects(data);
    } catch (err) {
      console.error('Failed to load projects:', err);
      setError('Failed to load projects');
    } finally {
      setLoading(false);
    }
  };
  
  const visibleProjects = useMemo(() => {
    return projects.filter(p => p.parent_id === currentFolderId);
  }, [projects, currentFolderId]);
  
  const handleTileClick = (projectId: number) => {
    const project = projects.find(p => p.id === projectId);
    if (!project) return;
    
    if (project.is_leaf) {
      navigate(`/project/${projectId}`);
    } else {
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
  
  if (loading) return <LoadingSpinner />;
  if (error) return <div className="text-red-600">{error}</div>;
  
  return (
    <div className="container mx-auto px-4 py-8">
      <h1 className="text-3xl font-bold mb-6">Browse Projects</h1>
      
      {breadcrumbs.length > 1 && (
        <Breadcrumb 
          items={breadcrumbs}
          onNavigate={handleBreadcrumbClick}
        />
      )}
      
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

### Phase 3: Update App.tsx

**File**: `frontend/src/App.tsx`

```tsx
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { NavigationProvider } from './store/navigationContext';
import { SearchProvider } from './store/searchContext';
import { ToastProvider } from './components/common/Toast';
import { ErrorBoundary } from './components/ErrorBoundary';
import { NavBar } from './components/common/NavBar';  // NEW
import HomePage from './pages/HomePage';
import BrowsePage from './pages/BrowsePage';
import ProjectPage from './pages/ProjectPage';
import { SearchPage } from './pages/SearchPage';
import './index.css';

function App() {
  return (
    <ErrorBoundary>
      <Router>
        <ToastProvider>
          <NavigationProvider>
            <SearchProvider>
              <div className="app min-h-screen bg-gray-50">
                <NavBar />  {/* NEW - replaces old header */}
                
                <main className="app-main px-3 sm:px-4">
                  <Routes>
                    <Route path="/" element={<HomePage />} />
                    <Route path="/browse" element={<BrowsePage />} />
                    <Route path="/project/:id" element={<ProjectPage />} />
                    <Route path="/projects/:id" element={<ProjectPage />} />
                    <Route path="/search" element={<SearchPage />} />
                  </Routes>
                </main>
              </div>
            </SearchProvider>
          </NavigationProvider>
        </ToastProvider>
      </Router>
    </ErrorBoundary>
  );
}

export default App;
```

## Testing

### Run Unit Tests

```bash
npm run test
```

**Expected Output**:
```
✓ NavBar renders (X ms)
✓ ScanButton triggers scan (X ms)
✓ ProjectTile displays metadata (X ms)
✓ ProjectGrid handles navigation (X ms)
✓ BrowsePage filters by folder (X ms)
...
```

### Manual UI Testing

```bash
npm run dev
```

**Test Checklist**:
- [ ] Home page displays correctly
- [ ] Browse page shows tiles in grid
- [ ] Click folder tile → navigate to children
- [ ] Click project tile → navigate to detail
- [ ] Breadcrumb navigation works
- [ ] Rescan button appears in top-right nav
- [ ] Search bar works
- [ ] Responsive layout (mobile/tablet/desktop)
- [ ] Keyboard navigation (Tab, arrows, Enter)
- [ ] Hover effects on tiles
- [ ] Loading states display

### Validate with chrome-devtools-mcp

```bash
# See test-strategy.md for chrome-devtools validation commands
```

## Common Issues

### Issue 1: Tailwind classes not applying

**Problem**: Classes like `bg-blue-600` have no effect

**Solution**:
1. Verify `tailwind.config.js` content array includes `./src/**/*.{tsx,ts}`
2. Check `index.css` has `@tailwind` directives
3. Restart dev server (`npm run dev`)

### Issue 2: Images not lazy loading

**Problem**: All images load immediately

**Solution**:
- Check browser support for `loading="lazy"` (96%+ support)
- Ensure `lazyLoad` prop is not set to `false`
- Use Intersection Observer fallback for older browsers

### Issue 3: Keyboard navigation not working

**Problem**: Arrow keys don't navigate grid

**Solution**:
- Verify `ProjectGrid` has `role="grid"`
- Check `onKeyDown` handler is attached
- Ensure roving tabindex is implemented

## Next Steps

1. **Review**:
   - Read [spec.md](../spec.md) for full requirements
   - Check [contracts/component-api.md](../contracts/component-api.md) for API details

2. **Implement**:
   - Follow phase order (NavBar → Tiles → Pages)
   - Test each component before moving on
   - Validate with chrome-devtools-mcp

3. **Polish**:
   - Add animations (fade-in, hover effects)
   - Optimize images (WebP format)
   - Add loading skeletons
   - Test accessibility (keyboard, screen reader)

4. **Deploy**:
   - Build production: `npm run build`
   - Test build: `npm run preview`
   - Deploy to production

## Resources

**Documentation**:
- [Tailwind CSS Docs](https://tailwindcss.com/docs)
- [React Router Docs](https://reactrouter.com/en/main)
- [Vitest Docs](https://vitest.dev)

**Design References**:
- [Tailwind UI Components](https://tailwindui.com/components)
- [Material Design](https://m3.material.io)

**Internal Docs**:
- [research.md](../research.md) - Design decisions
- [data-model.md](../data-model.md) - Component architecture
- [contracts/component-api.md](../contracts/component-api.md) - API contracts

---

**Guide Version**: 1.0.0  
**Last Updated**: 2025-11-17  
**Need Help?** Check existing components for examples or refer to contracts
