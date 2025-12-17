# Glyptotheka - 3D Print Model Library

> 🤖 **AI-Powered Development**: This project was entirely developed using [GitHub Copilot CLI](https://github.com/features/copilot/cli) with support from [GitHub Spec-Kit](https://github.com/github/spec-kit). Models used: **Claude Sonnet 4.5** and **Claude Opus 4.5**.

A modern web-based application for managing and browsing your 3D print model collection. Features a tile-based UI, hierarchical organization, automatic STL preview generation with smart caching, search & tagging, and easy file downloads.

## Features

### Core Functionality
- 📁 **Folder-by-folder navigation** - Browse like a file explorer with hierarchical organization
- 🖼️ **Automatic STL preview generation** with smart caching
- 🌊 **Image inheritance** - Parent folder images flow down to all child projects
- 🎯 **Priority-based image sorting** (regular images, STL previews, composite previews)
- 📦 **STL category grouping** - Organize files by size/type (e.g., "1 inch", "2 inch", "40 mm")
- 🔍 Full-text search with tag filtering
- 🏷️ Custom tagging system
- ⬇️ Individual file and ZIP archive downloads
- 🔄 Rescan functionality with intelligent preview regeneration
- 💾 Local-first architecture (SQLite database)

### STL Preview System (Latest)
- ⚡ **Smart caching** - Only regenerates when files change (90%+ cache hit rate)
- 🔄 **Hybrid generation** - First 2 previews sync, remainder async
- 🎨 **Priority system** - Regular images display before STL previews
- 🛡️ **Graceful error handling** - Corrupted files don't break scanning
- ⏱️ **Timeout protection** - 30-second limit per preview
- 📏 **Size validation** - 100MB file size limit for safety

### Modern UI
- 🎨 **Tile-based card design** with responsive grid layout
- 🌓 **Dark mode support** throughout the interface
- ⌨️ **Full keyboard navigation** (Tab, Enter, Space keys)
- ♿ **WCAG AA accessibility** (ARIA labels, focus indicators, screen reader support)
- 🚀 **Performance optimized** for large collections (500+ projects)
- 📱 **Responsive design** from mobile (320px) to ultra-wide displays (2560px+)
- ✨ **Smooth animations** and transitions
- 🎯 **Visual hierarchy** with proper spacing and typography

## Tech Stack

**Backend:**
- Rust 1.75+ with Axum web framework
- SQLite with rusqlite for local storage
- tokio for async runtime
- stl-thumb library (integrated) for STL preview generation

**Frontend:**
- React 18 with TypeScript
- **Tailwind CSS 3.4** for modern styling
- Vite for build tooling
- React Router for navigation
- Zustand for state management
- Axios for HTTP client

## Prerequisites

### Required

1. **Rust** 1.75 or later
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. **OpenGL Libraries** (Linux) - For STL preview rendering
   ```bash
   # Debian/Ubuntu
   sudo apt-get install -y libgl1-mesa-glx libglu1-mesa
   
   # Fedora/RHEL
   sudo dnf install -y mesa-libGL mesa-libGLU
   
   # Arch Linux
   sudo pacman -S mesa
   ```
   
   **Note**: Most Linux systems already have these libraries installed.

### Optional

- **SQLite CLI** for database inspection (usually pre-installed on Linux/macOS)

## Quick Start

### 1. Clone and Setup

```bash
git clone <repository-url>
cd Glyptotheka
```

### 2. Backend Setup

```bash
cd backend

# Build the project (includes stl-thumb library)
cargo build --release

# Run the application (migrations run automatically)
cargo run --release

# The backend will start on http://localhost:3000
```

### 3. Frontend Setup

In a new terminal:

```bash
cd frontend

# Install dependencies
npm install

# Start development server
npm run dev

# The frontend will start on http://localhost:5173
```

### 4. Configure Your Library

1. Open your browser to http://localhost:5173
2. Enter the path to your 3D print files (e.g., `/home/user/3d-prints`)
3. Click "Start Scan" to index your collection
4. Browse and enjoy!

## Development

### Project Structure

```
Glyptotheka/
├── backend/               # Rust backend
│   ├── src/
│   │   ├── api/          # HTTP API handlers
│   │   ├── db/           # Database layer
│   │   ├── models/       # Data models
│   │   ├── services/     # Business logic
│   │   └── utils/        # Utilities
│   ├── migrations/       # Database migrations
│   └── tests/            # Integration tests
│
├── frontend/             # React frontend
│   ├── src/
│   │   ├── api/         # API client
│   │   ├── components/  # React components
│   │   ├── pages/       # Page components
│   │   ├── hooks/       # Custom hooks
│   │   ├── store/       # State management
│   │   └── types/       # TypeScript types
│   └── tests/           # Component tests
│
└── specs/               # Feature specifications
    └── 001-3d-print-library/
        ├── spec.md      # Feature specification
        ├── plan.md      # Implementation plan
        ├── data-model.md
        ├── quickstart.md
        ├── research.md
        ├── tasks.md
        └── contracts/   # API contracts
```

### Running Tests

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

### Code Quality

**Backend:**
```bash
cd backend
cargo fmt        # Format code
cargo clippy     # Linting
```

**Frontend:**
```bash
cd frontend
npm run lint     # ESLint
npm run format   # Prettier (if configured)
```

## Usage

### Setting Up Your Library

1. **Configure Root Path**: Specify the root folder containing your 3D print files
2. **Initial Scan**: The system recursively scans for STL files and images
3. **Preview Generation**: STL thumbnails are generated automatically using integrated library

### Browsing

- Navigate through the hierarchical folder structure using tile-based interface
- Click on folders to drill down, use breadcrumbs to navigate up
- View project details, STL files, and associated images

### Search & Filtering

- Use the search bar to find projects by name
- Filter by tags for cross-cutting organization
- Combine text search with tag filters

### Tagging

- Add custom tags to projects from the project detail page
- Tags autocomplete from existing tags
- Use tags to create custom organization schemes

### Downloading

- Download individual STL or image files
- Download entire project as ZIP archive
- All downloads are streamed for efficient memory usage

### Rescanning

- Trigger a rescan to update the library when files change
- New projects are added, deleted ones removed
- Tags are preserved across rescans

## Database

The application uses SQLite with the following key tables:

- `projects` - Hierarchical project structure
- `stl_files` - STL file metadata
- `image_files` - Associated images (direct and inherited)
- `tags` - Custom tags
- `project_tags` - Many-to-many relationship
- `cached_files` - Image and preview cache tracking
- `scan_sessions` - Scan history and debugging

Database location: `backend/glyptotheka.db`

## Caching

Generated previews and cached images are stored in:
- `backend/cache/previews/` - STL thumbnails
- `backend/cache/images/` - Cached user images

Cache is managed automatically with LRU eviction when size limits are reached.

## Troubleshooting

### OpenGL Libraries Not Found

If preview generation fails with OpenGL errors:
```bash
# Debian/Ubuntu
sudo apt-get install -y libgl1-mesa-glx libglu1-mesa

# Fedora/RHEL
sudo dnf install -y mesa-libGL mesa-libGLU
```

On headless servers, Mesa provides software rendering (llvmpipe) that works without GPU.

### Database locked

If you see "database is locked" errors:
1. Ensure only one backend instance is running
2. Check for stale WAL files
3. Restart the backend

### Scan hangs or fails

1. Check file permissions on root path
2. Review error log in scan session
3. Look for corrupted STL files
4. Check available disk space for cache

## Performance

Expected performance on modern hardware:
- 100+ projects scanned per minute
- Sub-second search for 10,000 projects
- <2 second tile navigation load times
- <10 second ZIP generation for 50-file projects

## Production Deployment

### Environment Variables

Create a `.env` file in the backend directory:

```bash
# Database configuration
DATABASE_PATH=./glyptotheka.db

# Cache directory
CACHE_DIR=./cache

# Logging level
RUST_LOG=info,glyptotheka_backend=debug
```

### Docker Deployment (Recommended)

The easiest way to deploy is using Docker:

```bash
# Build and run with docker-compose
docker-compose up -d

# View logs
docker-compose logs -f

# Stop
docker-compose down
```

The application will be available at:
- Frontend: http://localhost:8080
- Backend API: http://localhost:3000

### Manual Production Build

**Backend:**
```bash
cd backend
cargo build --release

# The binary will be at target/release/glyptotheka-backend
./target/release/glyptotheka-backend
```

**Frontend:**
```bash
cd frontend
npm run build

# Serve the dist/ folder with any static file server
# Or use a reverse proxy like nginx
```

### System Service (Linux)

Create a systemd service file `/etc/systemd/system/glyptotheka.service`:

```ini
[Unit]
Description=Glyptotheka 3D Print Library
After=network.target

[Service]
Type=simple
User=glyptotheka
WorkingDirectory=/opt/glyptotheka/backend
Environment="DATABASE_PATH=/var/lib/glyptotheka/glyptotheka.db"
Environment="CACHE_DIR=/var/lib/glyptotheka/cache"
Environment="RUST_LOG=info"
ExecStart=/opt/glyptotheka/backend/target/release/glyptotheka-backend
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

Enable and start:
```bash
sudo systemctl enable glyptotheka
sudo systemctl start glyptotheka
sudo systemctl status glyptotheka
```

### Nginx Reverse Proxy

Example nginx configuration:

```nginx
server {
    listen 80;
    server_name your-domain.com;

    # Frontend
    location / {
        root /opt/glyptotheka/frontend/dist;
        try_files $uri $uri/ /index.html;
    }

    # Backend API
    location /api {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }

    # Health check
    location /health {
        proxy_pass http://localhost:3000;
    }
}
```

### Performance Tuning

For production workloads:

1. **Database**: Use WAL mode (enabled by default)
   ```sql
   PRAGMA journal_mode=WAL;
   PRAGMA synchronous=NORMAL;
   ```

2. **Cache Size**: Adjust in database config
   ```sql
   UPDATE config SET cache_max_size_mb = 10000;
   ```

3. **Connection Pool**: Set environment variable
   ```bash
   export DB_POOL_SIZE=10
   ```

4. **Frontend**: Enable gzip compression in nginx/caddy

### Backup & Restore

**Backup:**
```bash
# Database backup
sqlite3 glyptotheka.db ".backup glyptotheka-backup.db"

# Or use cp while backend is stopped
cp glyptotheka.db glyptotheka-backup.db

# Backup cache (optional)
tar -czf cache-backup.tar.gz cache/
```

**Restore:**
```bash
# Stop backend
systemctl stop glyptotheka

# Restore database
cp glyptotheka-backup.db glyptotheka.db

# Restart
systemctl start glyptotheka
```

## Contributing

1. Check `specs/001-3d-print-library/` for feature specifications
2. Follow the implementation plan in `tasks.md`
3. Write tests for new features
4. Ensure CI passes before submitting PR

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [stl-thumb](https://github.com/unlimitedbacon/stl-thumb) for STL preview generation
- Rust and React communities for excellent tooling

## Support

For issues and questions:
- Check documentation in `specs/001-3d-print-library/`
- Review troubleshooting section above
- Open an issue on GitHub

---

**Status**: 🚀 Phase 9 - Polish & Testing

See `specs/001-3d-print-library/tasks.md` for implementation progress.
