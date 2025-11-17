# Bug Fix Report - Backend Network Binding Issue

**Date:** 2025-11-17  
**Issue:** Frontend unable to connect to backend API  
**Status:** ✅ FIXED

## Problem Description

When starting the backend and frontend services:
1. ❌ Config path field was empty despite `.env` configuration
2. ❌ Rescan button showed "Failed to start scan" error
3. ❌ Frontend couldn't communicate with backend API

## Root Cause

The backend server was binding to `127.0.0.1:3000` (localhost only) instead of `0.0.0.0:3000` (all interfaces), making it inaccessible from Docker containers and external connections.

**Location:** `backend/src/main.rs` line 61

```rust
// BEFORE (broken):
let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

// AFTER (fixed):
let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
```

## Impact

- Backend was running but only accepting connections from within its own container
- Frontend (running in separate container) couldn't reach the API
- All API calls (`/api/config`, `/api/scan`, `/api/projects`, etc.) were failing

## Solution

Changed the server binding address from `127.0.0.1` to `0.0.0.0` to accept connections from all network interfaces.

### File Modified
- `backend/src/main.rs` (line 61)

### Changes Made
```diff
- let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
+ let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
```

## Testing Results

### ✅ Configuration Loading
- Config endpoint now accessible: `GET /api/config`
- Returns default config with empty `root_path`
- Frontend loads config successfully

### ✅ Configuration Saving  
- Can enter `/projects` path in UI
- Save button works correctly
- Shows "Configuration saved successfully!" alert
- Config persists to database

### ✅ Scanning Functionality
- Rescan button triggers scan successfully
- Shows "Scanning..." status during scan
- Polls status endpoint every 1s
- Returns to "Rescan" after completion

### ✅ Browse Projects
- Found 1 project (`projects`)
- Can navigate into projects
- Shows sub-projects (Miniaturen folder)
- Project detail page loads correctly

### ✅ All API Endpoints Working
- `GET /api/config` ✅
- `POST /api/config` ✅
- `POST /api/scan` ✅
- `GET /api/scan/status` ✅
- `GET /api/projects` ✅
- `GET /api/projects/:id` ✅
- `GET /api/projects/:id/children` ✅

## Additional Notes

### Environment Configuration
The `.env` file sets `PROJECTS_PATH=/home/stefffri/Workspace/Glyptotheka/example` which is mounted to `/projects` inside the container. This path must be configured in the UI for the first time after database reset.

### Database Initialization
After cleaning the database:
1. Migrations run automatically on startup
2. Default config row created with `id=1`
3. User must set `root_path` via UI
4. Then run scan to discover projects

## Prevention

This type of issue can be prevented by:
1. Using `0.0.0.0` for containerized services by default
2. Making bind address configurable via environment variable
3. Adding health check endpoints early in development
4. Testing container-to-container communication

## Recommendation

Consider making the bind address configurable:
```rust
let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
```

## Status

**RESOLVED** - All frontend-backend connectivity is now working correctly. The application is fully functional.
