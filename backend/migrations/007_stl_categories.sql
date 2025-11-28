-- Add category column to stl_files table for grouping STL files by folder keywords
ALTER TABLE stl_files ADD COLUMN category TEXT;

-- Create index for faster category-based queries
CREATE INDEX IF NOT EXISTS idx_stl_files_category ON stl_files(project_id, category);

-- Record migration
INSERT INTO schema_migrations (version, description, applied_at) 
VALUES (7, 'Add STL file categories', CAST(strftime('%s', 'now') AS INTEGER));
