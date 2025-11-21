# Feature Specification: Search View Refinement

**Feature Branch**: `001-search-refinement`  
**Created**: 2025-11-21  
**Status**: Draft  
**Input**: User description: "Create a specification for refining the search view in the Glyptotheka 3D Print Library application."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Filter Search to Leaf Projects Only (Priority: P0)

As a user searching for 3D printable models, I want search results to show only projects that contain STL files (leaf projects), so I don't waste time navigating through empty parent folders that only contain sub-projects.

**Why this priority**: This is the most critical improvement because it directly addresses user frustration with the current search behavior. Users searching for printable content should immediately see actionable results, not organizational folders. This is the foundation that makes the search feature truly useful.

**Independent Test**: Can be fully tested by performing searches with known hierarchical project structures (parent folders with sub-projects). Success means only projects containing STL files appear in results, and parent folders are excluded. Delivers immediate value by eliminating unnecessary navigation clicks.

**Acceptance Scenarios**:

1. **Given** a hierarchical project structure with parent folders containing only sub-projects, **When** I search for a term that matches both parent folders and leaf projects, **Then** only leaf projects with STL files are shown in results
2. **Given** I search for "Dwarf" in a database with parent folder "Cast'N'Play" and leaf project "819_Dwarf Gemtreasure Trader", **When** results are displayed, **Then** only "819_Dwarf Gemtreasure Trader" appears (parent folders excluded)
3. **Given** a search term that only matches parent folders with no leaf projects, **When** search executes, **Then** an empty result message is shown explaining no projects with STL files match
4. **Given** I search with no query term (empty search), **When** results load, **Then** all leaf projects across the library are shown
5. **Given** a leaf project with multiple STL files, **When** it appears in search results, **Then** the STL file count is displayed on the project tile

---

### User Story 2 - Visual Preview with Image Carousel (Priority: P1)

As a user browsing search results, I want to see an image carousel on each project tile showing both inherited images and STL preview thumbnails, so I can quickly assess whether a project matches what I'm looking for without clicking into the full project page.

**Why this priority**: Visual previews dramatically improve search usability by enabling quick assessment of project content. Users can browse multiple projects visually in seconds rather than clicking through each one. This transforms search from a text-based navigation tool into a visual discovery experience.

**Independent Test**: Can be fully tested by viewing search results for projects with various image configurations (some with images, some without, some with many images). Success means carousels display correctly, images load properly, and navigation controls work. Delivers value by enabling visual browsing of search results.

**Acceptance Scenarios**:

1. **Given** a search result for a project with inherited images from parent folders, **When** viewing the project tile, **Then** an image carousel displays those inherited images with navigation controls
2. **Given** a project with STL files that have generated preview thumbnails, **When** viewing the search result tile, **Then** the carousel includes STL preview images after inherited images
3. **Given** a project with no images or STL previews, **When** viewing the search result tile, **Then** a placeholder image or icon is shown with text "No images available"
4. **Given** a project with 25 images, **When** viewing the carousel in search results, **Then** only the first 15 images are loaded to maintain performance
5. **Given** I'm viewing a carousel with multiple images, **When** I click the "next" control, **Then** the next image is displayed with smooth transition
6. **Given** a carousel with 8 images, **When** viewing the tile, **Then** dot indicators show which image is currently displayed (e.g., ● ○ ○ ○ ○ ○ ○ ○)
7. **Given** images are loading slowly, **When** the carousel is displayed, **Then** a loading skeleton animation appears until images are ready

---

### User Story 3 - Image Carousel Auto-Advance (Priority: P2)

As a user browsing search results, I want the image carousels to optionally auto-advance through images, so I can passively view multiple images per project as I scroll through results.

**Why this priority**: Auto-advance enhances the visual browsing experience by animating carousels automatically. This is lower priority because manual navigation already provides core functionality. Auto-advance is a polish feature that improves discoverability but isn't essential.

**Independent Test**: Can be fully tested by loading search results and observing carousel behavior over time. Success means carousels advance automatically at appropriate intervals without disrupting user interaction. Delivers value by creating a more dynamic and engaging search interface.

**Acceptance Scenarios**:

1. **Given** a search result tile with multiple images, **When** I view the tile without interacting, **Then** the carousel automatically advances to the next image after 5 seconds
2. **Given** an auto-advancing carousel, **When** I manually click "next" or "previous", **Then** auto-advance pauses for 10 seconds before resuming
3. **Given** a carousel currently auto-advancing, **When** I hover over the carousel area, **Then** auto-advance is paused until I move my cursor away
4. **Given** multiple search result tiles visible on screen, **When** carousels are auto-advancing, **Then** they advance at staggered intervals (not all at once) to avoid visual overload

---

### Edge Cases

- **No images available**: Display a placeholder icon with "No images available" text in a visually consistent style
- **Project with >20 images**: Limit carousel to first 15 images; display image count indicator like "15+ images" to show truncation
- **Large image files (>5MB)**: Use thumbnail/preview versions if available; implement lazy loading so images load only when tile is visible in viewport
- **Slow image loading**: Show skeleton loading animation; if image fails to load after 10 seconds, show broken image placeholder with retry option
- **Mixed inherited and STL preview images**: Display inherited images first, then STL previews; optionally add subtle caption indicating source (e.g., "STL Preview" label)
- **Single image projects**: Hide carousel navigation controls (prev/next arrows) and show static image; dot indicators hidden for single images
- **Very long project names**: Truncate project name in tile header with ellipsis; show full name on hover tooltip
- **Search with zero leaf project matches**: Display user-friendly message "No projects with STL files found. Try different search terms or browse all projects."
- **Carousel navigation on touch devices**: Support swipe gestures for next/previous in addition to button controls
- **Accessibility**: Ensure carousel has keyboard navigation (arrow keys) and screen reader support (announcing current image index)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST filter search results to only include projects where `is_leaf = true` (projects containing at least one STL file)
- **FR-002**: System MUST exclude parent folders that contain only sub-projects from search results
- **FR-003**: Search API MUST accept a `leaf_only` parameter (default `true`) to enable/disable leaf project filtering
- **FR-004**: System MUST aggregate images from both the project folder and all parent folders in the hierarchy
- **FR-005**: System MUST include STL preview thumbnail images for all STL files within a project
- **FR-006**: System MUST combine inherited images and STL preview images into a single ordered list (inherited first, STL previews second)
- **FR-007**: Each search result tile MUST display an image carousel component showing the combined image list
- **FR-008**: Image carousel MUST provide manual navigation controls (previous/next buttons)
- **FR-009**: Image carousel MUST display dot indicators showing total image count and current position
- **FR-010**: System MUST limit carousel images to a maximum of 15 images per project to maintain performance
- **FR-011**: Image carousel MUST display a placeholder when no images are available for a project
- **FR-012**: System MUST implement lazy loading for carousel images (load only when tile is visible in viewport)
- **FR-013**: Image carousel MUST show loading skeleton animation while images are being fetched
- **FR-014**: Each search result tile MUST display the count of STL files in the project (e.g., "8 STL files")
- **FR-015**: System MUST return image metadata including image ID, path, and type (inherited vs STL preview) for each image
- **FR-016**: Image carousel MUST pause auto-advance when user hovers over carousel area
- **FR-017**: Image carousel MUST pause auto-advance for 10 seconds after manual navigation
- **FR-018**: System MUST maintain search functionality for project names, tags, and paths when filtering to leaf projects
- **FR-019**: Empty search results MUST display a helpful message indicating no leaf projects match the search criteria
- **FR-020**: Image carousel MUST be responsive and adapt to different tile sizes in the project grid layout

### Key Entities

- **Leaf Project**: A project that contains at least one STL file directly (not just through sub-projects). Identified by `is_leaf = true` flag in the project model. Key attributes include project name, path, parent hierarchy, STL file list, and associated images.

- **Image**: Visual content associated with a project, either an inherited image from the project/parent folders or an auto-generated STL preview thumbnail. Key attributes include image ID, file path, image type (inherited vs stl_preview), and source project/STL file reference.

- **Search Result**: A leaf project returned by the search query, enriched with aggregated image data. Contains project metadata (name, path, tags, STL count) plus ordered image list for carousel display.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can identify relevant 3D printable projects in search results 50% faster compared to current implementation (measured by time from search query to clicking desired project)
- **SC-002**: Zero clicks on parent folders in search results (validated by analytics showing 0% click-through rate on non-leaf items)
- **SC-003**: Search result pages load and display within 2 seconds for queries returning up to 50 projects
- **SC-004**: Image carousels render within 1 second of search results appearing on screen
- **SC-005**: 90% of users successfully use carousel navigation controls on first attempt (measured by click tracking on prev/next buttons)
- **SC-006**: Image carousel performance remains smooth (60 FPS) when scrolling through search results with multiple visible carousels
- **SC-007**: 80% reduction in unnecessary project page visits (users finding desired content directly from search results)
- **SC-008**: Zero failed image loads for properly generated STL previews and valid inherited images
- **SC-009**: Search results accurately reflect leaf project filter (100% of results must be projects with STL files)
- **SC-010**: Users can browse 20+ search results in under 30 seconds by using carousel previews without opening individual projects

## Assumptions

1. **STL Preview Generation**: Assumed that STL preview thumbnails are already generated by existing backend service and stored in accessible location
2. **Image Storage**: Assumed inherited images are stored in project folders with predictable naming/location patterns accessible via API
3. **Database Schema**: Assumed `is_leaf` flag exists or can be added to project model without major schema migration
4. **Performance**: Assumed database can efficiently query leaf projects and aggregate images without significant performance degradation up to 10,000+ projects
5. **Image Format**: Assumed images are in web-compatible formats (JPEG, PNG, WebP) and thumbnails are available or can be generated
6. **Carousel Component**: Assumed existing `ImageCarousel.tsx` component can be adapted or new component created without major architectural changes
7. **API Capability**: Assumed search API can be extended to include image data without breaking existing clients
8. **Browser Support**: Assumed target browsers support modern CSS/JavaScript features needed for carousel (CSS Grid, Flexbox, async/await)
9. **User Behavior**: Assumed users prefer visual browsing and will benefit from seeing images in search results rather than text-only tiles
10. **Content Availability**: Assumed majority of leaf projects have at least one image (inherited or STL preview) available for display
