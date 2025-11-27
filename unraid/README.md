# Installing Glyptotheka on Unraid

This guide explains how to install and configure Glyptotheka as a Docker container on Unraid.

## Prerequisites

- Unraid 6.9 or later
- Docker service enabled
- Directory with your 3D print files (STL files and images)

## Docker Image

The Docker image is automatically built and published to GitHub Container Registry:
- **Image**: `ghcr.io/steffenfriedrich0481/glyptotheka:main`
- **Registry**: GitHub Container Registry (GHCR)
- **Access**: Public (no authentication required)

## Installation Methods

### Method 1: Manual Container Configuration (Recommended)

1. **Open Unraid Docker Tab**
   - Navigate to the **Docker** tab in your Unraid web interface
   - Click **Add Container** at the bottom

2. **Configure Basic Settings**
   - **Name**: `glyptotheka` (or your preferred name)
   - **Repository**: `ghcr.io/steffenfriedrich0481/glyptotheka:main`
   - **Network Type**: `bridge`

3. **Configure Port Mapping**
   - **Container Port**: `8080`
   - **Host Port**: `8080` (or your preferred port)
   - **Connection Type**: `TCP`
   
   This will make the web UI available at `http://YOUR-UNRAID-IP:8080`

4. **Configure Path Mappings (Volumes)**

   Add the following three path mappings:

   | Container Path | Host Path | Access Mode | Description |
   |----------------|-----------|-------------|-------------|
   | `/projects` | `/mnt/user/YOUR-3D-PRINTS-SHARE` | Read-Only | Your 3D print files directory |
   | `/app/data` | `/mnt/user/appdata/glyptotheka/data` | Read/Write | Database storage |
   | `/app/cache` | `/mnt/user/appdata/glyptotheka/cache` | Read/Write | Generated thumbnails |

   **Example configurations:**
   - `/projects` → `/mnt/user/3D-Prints` (read-only)
   - `/app/data` → `/mnt/user/appdata/glyptotheka/data`
   - `/app/cache` → `/mnt/user/appdata/glyptotheka/cache`

5. **Configure Environment Variables**

   Add the following environment variables:

   | Variable | Value | Required | Description |
   |----------|-------|----------|-------------|
   | `ROOT_PATH` | `/projects` | Yes | Root path for scanning files |
   | `DATABASE_PATH` | `/app/data/glyptotheka.db` | Yes | SQLite database location |
   | `CACHE_DIR` | `/app/cache` | Yes | Cache directory for thumbnails |
   | `RUST_LOG` | `info,glyptotheka_backend=debug` | No | Logging level |
   | `IGNORED_KEYWORDS` | `PRESUPPORTED_STL,STL,UNSUPPORTED_STL,Unsupported,Pre-Supported` | No | Folder names to skip in display |

6. **Start Container**
   - Click **Apply** to create and start the container
   - Unraid will pull the image and start the container

7. **Access the Application**
   - Open your browser and navigate to `http://YOUR-UNRAID-IP:8080`
   - Click the **Scan** button in the top right to index your files

### Method 2: Using Community Applications Template

If you have the Community Applications plugin installed:

1. Copy `unraid/my-glyptotheka.xml` to `/boot/config/plugins/dockerMan/templates-user/my-glyptotheka.xml` on your Unraid USB
2. Go to **Docker** tab → **Add Container**
3. Select **Glyptotheka** from the template dropdown
4. Adjust the paths to match your system
5. Click **Apply**

## Configuration Details

### File Categories

Glyptotheka organizes STL files into categories based on folder names. The `IGNORED_KEYWORDS` environment variable defines folder names that are treated as categories rather than subprojects:

- `PRESUPPORTED_STL` / `Pre-Supported`
- `STL`
- `UNSUPPORTED_STL` / `Unsupported`

**Example structure:**
```
/projects/Miniatures/Dragons/RedDragon/
├── Pre-Supported/     ← Category folder
│   └── dragon.stl
├── STL/               ← Category folder
│   └── dragon.stl
└── preview.jpg
```

In this example, "RedDragon" is the project, and files are grouped by category (Pre-Supported, STL).

### Image Inheritance

Images are inherited from parent folders:
- Images in `/projects/Miniatures/` appear for all child projects
- Images in `/projects/Miniatures/Dragons/` appear for all dragon projects
- Images in `/projects/Miniatures/Dragons/RedDragon/` appear only for this project

### STL Preview Generation

The application automatically generates preview thumbnails from STL files during the scan process using the embedded stl-thumb library.

## Troubleshooting

### Container Won't Start
- Check Docker logs in Unraid: Docker tab → click container icon → View Logs
- Ensure all required environment variables are set
- Verify path mappings are correct and directories exist

### No Projects Found After Scan
- Verify `/projects` path mapping points to your STL files
- Check that the directory contains STL files
- Review logs for scanning errors

### Permission Issues
- Ensure cache and data directories are writable by the container
- Check Unraid file permissions on mapped directories

### Database Issues
- Stop the container
- Delete `/mnt/user/appdata/glyptotheka/data/glyptotheka.db`
- Restart container and rescan

## Updating

To update Glyptotheka:

1. Go to **Docker** tab in Unraid
2. Click the container icon → **Force Update**
3. Or manually pull: `docker pull ghcr.io/steffenfriedrich0481/glyptotheka:main`
4. Recreate the container (click **Apply** in container settings)

Your database and cache will be preserved across updates.

## Support

For issues, feature requests, or questions:
- GitHub: https://github.com/steffenfriedrich0481/Glyptotheka
- Issues: https://github.com/steffenfriedrich0481/Glyptotheka/issues
