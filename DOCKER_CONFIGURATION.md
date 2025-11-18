# Docker Configuration Guide

## Path Configuration

When running Glyptotheka with Docker Compose, it's important to understand the difference between **host paths** and **container paths**.

### Host vs Container Paths

1. **Host Path** (`.env` file):
   ```
   PROJECTS_PATH=/home/stefffri/Workspace/Glyptotheka/example
   ```
   This is the path on your local machine where your 3D models are stored.

2. **Container Path** (in the UI):
   ```
   /projects
   ```
   This is where the host path is mounted **inside** the Docker container.

### Configuration Steps

1. **Edit `.env` file**: Set `PROJECTS_PATH` to point to your 3D models directory on your host machine:
   ```bash
   PROJECTS_PATH=/home/stefffri/Workspace/Glyptotheka/example
   ```

2. **Start Docker Compose**:
   ```bash
   docker-compose up -d
   ```

3. **Open the UI**: Navigate to `http://localhost:8080`

4. **Configure in UI**: In the configuration page, enter `/projects` (not the host path!)
   - ✅ Correct: `/projects`
   - ❌ Wrong: `/home/stefffri/Workspace/Glyptotheka/example`

### Why This Matters

The backend runs inside a Docker container and can only see paths that exist within that container. The `docker-compose.yml` file maps your host directory to `/projects` inside the container:

```yaml
volumes:
  - ${PROJECTS_PATH:-./example}:/projects:ro
```

So when you configure the scan path in the UI, you must use the container path (`/projects`), not the host path.

### Validation

As of the latest update, the backend now validates that the configured path exists before saving it. If you try to save a path that doesn't exist in the container, you'll get a helpful error message:

```
Path does not exist: /home/stefffri/Workspace/Glyptotheka/example. 
Please ensure the path is accessible within the container.
```

This validation prevents configuration errors and makes it clear that you need to use the container path.

### Troubleshooting

**Problem**: "Configuration saving fails"
- **Solution**: Make sure you're using `/projects` in the UI, not your host machine path

**Problem**: "Scan fails with 'path does not exist'"
- **Solution**: 
  1. Check that `PROJECTS_PATH` in `.env` points to an existing directory on your host
  2. Restart docker-compose: `docker-compose down && docker-compose up -d`
  3. Verify the mount: `docker exec glyptotheka-backend ls -la /projects`

**Problem**: "No models found after scanning"
- **Solution**: Make sure your PROJECTS_PATH directory contains STL files and/or subdirectories with STL files
