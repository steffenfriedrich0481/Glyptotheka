# Feature Request: Image Inheritance from Parent to Child Projects

**Status:** To Be Implemented  
**Priority:** High  
**Date:** 2025-11-17

## Problem Description

Currently, images are only shown in the exact folder where they are found. Images in parent folders are NOT inherited by child projects. This means:
- An image in `/Cast'N'Play/` folder is only shown when viewing that folder
- Child projects deep in the tree don't show parent folder images
- STL files at the lowest level have no visual preview unless there's an image in the same folder

## Example Structure

```
/projects
  /Miniaturen
    /Cast'N'Play
      header_image.jpg                         ← IMAGE IN PARENT FOLDER
      /[CNP] 24_04 - Dwarven Legacy
        promo_image.png                        ← IMAGE IN GRANDPARENT FOLDER
        /819_Dwarf Gemtreasure Trader
          819_Dwarf_Gemtreasure_Trader.png    ← IMAGE IN PROJECT FOLDER (direct)
          /Pre-Supported
            /STL
              STL_Treasure_A.stl              ← STL FILE (leaf project)
```

**Current behavior:**
- `STL` folder (leaf project with STL files): Shows 0 images ❌
- Only shows the image if it's in the same folder as the STL

**Expected behavior:**
- `STL` folder should inherit all images from parent folders:
  - `819_Dwarf_Gemtreasure_Trader.png` (direct parent)
  - `promo_image.png` (grandparent)
  - `header_image.jpg` (great-grandparent)
- Images marked as "inherited" with `source_project_id` pointing to the folder where they were found

## Use Cases

1. **Collection Headers** - Put a header image in `/Cast'N'Play/` that all models inherit
2. **Brand Images** - Creator logo in top folder applies to all their models
3. **Theme Images** - Fantasy/SciFi category images inherited by all sub-projects
4. **Fallback Previews** - Ensure all projects have at least some visual preview

## Database Schema

The `image_files` table already supports this with:
```sql
source_type TEXT NOT NULL DEFAULT 'direct',  -- 'direct' or 'inherited'
source_project_id INTEGER,                    -- ID of project where image was originally found
```

## Implementation Plan

### 1. Update Scanner Service - Downward Propagation

Modify `backend/src/services/scanner.rs`:

**After creating the project hierarchy and adding STL files, add a second pass to propagate images downward:**

```rust
// Second pass: Propagate images from parents to children
info!("Propagating images from parent folders to children");
for (folder, _) in project_folders.iter() {
    if let Some(&project_id) = path_to_id.get(folder) {
        if let Err(e) = self.inherit_images_from_parents(project_id, folder, root, &path_to_id) {
            let error_msg = format!(
                "Error inheriting images for project {}: {}",
                folder.display(),
                e
            );
            warn!("{}", error_msg);
            errors.push(error_msg);
        }
    }
}
```

**Add new method to inherit images from all ancestor folders:**

```rust
fn inherit_images_from_parents(
    &self,
    project_id: i64,
    folder: &Path,
    root: &Path,
    path_to_id: &HashMap<PathBuf, i64>,
) -> Result<(), AppError> {
    let mut inherited_images = Vec::new();
    
    // Walk UP the tree from current folder to root
    let mut current_folder = folder;
    while let Some(parent_folder) = current_folder.parent() {
        if parent_folder < root {
            break;
        }
        
        // Check if parent folder has any images (scan directory directly)
        if let Ok(entries) = fs::read_dir(parent_folder) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(ext) = entry.path().extension() {
                            let image_extensions = ["jpg", "jpeg", "png", "gif", "webp"];
                            if image_extensions.iter().any(|e| ext.eq_ignore_ascii_case(e)) {
                                // Get the parent project ID for this folder
                                let source_project_id = path_to_id.get(parent_folder)
                                    .copied()
                                    .or_else(|| {
                                        // Parent folder might not have a project yet, create one
                                        self.ensure_project_exists(parent_folder, root, path_to_id).ok()
                                    });
                                
                                if let Some(source_id) = source_project_id {
                                    inherited_images.push((
                                        entry.file_name().to_str().unwrap_or("").to_string(),
                                        entry.path().to_str().unwrap_or("").to_string(),
                                        fs::metadata(entry.path())
                                            .map(|m| m.len() as i64)
                                            .unwrap_or(0),
                                        source_id,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
        
        current_folder = parent_folder;
    }
    
    // Add all inherited images to this project
    for (filename, file_path, file_size, source_id) in inherited_images {
        self.file_repo.add_image_file(
            project_id,
            &filename,
            &file_path,
            file_size,
            "inherited",
            Some(source_id),
            0,
        )?;
    }
    
    Ok(())
}

fn ensure_project_exists(
    &self,
    folder: &Path,
    root: &Path,
    path_to_id: &HashMap<PathBuf, i64>,
) -> Result<i64, AppError> {
    if let Some(&existing_id) = path_to_id.get(folder) {
        return Ok(existing_id);
    }
    
    let full_path = folder.to_str().unwrap().to_string();
    if let Some(project) = self.project_repo.get_by_path(&full_path)? {
        return Ok(project.id);
    }
    
    // Create project for this folder
    let parent_id = if folder != root {
        folder.parent()
            .and_then(|p| path_to_id.get(p).copied())
    } else {
        None
    };
    
    let name = folder
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or("Unknown")
        .to_string();
    
    let create_project = CreateProject {
        name,
        full_path,
        parent_id,
        is_leaf: false,  // Parent folders are not leaves
    };
    
    self.project_repo.create(&create_project)
}
```

### 2. Update Rescan Service

Modify `backend/src/services/rescan.rs` to handle inherited images:

**Before rescanning, remove all inherited images:**
```rust
// Remove all inherited images (they will be regenerated)
conn.execute(
    "DELETE FROM image_files WHERE source_type = 'inherited'",
    [],
)?;
```

**After rescanning completes, rebuild inheritance is handled automatically by scanner.**

### 3. Key Points

- **Scan Direction:** Images flow DOWN from parent to child
- **Multiple Inheritance:** Child inherits from ALL ancestors up to root
- **Order:** Closer parents' images appear first (optional via `display_order`)
- **Source Tracking:** `source_project_id` tracks which folder originally had the image

## Benefits

1. **Reusable Headers** - One header image applies to entire collection
2. **Brand Consistency** - Creator logos inherited by all models
3. **Fallback Previews** - Every project has at least parent folder images
4. **Less Redundancy** - Don't need to copy images to every subfolder

## Example After Implementation

```
/Cast'N'Play/
  header_image.jpg                    (direct image)
  
  /[CNP] 24_04 - Dwarven Legacy/
    promo_image.png                   (direct image)
    header_image.jpg                  (inherited from parent)
    
    /819_Dwarf Gemtreasure Trader/
      819_Dwarf.png                   (direct image)
      promo_image.png                 (inherited from parent)
      header_image.jpg                (inherited from grandparent)
      
      /Pre-Supported/STL/
        [STL files here]
        819_Dwarf.png                 (inherited from great-grandparent)
        promo_image.png               (inherited from great-great-grandparent)
        header_image.jpg              (inherited from great-great-great-grandparent)
```

Each level inherits ALL images from all ancestors!

## Testing Plan

1. **Simple Inheritance**
   - Place image in parent folder
   - Scan
   - Verify child projects show inherited image
   
2. **Multi-Level Inheritance**
   - Place images at multiple levels
   - Verify deepest child inherits from all ancestors
   - Check `source_project_id` points to correct source

3. **Rescan**
   - Add new parent image
   - Rescan
   - Verify new image appears in all children
   
4. **Mixed Images**
   - Parent has image A
   - Child has direct image B
   - Verify child shows both A (inherited) and B (direct)

## Performance Considerations

- Image inheritance scanned once during initial scan
- Minimal overhead: just reading parent directories
- No duplicate files: only database references
- Query performance maintained with existing indexes

## Status

**READY FOR IMPLEMENTATION** - Schema supports it, clear implementation path defined.
