# Implementation Plan - Search Display Name Refinement

## Tasks

- [ ] T001 Update `SearchService` struct to include `ignored_keywords` field
- [ ] T002 Implement `resolve_display_name` method in `SearchService`
- [ ] T003 Update `search_fts`, `search_by_tags`, `search_all`, `search_combined` to use `resolve_display_name`
- [ ] T004 Update `create_router` in `api/routes.rs` to accept `ignored_keywords`
- [ ] T005 Update `main.rs` to read `IGNORED_KEYWORDS` from env and pass to `create_router`
- [ ] T006 Verify changes with manual testing
