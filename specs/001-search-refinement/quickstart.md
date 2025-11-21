# Quickstart Guide: Search View Refinement

**Feature**: 001-search-refinement  
**Audience**: Developers implementing this feature  
**Date**: 2025-11-21

## Overview

This guide provides step-by-step instructions for implementing the search view refinement feature. Follow these steps in order for a successful implementation.

---

## Prerequisites

- ✅ Glyptotheka repository cloned and building successfully
- ✅ Backend: Rust 1.75+, Cargo installed
- ✅ Frontend: Node.js 18+, npm installed
- ✅ Development database with sample projects and STL files
- ✅ Familiarity with Axum (backend) and React (frontend)

**Verify Setup**:
```bash
# Backend
cd backend && cargo test
# Should pass all tests

# Frontend
cd frontend && npm test
# Should pass all tests
```

---

## Implementation Phases

### Phase 1: Backend - Search Service Enhancement (Priority: P0)

**Goal**: Add leaf project filtering and image aggregation to search API

#### Step 1.1: Extend SearchParams Structure

**File**: `backend/src/services/search.rs`

**Action**: Add `leaf_only` field to `SearchParams` struct

```rust
#[derive(Debug, Clone)]
pub struct SearchParams {
    pub query: Option<String>,
    pub tags: Vec<String>,
    pub page: usize,
    pub per_page: usize,
    pub leaf_only: bool,  // NEW: Filter to leaf projects
}
```

#### Step 1.2: Add Leaf Filtering to Search Queries

**File**: `backend/src/services/search.rs`

**Action**: Update all search methods to include `WHERE is_leaf = 1` when `leaf_only = true`

**Methods to Update**:
- `search_fts()` - Full-text search
- `search_by_tags()` - Tag filtering
- `search_combined()` - Combined FTS + tags
- `search_all()` - Already has this filter, ensure it respects `leaf_only` parameter

**Example** (for `search_fts`):
```rust
fn search_fts(
    &self,
    conn: &rusqlite::Connection,
    params: &SearchParams,
    offset: usize,
) -> Result<(Vec<Project>, usize), AppError> {
    let search_query = params.query.as_ref().unwrap();
    let fts_query = format!("{}*", search_query);
    
    // Build WHERE clause
    let leaf_filter = if params.leaf_only {
        "AND p.is_leaf = 1"
    } else {
        ""
    };
    
    // Count query
    let count_query = format!(
        "SELECT COUNT(DISTINCT p.id)
         FROM projects p
         INNER JOIN projects_fts fts ON p.id = fts.project_id
         WHERE projects_fts MATCH ?1 {}",
        leaf_filter
    );
    
    // ... rest of method
}
```

**Repeat for other search methods**.

#### Step 1.3: Add Image Aggregation Repository Method

**File**: `backend/src/db/repositories/file_repo.rs`

**Action**: Add method to get aggregated images (inherited + STL previews)

```rust
/// Get aggregated images for a project (inherited from parents + STL previews)
/// Returns up to `limit` images sorted by priority (higher first)
pub fn get_aggregated_images(
    &self,
    project_id: i64,
    limit: i64,
) -> Result<Vec<ImageFile>, AppError> {
    let conn = self.pool.get()?;
    
    // Step 1: Get parent chain using recursive CTE
    let parent_ids_query = "
        WITH RECURSIVE parent_chain AS (
            SELECT id, parent_id, 0 AS depth
            FROM projects WHERE id = ?1
            UNION ALL
            SELECT p.id, p.parent_id, pc.depth + 1
            FROM projects p
            INNER JOIN parent_chain pc ON p.id = pc.parent_id
            WHERE pc.depth < 5
        )
        SELECT id FROM parent_chain
    ";
    
    let mut stmt = conn.prepare(parent_ids_query)?;
    let parent_ids: Vec<i64> = stmt
        .query_map([project_id], |row| row.get(0))?
        .collect::<Result<Vec<_>, _>>()?;
    
    if parent_ids.is_empty() {
        return Ok(Vec::new());
    }
    
    // Step 2: Get images from all projects in chain
    let placeholders = parent_ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
    let images_query = format!(
        "SELECT id, project_id, filename, file_path, file_size, 
                source_type, source_project_id, display_order, 
                image_priority, image_source, created_at, updated_at
         FROM image_files
         WHERE project_id IN ({})
         ORDER BY image_priority DESC, display_order ASC, created_at ASC
         LIMIT ?",
        placeholders
    );
    
    let mut params: Vec<&dyn rusqlite::ToSql> = parent_ids.iter()
        .map(|id| id as &dyn rusqlite::ToSql)
        .collect();
    params.push(&limit);
    
    let mut stmt = conn.prepare(&images_query)?;
    let images = stmt
        .query_map(rusqlite::params_from_iter(params.iter()), |row| {
            Ok(ImageFile {
                id: row.get(0)?,
                project_id: row.get(1)?,
                filename: row.get(2)?,
                file_path: row.get(3)?,
                file_size: row.get(4)?,
                source_type: row.get(5)?,
                source_project_id: row.get(6)?,
                display_order: row.get(7)?,
                image_priority: row.get(8)?,
                image_source: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    
    Ok(images)
}
```

#### Step 1.4: Update Search Handler

**File**: `backend/src/api/handlers/search.rs`

**Action**: 
1. Add `leaf_only` query parameter
2. Include images in response

```rust
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub tags: Option<String>,
    #[serde(default = "default_page")]
    pub page: usize,
    #[serde(default = "default_per_page")]
    pub per_page: usize,
    #[serde(default = "default_leaf_only")]
    pub leaf_only: bool,  // NEW
}

fn default_leaf_only() -> bool {
    true
}

pub async fn search_projects(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<SearchResponse>, (StatusCode, Json<serde_json::Value>)> {
    let tags = /* ... parse tags ... */;

    let params = SearchParams {
        query: query.q,
        tags,
        page: query.page,
        per_page: query.per_page.min(100),
        leaf_only: query.leaf_only,  // NEW
    };

    let result = state.search_service.search(&params).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;

    // NEW: For each project, get aggregated images
    let data: Vec<serde_json::Value> = result
        .projects
        .into_iter()
        .map(|p| {
            let images = state.file_repo
                .get_aggregated_images(p.id, 15)
                .unwrap_or_default();
            
            let stl_count = state.file_repo
                .get_stl_files_by_project(p.id)
                .unwrap_or_default()
                .len();
            
            let tags = state.tag_repo
                .get_tags_by_project(p.id)
                .unwrap_or_default();
            
            serde_json::json!({
                "id": p.id,
                "name": p.name,
                "full_path": p.full_path,
                "parent_id": p.parent_id,
                "is_leaf": p.is_leaf,
                "description": p.description,
                "stl_count": stl_count,
                "image_count": images.len(),
                "images": images.into_iter().map(|img| {
                    serde_json::json!({
                        "id": img.id,
                        "filename": img.filename,
                        "image_source": img.image_source,
                        "source_type": img.source_type,
                        "source_project_id": img.source_project_id,
                        "image_priority": img.image_priority,
                    })
                }).collect::<Vec<_>>(),
                "tags": tags,
                "created_at": p.created_at,
                "updated_at": p.updated_at,
            })
        })
        .collect();

    Ok(Json(SearchResponse {
        data,
        meta: SearchMeta {
            total: result.total,
            page: result.page,
            per_page: result.per_page,
            total_pages: result.total_pages,
        },
    }))
}
```

**Test Backend**:
```bash
cd backend
cargo test
cargo run

# In another terminal, test API:
curl "http://localhost:8000/api/search?leaf_only=true&per_page=5" | jq
```

---

### Phase 2: Frontend - API Client Update (Priority: P0)

**Goal**: Update API client to support new parameters and response structure

#### Step 2.1: Update TypeScript Types

**File**: `frontend/src/types/project.ts`

**Action**: Add new types for search results with images

```typescript
// NEW: Image preview for search carousels
export interface ImagePreview {
  id: number;
  filename: string;
  image_source: 'regular' | 'stl_preview' | 'composite';
  source_type: 'direct' | 'inherited';
  source_project_id: number | null;
  image_priority: number;
}

// NEW: Extended project for search results
export interface SearchResultProject extends Project {
  stl_count: number;
  image_count: number;
  images: ImagePreview[];
  tags: Tag[];
}
```

#### Step 2.2: Update Search API Client

**File**: `frontend/src/api/projects.ts`

**Action**: Add `leaf_only` parameter and update response type

```typescript
export interface SearchParams {
  q?: string;
  tags?: string;
  page?: number;
  per_page?: number;
  leaf_only?: boolean;  // NEW
}

export async function searchProjects(
  params: SearchParams
): Promise<{ data: SearchResultProject[]; meta: SearchMeta }> {
  const queryParams = new URLSearchParams();
  
  if (params.q) queryParams.set('q', params.q);
  if (params.tags) queryParams.set('tags', params.tags);
  if (params.page) queryParams.set('page', params.page.toString());
  if (params.per_page) queryParams.set('per_page', params.per_page.toString());
  if (params.leaf_only !== undefined) {
    queryParams.set('leaf_only', params.leaf_only.toString());
  }
  
  const response = await fetch(`/api/search?${queryParams}`);
  if (!response.ok) {
    throw new Error(`Search failed: ${response.statusText}`);
  }
  
  return response.json();
}
```

---

### Phase 3: Frontend - Carousel Component (Priority: P1)

**Goal**: Create compact carousel component for search result tiles

#### Step 3.1: Create SearchTileCarousel Component

**File**: `frontend/src/components/project/SearchTileCarousel.tsx`

```typescript
import React, { useState, useEffect, useRef } from 'react';
import { ImagePreview } from '../../types/project';

interface Props {
  images: ImagePreview[];
  projectId: number;
  autoAdvance?: boolean;
}

const SearchTileCarousel: React.FC<Props> = ({ 
  images, 
  projectId,
  autoAdvance = false 
}) => {
  const [currentIndex, setCurrentIndex] = useState(0);
  const [isHovered, setIsHovered] = useState(false);
  const [isLoaded, setIsLoaded] = useState(false);
  const intervalRef = useRef<NodeJS.Timeout>();

  // Handle no images case
  if (images.length === 0) {
    return (
      <div className="w-full h-48 bg-gray-100 flex items-center justify-center">
        <div className="text-center text-gray-400">
          <svg className="w-16 h-16 mx-auto mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
          </svg>
          <p className="text-sm">No images available</p>
        </div>
      </div>
    );
  }

  // Auto-advance logic
  useEffect(() => {
    if (!autoAdvance || isHovered || images.length <= 1) {
      return;
    }

    const advanceDelay = 3000 + Math.random() * 2000; // 3-5 seconds
    intervalRef.current = setInterval(() => {
      setCurrentIndex((prev) => (prev + 1) % images.length);
    }, advanceDelay);

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, [autoAdvance, isHovered, images.length]);

  const goToPrevious = () => {
    setCurrentIndex((prev) => (prev === 0 ? images.length - 1 : prev - 1));
  };

  const goToNext = () => {
    setCurrentIndex((prev) => (prev + 1) % images.length);
  };

  const currentImage = images[currentIndex];
  const imageUrl = `/api/files/images/${currentImage.id}`;

  return (
    <div 
      className="relative w-full h-48 bg-gray-800 overflow-hidden group"
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
    >
      {/* Image */}
      <img
        src={imageUrl}
        alt={currentImage.filename}
        className={`w-full h-full object-cover transition-opacity duration-300 ${
          isLoaded ? 'opacity-100' : 'opacity-0'
        }`}
        loading="lazy"
        onLoad={() => setIsLoaded(true)}
      />
      
      {/* Loading skeleton */}
      {!isLoaded && (
        <div className="absolute inset-0 bg-gray-700 animate-pulse" />
      )}

      {/* Navigation arrows (show on hover, only if multiple images) */}
      {images.length > 1 && (
        <>
          <button
            onClick={goToPrevious}
            className="absolute left-2 top-1/2 -translate-y-1/2 bg-black/50 hover:bg-black/75 text-white p-2 rounded-full opacity-0 group-hover:opacity-100 transition-opacity"
            aria-label="Previous image"
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
            </svg>
          </button>

          <button
            onClick={goToNext}
            className="absolute right-2 top-1/2 -translate-y-1/2 bg-black/50 hover:bg-black/75 text-white p-2 rounded-full opacity-0 group-hover:opacity-100 transition-opacity"
            aria-label="Next image"
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
            </svg>
          </button>

          {/* Dot indicators */}
          <div className="absolute bottom-2 left-1/2 -translate-x-1/2 flex gap-1.5">
            {images.map((_, index) => (
              <button
                key={index}
                onClick={() => setCurrentIndex(index)}
                className={`w-2 h-2 rounded-full transition-all ${
                  index === currentIndex
                    ? 'bg-white scale-125'
                    : 'bg-white/50 hover:bg-white/75'
                }`}
                aria-label={`Go to image ${index + 1}`}
              />
            ))}
          </div>
        </>
      )}

      {/* Image source badge */}
      {currentImage.image_source === 'stl_preview' && (
        <div className="absolute top-2 left-2 bg-blue-600/90 text-white text-xs px-2 py-1 rounded">
          STL Preview
        </div>
      )}
      
      {currentImage.source_type === 'inherited' && (
        <div className="absolute top-2 right-2 bg-green-600/90 text-white text-xs px-2 py-1 rounded">
          Inherited
        </div>
      )}
    </div>
  );
};

export default React.memo(SearchTileCarousel);
```

#### Step 3.2: Update ProjectTile Component

**File**: `frontend/src/components/project/ProjectTile.tsx`

**Action**: Replace static icon with SearchTileCarousel

```typescript
import SearchTileCarousel from './SearchTileCarousel';
import { SearchResultProject } from '../../types/project';

interface Props {
  project: SearchResultProject;  // Use SearchResultProject type
  onClick: () => void;
}

const ProjectTile: React.FC<Props> = ({ project, onClick }) => {
  return (
    <div
      className="project-tile card cursor-pointer transition-all duration-200 overflow-hidden"
      onClick={onClick}
      // ... other props
    >
      {/* Replace static icon with carousel */}
      <SearchTileCarousel 
        images={project.images}
        projectId={project.id}
        autoAdvance={true}
      />

      {/* Content */}
      <div className="p-4 space-y-3">
        <h3 className="text-base font-semibold text-gray-900 line-clamp-2" title={project.name}>
          {project.name}
        </h3>
        
        <div className="flex items-center justify-between text-xs text-gray-500">
          {/* STL count */}
          <div className="flex items-center gap-1">
            <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
              <path d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4"/>
            </svg>
            <span>{project.stl_count} STL{project.stl_count !== 1 ? 's' : ''}</span>
          </div>
          
          {/* Image count */}
          {project.image_count > 0 && (
            <div className="flex items-center gap-1">
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
              </svg>
              <span>{project.image_count > 15 ? '15+' : project.image_count} {project.image_count === 1 ? 'image' : 'images'}</span>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default React.memo(ProjectTile);
```

#### Step 3.3: Update SearchPage Component

**File**: `frontend/src/pages/SearchPage.tsx`

**Action**: Pass `leaf_only=true` to search API

```typescript
const { data, isLoading, error } = useQuery({
  queryKey: ['search', searchTerm, selectedTags, page],
  queryFn: () => searchProjects({
    q: searchTerm || undefined,
    tags: selectedTags.length > 0 ? selectedTags.join(',') : undefined,
    page,
    per_page: 20,
    leaf_only: true,  // NEW: Filter to leaf projects
  }),
  keepPreviousData: true,
});
```

**Test Frontend**:
```bash
cd frontend
npm run dev

# Open browser to http://localhost:5173
# Search for projects and verify carousels appear
```

---

## Testing Checklist

### Backend Tests

```bash
cd backend
cargo test services::search  # Test search filtering
cargo test repositories::file_repo  # Test image aggregation
```

### Frontend Tests

```bash
cd frontend
npm test SearchTileCarousel  # Test carousel component
npm test ProjectTile  # Test tile with carousel
```

### Manual Testing

- [ ] Search with no query returns all leaf projects
- [ ] Search with query filters correctly and shows only leaf projects
- [ ] Projects with images show carousel
- [ ] Projects without images show placeholder
- [ ] Carousel navigation works (prev/next, dots)
- [ ] Auto-advance works when enabled
- [ ] Inherited images are labeled correctly
- [ ] STL preview images are labeled correctly
- [ ] Images load lazily as tiles scroll into view
- [ ] Empty search results show appropriate message

---

## Performance Validation

### Database Query Performance

```sql
-- Test leaf filter performance
EXPLAIN QUERY PLAN
SELECT * FROM projects WHERE is_leaf = 1;
-- Should use idx_projects_leaf

-- Test image aggregation performance
EXPLAIN QUERY PLAN
WITH RECURSIVE parent_chain AS (...)
SELECT * FROM image_files WHERE project_id IN (SELECT id FROM parent_chain)
ORDER BY image_priority DESC LIMIT 15;
-- Should use idx_image_files_priority
```

### Frontend Performance

```bash
# Lighthouse audit
npm run build
npx serve -s dist
# Open Chrome DevTools > Lighthouse
# Run audit on search page
# Target: Performance score > 90
```

---

## Troubleshooting

### Issue: Images not loading

**Solution**: Check image API endpoint and CORS settings

```bash
curl http://localhost:8000/api/files/images/1
# Should return image data
```

### Issue: Leaf filtering not working

**Solution**: Verify `is_leaf` flag is set correctly in database

```bash
sqlite3 data/glyptotheka.db
SELECT id, name, is_leaf FROM projects WHERE is_leaf = 1 LIMIT 5;
```

### Issue: Carousel performance is slow

**Solution**: 
- Enable lazy loading: `<img loading="lazy" />`
- Reduce auto-advance frequency
- Check browser DevTools > Performance tab

---

## Next Steps

1. ✅ Complete Phase 1-3 implementation
2. Run full test suite
3. Performance testing with large dataset (10,000+ projects)
4. User acceptance testing
5. Deploy to staging environment
6. Monitor performance metrics
7. Deploy to production

---

## Support

- **Documentation**: See `data-model.md` and `contracts/search-api.yaml`
- **API Spec**: OpenAPI spec in `contracts/search-api.yaml`
- **Research**: See `research.md` for architectural decisions
