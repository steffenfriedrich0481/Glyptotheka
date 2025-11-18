# Feature Specification: STL Preview Image Generation During Scanning

**Feature Branch**: `002-stl-preview-generation`  
**Created**: 2025-11-18  
**Status**: Draft  
**Input**: User description: "Create a feature specification for STL preview image generation during scanning."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Automatic STL Preview Generation on Scan (Priority: P1)

When a user scans a project folder containing STL files, the system automatically generates preview images for each STL file and makes them available alongside regular project images.

**Why this priority**: This is the core functionality that delivers immediate value - users can visually browse their STL files without opening 3D modeling software. This forms the foundation for all other features.

**Independent Test**: Can be fully tested by scanning a folder with STL files and verifying preview images are generated and accessible. Delivers standalone value of visual STL browsing.

**Acceptance Scenarios**:

1. **Given** a project folder contains 3 STL files and 2 JPG images, **When** the user scans the folder, **Then** preview images are generated for all 3 STL files and stored in the cache directory
2. **Given** an STL file has an existing preview image, **When** the user rescans and the STL file has not been modified, **Then** the existing preview is reused without regeneration
3. **Given** an STL file has been modified since the last scan, **When** the user rescans, **Then** a new preview image is generated replacing the old one
4. **Given** the first 2 STL files in a folder are being processed, **When** preview generation starts, **Then** these previews are generated before the scan completes (synchronous)
5. **Given** a folder contains more than 2 STL files, **When** the scan processes them, **Then** the first 2 are generated synchronously and the remaining ones are generated asynchronously in the background

---

### User Story 2 - STL Previews in Project Image Gallery (Priority: P2)

When a user views project images, STL preview images appear in the gallery with lower priority than regular photos, ensuring actual photos are shown first but STL previews are available as fallback.

**Why this priority**: This integrates STL previews into existing workflows, enhancing the user experience without disrupting the current image prioritization. Essential for visual project browsing.

**Independent Test**: Can be tested by viewing a project's image gallery and verifying STL previews appear with correct ranking. Delivers value of unified visual browsing.

**Acceptance Scenarios**:

1. **Given** a project has 3 regular images and 2 STL preview images, **When** the user retrieves project images, **Then** the 3 regular images are returned first, followed by the 2 STL previews
2. **Given** a project has only STL files (no regular images), **When** the user views the project, **Then** STL preview images are displayed as the primary visual representation
3. **Given** a child folder has no regular images but parent folder has both regular images and STL previews, **When** the user views the child folder, **Then** parent's regular images are inherited first, then STL previews

---

### User Story 3 - STL Previews in Composite Previews (Priority: P3)

When the system generates composite preview images (combining multiple images into one), STL preview images are considered as candidates but prioritized lower than regular images.

**Why this priority**: This enhances the composite preview feature by ensuring projects without regular photos still get visual representation through STL previews. Adds polish to the user experience.

**Independent Test**: Can be tested by generating composite previews for projects with various combinations of regular images and STL previews. Delivers value of comprehensive visual project summaries.

**Acceptance Scenarios**:

1. **Given** a project has 6 regular images and 4 STL previews, **When** a composite preview is generated, **Then** the composite uses the first 4 regular images only
2. **Given** a project has 2 regular images and 4 STL previews, **When** a composite preview is generated, **Then** the composite uses the 2 regular images first, then fills remaining slots with 2 STL previews
3. **Given** a project has only 3 STL files and no regular images, **When** a composite preview is generated, **Then** the composite uses up to 3 STL preview images

---

### User Story 4 - Graceful Handling of Preview Generation Failures (Priority: P2)

When STL preview generation fails (corrupted file, missing tool, insufficient resources), the system continues scanning and logs the issue without blocking the entire scan process.

**Why this priority**: Ensures system reliability and resilience. A single problematic STL file should not prevent users from scanning their entire project library.

**Independent Test**: Can be tested by scanning folders with corrupted STL files or disabling the preview generation tool. Delivers value of reliable scanning even with problematic files.

**Acceptance Scenarios**:

1. **Given** a project folder contains a corrupted STL file, **When** the scan attempts to generate a preview, **Then** a warning is logged, the STL preview is skipped, and the scan continues with other files
2. **Given** the stl-thumb tool is not available, **When** the scan encounters an STL file, **Then** a warning is logged once, STL preview generation is disabled for the scan, and regular scanning continues
3. **Given** STL preview generation times out after a reasonable duration, **When** processing a complex STL file, **Then** the preview generation is cancelled, a warning is logged, and the scan continues

---

### Edge Cases

- What happens when a project contains hundreds of STL files? (System should generate first 2 synchronously, queue the rest for background processing, and indicate progress to user)
- What happens when the cache directory is full or has write permission issues? (System logs error, disables STL preview generation for the scan, continues with other operations)
- What happens when an STL file is extremely large (e.g., 500MB)? (System should enforce file size limits for preview generation, skip files exceeding the limit with a logged warning)
- What happens when the same STL file exists in multiple project folders? (Each location gets its own preview copy to maintain independence between projects)
- What happens when STL preview generation is still running from a previous scan when a new scan starts? (System should allow only one preview generation session per STL file, newer requests wait or queue)
- What happens when the stl-thumb binary crashes during generation? (System catches the error, logs it, and continues scan without that preview)
- What happens when a user deletes an STL file but the preview still exists in cache? (Orphaned previews are acceptable; a cleanup routine can be implemented separately as maintenance)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST detect all STL files (files with .stl extension) during project folder scanning
- **FR-002**: System MUST generate preview images for STL files using the stl-thumb external tool located at ../stl-thumb relative to project root
- **FR-003**: System MUST store generated STL preview images in a dedicated cache directory at cache/stl-previews/
- **FR-004**: System MUST use a hybrid generation approach: generate previews for the first 2 STL files synchronously during scan, and queue remaining STL files for asynchronous background generation
- **FR-005**: System MUST implement smart regeneration: only regenerate an STL preview if the STL file's modification timestamp is newer than the existing preview image's timestamp
- **FR-006**: System MUST treat STL preview images as project images that can be retrieved via the same API endpoints as regular images
- **FR-007**: System MUST assign lower ranking/priority to STL preview images compared to regular images (JPG, PNG, etc.) found in project folders
- **FR-008**: System MUST include STL preview images in the pool of candidates when generating composite project previews
- **FR-009**: System MUST respect image prioritization in composite previews: use regular images first to fill all available slots, then use STL preview images only if slots remain unfilled
- **FR-010**: System MUST integrate the existing StlPreviewService (backend/src/services/stl_preview.rs) for all STL preview generation operations
- **FR-011**: System MUST fail gracefully when STL preview generation fails: log a warning message, skip that STL file's preview generation, and continue scanning other files
- **FR-012**: System MUST continue the entire scan process even if the stl-thumb tool is unavailable or encounters errors
- **FR-013**: System MUST support STL preview regeneration during rescan operations using the same smart regeneration logic
- **FR-014**: System MUST inherit STL preview images from parent folders to child folders following the same inheritance rules as regular images, but with lower priority
- **FR-015**: System MUST generate STL previews with consistent naming that maps back to the source STL file for cache management and retrieval

### Key Entities *(include if feature involves data)*

- **STL Preview Image**: A generated preview image representing an STL file; attributes include source STL file path, preview image file path, generation timestamp, preview image dimensions, and file format (likely PNG or JPG)
- **STL File**: The source 3D model file; attributes include file path, file size, modification timestamp, and associated preview image reference
- **Image Ranking**: A prioritization system for project images; attributes include image source type (regular image vs STL preview), rank value (regular images have higher rank), and position in display order
- **Cache Entry**: A record tracking generated previews; attributes include cache key (derived from STL file path), cache file path, creation timestamp, and validity status

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: STL preview images are successfully generated for at least 95% of valid STL files during scanning operations
- **SC-002**: Scan completion time increases by no more than 10% when processing folders with STL files compared to folders with only regular images (thanks to hybrid sync/async generation)
- **SC-003**: Users can retrieve and view STL preview images within 2 seconds of scan completion for the first 2 STL files in a folder
- **SC-004**: Regular project images always appear before STL preview images in project galleries 100% of the time
- **SC-005**: Composite previews correctly include STL preview images when fewer than 4 regular images are available
- **SC-006**: The system successfully continues scanning and completes the operation even when STL preview generation fails for individual files in at least 99% of scan operations
- **SC-007**: Smart regeneration reduces redundant preview generation by at least 90% on rescan operations when STL files have not been modified
- **SC-008**: Preview generation for a typical STL file (under 50MB) completes within 30 seconds on standard hardware
- **SC-009**: Users can visually browse project STL files without needing to open external 3D modeling software
- **SC-010**: The system maintains stable memory usage during background STL preview generation, with no more than 500MB additional memory consumption during async processing


## Assumptions

- The stl-thumb binary is available at ../stl-thumb relative to the project root and is executable
- The cache/stl-previews/ directory can be created if it does not exist and has write permissions
- STL files follow standard STL format specifications (both ASCII and binary formats)
- The existing StlPreviewService in backend/src/services/stl_preview.rs has the necessary interfaces to be integrated into the scanning workflow
- Preview image format will be PNG or JPG with reasonable dimensions (e.g., 512x512 or 1024x1024 pixels) for gallery display
- The system has sufficient disk space to store STL preview images in cache (previews are typically 50-500KB each)
- File modification timestamps are reliable indicators of file changes
- The scanner service has access to file system metadata (modification dates, file sizes)
- Background/asynchronous processing infrastructure exists or can be added to handle queued STL preview generation
- A reasonable timeout for STL preview generation is 30-60 seconds per file to prevent blocking on extremely complex models

## Dependencies

- **stl-thumb tool**: External binary required for generating STL thumbnail images; must be available at specified location
- **Existing StlPreviewService**: The backend/src/services/stl_preview.rs service must be functional and provide the necessary API for preview generation
- **Scanner Service**: Must be extended to detect STL files and trigger preview generation during scan operations
- **Rescan Service**: Must support triggering STL preview regeneration with smart update logic
- **Image Retrieval API**: Must be updated to include STL preview images in responses with appropriate ranking/priority
- **Composite Preview Service**: Must be updated to consider STL previews as candidate images when generating composite views
- **Cache Management**: Requires cache directory structure and management for storing generated previews
- **Database Schema**: May require updates to track STL preview images separately from regular images or add metadata fields for image type/priority
- **Background Job System**: Requires async processing capability for queued STL preview generation

## Scope

### In Scope

- Automatic STL preview generation during project scanning
- Smart regeneration based on file modification timestamps
- Hybrid synchronous/asynchronous generation (first 2 sync, rest async)
- Integration with existing image retrieval and ranking systems
- STL preview inclusion in composite preview generation
- Graceful error handling and scan continuation on failures
- Cache storage and management for STL preview images
- Image inheritance for STL previews following existing patterns

### Out of Scope

- Manual STL preview regeneration triggered by users (future enhancement)
- Customizable preview image dimensions or quality settings (use defaults)
- Preview generation for other 3D file formats (OBJ, FBX, GLTF, etc.)
- 3D model validation or repair before preview generation
- Progress indicators or notifications for background preview generation (future enhancement)
- Cache cleanup routines for orphaned previews (future maintenance feature)
- Batch preview generation tools separate from scanning (future enhancement)
- Custom preview angles or rendering settings (use stl-thumb defaults)
- Preview generation statistics or analytics (future enhancement)
