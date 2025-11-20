# STL-Thumb Git Integration

## Overview

Glyptotheka now uses your stl-thumb fork from GitHub instead of the crates.io version. This allows using your custom modifications and updates while maintaining Docker compatibility.

## Changes Made

### 1. Backend Cargo.toml
Changed stl-thumb dependency from:
```toml
stl-thumb = { path = "/home/stefffri/Workspace/stl-thumb" }
```

To:
```toml
stl-thumb = { git = "https://github.com/steffenfriedrich0481/stl-thumb" }
```

### 2. API Compatibility
The code uses the updated stl-thumb API:
- `model_filename` field in Config struct (instead of `stl_filename`)

### 3. GitHub Fork
Your fork includes the compatibility fix:
- `image = "0.24"` for Rust 1.83.0 compatibility
- Commit: ab20ce6b

## Benefits

✅ **Works in Docker**: Git dependencies are fetched during Docker build
✅ **Always up-to-date**: Pulls latest from your GitHub fork
✅ **Custom features**: Uses all your stl-thumb modifications
✅ **Easy updates**: `cargo update` pulls latest changes
✅ **Team friendly**: Others can build without needing local stl-thumb

## STL Preview Generation During Scanning

The scanner service automatically generates STL previews:

1. **First 2 STL files**: Generated synchronously
2. **Remaining STL files**: Queued for async generation
3. **Smart caching**: Checks modification times

### Features

- **512x512 PNG output**: Consistent preview size
- **Headless rendering**: No GUI required
- **Background processing**: Non-blocking generation
- **Database tracking**: Stores preview paths and timestamps
- **Error handling**: Graceful degradation if preview fails

## Your STL-Thumb Fork

**Repository**: https://github.com/steffenfriedrich0481/stl-thumb
**Latest commit**: ab20ce6b - Pin image to 0.24 for compatibility

## Updating to Latest Changes

When you push changes to your fork:

```bash
cd backend
cargo update -p stl-thumb
cargo build --release
```

Or force a clean fetch:
```bash
rm -rf ~/.cargo/git/checkouts/stl-thumb-*
cargo clean -p stl-thumb
cargo build --release
```

## Docker Build

The Dockerfile will automatically fetch your fork during build:
```bash
docker-compose build
docker-compose up -d
```

No manual steps needed!

## Testing

1. Start the app: `docker-compose up -d`
2. Access: http://localhost:8080
3. Configure root folder and scan
4. Previews generated automatically with your custom stl-thumb!

## Maintenance

- Push changes to your fork to update stl-thumb
- Run `cargo update` in Glyptotheka to pull latest
- The image = "0.24" pin will be removed once Rust 1.85+ is available

