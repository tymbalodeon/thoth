CREATE TABLE IF NOT EXISTS file_links (
    id INTEGER NOT NULL PRIMARY KEY,
    score_id INTEGER NOT NULL,
    included_file_id INTEGER NOT NULL,
    FOREIGN KEY (score_id) REFERENCES scores (id),
    FOREIGN KEY (included_file_id) REFERENCES included_files (id)
)
