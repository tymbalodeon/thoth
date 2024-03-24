use diesel::prelude::*;

use crate::schema::scores;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::scores)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Score {
    pub id: i32,
    pub title: String,
    pub composer: String,
}

#[derive(Insertable)]
#[diesel(table_name = scores)]
pub struct NewScore<'a> {
    pub title: &'a str,
    pub composer: &'a str,
}
