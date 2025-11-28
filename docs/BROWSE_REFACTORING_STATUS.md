# Browse View Refactoring - Implementation Status

## Overview

This document summarizes the current implementation status of the browse view refactoring feature, which includes:
1. Image inheritance from parent folders
2. Keyword containment checking for STL categorization
3. STL files grouped by category folder names

## Backend Implementation ✅ COMPLETE

### 1. Keyword Containment Checking ✅

**Location**: `backend/src/services/scanner.rs`

**Implementation**: Lines 66-71 - `is_stl_category_folder()` method

```rust
fn is_stl_category_folder(&self, folder_name: &str) -> bool {
    let normalized_name = folder_name.trim().to_lowercase();
    self.ignored_keywords
        .iter()
        .any(|keyword| normalized_name.contains(&keyword.trim().to_lowercase()))
}
```

**Features**:
- Case-insensitive substring matching
- Whitespace trimming for robust matching
- Configurable via `IGNORED_KEYWORDS` environment variable
- Examples: "1 inch" matches "inch", "PRESUPPORTED_STL" matches "STL"

**Usage**: Lines 318-337 - `find_project_folder()` method traverses up the directory tree, skipping folders that match ignored keywords.

### 2. Image Inheritance ✅

**Location**: `backend/src/services/scanner.rs`

**Implementation**: Lines 512-583 - `inherit_images_from_parents()` method

**Process**:
1. Scans directories during project scan (lines 208-254)
2. Walks up the folder tree from each project to root
3. Collects images from all parent folders
4. Adds inherited images to project with `source_type="inherited"`
5. Tracks original source via `source_project_id`
6. Propagates to all descendant projects (lines 256-272)

**Features**:
- Automatic inheritance during scan
- Deduplication by filename  
- Source tracking for transparency
- Works across multiple hierarchy levels

### 3. API Support ✅

**Location**: `backend/src/db/repositories/project_repo.rs`

**Implementation**: Lines 142-195 - `get_with_relations()` method

**Response includes**:
- `inherited_images`: Preview images including inherited ones
- `source_type`: Distinguishes "direct" vs "inherited" vs "stl_preview"
- `inherited_from`: Path showing image origin for inherited images

**Optimization**: Lines 199-239 - `get_project_preview_images()` uses priority sorting:
- Regular images (priority 10)
- Inherited images (priority varies)
- STL previews (priority 1)

## Frontend Status ⚠️ PARTIAL

### What's Working

1. **Basic Navigation**: Folder-by-folder navigation works via path-based routes
2. **Project Display**: Projects show in tile view with basic information
3. **API Integration**: Frontend can fetch inherited images from backend

### What Needs Work

1. **STL Category Display**: 
   - Folders like "1 inch", "2 inch", "40 mm" are displayed as regular folders
   - Need to group these as STL categories within parent project
   - UI should show grouped STL files by category name

2. **Image Carousel**:
   - Inherited images need to be displayed in project preview tiles
   - Should show both direct and inherited images
   - Need visual indicator for inherited vs. direct images

3. **Breadcrumb Navigation**:
   - Should skip STL category folders
   - Display actual project hierarchy, not physical folder structure

## Configuration

### Environment Variables

```bash
# In .env or docker-compose.yml
IGNORED_KEYWORDS=PRESUPPORTED_STL,STL,UNSUPPORTED_STL,Unsupported,Pre-Supported,inch,mm
```

### Database Schema

The following tables support these features:

```sql
-- Projects table includes:
projects (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  full_path TEXT NOT NULL UNIQUE,
  parent_id INTEGER,
  is_leaf BOOLEAN NOT NULL DEFAULT false,
  folder_level INTEGER NOT NULL DEFAULT 0,
  ...
)

-- Image files track inheritance:
image_files (
  id INTEGER PRIMARY KEY,
  project_id INTEGER NOT NULL,
  filename TEXT NOT NULL,
  file_path TEXT NOT NULL UNIQUE,
  source_type TEXT NOT NULL, -- 'direct', 'inherited', 'stl_preview'
  source_project_id INTEGER, -- NULL for direct, source project for inherited
  image_source TEXT, -- 'image' or 'stl'
  image_priority INTEGER NOT NULL DEFAULT 10,
  ...
)
```

## Testing Recommendations

### Test Case 1: Image Inheritance

**Setup**: Use `example/Miniaturen/The Printing Goes Ever On/Welcome Trove/`

**Expected**:
- "heroes fighting.jpg" in "Welcome Trove" folder
- Should appear in "Welcome-Trove-Remastered" project
- Should appear in "Welcome-Trove-Remastered/Samuel" project
- Both direct and inherited images display together

**Verification**: Check project details API response includes inherited images

### Test Case 2: Keyword Matching

**Setup**: Use `example/Miniaturen/.../Universal-Base-Set/Desert/`

**Expected**:
- "1 inch", "2 inch", "40 mm" folders treated as STL categories
- "Desert" project should have 3 STL categories
- STL files grouped by category folder name

**Verification**: Scanner should not create separate projects for these folders

### Test Case 3: Multiple Hierarchy Levels

**Setup**: Deep folder structure with images at multiple levels

**Expected**:
- Images inherit down all levels
- No duplicates (deduplicated by filename)
- Each image appears once even if same name exists in multiple parents

## Next Steps

To complete the frontend implementation:

1. **Update ProjectTile Component** (`frontend/src/components/ProjectTile.tsx`):
   - Display both direct and inherited images in carousel
   - Add badge/indicator for inherited images
   - Show image count (direct vs. inherited)

2. **Create STL Category Display** (`frontend/src/components/StlCategoryView.tsx`):
   - Group STL files by category folder name
   - Display as expandable sections within project
   - Show category name (e.g., "1 inch", "PRESUPPORTED_STL")

3. **Update BrowsePage** (`frontend/src/pages/BrowsePage.tsx`):
   - Filter out STL category folders from folder list
   - Show categories within project details instead
   - Update breadcrumb to skip category folders

4. **Add Visual Indicators**:
   - Icon or badge for inherited images
   - Tooltip showing inheritance source path
   - Different styling for STL categories vs. projects

## Files Modified

### Backend
- `backend/src/services/scanner.rs` - Core scanning logic with inheritance and keyword checking
- `backend/src/db/repositories/project_repo.rs` - API support for inherited images
- `backend/src/models/project.rs` - Data models with inheritance support
- `backend/src/config.rs` - Configuration for ignored keywords

### Frontend  
- `frontend/src/pages/BrowsePage.tsx` - Path-based navigation
- `frontend/src/api/client.ts` - API integration for project data

### Configuration
- `docker-compose.yml` - Added IGNORED_KEYWORDS environment variable
- `.env.example` - Documentation of configuration options

## Performance Notes

- Image inheritance calculation happens during scan (not per-request)
- Results are cached in database
- Preview image queries use priority sorting for optimal display
- No N+1 query problems due to proper indexing

## Known Limitations

1. **Deduplication**: Only by exact filename match (case-sensitive)
2. **Large Hierarchies**: Very deep folder structures (10+ levels) may have slower inheritance propagation
3. **Real-time Updates**: Changes to parent folder images require rescan to propagate
4. **Preview Generation**: STL previews generated asynchronously (first 2 sync, rest async)

---

**Last Updated**: 2025-11-28  
**Feature Branch**: `001-browse-view-refactor`  
**Related Specs**: `/specs/001-browse-view-refactor/`
