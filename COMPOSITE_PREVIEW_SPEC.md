# Composite Preview Image Generation - Feature Specification

**Status:** 📋 Planning  
**Priority:** Medium  
**Complexity:** High  

## Problem Statement

Currently, subproject cards show only the first image as a preview. This doesn't give a good overview of the variety of content in a subproject with multiple images.

## Requirements

### User Stories

1. **As a user**, when I view a project with subprojects, I want to see a composite preview of multiple images from each subproject so I can quickly understand what's inside.

2. **As a user**, when I add new images to a folder and click Rescan, I want the preview to update to include the new images.

3. **As a user**, I want the composite preview to show up to 4 images in a grid layout.

### Functional Requirements

- **FR1**: Generate composite preview images during scan
- **FR2**: Combine up to 4 images from a subproject into a 2x2 grid
- **FR3**: Store composite previews in cache directory
- **FR4**: Update composite previews when rescanning
- **FR5**: Fallback to single image if < 4 images available
- **FR6**: Use folder icon if no images available

### Non-Functional Requirements

- **NFR1**: Preview generation should not significantly slow down scanning
- **NFR2**: Previews should be cached and reused
- **NFR3**: Composite images should be optimized for size (max 500KB)
- **NFR4**: Preview resolution: 800x800px (adequate for cards)

## Design

### Architecture

```
Scanner Service
    │
    ├─> Scan for STL files
    ├─> Scan for images
    ├─> Image inheritance
    │
    └─> Generate Composite Previews  ← NEW
        ├─> Collect first 4 images per project
        ├─> Composite Generator Service
        │   ├─> Load images
        │   ├─> Resize to 400x400 each
        │   ├─> Arrange in 2x2 grid
        │   └─> Save as PNG
        └─> Store preview path in DB
```

### Database Schema

**New Table:** `project_previews`

```sql
CREATE TABLE project_previews (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    preview_path TEXT NOT NULL,
    image_count INTEGER NOT NULL,     -- How many images used (1-4)
    source_image_ids TEXT NOT NULL,   -- JSON array of image IDs
    generated_at INTEGER NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);
```

### Implementation Steps

#### Phase 1: Composite Image Generator Service

**File:** `backend/src/services/composite_preview.rs`

```rust
pub struct CompositePreviewService {
    cache_dir: PathBuf,
}

impl CompositePreviewService {
    pub fn generate_preview(
        &self,
        project_id: i64,
        image_paths: &[String],  // Up to 4 paths
    ) -> Result<PathBuf, AppError> {
        // 1. Load images
        // 2. Resize each to 400x400
        // 3. Create 800x800 canvas
        // 4. Arrange in grid:
        //    [0][1]
        //    [2][3]
        // 5. Save to cache/previews/project_{id}_composite.png
        // 6. Return path
    }
}
```

**Dependencies needed:**
```toml
[dependencies]
image = "0.24"  # Image processing
```

#### Phase 2: Integrate into Scanner

**File:** `backend/src/services/scanner.rs`

Add after image inheritance:

```rust
// Third pass: Generate composite previews
info!("Generating composite previews for projects");
for (folder, _) in project_folders.iter() {
    if let Some(&project_id) = path_to_id.get(folder) {
        // Get first 4 images for this project
        let images = self.file_repo.get_project_images(project_id, 4)?;
        
        if images.len() >= 2 {
            // Generate composite preview
            let preview_path = self.composite_preview_service
                .generate_preview(project_id, &images)?;
            
            // Store in database
            self.preview_repo.store_preview(
                project_id,
                &preview_path,
                images.len(),
                &images.iter().map(|i| i.id).collect::<Vec<_>>(),
            )?;
        }
    }
}
```

#### Phase 3: API Endpoint

**New endpoint:** `GET /api/projects/:id/preview`

Returns the composite preview image or 404 if not available.

```rust
pub async fn get_project_preview(
    State(state): State<AppState>,
    AxumPath(id): AxumPath<i64>,
) -> Result<impl IntoResponse, AppError> {
    // Get preview from DB
    let preview = state.preview_repo.get_preview(id)?
        .ok_or_else(|| AppError::NotFound(format!("Preview not found for project {}", id)))?;
    
    // Serve the image
    let file = File::open(&preview.preview_path).await?;
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);
    
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "image/png")
        .body(body)
        .unwrap())
}
```

#### Phase 4: Frontend Integration

**File:** `frontend/src/pages/ProjectPage.tsx`

Update preview URL:

```typescript
{preview ? (
  <img
    src={`/api/projects/${child.id}/preview`}
    alt={child.name}
    className="w-full h-full object-cover"
    loading="lazy"
    onError={(e) => {
      // Fallback to first image
      (e.target as HTMLImageElement).src = `/api/files/images/${preview.id}`;
    }}
  />
) : (
  <div className="w-full h-full flex items-center justify-center text-6xl text-gray-400">
    📁
  </div>
)}
```

### Image Layout Examples

#### 1 Image
```
┌──────────┐
│          │
│  Image   │
│          │
└──────────┘
```

#### 2 Images
```
┌─────┬─────┐
│  1  │  2  │
│     │     │
└─────┴─────┘
```

#### 3 Images
```
┌─────┬─────┐
│  1  │  2  │
├─────┴─────┤
│     3     │
└───────────┘
```

#### 4 Images
```
┌─────┬─────┐
│  1  │  2  │
├─────┼─────┤
│  3  │  4  │
└─────┴─────┘
```

## Testing Plan

### Unit Tests
- [ ] Composite generation with 1 image
- [ ] Composite generation with 2 images
- [ ] Composite generation with 3 images
- [ ] Composite generation with 4 images
- [ ] Error handling for missing images
- [ ] Performance test (generation time < 200ms per preview)

### Integration Tests
- [ ] Full scan generates previews
- [ ] Rescan updates previews
- [ ] Preview endpoint serves correct image
- [ ] Frontend displays composite previews
- [ ] Fallback to folder icon when no preview

### Manual Tests
1. Add multiple images to a folder
2. Run scan
3. View project page
4. Verify composite preview shows multiple images
5. Add new image
6. Click Rescan
7. Verify preview updates with new image

## Performance Considerations

- **Parallel generation**: Process multiple previews concurrently
- **Caching**: Store generated previews, only regenerate on rescan
- **Optimization**: Use WebP format for smaller file sizes (optional)
- **Lazy generation**: Generate on-demand vs during scan (trade-off)

## Alternative Approaches

### Option 1: Generate during scan (Recommended)
- **Pros**: Previews ready immediately, no delay when viewing
- **Cons**: Slightly slower scan time

### Option 2: Generate on-demand
- **Pros**: Faster scan time
- **Cons**: Delay when first viewing project, more complex caching

### Option 3: Client-side composition
- **Pros**: No backend changes needed
- **Cons**: More network requests, slower page load, no caching

## Timeline

- **Phase 1** (Composite Generator): 2-3 hours
- **Phase 2** (Scanner Integration): 1-2 hours
- **Phase 3** (API Endpoint): 1 hour
- **Phase 4** (Frontend): 1 hour
- **Testing**: 1-2 hours

**Total Estimate**: 6-9 hours

## Dependencies

- `image` crate for Rust (image processing)
- Database migration for `project_previews` table
- Cache directory structure

## Future Enhancements

- [ ] Add padding/borders between images in composite
- [ ] Show image count badge on preview
- [ ] Allow users to regenerate previews manually
- [ ] Support different grid layouts (3x3, etc.)
- [ ] Animated previews (GIF/WebP animation)
- [ ] Smart selection (prefer images with different content)

## Questions for Clarification

1. Should we always use the first 4 images, or select diverse images?
2. Should inherited images be included in composite, or only direct images?
3. What should happen if a project has 100+ images? Still use first 4?
4. Should we add a "Regenerate Preview" button in the UI?

## Status

- [x] Specification complete
- [ ] Implementation started
- [ ] Testing completed
- [ ] Deployed to production

---

**Next Steps**: Get approval on design, then implement Phase 1 (Composite Generator Service)
