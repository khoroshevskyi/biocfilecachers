CREATE TABLE resource (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    rid TEXT,
    rname TEXT UNIQUE,
    create_time DATETIME DEFAULT CURRENT_TIMESTAMP,
    access_time DATETIME DEFAULT CURRENT_TIMESTAMP,
    rpath TEXT,
    rtype TEXT,
    fpath TEXT,
    last_modified_time DATETIME DEFAULT CURRENT_TIMESTAMP,
    etag TEXT,
    expires DATETIME
);

-- Optional indexes
CREATE INDEX idx_resource_rid ON resource (rid);
CREATE INDEX idx_resource_rname ON resource (rname);-- Your SQL goes here
