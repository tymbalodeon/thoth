CREATE TABLE files (
    id INTEGER PRIMARY KEY,
    file_type TEXT NOT NULL,
    path TEXT NOT NULL,
    score_id INTEGER NOT NULL REFERENCES scores(id)
)
