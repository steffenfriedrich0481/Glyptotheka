# Feature Request: Image Inheritance for Parent Projects

**Status:** To Be Implemented  
**Priority:** High  
**Date:** 2025-11-17

## Problem Description

Currently, when scanning projects, images are only associated with the folder they are found in. This means:
- Parent folders/projects don't show any images from their children
- Users must navigate deep into the hierarchy to see project previews
- Browse view shows empty tiles with no visual preview

## Example Structure

```
/projects
  /Miniaturen
    /Cast'N'Play
      /[CNP] 24_04 - Dwarven Legacy
        /819_Dwarf Gemtreasure Trader
          819_Dwarf Gemtreasure Trader.png    ← IMAGE HERE
          /Pre-Supported
            /STL
              STL_Treasure_A.stl              ← STL FILES HERE
```

**Current behavior:**
- Image only shows on `819_Dwarf Gemtreasure Trader` project
- Parent projects (`Miniaturen`, `Cast'N'Play`, etc.) show 0 images

**Expected behavior:**
- Image should "bubble up" and be visible on all parent projects
- When viewing `Miniaturen`, should see images from all child projects
- Images marked as "inherited" with `source_type='inherited'`

## Database Schema

The `image_files` table already supports this with:
```sql
source_type TEXT NOT NULL DEFAULT 'direct',  -- 'direct' or 'inherited'
source_project_id INTEGER,                    -- Original project ID for inherited images
```

## Implementation Plan

### 1. Update Scanner Service

Modify `backend/src/services/scanner.rs`:

**Add after line 127 (after adding direct images):**
```rust
// Propagate images to parent projects
if let Err(e) = self.propagate_images_to_parents(project_id, folder, root, &path_to_id) {
    let error_msg = format!(
        "Error propagating images for project {}: {}",
        folder.display(),
        e
    );
    warn!("{}", error_msg);
    errors.push(error_msg);
}
```

**Add new method:**
```rust
fn propagate_images_to_parents(
    &self,
    project_id: i64,
    folder: &Path,
    root: &Path,
    path_to_id: &HashMap<PathBuf, i64>,
) -> Result<(), AppError> {
    // Get all images for this project
    let images = self.file_repo.get_images_for_project(project_id)?;
    
    if images.is_empty() {
        return Ok(());
    }
    
    // Walk up the parent chain
    let mut current_folder = folder;
    while let Some(parent_folder) = current_folder.parent() {
        if parent_folder < root {
            break;
        }
        
        // Get parent project ID
        if let Some(&parent_id) = path_to_id.get(parent_folder) {
            // Add inherited images to parent
            for image in &images {
                self.file_repo.add_image_file(
                    parent_id,
                    &image.filename,
                    &image.file_path,
                    image.file_size,
                    "inherited",
                    Some(project_id),  // Original source project
                    0,
                )?;
            }
        }
        
        current_folder = parent_folder;
    }
    
    Ok(())
}
```

### 2. Update Rescan Service

Modify `backend/src/services/rescan.rs` to handle inherited images:

**In cleanup phase, remove inherited images:**
```rust
// Remove inherited images for projects that no longer exist
conn.execute(
    "DELETE FROM image_files 
     WHERE source_type = 'inherited' 
     AND source_project_id NOT IN (SELECT id FROM projects)",
    [],
)?;
```

**After rescanning, rebuild inheritance:**
```rust
// Rebuild image inheritance for all projects
self.rebuild_image_inheritance()?;
```

### 3. Update File Repository

Add method in `backend/src/db/repositories/file_repo.rs`:

```rust
pub fn get_images_for_project(&self, project_id: i64) -> Result<Vec<ImageFile>, AppError> {
    let conn = self.pool.get()?;
    let mut stmt = conn.prepare(
        "SELECT id, project_id, filename, file_path, file_size, 
                source_type, source_project_id, display_order, 
                created_at, updated_at
         FROM image_files
         WHERE project_id = ?1 AND source_type = 'direct'
         ORDER BY display_order"
    )?;
    
    let images = stmt.query_map(params![project_id], |row| {
        Ok(ImageFile {
            id: row.get(0)?,
            project_id: row.get(1)?,
            filename: row.get(2)?,
            file_path: row.get(3)?,
            file_size: row.get(4)?,
            source_type: row.get(5)?,
            source_project_id: row.get(6)?,
            display_order: row.get(7)?,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(images)
}
```

### 4. Frontend Display Enhancement

Update `ProjectTile.tsx` to show image count including inherited:

```typescript
const totalImageCount = metadata.imageCount + (metadata.inheritedImageCount || 0);

// In the display:
{totalImageCount > 0 && (
  <div className="flex items-center gap-1">
    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} 
            d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
    </svg>
    <span>{totalImageCount}</span>
  </div>
)}
```

## Benefits

1. **Better UX** - Users see visual previews at every level
2. **Faster Navigation** - Can identify interesting projects without drilling down
3. **Gallery Views** - Parent folders show all descendant images
4. **Search Results** - More visually appealing with inherited images

## Testing Plan

1. **Initial Scan**
   - Scan project with nested structure
   - Verify leaf projects have direct images
   - Verify parent projects have inherited images
   - Check `source_type` and `source_project_id` are correct

2. **Rescan**
   - Add new images to leaf project
   - Rescan
   - Verify new images propagate to parents
   
3. **Deletion**
   - Delete a project
   - Verify its inherited images are removed from parents
   
4. **UI Display**
   - Browse root folder - should show inherited images
   - Browse intermediate folder - should show inherited images
   - Browse leaf project - should show direct images
   - Verify image gallery shows all images with proper attribution

## Migration

No database migration needed - schema already supports this feature.

## Performance Considerations

- Image propagation happens during scan (one-time cost)
- Queries remain fast with existing indexes
- May increase database size (one row per image per ancestor)
- Consider adding `LIMIT` on inherited images per project

## Alternative Approach

Instead of storing inherited images, could compute them dynamically:
- Query all descendant project images when displaying parent
- More complex queries but less storage
- Slower for deep hierarchies

**Recommendation:** Implement storage-based approach first for simplicity and performance.

## Status

**READY FOR IMPLEMENTATION** - Schema supports it, clear implementation path defined.
