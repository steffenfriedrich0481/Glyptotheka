# Tasks: Integrate stl-thumb as Rust Library

**Input**: Design documents from `/specs/001-integrate-stl-thumb/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/api-changes.md

**Feature Branch**: `001-integrate-stl-thumb`

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Add stl-thumb library dependency and prepare build system

- [ ] T001 Add stl-thumb = "0.5" dependency to backend/Cargo.toml
- [ ] T002 Run cargo update and verify build succeeds with new dependency in backend/
- [ ] T003 Verify stl-thumb library API available by checking docs.rs/stl-thumb documentation

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Database migration to remove external tool configuration

**⚠️ CRITICAL**: This phase must complete before user story implementation can begin

- [ ] T004 Create migration to remove stl_thumb_path column in backend/src/db/migrations.rs
- [ ] T005 Test migration up/down paths with test database
- [ ] T006 Verify migration runs successfully on startup

**Checkpoint**: Database schema updated - user story implementation can now begin

---

## Phase 3: User Story 1 - Simplified Deployment Without External Dependencies (Priority: P1) 🎯 MVP

**Goal**: Replace subprocess calls to stl-thumb binary with direct library integration, eliminating external tool installation requirements

**Independent Test**: Deploy application without installing stl-thumb binary, upload STL file, verify preview generates successfully

### Implementation for User Story 1

- [ ] T007 [P] [US1] Remove stl_thumb_path field from Config struct in backend/src/config.rs
- [ ] T008 [P] [US1] Remove stl_thumb_path field from AppConfig struct in backend/src/config.rs  
- [ ] T009 [P] [US1] Remove stl_thumb_path field from UpdateConfigRequest struct in backend/src/config.rs
- [ ] T010 [US1] Update Config::default() implementation to remove stl_thumb_path initialization in backend/src/config.rs
- [ ] T011 [US1] Remove stl_thumb_path parameter from StlPreviewService::new() in backend/src/services/stl_preview.rs
- [ ] T012 [US1] Remove stl_thumb_path field from StlPreviewService struct in backend/src/services/stl_preview.rs
- [ ] T013 [US1] Replace run_stl_thumb() subprocess function with render_stl_preview() library function in backend/src/services/stl_preview.rs
- [ ] T014 [US1] Add use stl_thumb::Config as StlConfig import in backend/src/services/stl_preview.rs
- [ ] T015 [US1] Implement render_stl_preview() using stl_thumb::render_to_file() with spawn_blocking in backend/src/services/stl_preview.rs
- [ ] T016 [US1] Update generate_preview() to call render_stl_preview() instead of run_stl_thumb() in backend/src/services/stl_preview.rs
- [ ] T017 [US1] Update error handling to map library errors to AppError::InternalServer in backend/src/services/stl_preview.rs
- [ ] T018 [US1] Remove subprocess-related imports (std::process::Command) from backend/src/services/stl_preview.rs
- [ ] T019 [US1] Update AppState initialization to remove stl_thumb_path parameter in backend/src/api/routes.rs
- [ ] T020 [US1] Update ConfigService::get_config() to remove stl_thumb_path from SELECT query in backend/src/config.rs
- [ ] T021 [US1] Update ConfigService::get_config() to remove stl_thumb_path from row mapping in backend/src/config.rs
- [ ] T022 [US1] Update ConfigService::update_config() to remove stl_thumb_path update logic in backend/src/config.rs
- [ ] T023 [US1] Build and test preview generation with small test STL file (e.g., backend/tests/fixtures/cube.stl)
- [ ] T024 [US1] Test preview generation with ASCII format STL file
- [ ] T025 [US1] Test preview generation with binary format STL file
- [ ] T026 [US1] Test preview generation with complex STL file (>10MB)
- [ ] T027 [US1] Verify existing cached previews still load correctly
- [ ] T028 [US1] Verify preview format remains 512x512 PNG

**Checkpoint**: At this point, preview generation works with library integration and existing cache is compatible

---

## Phase 4: User Story 2 - Maintain Existing Preview Quality (Priority: P1)

**Goal**: Ensure visually identical preview output with same resolution and rendering quality

**Independent Test**: Generate previews for test STL files and compare visual output, resolution, and generation time against baseline

### Implementation for User Story 2

- [ ] T029 [P] [US2] Create benchmark script to test 20 diverse STL files in backend/tests/
- [ ] T030 [P] [US2] Generate baseline previews for comparison (if not already available)
- [ ] T031 [US2] Run benchmark and compare preview generation times (target: within 10% of baseline)
- [ ] T032 [US2] Validate PNG format and dimensions (512x512) for all generated previews
- [ ] T033 [US2] Visual comparison of generated previews against baseline (spot check 5-10 files)
- [ ] T034 [US2] Test error handling for invalid STL files
- [ ] T035 [US2] Test error handling for missing STL files
- [ ] T036 [US2] Verify error messages are clear and actionable

**Checkpoint**: Preview quality matches or exceeds previous implementation

---

## Phase 5: User Story 3 - Simplified Configuration Management (Priority: P2)

**Goal**: Remove STL_THUMB_PATH configuration requirement from all deployment scenarios

**Independent Test**: Deploy application with default configuration and verify preview generation works without STL_THUMB_PATH

### Implementation for User Story 3

- [ ] T037 [P] [US3] Remove STL_THUMB_PATH from .env.example
- [ ] T038 [P] [US3] Remove STL_THUMB_PATH from docker-compose.yml environment section
- [ ] T039 [P] [US3] Update backend/Dockerfile to add OpenGL runtime libraries (libgl1-mesa-glx libglu1-mesa)
- [ ] T040 [P] [US3] Remove any stl-thumb binary installation steps from backend/Dockerfile (if present)
- [ ] T041 [US3] Update README.md installation section to remove stl-thumb installation instructions
- [ ] T042 [US3] Update README.md prerequisites section to add OpenGL libraries requirement
- [ ] T043 [US3] Update docs/quickstart.md to remove external tool installation steps (replace with content from specs/001-integrate-stl-thumb/quickstart.md)
- [ ] T044 [US3] Update systemd service example to remove STL_THUMB_PATH environment variable (if documented)
- [ ] T045 [US3] Test Docker build completes successfully
- [ ] T046 [US3] Test Docker container starts and generates previews
- [ ] T047 [US3] Verify Docker build time improvement (measure and document)
- [ ] T048 [US3] Test deployment on clean system without stl-thumb binary

**Checkpoint**: All configuration and deployment documentation updated and tested

---

## Phase 6: Frontend Updates (Optional - if frontend displays stl_thumb_path)

**Goal**: Update frontend to remove stl_thumb_path configuration field

**Independent Test**: Open configuration page, verify stl_thumb_path field is not displayed

**Note**: These tasks are only needed if frontend has configuration UI for stl_thumb_path

- [ ] T049 [P] [US3] Remove stl_thumb_path field from AppConfig interface in frontend/src/types/config.ts
- [ ] T050 [P] [US3] Remove stl_thumb_path field from UpdateConfigRequest interface in frontend/src/types/config.ts
- [ ] T051 [P] [US3] Remove stl_thumb_path input field from configuration form component
- [ ] T052 [P] [US3] Remove stl_thumb_path validation from configuration schema
- [ ] T053 [US3] Test configuration page displays correctly without stl_thumb_path field
- [ ] T054 [US3] Test configuration update API calls work without stl_thumb_path

---

## Phase 7: Integration Testing & Validation

**Purpose**: End-to-end testing across all user stories

- [ ] T055 [P] Integration test: Deploy via Docker and test complete workflow (scan → upload → preview)
- [ ] T056 [P] Integration test: Deploy natively and test complete workflow
- [ ] T057 [P] Integration test: Test concurrent preview generation (multiple files)
- [ ] T058 Performance test: Benchmark preview generation time across 50 diverse STL files
- [ ] T059 Performance test: Verify no regression in API response times
- [ ] T060 Compatibility test: Verify existing database migrates successfully
- [ ] T061 Compatibility test: Verify existing cached previews remain valid
- [ ] T062 Error handling test: Test all error scenarios document improved error messages

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, cleanup, and final validation

- [ ] T063 [P] Update CHANGELOG.md with breaking changes and migration notes
- [ ] T064 [P] Update API documentation (if exists) to reflect config endpoint changes
- [ ] T065 [P] Add troubleshooting section for OpenGL library issues to docs/
- [ ] T066 [P] Document headless server requirements (Mesa software rendering) in docs/
- [ ] T067 Code review: Check for any remaining references to external stl-thumb binary
- [ ] T068 Code review: Verify all subprocess-related code removed
- [ ] T069 Run cargo fmt and cargo clippy on backend code
- [ ] T070 Run npm run lint on frontend code (if frontend changes made)
- [ ] T071 Final validation: Run quickstart.md steps on clean system
- [ ] T072 Final validation: Verify all success criteria from spec.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup (Phase 1) completion - BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational (Phase 2) - Core service refactoring
- **User Story 2 (Phase 4)**: Depends on User Story 1 (Phase 3) - Quality validation requires working implementation
- **User Story 3 (Phase 5)**: Depends on User Story 1 (Phase 3) - Can run in parallel with User Story 2
- **Frontend Updates (Phase 6)**: Depends on User Story 1 (Phase 3) - Can run in parallel with other phases
- **Integration Testing (Phase 7)**: Depends on User Stories 1, 2, 3 completion
- **Polish (Phase 8)**: Depends on all previous phases completion

### User Story Dependencies

- **User Story 1 (P1)**: BLOCKING - All other stories depend on this core refactoring
- **User Story 2 (P1)**: Depends on US1 - Cannot validate quality without working implementation
- **User Story 3 (P2)**: Depends on US1 - Can proceed in parallel with US2 once US1 complete

### Within Each Phase

**Phase 3 (User Story 1)**:
- T007-T009 can run in parallel (different structs, same file - separate sections)
- T010 depends on T007
- T011-T012 must precede T013-T018
- T019 depends on T011
- T020-T022 can run after T007-T009
- T023-T028 are sequential validation tasks

**Phase 4 (User Story 2)**:
- T029-T030 can run in parallel
- T031-T036 are sequential validation tasks

**Phase 5 (User Story 3)**:
- T037-T040 can run in parallel (different files)
- T041-T044 can run in parallel (different documentation files)
- T045-T048 are sequential validation tasks

**Phase 6 (Frontend)**:
- T049-T052 can run in parallel (different files/sections)
- T053-T054 are sequential validation tasks

**Phase 7 (Integration)**:
- T055-T057 can run in parallel (independent test scenarios)
- T058-T062 are sequential focused tests

**Phase 8 (Polish)**:
- T063-T066 can run in parallel (different documentation)
- T067-T072 are sequential review/validation tasks

### Parallel Opportunities

**Setup Phase**:
```bash
# All setup tasks are sequential (cargo dependency)
Task T001 → Task T002 → Task T003
```

**User Story 1 - Model Updates**:
```bash
# Launch struct updates together (different sections):
Task T007: "Remove stl_thumb_path from Config struct"
Task T008: "Remove stl_thumb_path from AppConfig struct"
Task T009: "Remove stl_thumb_path from UpdateConfigRequest struct"
```

**User Story 3 - Documentation**:
```bash
# Launch all documentation updates together:
Task T037: "Remove STL_THUMB_PATH from .env.example"
Task T038: "Remove STL_THUMB_PATH from docker-compose.yml"
Task T039: "Update Dockerfile with OpenGL libraries"
Task T040: "Remove stl-thumb installation from Dockerfile"
```

**Integration Testing**:
```bash
# Launch independent integration tests together:
Task T055: "Integration test: Docker deployment"
Task T056: "Integration test: Native deployment"
Task T057: "Integration test: Concurrent preview generation"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (add library dependency)
2. Complete Phase 2: Foundational (database migration)
3. Complete Phase 3: User Story 1 (core service refactoring)
4. **STOP and VALIDATE**: Test preview generation works with library
5. Verify existing cache compatibility
6. Measure performance against baseline

**At this point, you have a working preview system with library integration**

### Incremental Delivery

1. **Foundation** (Phase 1-2): Setup + Database Migration → ~1-2 hours
2. **MVP** (Phase 3): User Story 1 → Test independently → Deploy/Demo → ~4-6 hours
3. **Quality** (Phase 4): User Story 2 → Validate quality → ~2-3 hours
4. **Deployment** (Phase 5): User Story 3 → Update deployment → ~2-3 hours
5. **Polish** (Phase 6-8): Frontend + Integration + Polish → ~3-4 hours

**Total Estimate**: 12-18 hours

### Parallel Team Strategy

With multiple developers:

1. **Together**: Complete Setup (Phase 1) + Foundational (Phase 2) → ~2 hours
2. **Once Phase 2 complete**:
   - Developer A: User Story 1 (Phase 3) - Core service refactoring
   - Developer B: Start preparing Phase 5 tasks (documentation) - can draft updates
3. **Once Phase 3 complete**:
   - Developer A: User Story 2 (Phase 4) - Quality validation
   - Developer B: User Story 3 (Phase 5) - Deployment updates
   - Developer C: Phase 6 (Frontend updates) - if needed
4. **All together**: Integration testing (Phase 7) + Polish (Phase 8)

---

## Critical Success Factors

### From Spec.md Success Criteria

- **SC-001**: Deployment without external tools
  - ✅ Validated by T048: Test deployment on clean system without stl-thumb binary
  - ✅ Validated by T045-T046: Docker build and container tests

- **SC-002**: Visually identical output
  - ✅ Validated by T029-T033: Benchmark and visual comparison tests
  - ✅ Validated by T028: Verify 512x512 PNG format

- **SC-003**: Performance within 10%
  - ✅ Validated by T031: Benchmark generation times
  - ✅ Validated by T058: Performance test across 50 files

- **SC-004**: Docker build time reduction
  - ✅ Validated by T047: Measure and document build time improvement

- **SC-005**: Configuration reduction
  - ✅ Validated by T037-T038: Remove STL_THUMB_PATH from config files

- **SC-006**: Zero deployment failures
  - ✅ Validated by T048: Clean system deployment test
  - ✅ Validated by T055-T056: Integration tests

### Quality Gates

**Cannot proceed to next phase without**:

- Phase 2 → Phase 3: Database migration must succeed
- Phase 3 → Phase 4: Preview generation must work with library
- Phase 3 → Phase 5: Core refactoring must be complete
- Phase 7: All user stories must pass independent tests
- Phase 8: All integration tests must pass

---

## Testing Notes

- Tests are integrated throughout implementation phases
- Each user story has independent test criteria
- Integration testing validates cross-story functionality
- Performance testing ensures no regression
- All success criteria validated before completion

---

## Notes

- [P] tasks = different files or independent sections, can run in parallel
- [Story] label maps task to specific user story for traceability
- Each user story delivers incremental value
- Core refactoring (US1) is blocking for other stories
- Configuration/documentation updates can proceed in parallel with validation
- Commit after each task or logical group
- Stop at any checkpoint to validate independently
- OpenGL libraries are runtime requirement (already present on most Linux systems)
- Existing cache remains valid (no regeneration needed)
