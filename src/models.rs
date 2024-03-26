use std::fs::read_to_string;

use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use regex::Regex;

use crate::schema::{file_links, included_files, scores};

#[derive(Default, Insertable)]
#[diesel(table_name = included_files)]
pub struct NewIncludedFile<'a> {
    pub path: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = included_files)]
#[diesel(check_for_backend(Sqlite))]
pub struct IncludedFile {
    pub id: i32,
    pub path: String,
}

#[derive(Default, Insertable)]
#[diesel(table_name = file_links)]
pub struct NewFileLink {
    pub score_id: i32,
    pub included_file_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = file_links)]
#[diesel(check_for_backend(Sqlite))]
pub struct FileLink {
    pub id: i32,
    pub score_id: i32,
    pub included_file_id: i32,
}

#[derive(Default, Insertable)]
#[diesel(table_name = scores)]
pub struct NewScore {
    pub dedication: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub subsubtitle: Option<String>,
    pub instrument: Option<String>,
    pub poet: Option<String>,
    pub composer: Option<String>,
    pub meter: Option<String>,
    pub arranger: Option<String>,
    pub tagline: Option<String>,
    pub copyright: Option<String>,
    pub piece: Option<String>,
    pub opus: Option<String>,
    pub ly_file_path: String,
}

impl NewScore {
    fn get_lilypond_header_value(path: &str, key: &str) -> Option<String> {
        let Ok(regex) = Regex::new(&format!("{key}\\s*=\\s*.*")) else {
            return None;
        };

        read_to_string(path).map_or(None, |contents| {
            regex.find(&contents).and_then(|result| {
                result
                    .as_str()
                    .split('=')
                    .last()
                    .map(|thing| thing.trim().replace('"', ""))
            })
        })
    }

    pub fn from_file(path: &str) -> Self {
        let get_header_value = Self::get_lilypond_header_value;

        let dedication = get_header_value(path, "dedication");
        let title = get_header_value(path, "title");
        let subtitle = get_header_value(path, "subtitle");
        let subsubtitle = get_header_value(path, "subsubtitle");
        let instrument = get_header_value(path, "instrument");
        let poet = get_header_value(path, "poet");
        let composer = get_header_value(path, "composer");
        let meter = get_header_value(path, "meter");
        let arranger = get_header_value(path, "arranger");
        let tagline = get_header_value(path, "tagline");
        let copyright = get_header_value(path, "copyright");
        let piece = get_header_value(path, "piece");
        let opus = get_header_value(path, "opus");

        Self {
            dedication,
            title,
            subtitle,
            subsubtitle,
            instrument,
            poet,
            composer,
            meter,
            arranger,
            tagline,
            copyright,
            piece,
            opus,
            ly_file_path: path.to_string(),
        }
    }
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = scores)]
#[diesel(check_for_backend(Sqlite))]
pub struct Score {
    pub id: i32,
    pub dedication: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub subsubtitle: Option<String>,
    pub instrument: Option<String>,
    pub poet: Option<String>,
    pub composer: Option<String>,
    pub meter: Option<String>,
    pub arranger: Option<String>,
    pub tagline: Option<String>,
    pub copyright: Option<String>,
    pub piece: Option<String>,
    pub opus: Option<String>,
    pub ly_file_path: String,
}
