# Feature Specification: Integrate STL Preview Generation

**Feature Branch**: `001-integrate-stl-thumb`  
**Created**: 2025-11-17  
**Status**: Draft  
**Input**: User description: "Create a feature specification for integrating stl-thumb directly into the Glyptotheka project instead of requiring it as an external dependency."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Simplified Deployment Without External Dependencies (Priority: P1)

System operators can deploy the Glyptotheka application without manually installing external tools. The application includes all necessary components to generate STL file previews automatically during the build process.

**Why this priority**: This is the primary driver for the change - eliminating external dependency installation simplifies deployment, reduces configuration errors, and improves the out-of-box experience.

**Independent Test**: Can be fully tested by deploying the application using standard deployment procedures (Docker or native) without installing stl-thumb separately, then uploading an STL file and verifying a preview is generated.

**Acceptance Scenarios**:

1. **Given** a clean system without stl-thumb installed, **When** operator deploys the application using Docker, **Then** STL preview generation works without additional installation steps
2. **Given** the application is built from source, **When** the build process completes, **Then** all preview generation capabilities are included in the build artifacts
3. **Given** an STL file is uploaded, **When** the system generates a preview, **Then** the preview is created without calling external processes

---

### User Story 2 - Maintain Existing Preview Quality (Priority: P1)

Users continue to receive the same quality STL file previews after the integration as they did before. Preview generation produces identical visual output with the same resolution and rendering quality.

**Why this priority**: This ensures no regression in user-facing functionality - users should not experience any difference in preview quality or generation speed.

**Independent Test**: Can be fully tested by generating previews for a test set of STL files and comparing visual output quality, resolution, and generation time against the previous implementation.

**Acceptance Scenarios**:

1. **Given** an STL file, **When** a preview is generated, **Then** the output is a 512x512 PNG image matching the quality of previous previews
2. **Given** a complex STL file with many triangles, **When** a preview is generated, **Then** all geometric details are rendered correctly
3. **Given** various STL file formats (ASCII, binary), **When** previews are generated, **Then** all formats are handled correctly

---

### User Story 3 - Simplified Configuration Management (Priority: P2)

System operators configure the application without managing external tool paths. Configuration no longer requires specifying the location of external binaries or managing PATH environment variables.

**Why this priority**: Removes configuration complexity and potential misconfiguration issues, though the application can still function if this is the only thing not yet implemented.

**Independent Test**: Can be fully tested by deploying the application with default configuration settings and verifying preview generation works without STL_THUMB_PATH or similar configuration.

**Acceptance Scenarios**:

1. **Given** a default configuration file, **When** the application starts, **Then** no STL_THUMB_PATH or external tool configuration is required
2. **Given** the application is running, **When** configuration is validated, **Then** no warnings about missing external tools appear
3. **Given** a deployment across multiple environments, **When** configuration is replicated, **Then** no environment-specific tool paths need adjustment

---

### Edge Cases

- What happens when the integrated preview library fails to initialize during application startup?
- How does the system handle STL files that previously worked with the external tool but may have edge cases with the integrated version?
- What happens if the integrated library has different error messages or failure modes than the external tool?
- How does the system handle preview generation failures without subprocess error codes?
- What happens when an STL file format is not supported by the integrated library version?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST generate STL file previews without requiring external command-line tools
- **FR-002**: System MUST include all preview generation capabilities as part of the main application binary or build artifacts
- **FR-003**: System MUST generate previews with the same visual quality (512x512 PNG) as the current external tool implementation
- **FR-004**: System MUST support both ASCII and binary STL file formats
- **FR-005**: System MUST handle preview generation errors gracefully with meaningful error messages
- **FR-006**: System MUST maintain the same preview generation API interface for cache integration
- **FR-007**: System MUST remove configuration requirements for external tool paths (STL_THUMB_PATH)
- **FR-008**: System MUST integrate preview generation within the same process space to eliminate subprocess overhead
- **FR-009**: Build process MUST successfully compile and include preview generation functionality on all supported platforms
- **FR-010**: System MUST maintain license compatibility between the integrated preview library and the main application (both use MIT License, ensuring full compatibility and permissive integration)
- **FR-011**: System MUST complete preview generation in the same or less time compared to the external tool approach
- **FR-012**: Docker build process MUST no longer require installing external preview tools in the image

### Key Entities

- **Preview Generator**: Component responsible for converting STL files to PNG images, replacing the external stl-thumb process call with an integrated library call
- **Preview Configuration**: Settings for preview generation (size, quality), no longer requiring external tool paths
- **Build Artifacts**: The compiled application including integrated preview generation, eliminating separate binary dependencies

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Deployment completes successfully without installing external tools (100% of test deployments)
- **SC-002**: Preview generation produces visually identical output to current implementation (validated by comparing 20 sample STL files)
- **SC-003**: Preview generation time remains within 10% of current performance (measured across 50 diverse STL files)
- **SC-004**: Docker image build time reduces by at least 20% by eliminating external tool installation steps
- **SC-005**: Configuration files reduce by at least one required setting (STL_THUMB_PATH removal)
- **SC-006**: Zero deployment failures related to missing external preview tools (compared to current baseline of potential configuration errors)
