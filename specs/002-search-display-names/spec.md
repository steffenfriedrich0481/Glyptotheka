# Specification: Search Display Name Refinement

## Goal
Improve the display of project names in search results by ignoring generic folder names (keywords) and using parent folder names instead.

## Requirements
1.  **Configuration**:
    *   Support a list of "ignored keywords" configured via environment variable `IGNORED_KEYWORDS`.
    *   Default keywords: `PRESUPPORTED_STL`, `STL`, `UNSUPPORTED_STL`, `Unsupported`.
    *   Matching should be case-insensitive and trim whitespace.

2.  **Search Service**:
    *   When returning search results, check if the project name matches an ignored keyword.
    *   If it matches, traverse up the `full_path` to find the first component that is NOT an ignored keyword.
    *   Use this component as the display name.
    *   If all components match (unlikely), fall back to the original name.

3.  **Example**:
    *   Path: `/.../Miniaturen/Mammoth Factory/Bahamut/STL`
    *   Keyword: `STL`
    *   Display Name: `Bahamut` (instead of `STL`)

    *   Path: `/.../Miniaturen/Cast'N'Play/Dwarven Legacy/Trader/Pre-Supported/STL`
    *   Keywords: `Pre-Supported`, `STL`
    *   Display Name: `Trader` (skips `STL` and `Pre-Supported`)

## Implementation Plan
1.  **Backend**:
    *   Modify `SearchService` to accept `ignored_keywords`.
    *   Implement `resolve_display_name` logic.
    *   Update `search` methods to apply this logic.
    *   Update `main.rs` to read `IGNORED_KEYWORDS` and pass to `create_router`.
    *   Update `create_router` to pass to `SearchService`.

2.  **Testing**:
    *   Verify with existing example data where `STL` and `Unsupported` folders exist.
