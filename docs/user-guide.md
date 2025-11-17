# Glyptotheka User Guide

Complete guide to using the 3D Print Model Library application.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Configuration](#configuration)
3. [Scanning Your Library](#scanning-your-library)
4. [Browsing Projects](#browsing-projects)
5. [Search & Filtering](#search--filtering)
6. [Tagging System](#tagging-system)
7. [Downloading Files](#downloading-files)
8. [Managing Your Library](#managing-your-library)
9. [Tips & Best Practices](#tips--best-practices)
10. [Troubleshooting](#troubleshooting)

---

## Getting Started

### First Launch

1. Open your browser to http://localhost:5173 (development) or your configured URL
2. You'll see the home page with configuration options
3. The application is ready to scan and organize your 3D print files

### System Requirements

- Modern web browser (Chrome 90+, Firefox 88+, Safari 14+)
- Sufficient disk space for cache (recommend 5-10GB)
- Read access to your 3D print file directory

---

## Configuration

### Setting Your Root Folder

The root folder is where all your 3D print projects are stored.

1. Navigate to the home page
2. Enter the **full path** to your projects folder
   - Example (Linux/Mac): `/home/username/3d-prints`
   - Example (Windows): `C:\Users\username\3d-prints`
3. Click **"Save Configuration"**
4. You can change this path at any time

### Optional: stl-thumb Configuration

For STL preview generation:

1. Install stl-thumb: https://github.com/unlimitedbacon/stl-thumb
2. The system will automatically detect it if in your PATH
3. Or configure the path in Settings

---

## Scanning Your Library

### Initial Scan

**When to scan:**
- First time setup
- After adding new projects
- After reorganizing your files
- After deleting projects

**How to scan:**

1. Ensure your root path is configured
2. Click **"Start Scan"** button
3. Watch the progress indicator
4. Wait for completion (may take several minutes for large libraries)

**What happens during scan:**

- Recursively walks through all subdirectories
- Identifies folders containing `.stl` files
- Creates hierarchical project structure
- Discovers and indexes image files (`.jpg`, `.png`, `.gif`, `.webp`)
- Generates STL preview thumbnails (if stl-thumb available)

**Scan statistics:**
- Projects found
- Files processed
- Any errors encountered

### Rescan

To update your library after changes:

1. Make changes to your file system (add/remove files)
2. Click **"Rescan"** from the Browse page
3. The system will:
   - Add new projects
   - Remove deleted projects
   - Update file listings
   - **Preserve your tags** (tags are not lost during rescan)

---

## Browsing Projects

### Hierarchical Navigation

The application presents your projects in a **tile-based interface**:

- **Folders** (collections): Contain other projects or folders
- **Leaf Projects**: Contain actual STL files and images

### Using the Browse Page

1. **Tiles**: Each tile represents a project or folder
   - Click any tile to navigate into it
   - Folder icon = navigate deeper
   - Project icon = view details

2. **Breadcrumbs**: Located at the top
   - Shows your current location in the hierarchy
   - Click any breadcrumb to jump back to that level
   - Home icon = return to root

3. **Project Details**: Click on a leaf project to see:
   - List of STL files
   - Gallery of images
   - Download options
   - Tagging interface

### Keyboard Navigation

- **Arrow Keys**: Navigate between tiles
- **Enter/Space**: Open selected tile
- **Home/End**: Jump to first/last tile
- **Tab**: Focus next interactive element

---

## Search & Filtering

### Text Search

1. Use the **search bar** in the header
2. Type your query (minimum 2 characters)
3. Press Enter or click Search
4. Results show all matching projects

**Search matches:**
- Project names
- Folder names
- Full file paths

### Tag Filtering

Filter projects by tags:

1. Click on any tag in search results or project details
2. See all projects with that tag
3. Combine multiple tags for AND filtering

### Advanced Search Tips

- **Phrase search**: Use quotes for exact matches
- **Case insensitive**: Search is not case-sensitive
- **Partial matches**: Will find partial word matches

---

## Tagging System

Tags provide flexible, cross-cutting organization beyond folders.

### Adding Tags

1. Navigate to a project detail page
2. Find the **Tags** section
3. Type tag name in the input field
4. Press Enter or click Add
5. Tag is immediately saved

**Tag suggestions:**
- System shows existing tags as you type
- Click a suggested tag to add it quickly
- Creates new tag if it doesn't exist

### Common Tag Categories

Organize by:
- **Status**: `printed`, `tested`, `favorite`, `needs-support`
- **Category**: `miniature`, `functional`, `decorative`, `tool`
- **Material**: `pla`, `petg`, `resin`, `abs`
- **Size**: `small`, `medium`, `large`, `multi-part`
- **Source**: `thingiverse`, `printables`, `original`, `commission`
- **Print Settings**: `0.2mm`, `supports-required`, `no-supports`

### Removing Tags

1. Navigate to project with the tag
2. Click the **X** button on the tag
3. Tag is immediately removed
4. Tag remains available for other projects

### Tag Management

- **Usage count**: System tracks how many projects use each tag
- **Autocomplete**: Popular tags appear first in suggestions
- **Case insensitive**: "PLA" and "pla" are the same tag

---

## Downloading Files

### Individual Files

**Download a single STL or image:**

1. Navigate to project detail page
2. Find the file in the file list or gallery
3. Click the **Download** button or icon
4. File downloads immediately to your browser's download folder

**File info:**
- Filename
- File size
- File type

### Project ZIP Archive

**Download entire project as ZIP:**

1. Navigate to project detail page
2. Click **"Download All as ZIP"** button
3. ZIP file is generated on-the-fly
4. Includes all STL files and images

**ZIP features:**
- Efficient streaming (doesn't load entire ZIP in memory)
- Maintains folder structure inside ZIP
- Named after the project
- Suitable for large projects (50+ files)

### Download Tips

- **Large files**: Be patient, downloads may take time
- **Multiple downloads**: You can queue multiple downloads
- **Interrupted downloads**: Browser will handle resume if supported

---

## Managing Your Library

### File Organization Best Practices

**Recommended structure:**
```
3d-prints/
├── Category1/
│   ├── Project1/
│   │   ├── model.stl
│   │   ├── photo1.jpg
│   │   └── photo2.jpg
│   └── Project2/
│       └── part.stl
└── Category2/
    └── Project3/
        ├── main.stl
        └── preview.png
```

**Guidelines:**
- One folder per project
- Keep STL files and images together
- Use descriptive folder names
- Maintain consistent naming conventions

### Adding New Projects

1. Add new folder with STL files to your root directory
2. Add images to the same folder (optional)
3. Run **Rescan** from the application
4. New project appears immediately
5. Add tags as needed

### Removing Projects

1. Delete the project folder from file system
2. Run **Rescan**
3. Project is removed from database
4. Tags are preserved (in case of re-add)
5. Cache is cleaned up automatically

### Updating Projects

**Add files to existing project:**
1. Copy new files to the project folder
2. Run **Rescan**
3. New files appear in listing

**Remove files from project:**
1. Delete files from file system
2. Run **Rescan**
3. Files are removed from listing

---

## Tips & Best Practices

### Performance Optimization

- **Limit image sizes**: Large images slow down loading
- **Use previews**: Let the system generate STL previews instead of large photos
- **Regular rescans**: Keep library in sync with file system
- **Tag consistently**: Use a consistent tagging scheme

### Organization Strategies

**By Print Status:**
- Tag projects as `printed`, `queued`, `testing`, `complete`
- Quickly find what you've already made

**By Material Requirements:**
- Tag with material types: `pla`, `petg`, `resin`
- Filter for compatible materials

**By Complexity:**
- Tag as `beginner`, `intermediate`, `advanced`
- Find projects matching your skill level

**By Source:**
- Track where designs came from
- Give credit to designers
- Manage licenses

### Workflow Examples

**Workflow 1: New Print**
1. Search or browse for a project
2. Check tags for print requirements
3. View images and STL previews
4. Download STL file
5. After printing, tag as `printed` and `tested`

**Workflow 2: Organizing Collection**
1. Scan entire library
2. Browse through projects
3. Add descriptive tags
4. Use search to verify organization
5. Create tag categories that make sense for you

**Workflow 3: Sharing Projects**
1. Find project to share
2. Download entire project as ZIP
3. ZIP includes all files
4. Share ZIP via email, cloud storage, etc.

---

## Troubleshooting

### Scan Issues

**Problem: Scan doesn't find projects**
- Check root path is correct
- Ensure folders contain `.stl` files
- Verify read permissions on directories
- Check scan error log

**Problem: Scan hangs or is very slow**
- Large library takes time (100+ projects = several minutes)
- Check for network-mounted drives (slow)
- Verify disk space available
- Check for corrupted STL files

### Preview Issues

**Problem: STL previews not generating**
- Verify stl-thumb is installed: `which stl-thumb`
- Check stl-thumb path in config
- Look at browser console for errors
- Fallback: system shows placeholder icons

**Problem: Preview images broken**
- Check cache directory has write permissions
- Verify sufficient disk space
- Try regenerating by rescanning

### Performance Issues

**Problem: Slow search results**
- Database may need optimization
- Check disk space for database growth
- Consider limiting number of projects
- Verify database file not corrupted

**Problem: Slow image loading**
- Check network connectivity (if remote storage)
- Verify cache is working
- Consider reducing image sizes
- Use lazy loading (enabled by default)

### Data Issues

**Problem: Lost tags after rescan**
- Tags are preserved by file path
- If you moved files, tags may not match
- Re-add tags if necessary
- Consider exporting tags before major reorganization

**Problem: Duplicate projects**
- Check for symbolic links in your directory structure
- Remove duplicate folders
- Rescan to clean up

**Problem: Missing files in project**
- Verify files exist in file system
- Check file extensions (must be .stl, .jpg, .png, etc.)
- Rescan to update database
- Check file permissions

### Browser Issues

**Problem: Application won't load**
- Check backend is running (port 3000)
- Check frontend is running (port 5173)
- Verify no firewall blocking
- Try clearing browser cache
- Check browser console for errors

**Problem: Can't click tiles/buttons**
- Try refreshing the page
- Check browser compatibility
- Disable browser extensions
- Try different browser

### Recovery

**If nothing works:**
1. Stop backend and frontend
2. Delete database file: `rm backend/glyptotheka.db`
3. Delete cache: `rm -rf backend/cache/`
4. Restart backend (recreates database)
5. Restart frontend
6. Reconfigure and rescan

**Backup before recovery:**
```bash
cp backend/glyptotheka.db backend/glyptotheka-backup.db
```

---

## Support

For additional help:
- Check README.md for setup instructions
- Review quickstart.md for validation steps
- Check API documentation for integration
- Open an issue on GitHub for bugs

---

**Last Updated**: 2025-11-16  
**Version**: Phase 9 - Polish & Testing
