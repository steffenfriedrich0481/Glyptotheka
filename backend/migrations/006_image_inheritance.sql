-- Migration 006: Image Inheritance Tracking
-- Version: 6
-- Description: Add support for tracking image inheritance down the folder hierarchy
-- This enables images found at any level to be inherited by all descendant projects
-- Note: folder_level column is now in migration 001

-- Create image_inheritance table to track which images are inherited by which projects
CREATE TABLE image_inheritance (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    image_id INTEGER NOT NULL,
    source_project_id INTEGER NOT NULL,
    inherited_from_path TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (image_id) REFERENCES image_files(id) ON DELETE CASCADE,
    FOREIGN KEY (source_project_id) REFERENCES projects(id) ON DELETE CASCADE,
    
    -- Prevent duplicate inheritance entries
    UNIQUE(project_id, image_id)
);

-- Indexes for efficient inheritance queries
CREATE INDEX idx_image_inheritance_project ON image_inheritance(project_id);
CREATE INDEX idx_image_inheritance_image ON image_inheritance(image_id);
CREATE INDEX idx_image_inheritance_source ON image_inheritance(source_project_id);

-- Insert migration record
INSERT INTO schema_migrations (version, applied_at) 
VALUES (6, strftime('%s', 'now'));
