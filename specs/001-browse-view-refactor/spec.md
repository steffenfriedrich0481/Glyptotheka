# Feature Specification: Browse View Refactoring - File Explorer Style with Image Inheritance

**Feature Branch**: `001-browse-view-refactor`  
**Created**: 2025-11-28  
**Status**: Draft  
**Input**: User description: "Browse View Refactoring - File Explorer Style with Image Inheritance"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Folder-by-Folder Navigation (Priority: P1)

Users can navigate through the project folder hierarchy one level at a time, similar to a file explorer, seeing contained folders and projects at each level before descending deeper.

**Why this priority**: This is the core navigation model that the entire feature depends on. Without folder-by-folder navigation, none of the other features (image inheritance, project previews) can function properly.

**Independent Test**: Can be fully tested by navigating through the folder structure (e.g., starting at "Miniaturen", clicking into "The Printing Goes Ever On", then "Welcome Trove") and verifying that each level shows only the immediate children folders/projects without automatically expanding the entire tree.

**Acceptance Scenarios**:

1. **Given** a user is viewing the root browse view, **When** they click on a folder, **Then** they navigate into that folder and see only its immediate children (subfolders and projects)
2. **Given** a user is viewing a folder at any level, **When** they click on a subfolder, **Then** they descend one level deeper into that subfolder
3. **Given** a user is viewing a nested folder, **When** they use breadcrumb navigation, **Then** they can navigate back up to parent folders

---

### User Story 2 - Project Preview Display (Priority: P2)

Users can see visual previews of projects contained within the current folder level, allowing them to identify interesting projects before clicking through.

**Why this priority**: Provides essential context and visual navigation aids, making it easier to find desired content. Depends on P1 navigation working first.

**Independent Test**: Can be tested by viewing any folder that contains projects and verifying that each project displays a preview image (either its own images or inherited ones).

**Acceptance Scenarios**:

1. **Given** a folder contains multiple projects, **When** viewing that folder level, **Then** each project shows a preview with at least one representative image
2. **Given** a project has its own images, **When** viewing its parent folder, **Then** those images appear in the project's preview
3. **Given** a project has no direct images but inherits them, **When** viewing its parent folder, **Then** the inherited images appear in the project's preview

---

### User Story 3 - Image Inheritance Down Hierarchy (Priority: P3)

Images found at any level in the folder structure are inherited and displayed by all projects below them in the hierarchy, providing visual context throughout nested structures.

**Why this priority**: Enhances user experience by showing relevant imagery throughout the hierarchy, but the browse view is still functional without it. Builds upon P1 and P2.

**Independent Test**: Can be tested by examining the "Welcome Trove/heroes fighting.jpg" image and verifying it appears in all descendant projects like "Welcome-Trove-Remastered" and "Samuel" subdirectories.

**Acceptance Scenarios**:

1. **Given** an image exists in folder "A", **When** viewing project "A/B", **Then** that image is displayed for project "A/B"
2. **Given** an image exists in folder "A", **When** viewing project "A/B/C", **Then** that image is inherited and displayed for project "A/B/C"
3. **Given** an image "X.jpg" appears at multiple levels, **When** displaying inherited images, **Then** "X.jpg" is deduplicated and shown only once
4. **Given** a project has both inherited images and its own images, **When** viewing that project, **Then** both inherited and own images are displayed together

---

### User Story 4 - Substring Keyword Matching for STL Categorization (Priority: P4)

The system uses substring matching (case-insensitive) for ignored keywords to correctly identify STL container folders versus actual projects, enabling proper categorization of STL files.

**Why this priority**: Improves accuracy of project detection and STL categorization but doesn't block core navigation functionality. Can be refined after basic browsing works.

**Independent Test**: Can be tested by verifying that folders like "1 inch" and "2 inch" are treated as STL categories under the "Desert" project when "inch" is in the IGNORED_KEYWORDS list.

**Acceptance Scenarios**:

1. **Given** IGNORED_KEYWORDS contains "inch", **When** scanning a folder named "1 inch", **Then** it is recognized as an STL container (not a project)
2. **Given** IGNORED_KEYWORDS contains "STL", **When** scanning a folder named "PRESUPPORTED_STL", **Then** it is recognized as an STL container through substring matching
3. **Given** a project folder contains STL category subfolders, **When** viewing that project, **Then** the STL files are grouped by their category folder names
4. **Given** multiple STL categories exist in a project, **When** browsing that project, **Then** all categories are displayed with their respective STL files

---

### Edge Cases

- What happens when a folder contains no images and has no parent images to inherit? Display a placeholder or default icon.
- How does the system handle circular folder references or symlinks? Follow standard filesystem behavior (ignore or break cycles).
- What happens when an image file is corrupt or cannot be loaded? Skip it and show remaining valid images.
- How are STL files displayed when their parent folder name matches multiple keywords? Use the first matching keyword for categorization.
- What happens when a folder contains both projects and STL files directly? Treat STL files as belonging to the current project level.
- How does the system handle very deep folder hierarchies (10+ levels)? Navigation should remain responsive; consider lazy loading or pagination if needed.
- What happens when folder names contain special characters or very long names? Display truncated names with tooltips showing full text.
- How are duplicate image names handled across different folder levels? Show each unique path once, preferring closer/more specific images.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST display folders and projects at the current navigation level without automatically expanding child folders
- **FR-002**: System MUST allow users to click on a folder to navigate one level deeper into that folder's contents
- **FR-003**: System MUST provide breadcrumb navigation allowing users to navigate back up to any parent folder level
- **FR-004**: System MUST display preview images for each project visible at the current folder level
- **FR-005**: System MUST inherit images from parent folders and make them available to all descendant projects
- **FR-006**: System MUST deduplicate inherited images by filename, showing each unique image only once per project
- **FR-007**: System MUST use case-insensitive substring matching when checking folder names against IGNORED_KEYWORDS list
- **FR-008**: System MUST treat folders whose names contain any IGNORED_KEYWORDS substring as STL container folders (not projects)
- **FR-009**: System MUST group STL files under their category folder names when displaying a project
- **FR-010**: System MUST display both inherited images and project-specific images together when viewing a project
- **FR-011**: System MUST maintain responsive navigation performance regardless of folder depth
- **FR-012**: System MUST display folder/project names clearly, truncating long names with tooltips if necessary

### Key Entities

- **Folder**: Represents a directory level in the hierarchy; contains subfolders, projects, and potentially images
- **Project**: A folder identified as containing 3D printing content (STL files); displays preview images and STL file groupings
- **Image**: A visual file (JPG, PNG, etc.) that can be displayed as a preview; has a filename and path; can be inherited by descendant projects
- **STL Container**: A folder whose name matches IGNORED_KEYWORDS criteria; contains STL files as a category within the parent project
- **Navigation State**: Tracks current folder level, breadcrumb path, and visible children at current level
- **Image Inheritance Chain**: The collection of images from current folder up through all parent folders, deduplicated by filename

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can navigate through folder hierarchies by clicking one folder at a time, seeing only immediate children at each level
- **SC-002**: Project previews display images (either direct or inherited) for at least 95% of projects that have any images in their hierarchy
- **SC-003**: Image inheritance works correctly such that an image placed at level N appears in all projects at levels N+1, N+2, etc.
- **SC-004**: Folders matching IGNORED_KEYWORDS substrings (case-insensitive) are correctly categorized as STL containers 100% of the time
- **SC-005**: Navigation remains responsive (page loads in under 2 seconds) even when browsing folders with 50+ items
- **SC-006**: Users can navigate back up the hierarchy using breadcrumbs without losing context
- **SC-007**: Duplicate images (same filename) are displayed only once per project, regardless of how many levels inherit them
- **SC-008**: STL files are properly grouped by their category folder names within each project view
