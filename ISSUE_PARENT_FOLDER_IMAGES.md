# Issue: Parent Folder Images Not Scanned

**Date:** 2025-11-17  
**Status:** Needs Fix  
**Branch:** 001-update-scan-service  
**Related Commit:** 64af6e1

## Problem

The image inheritance feature was implemented, but parent folder images are not being found during scanning. 

### Root Cause

The scanner only creates projects for folders that **directly contain STL files**. Parent folders that contain only images (but STL files in subfolders) are never scanned for images.

### Example Structure

```
/819_Dwarf Gemtreasure Trader/          ← Has image but NO direct STL files
  819_Dwarf_Gemtreasure_Trader.png      ← IMAGE NOT FOUND ❌
  /Pre-Supported/                        ← Project created (has STL files)
    /STL/
      model.stl                          ← STL files here
```

**Current behavior:**
- Scanner walks tree finding folders with STL files
- Creates project for `/Pre-Supported/STL/` folder
- Calls `add_images_for_project()` for that folder only
- Parent folder (`/819_Dwarf Gemtreasure Trader/`) never gets scanned for images
- `inherit_images_from_parents()` tries to inherit but finds nothing

## Test Evidence

```bash
# Project 5: 819_Dwarf Gemtreasure Trader (has image, no direct STL files)
$ curl 'http://localhost:3000/api/projects/5/files' | jq '.images | length'
0  # ❌ Should find 819_Dwarf_Gemtreasure_Trader.png

# Project 6: Pre-Supported (child with STL files)
$ curl 'http://localhost:3000/api/projects/6/files' | jq '.images | length'
0  # ❌ Should inherit image from parent

# File exists on disk:
$ ls "/projects/Miniaturen/.../819_Dwarf Gemtreasure Trader/"
819_Dwarf Gemtreasure Trader.png  # ✅ File exists
Pre-Supported/
STL/
```

## Solution Options

### Option 1: Scan All Parent Folders (Recommended)

During the first pass, also scan parent folders of each STL-containing folder for images:

```rust
// In scan() method, after creating project hierarchy:
for (folder, _) in project_folders.iter() {
    let project_id = path_to_id[folder];
    
    // Scan current folder for direct images
    self.add_images_for_project(project_id, folder)?;
    
    // NEW: Also scan parent folders and add their direct images
    let mut current = folder;
    while let Some(parent) = current.parent() {
        if parent < root {
            break;
        }
        
        // Create or get parent project
        let parent_id = self.ensure_project_exists(parent, root, &mut path_to_id)?;
        
        // Scan parent folder for images (only if not already scanned)
        if !scanned_folders.contains(parent) {
            self.add_images_for_project(parent_id, parent)?;
            scanned_folders.insert(parent.to_path_buf());
        }
        
        current = parent;
    }
}
```

### Option 2: Full Tree Walk for Images

Walk entire tree looking for images, create parent projects as needed:

```rust
// Before the STL scan:
let mut all_folders_with_images = HashMap::new();

for entry in WalkDir::new(root) {
    if let Ok(e) = entry {
        if e.file_type().is_file() {
            if let Some(ext) = e.path().extension() {
                if is_image_extension(ext) {
                    if let Some(parent) = e.path().parent() {
                        all_folders_with_images.entry(parent)
                            .or_insert_with(Vec::new)
                            .push(e.path().to_path_buf());
                    }
                }
            }
        }
    }
}

// Then process images for all folders (including parent-only folders)
```

### Option 3: Post-Process Parent Images

After main scan, walk up from each project and scan parent folders:

```rust
// After main scan loop:
for (folder, _) in project_folders.iter() {
    let mut current = folder.parent();
    while let Some(parent_folder) = current {
        if parent_folder < root {
            break;
        }
        
        // Ensure parent project exists
        let parent_id = self.ensure_project_exists(parent_folder, root, &mut path_to_id)?;
        
        // Scan and add parent's direct images
        self.add_images_for_project(parent_id, parent_folder)?;
        
        current = parent_folder.parent();
    }
}
```

## Recommendation

**Option 3 (Post-Process)** is cleanest and matches the current architecture:
1. Main loop scans STL-containing folders
2. Post-process ensures all parent folders have their direct images scanned
3. Second pass (existing) handles inheritance propagation

## Files to Modify

- `backend/src/services/scanner.rs` - Add parent folder image scanning
- Add tracking of scanned folders to avoid duplicates
- Ensure `path_to_id` is mutable in second pass

## Testing

After fix, should see:
```bash
# Project 5 should have direct image:
$ curl '.../projects/5/files' | jq '.images[0]'
{
  "filename": "819_Dwarf_Gemtreasure_Trader.png",
  "source_type": "direct",
  "source_project_id": null
}

# Project 6 should inherit image:
$ curl '.../projects/6/files' | jq '.images[0]'
{
  "filename": "819_Dwarf_Gemtreasure_Trader.png",
  "source_type": "inherited",
  "source_project_id": 5
}
```

## Status

**Blocked:** Image inheritance cannot work until parent folders are scanned for images.

**Priority:** High - Core feature blocker

**Estimated Effort:** 1-2 hours
