use diesel::prelude::*;
use diesel::sqlite::Sqlite;

use crate::schema::scores;

#[derive(Default, Insertable)]
#[diesel(table_name = scores)]
pub struct NewScore<'a> {
    pub dedication: Option<&'a str>,
    pub title: Option<&'a str>,
    pub subtitle: Option<&'a str>,
    pub subsubtitle: Option<&'a str>,
    pub instrument: Option<&'a str>,
    pub poet: Option<&'a str>,
    pub composer: Option<&'a str>,
    pub meter: Option<&'a str>,
    pub arranger: Option<&'a str>,
    pub tagline: Option<&'a str>,
    pub copyright: Option<&'a str>,
    pub piece: Option<&'a str>,
    pub opus: Option<&'a str>,
    pub ly_file_path: &'a str,
}

#[derive(Queryable, Selectable)]
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
    // pub ily_file: Option<String>,
}
