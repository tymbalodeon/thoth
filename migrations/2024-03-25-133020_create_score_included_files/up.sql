CREATE TABLE IF NOT EXISTS score_included_files ( 
    PRIMARY KEY (score_file, included_file),
    score_file NOT NULL REFERENCES scores, 
    included_file REFERENCES included_files
)
