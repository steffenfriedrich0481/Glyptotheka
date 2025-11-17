# STL-Thumb Library Integration - Manual Testing Guide

## Completed Implementation

The stl-thumb library has been successfully integrated into the Glyptotheka backend:

### Changes Made

1. **T002**: ✅ Cargo dependency updated - stl-thumb 0.5 downloaded and available
2. **T005-T006**: ✅ Database migration registered in migrations.rs (003_remove_stl_thumb_path.sql)
3. **T014**: ✅ Added `use stl_thumb::config::Config as StlConfig` import
4. **T015**: ✅ Implemented `render_stl_preview()` using `stl_thumb::render_to_file()`
   - Uses `tokio::task::spawn_blocking` for async compatibility
   - Renders at 512x512 PNG resolution
   - Headless rendering mode enabled
   - Proper error handling with library errors
5. **Library Structure**: ✅ Created lib.rs to enable testing and modular imports

### Implementation Details

**File**: `backend/src/services/stl_preview.rs`

The implementation:
- Generates preview in blocking thread pool (OpenGL rendering is CPU-intensive)
- Creates temporary file for stl-thumb output
- Reads PNG data and returns as Vec<u8>
- Cleans up temporary file after reading
- Integrates with existing cache system

**Configuration**:
```rust
let config = StlConfig {
    stl_filename: stl_path_str.clone(),
    img_filename: output_path.to_string_lossy().to_string(),
    width: 512,
    height: 512,
    visible: false,  // Headless rendering
    verbosity: 0,
    ..Default::default()
};
```

## Manual Testing Instructions

### Prerequisites

**System Requirements**:
- OpenGL libraries installed (for headless rendering):
  ```bash
  # Debian/Ubuntu
  sudo apt-get install libgl1-mesa-glx libglu1-mesa
  
  # For headless servers, also install:
  sudo apt-get install libosmesa6
  ```

### Test Cases

#### T023: Small STL File Test
```bash
# Example file (4.7 MB)
FILE="example/Miniaturen/Cast'N'Play/[CNP] 24_04 - Dwarven Legacy/819_Dwarf Gemtreasure Trader/STL/Treasure A - Trinkets.stl"

# 1. Start the backend server
cd backend
DATABASE_PATH=test.db CACHE_DIR=test_cache cargo run --release

# 2. In another terminal, trigger scan and preview
curl -X POST http://localhost:3000/api/scan \
  -H "Content-Type: application/json" \
  -d '{"root_path": "/full/path/to/example"}'

# 3. Check preview was generated
ls -lh test_cache/previews/
file test_cache/previews/*.png
```

#### T024-T025: ASCII and Binary STL Format Test
Most STL files in the example directory are binary format. The library handles both automatically.

#### T026: Large STL File Test (>10MB)
```bash
# Example large file (96-104 MB)
FILE="example/Miniaturen/Mammoth Factory/Bust Prince Voriel Bust/PRESUPPORTED_STL/BUST_Prince_Viorel_V2_Supported.stl"

# Same process as T023, but may take 10-30 seconds for rendering
```

#### T027: Cache Verification
```bash
# After generating a preview, restart server
# Preview should load from cache (instant, no re-render)
# Check logs for "Using cached preview for..." message
```

#### T028: PNG Format Verification
```bash
# Check generated preview files
for f in test_cache/previews/*.png; do
  echo "File: $f"
  file "$f"
  identify "$f" | grep "512x512"
done

# Expected output:
# - PNG image data, 512 x 512
# - All files should be valid PNG format
```

## Automated Testing (Future)

The integration test `backend/tests/stl_preview_test.rs` has been created but requires:
1. OpenGL context in CI environment
2. Mesa software rendering setup for headless testing
3. Test fixture STL files

For CI/CD, consider:
- Using xvfb-run for virtual display
- Mesa OSMesa for software rendering
- Docker container with OpenGL support

## Build Verification

✅ **Debug build**: `cargo build` - SUCCESS
✅ **Release build**: `cargo build --release` - SUCCESS
✅ **Library exports**: lib.rs created for test integration

## Next Steps

To complete the feature:

1. **Runtime Testing** (T023-T028):
   - Deploy to test environment with OpenGL libraries
   - Run manual test cases above
   - Verify preview generation works end-to-end
   - Check preview quality and performance

2. **Docker Support** (Phase 5 - User Story 3):
   - Update Dockerfile with OpenGL runtime libraries
   - Remove stl-thumb binary installation steps
   - Test Docker build and container preview generation

3. **Documentation Updates** (Phase 5):
   - Update README.md to remove stl-thumb installation
   - Add OpenGL library requirements
   - Update deployment guides

## Success Criteria Status

- ✅ **SC-001**: Library integrated (no external tool needed)
- ⏳ **SC-002**: Visual quality (pending runtime test)
- ⏳ **SC-003**: Performance (pending benchmark)
- ⏳ **SC-004**: Docker build time (pending Docker updates)
- ✅ **SC-005**: Configuration simplified (stl_thumb_path removed)
- ⏳ **SC-006**: Deployment verified (pending test deployment)

## Known Limitations

1. **OpenGL Required**: System must have OpenGL libraries installed
2. **Headless Rendering**: Works with Mesa software rendering (OSMesa)
3. **Test Environment**: Automated tests need CI environment with OpenGL support

## Migration Notes

The database migration (003_remove_stl_thumb_path.sql) will automatically run on next application startup, removing the old `stl_thumb_path` configuration column from the `config` table.
