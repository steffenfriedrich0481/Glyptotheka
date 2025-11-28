-- 3D Print Library Database Schema
-- Version: 1
-- Description: Initial schema with projects, files, tags, and caching

-- Enable foreign keys
PRAGMA foreign_keys = ON;

-- Projects table (hierarchical structure)
CREATE TABLE projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    full_path TEXT NOT NULL UNIQUE,
    parent_id INTEGER,
    is_leaf BOOLEAN NOT NULL DEFAULT 0,
    folder_level INTEGER NOT NULL DEFAULT 0,
    description TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    
    FOREIGN KEY (parent_id) REFERENCES projects(id) ON DELETE CASCADE,
    
    CHECK (length(name) > 0),
    CHECK (length(full_path) > 0),
    CHECK (created_at > 0),
    CHECK (updated_at >= created_at)
);

CREATE INDEX idx_projects_parent ON projects(parent_id);
CREATE INDEX idx_projects_path ON projects(full_path);
CREATE INDEX idx_projects_leaf ON projects(is_leaf);
CREATE INDEX idx_projects_name ON projects(name);
CREATE INDEX idx_projects_folder_level ON projects(folder_level);

-- Full-text search virtual table for projects
CREATE VIRTUAL TABLE projects_fts USING fts5(
    project_id UNINDEXED,
    name,
    full_path,
    tokenize='porter unicode61'
);

-- Triggers to keep FTS in sync
CREATE TRIGGER projects_fts_insert AFTER INSERT ON projects
BEGIN
    INSERT INTO projects_fts(project_id, name, full_path)
    VALUES (NEW.id, NEW.name, NEW.full_path);
END;

CREATE TRIGGER projects_fts_update AFTER UPDATE ON projects
BEGIN
    UPDATE projects_fts 
    SET name = NEW.name, full_path = NEW.full_path
    WHERE project_id = NEW.id;
END;

CREATE TRIGGER projects_fts_delete AFTER DELETE ON projects
BEGIN
    DELETE FROM projects_fts WHERE project_id = OLD.id;
END;

-- STL files table
CREATE TABLE stl_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    filename TEXT NOT NULL,
    file_path TEXT NOT NULL UNIQUE,
    file_size INTEGER NOT NULL,
    preview_path TEXT,
    preview_generated_at INTEGER,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    
    CHECK (length(filename) > 0),
    CHECK (length(file_path) > 0),
    CHECK (file_size >= 0),
    CHECK (created_at > 0),
    CHECK (updated_at >= created_at)
);

CREATE INDEX idx_stl_files_project ON stl_files(project_id);
CREATE INDEX idx_stl_files_path ON stl_files(file_path);

-- Image files table
CREATE TABLE image_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    filename TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    source_type TEXT NOT NULL DEFAULT 'direct',
    source_project_id INTEGER,
    display_order INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (source_project_id) REFERENCES projects(id) ON DELETE CASCADE,
    
    CHECK (length(filename) > 0),
    CHECK (length(file_path) > 0),
    CHECK (file_size >= 0),
    CHECK (source_type IN ('direct', 'inherited')),
    CHECK (created_at > 0),
    CHECK (updated_at >= created_at),
    CHECK (
        (source_type = 'direct' AND source_project_id IS NULL) OR
        (source_type = 'inherited' AND source_project_id IS NOT NULL)
    )
);

CREATE INDEX idx_image_files_project ON image_files(project_id);
CREATE INDEX idx_image_files_path ON image_files(file_path);
CREATE INDEX idx_image_files_source ON image_files(source_project_id);
CREATE INDEX idx_image_files_order ON image_files(project_id, display_order);

-- Cached files table
CREATE TABLE cached_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    original_path TEXT NOT NULL UNIQUE,
    cache_path TEXT NOT NULL UNIQUE,
    file_type TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    checksum TEXT,
    cached_at INTEGER NOT NULL,
    accessed_at INTEGER NOT NULL,
    
    CHECK (length(original_path) > 0),
    CHECK (length(cache_path) > 0),
    CHECK (file_type IN ('image', 'preview')),
    CHECK (file_size >= 0),
    CHECK (cached_at > 0),
    CHECK (accessed_at >= cached_at)
);

CREATE INDEX idx_cached_files_original ON cached_files(original_path);
CREATE INDEX idx_cached_files_type ON cached_files(file_type);
CREATE INDEX idx_cached_files_accessed ON cached_files(accessed_at);

-- Tags table
CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE COLLATE NOCASE,
    color TEXT,
    created_at INTEGER NOT NULL,
    usage_count INTEGER NOT NULL DEFAULT 0,
    
    CHECK (length(name) > 0 AND length(name) <= 50),
    CHECK (usage_count >= 0)
);

CREATE INDEX idx_tags_name ON tags(name COLLATE NOCASE);
CREATE INDEX idx_tags_usage ON tags(usage_count DESC);

-- Project tags junction table
CREATE TABLE project_tags (
    project_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    
    PRIMARY KEY (project_id, tag_id),
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

CREATE INDEX idx_project_tags_project ON project_tags(project_id);
CREATE INDEX idx_project_tags_tag ON project_tags(tag_id);

-- Triggers to maintain tag usage counts
CREATE TRIGGER project_tags_insert AFTER INSERT ON project_tags
BEGIN
    UPDATE tags SET usage_count = usage_count + 1 WHERE id = NEW.tag_id;
END;

CREATE TRIGGER project_tags_delete AFTER DELETE ON project_tags
BEGIN
    UPDATE tags SET usage_count = usage_count - 1 WHERE id = OLD.tag_id;
END;

-- Scan sessions table
CREATE TABLE scan_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    root_path TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'running',
    started_at INTEGER NOT NULL,
    completed_at INTEGER,
    projects_found INTEGER DEFAULT 0,
    files_processed INTEGER DEFAULT 0,
    errors_count INTEGER DEFAULT 0,
    error_log TEXT,
    
    CHECK (status IN ('running', 'completed', 'failed', 'cancelled')),
    CHECK (projects_found >= 0),
    CHECK (files_processed >= 0),
    CHECK (errors_count >= 0),
    CHECK (completed_at IS NULL OR completed_at >= started_at)
);

CREATE INDEX idx_scan_sessions_status ON scan_sessions(status);
CREATE INDEX idx_scan_sessions_started ON scan_sessions(started_at DESC);

-- Config table (single row)
CREATE TABLE config (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    root_path TEXT,
    last_scan_at INTEGER,
    stl_thumb_path TEXT,
    cache_max_size_mb INTEGER DEFAULT 5000,
    images_per_page INTEGER DEFAULT 20,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Insert default config
INSERT INTO config (id, cache_max_size_mb, images_per_page, created_at, updated_at)
VALUES (1, 5000, 20, strftime('%s', 'now'), strftime('%s', 'now'));

-- Schema version tracking
CREATE TABLE schema_migrations (
    version INTEGER PRIMARY KEY,
    applied_at INTEGER NOT NULL
);

INSERT INTO schema_migrations (version, applied_at)
VALUES (1, strftime('%s', 'now'));
