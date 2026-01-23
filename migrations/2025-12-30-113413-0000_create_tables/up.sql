CREATE TABLE groups (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    start_time TIMESTAMP,
    finish_time TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE posts (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    post_order INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE scans (
    id TEXT PRIMARY KEY NOT NULL,
    group_id TEXT NOT NULL REFERENCES groups(id),
    post_id TEXT NOT NULL REFERENCES posts(id),
    arrival_time TIMESTAMP NOT NULL,
    departure_time TIMESTAMP
);
