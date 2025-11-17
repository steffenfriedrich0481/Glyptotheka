-- Migration: Remove stl_thumb_path configuration
-- Removes the external tool path configuration as stl-thumb is now integrated as a library

BEGIN TRANSACTION;

-- Create new config table without stl_thumb_path column
CREATE TABLE config_new (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    root_path TEXT,
    last_scan_at INTEGER,
    cache_max_size_mb INTEGER DEFAULT 5000,
    images_per_page INTEGER DEFAULT 20,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Copy existing data (excluding stl_thumb_path)
INSERT INTO config_new (id, root_path, last_scan_at, cache_max_size_mb, images_per_page, created_at, updated_at)
SELECT id, root_path, last_scan_at, cache_max_size_mb, images_per_page, created_at, updated_at
FROM config;

-- Drop old table
DROP TABLE config;

-- Rename new table to config
ALTER TABLE config_new RENAME TO config;

COMMIT;
