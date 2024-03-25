use diesel::prelude::*;
use diesel::sqlite::Sqlite;

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
