# Quick Start Guide: 3D Print Model Library

**Feature**: 001-3d-print-library  
**Last Updated**: 2025-11-16

## Overview

This guide will help you get the 3D Print Model Library up and running on your local machine in under 10 minutes.

## Prerequisites

### Required

- **Rust**: 1.75 or later
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Node.js**: 18.0 or later
  ```bash
  # Using nvm (recommended)
  nvm install 18
  nvm use 18
  ```

- **stl-thumb**: STL preview generation tool
  ```bash
  # Clone and build stl-thumb
  git clone git@github.com:unlimitedbacon/stl-thumb.git
  cd stl-thumb
  cargo build --release
  sudo cp target/release/stl-thumb /usr/local/bin/
  
  # Verify installation
  stl-thumb --version
  ```

### Optional

- **SQLite CLI**: For database inspection (usually pre-installed on Linux/macOS)
  ```bash
  sqlite3 --version
  ```

## Quick Start

### 1. Clone the Repository

```bash
git clone <repository-url>
cd glyptotheka
git checkout 001-3d-print-library
```

### 2. Set Up Backend

```bash
cd backend

# Install dependencies and build
cargo build

# Run database migrations
cargo run --bin migrate

# Start the backend server
cargo run --release
```

The backend will start on `http://localhost:3000`.

### 3. Set Up Frontend

Open a new terminal:

```bash
cd frontend

# Install dependencies
npm install

# Start development server
npm run dev
```

The frontend will start on `http://localhost:5173`.

### 4. Configure Your Library

1. Open your browser to `http://localhost:5173`
2. Click "Configure Root Path"
3. Enter the absolute path to your 3D print files (e.g., `/home/user/3d-prints`)
4. Click "Save" and then "Start Scan"
5. Wait for the scan to complete
6. Browse your collection!

## Project Structure

```
.
├── backend/              # Rust/Axum API server
│   ├── src/
│   │   ├── main.rs      # Entry point
│   │   ├── models/      # Database entities
│   │   ├── db/          # Database layer
│   │   ├── services/    # Business logic
│   │   └── api/         # HTTP handlers
│   └── Cargo.toml
│
├── frontend/            # React/TypeScript UI
│   ├── src/
│   │   ├── main.tsx    # Entry point
│   │   ├── pages/      # Page components
│   │   ├── components/ # Reusable components
│   │   └── api/        # API client
│   └── package.json
│
└── specs/001-3d-print-library/
    ├── spec.md          # Feature specification
    ├── plan.md          # Implementation plan
    ├── research.md      # Technical research
    ├── data-model.md    # Database schema
    └── contracts/       # API contracts
```

## Common Tasks

### View Logs

**Backend:**
```bash
cd backend
RUST_LOG=debug cargo run
```

**Frontend:**
```bash
# Logs appear in browser console
```

### Reset Database

```bash
cd backend
rm glyptotheka.db
cargo run --bin migrate
```

### Clear Image Cache

```bash
cd backend
rm -rf cache/
```

### Run Tests

**Backend:**
```bash
cd backend
cargo test
```

**Frontend:**
```bash
cd frontend
npm test
```

### Build for Production

**Backend:**
```bash
cd backend
cargo build --release
# Binary at: target/release/glyptotheka-backend
```

**Frontend:**
```bash
cd frontend
npm run build
# Static files in: dist/
```

## Configuration

### Backend Configuration

Configuration is stored in the SQLite database (`config` table). You can also set via environment variables:

```bash
# .env file in backend/
ROOT_PATH=/path/to/3d/prints
STL_THUMB_PATH=/usr/local/bin/stl-thumb
CACHE_MAX_SIZE_MB=5000
IMAGES_PER_PAGE=20
DATABASE_URL=sqlite://glyptotheka.db
BIND_ADDRESS=127.0.0.1:3000
```

### Frontend Configuration

```typescript
// src/config.ts
export const config = {
  apiBaseUrl: import.meta.env.VITE_API_URL || 'http://localhost:3000/api',
  imagesPerPage: 20,
};
```

Create `.env` file in `frontend/`:
```
VITE_API_URL=http://localhost:3000/api
```

## API Documentation

The API documentation is available as OpenAPI/Swagger:

1. Start the backend server
2. Visit `http://localhost:3000/api/docs` (if Swagger UI is enabled)
3. Or view the raw spec at `specs/001-3d-print-library/contracts/openapi.yaml`

### Example API Calls

**Get root projects:**
```bash
curl http://localhost:3000/api/projects
```

**Search by name:**
```bash
curl "http://localhost:3000/api/search?q=dragon"
```

**Add tag to project:**
```bash
curl -X POST http://localhost:3000/api/projects/1/tags \
  -H "Content-Type: application/json" \
  -d '{"tagName": "painted"}'
```

## Troubleshooting

### Backend won't start

**Error**: `Failed to open database`
- **Solution**: Run migrations: `cargo run --bin migrate`

**Error**: `Address already in use`
- **Solution**: Change port in `.env` or kill existing process:
  ```bash
  lsof -i :3000
  kill -9 <PID>
  ```

### Frontend won't connect to backend

**Error**: `Network Error` or `CORS error`
- **Solution**: Check backend is running on port 3000
- **Solution**: Verify CORS middleware is enabled in backend

### Scan finds no projects

**Issue**: Scan completes but shows 0 projects
- **Check**: Root path is correct and contains STL files
- **Check**: STL files have `.stl` extension (case-insensitive)
- **Check**: File permissions allow reading

### STL previews not generating

**Error**: Preview shows placeholder image
- **Check**: stl-thumb is installed: `which stl-thumb`
- **Check**: stl-thumb path in config
- **Check**: Backend logs for generation errors: `RUST_LOG=debug cargo run`

### Images not displaying

**Issue**: Tiles show no images
- **Check**: Image files are in supported formats (JPG, PNG, GIF, WebP)
- **Check**: Cache directory exists: `ls backend/cache/`
- **Check**: Backend has write permissions to cache directory

### Search returns no results

**Issue**: Search by name finds nothing
- **Check**: FTS index is populated (automatic on scan)
- **Check**: Search query syntax (single words work best)
- **Try**: Searching by partial name or tag instead

## Development Tips

### Hot Reloading

- **Backend**: Use `cargo-watch` for auto-restart on code changes:
  ```bash
  cargo install cargo-watch
  cargo watch -x run
  ```

- **Frontend**: Vite provides hot module replacement automatically

### Database Inspection

```bash
sqlite3 backend/glyptotheka.db

# View schema
.schema

# Query projects
SELECT * FROM projects LIMIT 10;

# View tags
SELECT t.name, t.usage_count FROM tags t ORDER BY usage_count DESC;

# Exit
.quit
```

### Code Formatting

```bash
# Rust
cd backend
cargo fmt
cargo clippy

# TypeScript
cd frontend
npm run lint
npm run format
```

### Debugging

**Backend**:
- Use `dbg!()` macro for quick debugging
- Set `RUST_LOG=debug` for verbose logging
- Use `rust-gdb` or `lldb` for breakpoint debugging

**Frontend**:
- Use browser DevTools (F12)
- React DevTools extension recommended
- Check Network tab for API issues

## Next Steps

After getting the application running:

1. **Organize Your Collection**: Add tags to your projects for better organization
2. **Explore Navigation**: Click through folders to browse your hierarchy
3. **Try Search**: Use name search and tag filtering to find projects quickly
4. **Download Files**: Download individual STL files or entire projects as ZIP
5. **Rescan**: Add new files to your collection and trigger a rescan

## Sample Data

For testing without a real collection, use the provided sample data:

```bash
# Create sample structure
mkdir -p ~/3d-prints-sample/miniatures/fantasy
mkdir -p ~/3d-prints-sample/miniatures/scifi
mkdir -p ~/3d-prints-sample/terrain

# Backend will generate placeholder STL files and images for testing
cargo run --bin generate-sample-data -- ~/3d-prints-sample
```

Then set root path to `~/3d-prints-sample` and scan.

## Performance Tuning

### For Large Collections (>5000 projects)

1. **Increase SQLite cache**:
   ```sql
   -- In migrations or init
   PRAGMA cache_size = -128000;  -- 128MB cache
   ```

2. **Adjust scan concurrency**:
   ```rust
   // In services/scanner.rs
   .buffer_unordered(20)  // Process 20 dirs concurrently
   ```

3. **Enable database ANALYZE**:
   ```bash
   sqlite3 glyptotheka.db "ANALYZE;"
   ```

### For Slow Preview Generation

1. **Reduce preview size**:
   ```rust
   // In services/stl_preview.rs
   .arg("--size=256")  // Smaller previews
   ```

2. **Limit concurrent generation**:
   ```rust
   // Add preview queue with limited workers
   const MAX_PREVIEW_WORKERS: usize = 4;
   ```

## Support

- **Documentation**: See `specs/001-3d-print-library/` for detailed specs
- **Issues**: Report bugs via GitHub Issues
- **Contributing**: See CONTRIBUTING.md

## Architecture Overview

```
┌─────────────────┐
│   Browser UI    │  React + TypeScript
│   (Port 5173)   │  Vite dev server
└────────┬────────┘
         │ HTTP/REST
         ▼
┌─────────────────┐
│   Axum Server   │  Rust async web framework
│   (Port 3000)   │  Tower middleware
└────────┬────────┘
         │
    ┌────┴────┬────────────┬────────────┐
    ▼         ▼            ▼            ▼
┌────────┐ ┌──────┐ ┌──────────┐ ┌──────────┐
│ SQLite │ │ Cache│ │  Files   │ │stl-thumb │
│   DB   │ │ Dir  │ │  (user)  │ │ (extern) │
└────────┘ └──────┘ └──────────┘ └──────────┘
```

**Data Flow**:
1. User specifies root path via UI
2. Backend scans file system recursively
3. Projects, STL files, and images stored in SQLite
4. Images copied to cache directory
5. STL previews generated via stl-thumb
6. Frontend fetches data via REST API
7. User navigates, searches, downloads

## FAQ

**Q: Can I use this with Dropbox/OneDrive synced folders?**  
A: Yes, but trigger manual rescans after sync completes to pick up changes.

**Q: Does it support other 3D formats like OBJ or 3MF?**  
A: Not in v1. STL-only initially. Future versions may add support.

**Q: Can I edit STL files in the app?**  
A: No, this is a library/browser tool. Use external tools like Blender for editing.

**Q: Is my data private?**  
A: Yes, everything runs locally. No data leaves your machine.

**Q: Can I deploy this to a server for multi-user access?**  
A: Not in v1 (no authentication). Future versions may add multi-user support.

**Q: What if I have duplicate file names in different folders?**  
A: No problem. Projects are identified by full path, so duplicates are distinct.

**Q: Can I backup my database and cache?**  
A: Yes, copy `glyptotheka.db` and `cache/` directory. Restore by copying back.

## License

[License information to be added]

---

**Happy Browsing!** 🎨🖨️
