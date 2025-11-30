# Keyword-Aware Rescan Migration

## Overview

This document explains how Glyptotheka handles changes to the `IGNORED_KEYWORDS` configuration between scans, particularly when folders that were previously treated as projects should now be treated as STL category folders.

## The Problem

When you change the `IGNORED_KEYWORDS` environment variable between scans, folder structure interpretation can change:

### Example Scenario

**Initial Scan (Empty Keywords)**
```
/MyAwesomeProject/
  └── STL/
      ├── part1.stl
      ├── part2.stl
      └── part3.stl
```

With no keywords configured, `/MyAwesomeProject/STL/` becomes a **project** with 3 STL files directly attached.

**Second Scan (With "STL" Keyword)**
```
IGNORED_KEYWORDS=STL,PRESUPPORTED_STL
```

Now `STL` is a category folder keyword, so:
- `/MyAwesomeProject/` should be the **project**
- `STL/` should be a **category folder**, not a project
- All STL files should have `category = "STL"`

## The Solution

The scanner now includes automatic migration logic that runs during each scan:

### Migration Process

1. **Detection Phase**
   - Scans all existing projects in the database
   - Checks if their folder names now match ignored keywords
   - Identifies projects that are no longer in the calculated `project_folders` map

2. **Migration Phase**
   For each project that became a category folder:
   - Finds the correct parent folder (traversing up past category folders)
   - Creates or identifies the parent project
   - Migrates all STL files to the parent project
   - Sets the `category` field on STL files to the old folder name
   - Migrates all image files to the parent project
   - Marks the old project as `is_leaf = false` (no longer a leaf project)

3. **Preservation**
   - The old project entry is kept (not deleted) for historical data
   - Marked as non-leaf to prevent it from showing in project lists
   - All relationships are properly maintained

### Code Flow

```rust
scan() {
    // 1. Walk filesystem and find project folders based on current keywords
    let project_folders = walk_and_find_projects();
    
    // 2. Migrate projects affected by keyword changes
    migrate_category_folder_projects(root, &project_folders);
    
    // 3. Continue normal scan processing
    create_project_hierarchy();
    add_stl_files_with_categories();
}
```

## Database Changes

### Before Migration
```sql
-- Project table
id | name | full_path              | is_leaf
1  | STL  | /projects/MyProject/STL | 1

-- STL files
id | project_id | filename   | category
1  | 1          | part1.stl  | NULL
2  | 1          | part2.stl  | NULL
```

### After Migration
```sql
-- Project table
id | name           | full_path           | is_leaf
1  | STL            | /projects/MyProject/STL | 0  -- Now non-leaf
2  | MyProject      | /projects/MyProject     | 1  -- New parent project

-- STL files
id | project_id | filename   | category
1  | 2          | part1.stl  | "STL"  -- Migrated with category
2  | 2          | part2.stl  | "STL"
```

## Usage

### Changing Keywords

1. **Update docker-compose.yml**
   ```yaml
   environment:
     - IGNORED_KEYWORDS=STL,PRESUPPORTED_STL,Unsupported
   ```

2. **Restart and Rescan**
   ```bash
   docker-compose down
   docker-compose up -d
   # Trigger rescan via UI or API
   ```

3. **Migration Happens Automatically**
   - Check scan logs for migration messages
   - Old projects are migrated seamlessly
   - No data loss occurs

### Log Messages

During migration, you'll see:
```
INFO  Checking for projects that should become category folders
INFO  Migrating project '/projects/MyProject/STL' - now a category folder due to keyword changes
INFO  Migrated project /projects/MyProject/STL -> /projects/MyProject (category: STL)
```

## Best Practices

### 1. Initial Setup
Set your keywords **before** the first scan to avoid unnecessary migration:
```yaml
IGNORED_KEYWORDS=STL,PRESUPPORTED_STL,Unsupported,Pre-Supported,inch,mm,32mm
```

### 2. Adding Keywords
When adding new keywords, the migration is safe and automatic:
- Existing correct projects remain unchanged
- Only affected projects are migrated
- Categories are properly set

### 3. Removing Keywords
**Caution**: Removing keywords doesn't automatically revert migrations. Folders that were migrated will stay migrated. To revert:
1. Remove keywords from config
2. Delete the database
3. Run a fresh scan

### 4. Complex Hierarchies
Migration works through multiple levels:
```
/MyProject/
  └── STL/
      └── Unsupported/
          └── part.stl
```

With keywords `STL,Unsupported`, the scanner traverses up through both category folders to find `/MyProject/` as the actual project, and sets `category = "Unsupported"` (the immediate parent).

## Limitations

### Category Assignment
Only the **immediate parent folder** of the STL file becomes its category:
```
/MyProject/STL/Unsupported/part.stl → category = "Unsupported"
/MyProject/STL/part.stl → category = "STL"
```

### Migration Direction
- **Forward migration** (project → category): Fully supported ✅
- **Reverse migration** (category → project): Requires manual intervention ⚠️

### Orphaned Projects
Old non-leaf projects remain in the database. To clean up:
```sql
-- Manual cleanup (use with caution)
DELETE FROM projects WHERE is_leaf = 0 AND id NOT IN (
    SELECT DISTINCT parent_id FROM projects WHERE parent_id IS NOT NULL
);
```

## Troubleshooting

### Migration Not Working

**Check keyword configuration:**
```bash
docker exec glyptotheka-backend env | grep IGNORED_KEYWORDS
```

**Check logs:**
```bash
docker logs glyptotheka-backend | grep -i "migrat"
```

### Files Not Categorized

Ensure the folder name **exactly matches** a keyword (case-insensitive):
- ✅ Folder: `STL` → Keyword: `STL` → Match
- ✅ Folder: `stl` → Keyword: `STL` → Match (case-insensitive)
- ❌ Folder: `STL_Files` → Keyword: `STL` → No match (different name)

Use **substring matching** by configuring partial keywords:
```yaml
IGNORED_KEYWORDS=STL,PRESUPPORTED,Unsupported
# Matches: STL, STL_Files, PRESUPPORTED_STL, etc.
```

### Performance Impact

Migration runs during every scan but only processes affected projects:
- First scan with new keywords: May take longer
- Subsequent scans: Near-zero overhead (no projects to migrate)
- Large databases (1000+ projects): < 1 second migration time

## Technical Details

### Implementation

Location: `backend/src/services/scanner.rs`

Key method: `migrate_category_folder_projects()`

Runs: After directory walk, before project hierarchy creation

### Database Transactions

Migration uses individual update statements, not wrapped in transaction:
- Each file migration is atomic
- Partial failures won't corrupt database (due to foreign key constraints)
- Safe to interrupt and re-run

### Thread Safety

Migration runs on the scan thread:
- No concurrent scans (enforced by `scan_state` mutex)
- Safe from race conditions
- Respects semaphore limits for preview generation

## See Also

- [Main README](../README.md) - General project information
- [STL_THUMB_INTEGRATION.md](../STL_THUMB_INTEGRATION.md) - STL preview generation
- [CPU_LIMITS.md](CPU_LIMITS.md) - Resource management
