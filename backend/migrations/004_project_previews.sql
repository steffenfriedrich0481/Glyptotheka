-- Migration 004: Add project_previews table for composite preview images
-- This table stores composite preview images for projects with multiple images

CREATE TABLE IF NOT EXISTS project_previews (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    preview_path TEXT NOT NULL,
    image_count INTEGER NOT NULL,
    source_image_ids TEXT NOT NULL,
    generated_at INTEGER NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Index for fast lookups by project_id
CREATE INDEX IF NOT EXISTS idx_project_previews_project_id ON project_previews(project_id);
