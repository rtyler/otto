--
-- This migration adds the projects table which should be largely populated out
-- of the projects.d directory


CREATE TABLE IF NOT EXISTS projects
(
    uuid BLOB PRIMARY KEY NOT NULL,
    path TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    source_url TEXT,
    source_refspec TEXT,
    pipeline_path TEXT,
    pipeline_inline TEXT
);
