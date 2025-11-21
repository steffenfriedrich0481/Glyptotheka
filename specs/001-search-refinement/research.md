# Research: Search View Refinement

**Feature**: 001-search-refinement  
**Date**: 2025-11-21  
**Status**: Complete

## Overview

This document consolidates research findings for implementing the search view refinement feature. All unknowns from the Technical Context have been investigated, and best practices for each technology choice have been identified.

## 1. Leaf Project Filtering Strategy

### Decision: Server-side filtering using existing `is_leaf` flag

**Rationale**:
- The `is_leaf` boolean flag already exists in the `projects` table and is properly maintained by the scanner service
- Server-side filtering provides better performance by reducing data transfer
- The search service already filters by `is_leaf = 1` in the `search_all()` method when no query is provided
- Need to extend this filtering to all search methods (FTS, tags, combined)

**Alternatives Considered**:
- **Client-side filtering**: Would require transferring all results then filtering, wasting bandwidth and causing slow rendering
- **Virtual column/computed field**: Unnecessary complexity when `is_leaf` is already computed during scanning
- **Separate leaf projects table**: Over-engineering, would duplicate data and require complex sync logic

**Implementation Pattern** (from existing codebase):
```rust
// Already used in search_all() method:
let total: usize = conn.query_row(
    "SELECT COUNT(*) FROM projects WHERE is_leaf = 1",
    [],
    |row| row.get(0),
)?;
```

**Action**: Add `WHERE is_leaf = 1` clause to all search queries in `backend/src/services/search.rs`

---

## 2. Image Aggregation Architecture

### Decision: Hierarchical query with UNION for inherited + STL preview images

**Rationale**:
- Images are stored in `image_files` table with `source_type` field ('direct' or 'inherited')
- STL preview images are marked with `image_source = 'stl_preview'` and `image_priority = 50`
- Regular images have `image_priority = 100` (higher priority)
- The `get_images_by_priority()` method already sorts by priority DESC
- Need to extend this to traverse parent chain for inherited images

**Query Strategy**:
1. Get parent chain for project: `WITH RECURSIVE` CTE to traverse `parent_id` links
2. Fetch direct images from current project and all parents
3. Fetch STL preview images from current project
4. Combine and sort by `image_priority DESC, display_order ASC`
5. Limit to 15 images total

**Alternatives Considered**:
- **On-demand parent traversal**: Would require N+1 queries for each parent level
- **Denormalized image cache table**: Would require complex cache invalidation on every scan
- **Join-based approach**: Less readable and harder to maintain than CTE-based recursive query

**Example Query Pattern**:
```sql
WITH RECURSIVE parent_chain AS (
  SELECT id, parent_id FROM projects WHERE id = ?1
  UNION ALL
  SELECT p.id, p.parent_id FROM projects p
  INNER JOIN parent_chain pc ON p.id = pc.parent_id
)
SELECT img.* FROM image_files img
WHERE img.project_id IN (SELECT id FROM parent_chain)
ORDER BY img.image_priority DESC, img.display_order ASC
LIMIT 15
```

---

## 3. Frontend Carousel Component Strategy

### Decision: Create new `SearchTileCarousel` component adapted from existing `ImageCarousel`

**Rationale**:
- Existing `ImageCarousel.tsx` is designed for full-width project page display
- Search tiles need a compact, grid-friendly carousel variant
- Reuse core navigation logic but adapt styling for smaller tile context
- Implement lazy loading with Intersection Observer API

**Key Differences from Existing Carousel**:
| Aspect | Existing ImageCarousel | New SearchTileCarousel |
|--------|----------------------|------------------------|
| Size | Full-width, aspect-video | Fixed tile size (280x200px) |
| Thumbnails | Show below main image | Dot indicators only |
| Navigation | Large arrow buttons + thumbnails | Small inline arrows + dots |
| Image counter | Top-right overlay | Bottom dots indicator |
| Loading | Eager loading | Lazy loading (viewport-based) |

**Best Practices Adopted**:
- **React.memo()** for tile component to prevent unnecessary re-renders
- **Intersection Observer** for lazy loading images when tiles enter viewport
- **CSS containment** (`contain: layout style paint`) for carousel isolation
- **Debounced auto-advance** with pause on hover/interaction
- **Keyboard navigation** (arrow keys) for accessibility
- **ARIA labels** for screen readers

**Alternatives Considered**:
- **Third-party carousel library** (react-slick, swiper): Adds unnecessary bundle size (50-100KB) for simple use case
- **Adapt existing ImageCarousel**: Would complicate component with too many conditional branches
- **CSS-only carousel**: Lacks necessary interaction handling and lazy loading

---

## 4. Performance Optimization Strategies

### Decision: Multi-layered optimization approach

**A. Database Query Optimization**

**Indexes Required** (already exist):
```sql
CREATE INDEX idx_projects_leaf ON projects(is_leaf);  -- ✅ Exists
CREATE INDEX idx_image_files_priority ON image_files(project_id, image_priority DESC, display_order ASC);  -- ✅ Exists
```

**Query Optimization**:
- Use `EXPLAIN QUERY PLAN` to verify index usage
- Batch image queries for all search results (1 query instead of N queries)
- Limit images to 15 per project in database query, not in application code

**B. API Response Optimization**

**Decision**: Include images directly in search response, not separate endpoint

**Rationale**:
- Single request reduces latency (no N+1 problem)
- Enables faster initial render
- Search results are already paginated (20 per page), limiting payload size

**Response Structure**:
```typescript
interface SearchResultProject {
  id: number;
  name: string;
  full_path: string;
  is_leaf: boolean;
  stl_count: number;
  images: ImagePreview[];  // NEW: Up to 15 images
}

interface ImagePreview {
  id: number;
  filename: string;
  image_source: 'regular' | 'stl_preview';
  source_type: 'direct' | 'inherited';
}
```

**C. Frontend Rendering Optimization**

**Lazy Loading Strategy**:
1. Render only visible tiles initially (viewport + 1 screen buffer)
2. Use Intersection Observer to load images as tiles scroll into view
3. Placeholder skeleton while images load

**Image Loading**:
- Use native browser lazy loading: `<img loading="lazy" />`
- Serve thumbnails, not full-resolution images
- Cache API responses with TanStack Query (5 minute stale time)

**Carousel Optimization**:
- Auto-advance pauses when tile is out of viewport
- Stagger auto-advance intervals (random 3-7 seconds) to avoid simultaneous animations
- Use CSS transforms for smooth transitions (GPU-accelerated)

---

## 5. Backwards Compatibility Strategy

### Decision: Add optional `leaf_only` query parameter (default true)

**Rationale**:
- Existing API endpoint: `GET /api/search?q=...&tags=...`
- Add new parameter: `leaf_only=true` (default behavior for new search UI)
- Allow `leaf_only=false` for legacy/admin views that need all projects
- Non-breaking change: existing clients get same results by default if they don't send parameter

**API Contract**:
```
GET /api/search
Query Parameters:
  - q: string (optional) - Search query
  - tags: string (optional) - Comma-separated tag names
  - page: number (default 1)
  - per_page: number (default 20, max 100)
  - leaf_only: boolean (default true) - Filter to projects with STL files
```

**Alternatives Considered**:
- **New endpoint** (`/api/search/leaf`): Creates API fragmentation, harder to maintain
- **Default to false**: Would break new UI requirement, users would see folders again
- **Remove non-leaf support**: Too aggressive, may break admin tools or future features

---

## 6. Testing Strategy

### Decision: Multi-level test coverage

**A. Backend Tests** (Rust)

**Unit Tests**:
- Test `is_leaf` filtering in each search method
- Test image aggregation with various parent chain depths
- Test image limit enforcement (15 max)
- Test empty results handling (no images, no leaf projects)

**Integration Tests**:
- Create test database with known hierarchy
- Verify search returns only leaf projects
- Verify images include inherited + STL previews
- Verify correct image ordering by priority

**B. Frontend Tests** (Vitest + React Testing Library)

**Component Tests**:
- `SearchTileCarousel`: Test navigation, dot indicators, image loading
- `ProjectTile`: Test carousel integration, metadata display
- Test lazy loading with Intersection Observer mock

**Integration Tests**:
- Mock API with realistic search results
- Verify tiles render with images
- Test carousel interactions (next/prev, dots)

**C. Performance Tests**

**Benchmarks**:
- Search query execution time (target: <100ms)
- Page render time with 50 results (target: <2s)
- Carousel interaction responsiveness (target: <16ms per frame)

**Load Testing** (optional):
- 10,000 projects in database
- Concurrent search requests
- Memory usage during image loading

---

## 7. Migration & Rollout Plan

### Decision: Zero-downtime deployment (no schema changes needed)

**Rationale**:
- All required database columns already exist (`is_leaf`, `image_priority`, `image_source`)
- No migrations needed
- API change is backward compatible (new optional parameter)
- Frontend change is isolated to search page

**Deployment Steps**:
1. Deploy backend changes (new search filtering logic)
2. Verify API compatibility with existing frontend
3. Deploy frontend changes (new carousel components)
4. Monitor performance metrics (response times, error rates)

**Rollback Plan**:
- Frontend: Revert to previous version (no data changes)
- Backend: Remove `leaf_only` filtering (backward compatible)

**No Data Backfill Required**: All projects already have `is_leaf` flag set by scanner service

---

## 8. Edge Cases & Error Handling

### Identified Edge Cases (from spec):

**1. No images available**
- **Solution**: Show placeholder icon with "No images available" text
- **Implementation**: Default empty array in API response, conditional render in carousel

**2. Project with >15 images**
- **Solution**: Limit to 15 in SQL query with `LIMIT 15`
- **UI Indicator**: Show "15+ images" badge on tile

**3. Large image files (>5MB)**
- **Solution**: Already handled by thumbnail/preview system
- **Verification**: Check that STL previews and cached images are optimized

**4. Slow image loading**
- **Solution**: Skeleton loading animation
- **Fallback**: Show broken image icon after 10s timeout

**5. Single image projects**
- **Solution**: Hide navigation controls (prev/next arrows) when `images.length === 1`
- **Show static image**: No carousel behavior for single image

**6. Zero leaf project matches**
- **Solution**: Show empty state message: "No projects with STL files found. Try different search terms."

**7. Touch device support**
- **Solution**: Add touch event listeners for swipe gestures
- **Library**: Use native touch events or simple swipe detection

---

## Summary & Next Steps

### Research Complete ✅

All technical unknowns have been resolved:
- ✅ Leaf filtering strategy: Use existing `is_leaf` flag with server-side filtering
- ✅ Image aggregation: Recursive CTE for parent chain + priority-based ordering
- ✅ Carousel design: New compact component adapted from existing pattern
- ✅ Performance optimization: Database indexes, lazy loading, query batching
- ✅ API compatibility: Optional `leaf_only` parameter (backward compatible)
- ✅ Testing approach: Unit, integration, and performance tests
- ✅ Deployment: Zero-downtime, no migrations needed

### Risks Identified & Mitigated

| Risk | Mitigation |
|------|-----------|
| N+1 query problem for images | Batch image queries for all search results |
| Slow recursive parent chain query | Use indexed `parent_id`, limit depth to 5 levels |
| Poor carousel performance with many tiles | Lazy loading + viewport-based rendering |
| Large API response payload | Limit images to 15, use pagination |

### Ready for Phase 1: Design & Contracts

All research findings will inform:
- Data model definitions (search result structure with images)
- API contract specification (search endpoint with new parameters)
- Component architecture (carousel and tile design)
