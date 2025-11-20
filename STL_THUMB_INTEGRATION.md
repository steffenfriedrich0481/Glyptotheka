# STL-Thumb Local Version Integration

## Overview

Glyptotheka now uses a local version of stl-thumb from `~/Workspace/stl-thumb` instead of the crates.io version. This allows using custom modifications and updates to stl-thumb for preview generation.

## Changes Made

### 1. Backend Cargo.toml
Changed stl-thumb dependency from:
```toml
stl-thumb = "0.5"
```

To:
```toml
stl-thumb = { path = "/home/stefffri/Workspace/stl-thumb" }
```

### 2. API Compatibility Update
Updated `backend/src/services/stl_preview.rs` to use the new stl-thumb API:
- Changed `stl_filename` field to `model_filename` in Config struct
- This reflects the API changes in the updated stl-thumb version

### 3. Dependency Compatibility
Temporarily downgraded `image` dependency in local stl-thumb from `0.25.5` to `0.24` to maintain compatibility with current Rust toolchain (1.83.0).

## STL Preview Generation During Scanning

The scanner service (`backend/src/services/scanner.rs`) automatically generates STL previews during the scanning process:

1. **First 2 STL files**: Generated synchronously for immediate availability
2. **Remaining STL files**: Queued for asynchronous background generation
3. **Preview caching**: Smart caching checks modification times to avoid regeneration

### How It Works

When scanning a folder with STL files:
```rust
// In scanner.rs, lines 145-171
if self.stl_preview_service.is_some() && !stl_files_vec.is_empty() {
    // Split: first 2 sync, rest async
    let (sync_files, async_files) = stl_files_vec.split_at(
        std::cmp::min(2, stl_files_vec.len())
    );

    // Generate first 2 synchronously
    for stl_file in sync_files {
        self.generate_stl_preview_sync(project_id, stl_file)?;
    }

    // Queue remaining for async generation
    for stl_file in async_files {
        self.queue_stl_preview(project_id, stl_file)?;
    }
}
```

## Preview Generation Features

- **512x512 PNG output**: Consistent preview size
- **Headless rendering**: No GUI required (visible: false)
- **Smart caching**: Compares STL file mtime with preview timestamp
- **Background processing**: Non-blocking preview generation
- **Error handling**: Graceful degradation if preview fails
- **Database tracking**: Stores preview path and generation timestamp

## Local STL-Thumb Repository

Location: `~/Workspace/stl-thumb`
Fork: https://github.com/steffenfriedrich0481/stl-thumb

## Benefits

1. **Custom Features**: Can add project-specific enhancements to stl-thumb
2. **Latest Updates**: Use newest code without waiting for crates.io release
3. **Development Flexibility**: Easy to modify and test changes
4. **Automatic Generation**: Previews generated during library scanning

## Testing

To test preview generation:
1. Start the backend: `cd backend && cargo run --release`
2. Configure a root folder with STL files
3. Trigger a scan - previews will be generated automatically
4. Check `backend/cache/previews/` for generated thumbnails
5. View previews in the UI when browsing projects

## Maintenance Notes

- Keep `~/Workspace/stl-thumb` in sync with upstream if needed
- The `image = "0.24"` pin in stl-thumb/Cargo.toml may need updating when upgrading Rust toolchain
- Preview generation happens in background threads to avoid blocking the event loop
- Generated previews are cached in `backend/cache/previews/` directory

