-- Migration for articles and trends tables
CREATE TABLE IF NOT EXISTS articles (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    url TEXT NOT NULL DEFAULT '',
    source TEXT NOT NULL,
    score REAL DEFAULT 0.0,
    author TEXT DEFAULT '',
    timestamp INTEGER NOT NULL,
    tags TEXT DEFAULT '[]', -- JSON string
    comment_count INTEGER DEFAULT 0,
    is_hot_on_source BOOLEAN DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS trends (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp INTEGER NOT NULL,
    data TEXT NOT NULL, -- JSON data
    metadata TEXT DEFAULT '{}' -- JSON data
);

CREATE INDEX IF NOT EXISTS idx_articles_timestamp ON articles(timestamp);
CREATE INDEX IF NOT EXISTS idx_trends_timestamp ON trends(timestamp);
