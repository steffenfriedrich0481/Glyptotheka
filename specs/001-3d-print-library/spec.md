# Feature Specification: 3D Print Model Library

**Feature Branch**: `001-3d-print-library`  
**Created**: 2025-11-16  
**Status**: Draft  
**Input**: User description: "Create a feature specification for a 3D print model files library (mediathek) web application"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Browse and Navigate 3D Print Projects (Priority: P1)

A user wants to explore their collection of 3D print models organized in folders. They specify a root folder path, and the system scans it to discover all projects. The user can then navigate through a tile-based interface, clicking on folder tiles to drill down from parent categories (e.g., "miniatures") to specific projects (e.g., "wiking"). Images associated with each level are displayed on the tiles, making visual navigation intuitive.

**Why this priority**: This is the core value proposition - users need to see and navigate their existing collection before any other functionality makes sense. Without this, the application has no foundation.

**Independent Test**: Can be fully tested by specifying a folder with 3D models, observing the tile-based display, clicking through the hierarchy, and verifying that the correct projects and images are shown at each level. Delivers immediate value by making an existing collection browsable.

**Acceptance Scenarios**:

1. **Given** a user has specified a root folder containing 3D print projects, **When** the system completes scanning, **Then** the main screen displays all top-level folders and projects as tiles with associated images
2. **Given** the user is viewing a folder tile (e.g., "miniatures"), **When** they click on it, **Then** the system displays all child folders and projects within that folder as tiles
3. **Given** a parent folder contains images, **When** viewing its child projects, **Then** those parent images are inherited and displayed alongside project-specific images
4. **Given** a user is viewing a project (e.g., "wiking"), **When** they are on the project detail view, **Then** they see all associated images from the project folder and parent folders
5. **Given** a user is at any level in the hierarchy, **When** they use breadcrumb navigation or back button, **Then** they return to the previous level

---

### User Story 2 - Search and Filter Projects (Priority: P2)

A user has a large collection of 3D print models and wants to quickly find specific projects. They can search by project name or by tags they've previously assigned. Search results display as tiles, maintaining the same visual interface, and users can click on results to navigate to those projects.

**Why this priority**: Once users have a browsable collection (P1), search becomes essential for collections of any significant size. This enables quick access without manual navigation through deep folder structures.

**Independent Test**: Can be tested by adding tags to several projects, then using the search functionality to find projects by name or tag. Delivers value by providing quick access to specific content in large collections.

**Acceptance Scenarios**:

1. **Given** a user is on any screen in the application, **When** they enter text in the search box, **Then** the system displays all projects whose names contain that text
2. **Given** a user has assigned tags to projects, **When** they search by a tag name, **Then** the system displays all projects with that tag
3. **Given** search results are displayed, **When** the user clicks on a result tile, **Then** they are taken to that project's detail view
4. **Given** no projects match the search criteria, **When** the search completes, **Then** the system displays a "no results found" message with suggestions to refine the search
5. **Given** a user is viewing search results, **When** they clear the search, **Then** the system returns to the previous navigation context

---

### User Story 3 - Download Project Files (Priority: P2)

A user wants to use their 3D print models on another device or share them. From a project's detail view, they can download individual files (STL files or specific images) or download all project files as a single ZIP archive for convenience.

**Why this priority**: After finding a project (P1 or P2), users need to actually use the files. This completes the core workflow: browse/search → view → download → use.

**Independent Test**: Can be tested by navigating to a project, clicking download buttons for individual files and for the complete ZIP archive, and verifying the correct files are downloaded. Delivers value by enabling practical use of the stored 3D models.

**Acceptance Scenarios**:

1. **Given** a user is viewing a project detail page, **When** they click on an individual STL file download button, **Then** that STL file is downloaded to their device
2. **Given** a user is viewing a project detail page, **When** they click on an individual image download button, **Then** that image file is downloaded to their device
3. **Given** a user is viewing a project detail page, **When** they click "Download All as ZIP", **Then** a ZIP archive containing all project files (STL and images) is created and downloaded
4. **Given** a download is in progress, **When** the user waits, **Then** they see download progress indication
5. **Given** a download fails, **When** the error occurs, **Then** the user sees an error message and can retry the download

---

### User Story 4 - Tag and Organize Projects (Priority: P3)

A user wants to organize their 3D print projects beyond the folder structure. They can assign custom tags to projects (e.g., "painted", "printed", "priority", "fantasy", "sci-fi") to create cross-cutting organizational dimensions. These tags persist in the database and enable tag-based searching.

**Why this priority**: This enhances organization but isn't essential for basic usage. Users can browse and download (P1-P2) without tagging. Tags add value for power users with large, complex collections.

**Independent Test**: Can be tested by navigating to a project, adding/removing tags, and verifying tags persist across sessions and appear in search results. Delivers value through flexible, user-defined organization.

**Acceptance Scenarios**:

1. **Given** a user is viewing a project detail page, **When** they add a new tag, **Then** the tag is saved to the database and displayed on the project
2. **Given** a project has existing tags, **When** the user removes a tag, **Then** the tag is deleted from the database and no longer displayed
3. **Given** a user has tagged multiple projects with the same tag, **When** they search by that tag, **Then** all tagged projects appear in results
4. **Given** a user is adding a tag, **When** they start typing, **Then** the system suggests existing tags to maintain consistency
5. **Given** tags have been added to projects, **When** the user rescans the file system, **Then** all tags persist and remain associated with the correct projects

---

### User Story 5 - Rescan and Update Library (Priority: P3)

A user has added new 3D print files to their root folder or removed old ones. They can trigger a rescan of the file system, and the system updates the database to reflect changes - adding new projects, removing deleted ones, and updating project information. Previously assigned tags remain intact for projects that still exist.

**Why this priority**: This is maintenance functionality that enhances long-term usability but isn't needed for initial use. Users need to set up and use the library (P1-P3) before update functionality becomes relevant.

**Independent Test**: Can be tested by adding/removing files in the root folder, triggering a rescan, and verifying the library reflects the changes while preserving tags. Delivers value by keeping the library synchronized with the file system.

**Acceptance Scenarios**:

1. **Given** new 3D print projects have been added to the root folder, **When** the user triggers a rescan, **Then** new projects appear in the library
2. **Given** projects have been deleted from the file system, **When** the user triggers a rescan, **Then** those projects are removed from the library database
3. **Given** existing projects have new images added, **When** the user triggers a rescan, **Then** the new images are cached and displayed
4. **Given** projects had tags before a rescan, **When** those projects still exist after rescanning, **Then** their tags are preserved
5. **Given** a rescan is in progress, **When** the user waits, **Then** they see progress indication showing how many folders/files have been processed

---

### Edge Cases

- What happens when a folder contains both STL files and subfolders with STL files (mixed hierarchy)?
  - The folder becomes both a project (for its own STL files) and a parent category (for subfolders)
  
- What happens when an STL file has no associated images in its folder or any parent folders?
  - System uses a generated preview from the STL file itself or displays a default placeholder image
  
- What happens when scanning encounters permission errors or inaccessible folders?
  - System logs the error, skips that folder, and continues scanning other accessible folders; user sees a summary of skipped folders
  
- What happens when multiple projects have the same name in different folders?
  - Projects maintain their full path context, so "miniatures/wiking" and "vehicles/wiking" are distinct projects
  
- What happens when a user downloads a large ZIP file and it exceeds browser limitations?
  - System streams the ZIP file for all sizes, allowing users to download projects of any size without browser memory limitations
  
- What happens when the file system changes during a scan?
  - System completes the scan with the state at scan time; changes after scan start will be picked up in the next scan
  
- What happens when image files are in unsupported formats?
  - System logs unsupported formats and skips them, displaying only supported image types (common formats: JPG, PNG, GIF, WebP)
  
- What happens when a project folder contains hundreds of images?
  - System displays the first 20 images immediately for quick preview, with pagination controls to access additional images as needed

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST allow users to specify a root folder path to scan for 3D print models
- **FR-002**: System MUST recursively scan the specified folder for STL files and image files
- **FR-003**: System MUST identify folders containing STL files as "3D print projects"
- **FR-004**: System MUST associate images from a project folder and all parent folders with that project
- **FR-005**: System MUST persist all project information (paths, file references, hierarchy) in a database
- **FR-006**: System MUST cache all discovered images during scanning
- **FR-007**: System MUST generate preview images from STL files
- **FR-008**: System MUST cache generated STL preview images
- **FR-009**: System MUST display the folder/project hierarchy as a tile-based interface
- **FR-010**: System MUST allow users to navigate through the hierarchy by clicking on tiles
- **FR-011**: System MUST display associated images on folder and project tiles
- **FR-012**: System MUST allow users to assign custom tags to projects
- **FR-013**: System MUST persist project tags in the database
- **FR-014**: System MUST provide search functionality by project name
- **FR-015**: System MUST provide search functionality by project tags
- **FR-016**: System MUST allow users to download individual STL files from a project
- **FR-017**: System MUST allow users to download individual image files from a project
- **FR-018**: System MUST allow users to download all project files as a ZIP archive
- **FR-019**: System MUST support rescanning the root folder to update project information
- **FR-020**: System MUST preserve user-assigned tags when rescanning and updating projects
- **FR-021**: System MUST handle hierarchical project structures where a parent folder contains both images and subfolders with projects
- **FR-022**: System MUST recognize supported image formats (JPG, PNG, GIF, WebP at minimum)
- **FR-023**: System MUST provide breadcrumb or back navigation to move up the folder hierarchy
- **FR-024**: System MUST display progress indication during folder scanning operations
- **FR-025**: System MUST display progress indication during file download operations
- **FR-026**: System MUST handle errors gracefully (permission errors, unsupported files, missing files) and continue operation
- **FR-027**: System MUST log errors encountered during scanning for user review
- **FR-028**: System MUST distinguish between projects with the same name in different folders using full path context
- **FR-029**: System MUST stream ZIP file downloads to handle projects of any size without browser memory limitations
- **FR-030**: System MUST display the first 20 images for projects with multiple images
- **FR-031**: System MUST provide pagination controls to access additional images beyond the first 20

### Key Entities

- **Project**: Represents a 3D print project, identified by a folder containing at least one STL file. Attributes include name (derived from folder name), full path, parent project reference (if any), associated STL files, associated images (from project folder and inherited from parent folders), and user-assigned tags.

- **STL File**: Represents a 3D model file within a project. Attributes include file name, file path, file size, and reference to cached preview image.

- **Image File**: Represents an image associated with a project. Attributes include file name, file path, file size, association level (project-specific or inherited from parent), and reference to cached copy.

- **Tag**: Represents a custom label assigned to projects. Attributes include tag name and list of projects associated with this tag.

- **Folder/Category**: Represents a parent folder that contains multiple child projects or subfolders. Attributes include name, full path, associated images, and references to child folders/projects.

- **Scan Session**: Represents a file system scanning operation. Attributes include start time, completion time, root folder path, number of projects found, number of files processed, and errors encountered.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can specify a root folder and complete initial library setup within 5 minutes (excluding scanning time)
- **SC-002**: System scans and indexes at least 100 projects per minute on standard hardware
- **SC-003**: Users can navigate from root to a specific project within 3 clicks for reasonably organized collections (depth ≤ 3 levels)
- **SC-004**: Search results appear within 1 second for collections up to 10,000 projects
- **SC-005**: Users can complete the find-and-download workflow (search → open project → download) in under 30 seconds
- **SC-006**: Generated STL preview images are visually recognizable to users (90% of users can identify the model from the preview)
- **SC-007**: Tile-based navigation displays within 2 seconds at each hierarchy level
- **SC-008**: System successfully handles collections of at least 10,000 projects without performance degradation
- **SC-009**: ZIP file generation and download initiation occurs within 10 seconds for projects containing up to 50 files
- **SC-010**: Rescanning operations correctly identify 100% of added, removed, and modified projects
- **SC-011**: Users successfully find desired projects using search in 95% of attempts (with appropriate tagging)
- **SC-012**: System recovers gracefully from at least 95% of common errors (permission issues, corrupted files, missing files) without requiring restart

## Assumptions

- Users organize their 3D print files in a hierarchical folder structure
- STL files are the primary 3D model format; other formats (OBJ, 3MF) may be added later but are out of scope for initial version
- Users have read access to the folders they specify for scanning
- The root folder structure remains relatively stable (not constantly changing during use)
- Collections typically range from 100 to 10,000 projects; larger collections may require additional optimization
- Image files associated with projects are stored in the same folder as the STL files or in parent folders
- Users want to browse visually first, then use search for specific items
- Downloaded files are used outside the application (in slicing software, for printing, etc.)
- A single-user local application is acceptable; multi-user concurrent access is not required for initial version
- Tags are simple text labels without hierarchy or relationships
- Preview images generated from STL files show a default angle/perspective that makes models recognizable
- Performance expectations are based on modern consumer hardware (not mobile devices or low-end machines)

## Out of Scope

The following are explicitly excluded from this feature:

- **3D Model Editing**: Users cannot modify STL files within the application
- **Slicing Integration**: No direct integration with slicing software (Cura, PrusaSlicer, etc.)
- **Print Job Management**: No tracking of actual print jobs or printer status
- **Multi-User Support**: No user accounts, permissions, or sharing between users
- **Cloud Storage**: No cloud backup or synchronization features
- **Model Marketplace**: No ability to download models from online repositories
- **Version Control**: No tracking of model file versions or changes over time
- **Print History**: No logging of which models have been printed or when
- **Filament Management**: No tracking of filament inventory or usage
- **3D Model Preview Rotation**: Initial version shows static preview images, not interactive 3D viewers
- **Automatic Categorization**: No AI/ML-based automatic tagging or categorization
- **Batch Operations**: No bulk tagging, moving, or deleting of multiple projects at once (may be added in future versions)
- **Export/Import**: No ability to export library database or import from other applications
- **Mobile Application**: Web interface optimized for desktop browsers; mobile optimization is future work
