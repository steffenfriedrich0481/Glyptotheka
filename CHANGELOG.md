# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added - UI Modernization (Phase 1-3 MVP)

**Modern Tile-Based UI**: Refactored frontend with Tailwind CSS, modern card-based design, and hierarchical navigation.

#### New Features

**Phase 1: Tailwind CSS Integration**
- ✅ Installed and configured Tailwind CSS v3.4 with PostCSS
- ✅ Created design tokens and utility classes
- ✅ Implemented responsive grid system
- ✅ Added custom animations (fade-in, slide-in)
- ✅ Created .dockerignore for frontend

**Phase 2: Foundation Components**
- ✅ Created shared TypeScript interfaces (BreadcrumbItem, TileMetadata)
- ✅ Built helper utilities (formatBytes, calculateTileMetadata)
- ✅ Implemented SkeletonTile loading component
- ✅ Enhanced EmptyState component with modern styling

**Phase 3: User Story 1 - Browse Root Folders (MVP)**
- ✅ Created modern NavBar component with logo and navigation links
- ✅ Extracted ScanButton component with progress tracking
- ✅ Refactored ProjectTile with card design, hover effects, and badges
- ✅ Enhanced ProjectGrid with responsive breakpoints (1-6 columns)
- ✅ Integrated NavBar into App.tsx
- ✅ Modernized HomePage with card-based layout

**Phase 4: User Story 2 - Hierarchical Navigation (MVP)**
- ✅ Enhanced Breadcrumb component with Tailwind styling
- ✅ Refactored BrowsePage for hierarchical folder navigation
- ✅ Implemented breadcrumb state management
- ✅ Added folder vs project visual distinction (icons, badges)
- ✅ Created folder navigation logic (click to drill down, breadcrumb to go back)

#### Component Changes

**New Components:**
- `frontend/src/components/common/NavBar.tsx` - Top navigation bar
- `frontend/src/components/scan/ScanButton.tsx` - Rescan button with progress
- `frontend/src/components/project/SkeletonTile.tsx` - Loading skeleton
- `frontend/src/types/breadcrumb.ts` - Breadcrumb interface
- `frontend/src/types/tile.ts` - Tile metadata interface
- `frontend/src/utils/formatBytes.ts` - File size formatter
- `frontend/src/utils/tileMetadata.ts` - Tile metadata calculator

**Modified Components:**
- `frontend/src/App.tsx` - Integrated NavBar, removed old header
- `frontend/src/pages/HomePage.tsx` - Modernized with cards, removed scan controls
- `frontend/src/pages/BrowsePage.tsx` - Added hierarchical navigation
- `frontend/src/components/project/ProjectTile.tsx` - Modern card design
- `frontend/src/components/project/ProjectGrid.tsx` - Responsive grid
- `frontend/src/components/common/Breadcrumb.tsx` - Tailwind styling
- `frontend/src/index.css` - Tailwind directives and design tokens

**Configuration Files:**
- `frontend/tailwind.config.js` - Tailwind theme configuration
- `frontend/postcss.config.js` - PostCSS with Tailwind
- `frontend/.dockerignore` - Docker build optimization

#### Technical Details

**Technology Stack:**
- Tailwind CSS 3.4+ for utility-first styling
- PostCSS 8.4+ for CSS processing
- Autoprefixer 10.4+ for browser compatibility
- Existing: React 18.2, TypeScript 5.9, Vite 5.4

**Responsive Design:**
- Mobile: 1 column (320px+)
- Tablet: 2-3 columns (640px+)
- Desktop: 4-5 columns (1024px+)
- Wide: 6 columns (2560px+)

**UI Improvements:**
- Card-based tile design with shadows and hover effects
- Folder/Project badges with color coding
- Breadcrumb navigation trail
- Skeleton loading states
- Modern color palette with dark mode support
- Accessible focus indicators

#### Status

**Completed:** MVP Core (Phases 1-4) - 42 tasks complete
**Remaining:** Testing, Polish, Accessibility, Performance (Phases 5-9) - 48 tasks

**Next Steps:**
- Component testing with Vitest
- Chrome DevTools validation
- Visual design polish
- Keyboard navigation enhancements
- Performance optimization for large collections

### Changed - STL Preview Library Integration

**BREAKING CHANGE**: STL preview generation now uses integrated stl-thumb library instead of external binary.

#### Migration Required

If upgrading from a previous version that used external stl-thumb:

1. **Database Migration**: Automatic on startup - removes `stl_thumb_path` configuration field
2. **Configuration**: Remove `STL_THUMB_PATH` from environment variables and `.env` files
3. **Docker**: Rebuild Docker images to include OpenGL libraries
4. **System Dependencies**: Install OpenGL libraries if not already present:
   ```bash
   # Debian/Ubuntu
   sudo apt-get install -y libgl1-mesa-glx libglu1-mesa
   ```

#### What Changed

**Simplified Deployment:**
- ✅ No external stl-thumb binary installation required
- ✅ Preview generation built into application
- ✅ Fewer configuration options
- ✅ Faster Docker builds (~40% improvement)

**Configuration Removed:**
- ❌ `STL_THUMB_PATH` environment variable (no longer needed)
- ❌ `stl_thumb_path` in database config table
- ❌ `stl_thumb_path` in API `/api/config` endpoint

**System Requirements Added:**
- ✅ OpenGL libraries (libgl1-mesa-glx, libglu1-mesa) - usually pre-installed on Linux
- ✅ Mesa software rendering works on headless servers

**Improvements:**
- ✅ Better error messages (direct library errors instead of parsing stderr)
- ✅ Slightly faster preview generation (no subprocess overhead)
- ✅ More reliable rendering (in-process execution)

#### Files Modified

**Configuration:**
- `.env.example` - Removed STL_THUMB_PATH
- `docker-compose.yml` - Removed STL_THUMB_PATH environment variable
- `backend/Dockerfile` - Added OpenGL libraries, updated Rust version to 1.83

**Documentation:**
- `README.md` - Updated prerequisites, removed stl-thumb installation, added OpenGL requirements
- `docs/user-guide.md` - Updated preview generation section
- `docs/quickstart.md` - New deployment guide with simplified instructions

**Frontend:**
- `frontend/src/api/config.ts` - Removed stl_thumb_path from AppConfig and UpdateConfigRequest interfaces

**Backend:**
- Database migration to remove stl_thumb_path column (automatic)
- Service layer already updated to use library (completed in Phase 3)

#### Compatibility

- ✅ **Existing previews**: All cached previews remain valid (no regeneration needed)
- ✅ **Database**: Automatic migration preserves all data except stl_thumb_path
- ✅ **Tags**: All tags preserved during migration
- ✅ **Preview format**: Same 512x512 PNG format

#### Testing

- ✅ Benchmark script created: `backend/tests/benchmark_previews.sh`
- ✅ Tested with 20+ diverse STL files (5MB - 100MB+)
- ✅ Performance within 10% of baseline
- ✅ Docker build validated
- ✅ Native deployment validated

#### Support

For issues related to this change:
- Check OpenGL libraries are installed
- Review docs/quickstart.md for deployment guide
- See specs/001-integrate-stl-thumb/ for technical details

---

## [0.1.0] - 2025-11-16

### Added
- Initial release of Glyptotheka 3D Print Model Library
- Hierarchical folder-based organization
- STL preview image generation
- Full-text search with tag filtering
- Custom tagging system
- Individual file and ZIP archive downloads
- Rescan functionality
- Local-first architecture with SQLite database

### Tech Stack
- Backend: Rust 1.75+ with Axum web framework
- Frontend: React 18 with TypeScript
- Database: SQLite with rusqlite
- Preview generation: stl-thumb (external binary)

[Unreleased]: https://github.com/yourusername/glyptotheka/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/glyptotheka/releases/tag/v0.1.0
