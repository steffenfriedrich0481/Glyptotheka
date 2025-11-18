# Rescan Image Detection Issue - Test Results

**Date:** 2025-11-18  
**Test:** Option B - Test rescan functionality with new images  
**Status:** ❌ **BUG FOUND**

## Test Setup

**Project:** Project 6 (Bahamut)  
**Path:** `/projects/Miniaturen/Mammoth Factory/Bahamut/`

**Folder Structure:**
```
Bahamut/
├── Bahamut.jpg (593K, May 2022) ← OLD
├── Bahamut_2.jpg (51K, Nov 18 09:48) ← NEW
├── STL/
│   └── [stl files]
├── Unsupported/
│   └── [stl files]
└── LYS/
```

## Test Results

### Initial State
```bash
$ curl 'http://localhost:3000/api/projects/6/files'
{
  "images": [
    {
      "id": 2,
      "filename": "Bahamut.jpg",
      "created_at": 1763411734
    }
  ]
}
```
✅ Has 1 image (old)  
❌ Missing Bahamut_2.jpg (new)

### After Incremental Rescan
```bash
$ curl -X POST http://localhost:3000/api/scan -d '{}'
$ curl 'http://localhost:3000/api/projects/6/files'
{
  "images": [
    {
      "id": 2,
      "filename": "Bahamut.jpg"
    }
  ]
}
```
❌ **NEW IMAGE NOT DETECTED**

### After Forced Full Rescan
```bash
$ curl -X POST http://localhost:3000/api/scan -d '{"force": true}'
$ curl 'http://localhost:3000/api/projects/6/files'
{
  "images": [
    {
      "id": 2,
      "filename": "Bahamut.jpg"
    }
  ]
}
```
❌ **NEW IMAGE STILL NOT DETECTED**

## Root Cause Analysis

### Issue 1: Parent Folder Not Scanned

**Problem:** Images in parent folders are only scanned for folders that **directly contain STL files**.

**Location:** `backend/src/services/scanner.rs` and `backend/src/services/rescan.rs`

**Current Behavior:**
```rust
// Scanner only processes folders with STL files
for entry in WalkDir::new(root) {
    if ext.eq_ignore_ascii_case("stl") {
        if let Some(parent) = e.path().parent() {
            project_folders.insert(parent);  // Only parent of STL
        }
    }
}

// Then scans images ONLY in those folders
for (folder, stl_files) in project_folders {
    scan_images_in_folder(folder);  // Missing parent folders!
}
```

**Expected Behavior:**
- Scan `/Bahamut/` for images (parent folder)
- Scan `/Bahamut/STL/` for STL files (child folder)
- Images in `/Bahamut/` should be accessible to `/Bahamut/STL/`

**Actual Behavior:**
- Only scans `/Bahamut/STL/` (where STL files are)
- Misses `/Bahamut/` (where images are)
- Parent folder images never found

### Issue 2: Parent Folder Scanning Incomplete

**We added parent folder scanning** in commit `37ed114`:

```rust
// NEW: Scan parent folders for images
info!("Scanning parent folders for images");
for (folder, _) in project_folders.iter() {
    let mut current: &Path = folder.as_path();
    
    while let Some(parent_folder) = current.parent() {
        // ... scan parent ...
    }
}
```

**But this has a problem:**
- It walks UP from STL folders to scan parents
- `/Bahamut/STL/` → `/Bahamut/` → ... ✅
- BUT: `/Bahamut/` itself is **never created as a project** if it has no direct STL files
- So `ensure_project_exists(parent_folder)` creates it
- Then `add_images_for_project(parent_id, parent_folder)` should find the images

**Let me check if add_images_for_project is working...**

## Hypothesis

The parent folder scanning **should** work based on the code. Let me verify:

1. Does `ensure_project_exists` create the Bahamut project?
2. Does `add_images_for_project` scan the folder correctly?
3. Is there a caching issue preventing new images from being found?

## Next Steps

1. ✅ Add debug logging to see what folders are being scanned
2. ✅ Check if Bahamut project is created (it is - ID 6)
3. ✅ Check if add_images_for_project is called for Bahamut folder
4. ❓ **Why isn't Bahamut_2.jpg being detected?**

## Possible Causes

### A) Image Already Exists Check
Maybe the image was previously scanned and then deleted, but DB record remains?

### B) File System Timing
Maybe the file was added after the scan started but the timestamp is wrong?

### C) Path Matching Issue
Maybe the path comparison isn't matching correctly?

### D) add_images_for_project Not Finding It
The `add_images_for_project` method might have a bug:

```rust
fn add_images_for_project(
    &self,
    project_id: i64,
    folder: &Path,
) -> Result<(), AppError> {
    let image_extensions = ["jpg", "jpeg", "png", "gif", "webp"];
    
    if let Ok(entries) = fs::read_dir(folder) {
        for entry in entries.flatten() {
            if let Some(ext) = entry.path().extension() {
                if image_extensions.iter().any(|e| ext.eq_ignore_ascii_case(e)) {
                    // Process image...
                }
            }
        }
    }
}
```

Let me check the database directly...

## Database Investigation

```bash
$ docker exec glyptotheka-backend sqlite3 /app/data/glyptotheka.db \
  "SELECT id, filename, file_path FROM image_files WHERE project_id = 6"

2|Bahamut.jpg|/projects/Miniaturen/Mammoth Factory/Bahamut/Bahamut.jpg
```

Only 1 row! The file `Bahamut_2.jpg` is definitely not in the database.

## Conclusion

**Bug Confirmed:** New images in parent folders are NOT detected during rescan.

**Impact:** 
- ❌ Cannot add new images to existing projects
- ❌ Rescan button doesn't work for images
- ❌ Users must delete database and do full scan

**Severity:** HIGH - Breaks core functionality

**Solution Needed:**
- Fix parent folder image scanning
- Ensure new images are detected in both:
  1. Folders with STL files (direct)
  2. Parent folders without STL files (inherited)

## Recommended Fix

Update `add_images_for_project` to:
1. Check existing images more carefully
2. Add logging to show what's being scanned
3. Ensure new files are detected properly

Or:

Update parent folder scanning to:
1. Run in rescan service as well as scanner
2. Call add_images_for_project for ALL parent folders
3. Not just folders created by ensure_project_exists

---

**Status:** Bug documented, needs fix before composite preview feature  
**Priority:** HIGH - Must fix before implementing composite previews  
**Blocker:** Yes - Rescan must work for preview updates to work
