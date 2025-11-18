-- Migration 005: Add image priority and source columns for STL preview support
-- Date: 2025-11-18
-- Feature: STL Preview Image Generation

-- Add image priority column (higher = more important)
-- 100 = regular images, 50 = STL previews, 25 = composite previews
ALTER TABLE image_files 
ADD COLUMN image_priority INTEGER NOT NULL DEFAULT 100;

-- Add image source column to track image type
ALTER TABLE image_files 
ADD COLUMN image_source TEXT NOT NULL DEFAULT 'regular' 
  CHECK (image_source IN ('regular', 'stl_preview', 'composite'));

-- Create index for priority-based queries
CREATE INDEX idx_image_files_priority 
  ON image_files(project_id, image_priority DESC, display_order ASC);

-- Update schema version
INSERT INTO schema_migrations (version, applied_at)
VALUES (5, strftime('%s', 'now'));
