use std::fs::read_to_string;

use diesel::{
    insert_into, Connection, QueryDsl, RunQueryDsl, SelectableHelper,
    SqliteConnection,
};
use diesel_migrations::{
    embed_migrations, EmbeddedMigrations, MigrationHarness,
};
use regex::Regex;
use rust_search::SearchBuilder;
use shellexpand::tilde;

use crate::models::{NewScore, Score};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn run_migrations(
    connection: &mut impl MigrationHarness<diesel::sqlite::Sqlite>,
) {
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Could not run migrations");
}

fn insert_score(
    connection: &mut SqliteConnection,
    ly_file_path: &String,
    title: Option<String>,
    composer: Option<String>,
) -> Score {
    use crate::schema::scores;

    insert_into(scores::table)
        .values(&NewScore {
            ly_file_path: ly_file_path.to_string(),
            title,
            composer,
            ..NewScore::default()
        })
        .returning(Score::as_returning())
        .get_result(connection)
        .expect("Error saving new post")
}

fn show_scores(connection: &mut SqliteConnection) {
    use crate::schema::scores::dsl::scores;

    let results = scores
        .select(Score::as_select())
        .load(connection)
        .expect("Error loading scores");

    println!("Displaying {} scores", results.len());

    for score in results {
        println!("Score: {score:?}\n");
    }
}

fn get_title(path: &str) -> Option<String> {
    let regex = Regex::new(r"title\s*=\s*.*").unwrap();

    if let Ok(contents) = read_to_string(path) {
        regex.find(&contents).map(|result| {
            result
                .as_str()
                .split('=')
                .last()
                .unwrap()
                .trim()
                .replace('"', "")
        })
    } else {
        None
    }
}

fn get_composer(path: &str) -> Option<String> {
    let regex = Regex::new(r"composer\s*=\s*.*").unwrap();

    if let Ok(contents) = read_to_string(path) {
        regex.find(&contents).map(|result| {
            result
                .as_str()
                .split('=')
                .last()
                .unwrap()
                .trim()
                .replace('"', "")
        })
    } else {
        None
    }
}
pub fn main(import: &bool) {
    let database_url = tilde("~/.local/share/thoth/db.sqlite");

    let connection = &mut SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"));

    run_migrations(connection);

    if *import {
        let search: Vec<String> = SearchBuilder::default()
            .location("~")
            .ext("ly")
            .build()
            .collect();

        for path in search {
            let title = get_title(&path);
            let composer = get_composer(&path);
            let score = insert_score(connection, &path, title, composer);

            println!("{score:?}");
        }
    }

    show_scores(connection);
}
