-- Migration 007: Add STL file categories
-- Version: 7
-- Description: Add category column for grouping STL files by folder keywords (PRESUPPORTED_STL, STL, etc.)

-- Add category column to stl_files table for grouping STL files by folder keywords
-- SQLite doesn't support ADD COLUMN IF NOT EXISTS, so we need to check first
-- However, for a migration that runs only once, we can assume the column doesn't exist
ALTER TABLE stl_files ADD COLUMN category TEXT;

-- Create index for faster category-based queries
CREATE INDEX IF NOT EXISTS idx_stl_files_category ON stl_files(project_id, category);

-- Record migration
INSERT INTO schema_migrations (version, applied_at) 
VALUES (7, CAST(strftime('%s', 'now') AS INTEGER));
