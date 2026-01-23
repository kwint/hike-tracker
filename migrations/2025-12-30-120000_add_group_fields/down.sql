-- SQLite doesn't support DROP COLUMN directly, so we need to recreate the table
CREATE TABLE groups_backup (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    start_time TIMESTAMP,
    finish_time TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO groups_backup SELECT id, name, start_time, finish_time, created_at FROM groups;
DROP TABLE groups;
ALTER TABLE groups_backup RENAME TO groups;
