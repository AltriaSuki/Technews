-- Add migration script here
DROP TABLE IF EXISTS articles;
DROP TABLE IF EXISTS trends;

CREATE TABLE articles (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    url TEXT,
    source TEXT NOT NULL,
    score REAL DEFAULT 0.0,
    author TEXT DEFAULT '',
    timestamp INTEGER NOT NULL,
    tags TEXT DEFAULT '[]', -- JSON string
    comment_count INTEGER DEFAULT 0,
    is_hot_on_source BOOLEAN DEFAULT FALSE
);

CREATE TABLE trends (
    timestamp INTEGER PRIMARY KEY,
    data TEXT NOT NULL, -- JSON data
    metadata TEXT DEFAULT '{}' -- JSON data
);

CREATE INDEX idx_articles_timestamp ON articles(timestamp);
