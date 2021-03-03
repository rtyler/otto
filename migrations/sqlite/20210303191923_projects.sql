--
-- This migration adds the projects table which should be largely populated out
-- of the projects.d directory


CREATE TABLE IF NOT EXISTS projects
(
    uuid TEXT PRIMARY KEY NOT NULL,
    path TEXT UNIQUE NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    source_url TEXT,
    source_refspec TEXT,
    pipeline_path TEXT,
    pipeline_inline TEXT,
    created_at DATETIME DEFAULT (DATETIME('now')),
    last_updated_at DATETIME DEFAULT (DATETIME('now'))
);
