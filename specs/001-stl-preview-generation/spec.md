# Feature Specification: STL Preview Image Generation

**Feature Branch**: `001-stl-preview-generation`  
**Created**: 2025-11-18  
**Status**: Draft  
**Input**: User description: "Create a feature specification for STL preview image generation during scanning."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - View STL Preview Images Alongside Project Images (Priority: P1)

As a user, when I scan a project folder containing STL files, I want preview images automatically generated for each STL file so that I can visually browse my 3D models without opening specialized software.

**Why this priority**: This is the core value proposition - users need to see visual representations of their STL files. Without this, the feature delivers no value.

**Independent Test**: Can be fully tested by scanning a folder with STL files and verifying that preview images are accessible through the image retrieval API. Delivers immediate value by showing STL content visually.

**Acceptance Scenarios**:

1. **Given** a project folder containing STL files, **When** user initiates a scan, **Then** preview images are generated for each STL file and stored in the cache directory
2. **Given** STL preview images have been generated, **When** user requests project images, **Then** STL previews are returned alongside regular images with appropriate ranking
3. **Given** a project with both regular images and STL files, **When** images are retrieved, **Then** regular images appear before STL previews in the response

---

### User Story 2 - Use STL Previews in Composite Project Thumbnails (Priority: P2)

As a user, when viewing a project list or gallery, I want composite preview images that include STL previews so that projects without regular images still have visual representation.

**Why this priority**: Enhances discoverability and navigation, but requires User Story 1 to be complete. Adds value when regular images are absent or insufficient.

**Independent Test**: Can be tested by creating composite previews for projects with varying image counts (0 regular images, <4 regular images, ≥4 regular images) and verifying STL previews are used appropriately.

**Acceptance Scenarios**:

1. **Given** a project with no regular images but has STL files, **When** composite preview is generated, **Then** STL preview images are used to create the composite
2. **Given** a project with 2 regular images and 3 STL files, **When** composite preview is generated, **Then** the 2 regular images are used first, followed by up to 2 STL previews to reach 4 total images
3. **Given** a project with 4+ regular images, **When** composite preview is generated, **Then** only regular images are used, STL previews are not included

---

### User Story 3 - Regenerate STL Previews on Rescan (Priority: P3)

As a user, when I modify STL files or trigger a rescan, I want STL preview images to be regenerated so that previews stay synchronized with the actual file content.

**Why this priority**: Important for data consistency but lower priority than initial generation. Users can work with slightly outdated previews temporarily.

**Independent Test**: Can be tested by modifying an STL file, rescanning the project, and verifying the preview image is regenerated with updated content.

**Acceptance Scenarios**:

1. **Given** a project with existing STL previews, **When** user triggers a rescan, **Then** all STL preview images are regenerated
2. **Given** an STL file has been deleted, **When** rescan occurs, **Then** the corresponding preview image is removed from cache
3. **Given** new STL files have been added to a project, **When** rescan occurs, **Then** preview images are generated for the new STL files

---

### User Story 4 - Inherit STL Previews from Parent Folders (Priority: P4)

As a user, when viewing a child folder that has no images of its own, I want STL previews from parent folders to be accessible so that all projects have visual representation following the existing inheritance pattern.

**Why this priority**: Consistency with existing image inheritance behavior, but adds complexity. Lower priority as it's an enhancement to the navigation experience.

**Independent Test**: Can be tested by creating a nested folder structure where parent has STL files and child has none, then verifying child inherits STL previews.

**Acceptance Scenarios**:

1. **Given** a parent folder with STL previews and a child folder with no images, **When** child folder images are requested, **Then** parent STL previews are included with inheritance indicator
2. **Given** a child folder has its own STL previews, **When** images are requested, **Then** child's STL previews are prioritized over inherited parent STL previews
3. **Given** image inheritance chain includes both regular images and STL previews, **When** images are retrieved, **Then** regular images from any level are ranked higher than all STL previews

---

### Edge Cases

- What happens when an STL file is corrupted or cannot be parsed by the preview generator?
- How does the system handle very large STL files that take significant time to generate previews?
- What happens when the cache directory becomes full or inaccessible?
- How does the system handle STL files with special characters or very long filenames?
- What happens when preview generation fails midway through a scan?
- How does the system handle concurrent scan requests that might generate the same preview?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST detect STL files during project folder scanning
- **FR-002**: System MUST generate preview images for each discovered STL file using the external preview generation tool
- **FR-003**: System MUST store STL preview images in a dedicated cache directory separate from project files
- **FR-004**: System MUST track STL preview images in the system's image database with metadata indicating source STL file
- **FR-005**: System MUST assign lower ranking/priority to STL preview images compared to regular project images
- **FR-006**: System MUST include STL preview images in image retrieval API responses with appropriate ranking
- **FR-007**: System MUST consider STL preview images when generating composite project preview images
- **FR-008**: System MUST regenerate STL preview images when rescanning a project
- **FR-009**: System MUST remove cached preview images when source STL files are deleted during rescan
- **FR-010**: System MUST handle preview generation failures gracefully without blocking the overall scan process
- **FR-011**: System MUST allow STL preview images to participate in the image inheritance mechanism (child folders inherit from parent)
- **FR-012**: System MUST prioritize regular images over STL previews when both exist in the inheritance chain
- **FR-013**: System MUST [NEEDS CLARIFICATION: Should preview generation be synchronous (blocking scan) or asynchronous (background task)?]
- **FR-014**: System MUST [NEEDS CLARIFICATION: Should existing cached previews be regenerated on every scan or only when STL file modified date changes?]
- **FR-015**: System MUST [NEEDS CLARIFICATION: What should happen if preview generation tool is missing or inaccessible?]

### Key Entities

- **STL Preview Image**: A generated preview image representing an STL file. Key attributes: source STL file path, cache file path, generation timestamp, file size, image dimensions, generation status (success/failed/pending). Related to Project via the STL file's location.

- **STL File Reference**: Metadata about an STL file discovered during scanning. Key attributes: file path, file size, modification date, last scan timestamp, preview image reference. Related to Project and STL Preview Image.

- **Image Ranking Metadata**: Priority/ranking information for all images in a project. Key attributes: image source type (regular/STL preview/inherited), priority score, inheritance depth. Used to determine image ordering in API responses.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Preview images are generated for 95%+ of valid STL files during scanning
- **SC-002**: STL preview images are retrievable via the image API within 1 second of request
- **SC-003**: Regular project images always appear before STL preview images in all API responses
- **SC-004**: Composite preview generation successfully includes STL previews when fewer than 4 regular images exist
- **SC-005**: Scan completion time increases by no more than [NEEDS CLARIFICATION: acceptable performance impact - 10%? 25%? 50%?] when processing folders with STL files
- **SC-006**: Cache directory size remains manageable (preview images average less than 500KB each)
- **SC-007**: 100% of STL preview images are regenerated or removed appropriately during rescan operations

## Assumptions

- The external preview generation tool (stl-thumb) is installed and accessible at the expected location relative to the project
- STL files are stored in standard STL format (ASCII or binary)
- Preview images will be generated in a standard image format (PNG or JPEG) suitable for web display
- The system has sufficient disk space for caching preview images
- Preview image resolution will follow similar standards to existing composite previews (likely 200-400 pixels for thumbnails)
- If asynchronous processing is chosen (pending FR-013 clarification), users accept that previews may not be immediately available after scan initiation
- Cache invalidation will be handled based on file modification dates (pending FR-014 clarification)
- The existing StlPreviewService provides the necessary interface for integrating with the preview generation tool
