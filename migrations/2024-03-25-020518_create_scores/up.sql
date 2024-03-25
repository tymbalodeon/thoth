CREATE TABLE IF NOT EXISTS scores (
    id INTEGER NOT NULL PRIMARY KEY,
    dedication TEXT,
    title TEXT,
    subtitle TEXT,
    subsubtitle TEXT,
    instrument TEXT,
    poet TEXT,
    composer TEXT,
    meter TEXT,
    arranger TEXT,
    tagline TEXT,
    copyright TEXT,
    piece TEXT,
    opus TEXT,
    ly_file_path TEXT NOT NULL UNIQUE
)
