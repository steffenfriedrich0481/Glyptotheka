-- Performance optimization migration
-- Version: 2
-- Description: Add composite indexes and query optimizations

-- Add composite indexes for common query patterns
CREATE INDEX IF NOT EXISTS idx_projects_parent_leaf ON projects(parent_id, is_leaf);
CREATE INDEX IF NOT EXISTS idx_projects_parent_name ON projects(parent_id, name);

-- Add covering index for project listing
CREATE INDEX IF NOT EXISTS idx_projects_list ON projects(parent_id, is_leaf, name, id);

-- Optimize tag search queries
CREATE INDEX IF NOT EXISTS idx_project_tags_tag_project ON project_tags(tag_id, project_id);

-- Optimize file queries with composite indexes
CREATE INDEX IF NOT EXISTS idx_stl_files_project_name ON stl_files(project_id, filename);
CREATE INDEX IF NOT EXISTS idx_image_files_project_order ON image_files(project_id, display_order, id);

-- Add index for timestamp-based queries
CREATE INDEX IF NOT EXISTS idx_projects_updated ON projects(updated_at DESC);
CREATE INDEX IF NOT EXISTS idx_stl_files_updated ON stl_files(updated_at DESC);

-- Optimize cache lookup queries
CREATE INDEX IF NOT EXISTS idx_cached_files_type_accessed ON cached_files(file_type, accessed_at DESC);

-- Update schema version
INSERT INTO schema_migrations (version, applied_at)
VALUES (2, strftime('%s', 'now'));
