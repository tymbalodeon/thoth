use diesel::{
    insert_into, Connection, QueryDsl, RunQueryDsl, SelectableHelper,
    SqliteConnection,
};
use diesel_migrations::{
    embed_migrations, EmbeddedMigrations, MigrationHarness,
};
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
    new_score: NewScore,
) -> Score {
    use crate::schema::scores;

    insert_into(scores::table)
        .values(new_score)
        .returning(Score::as_returning())
        .get_result(connection)
        .expect("Error saving new post")
}

fn show_scores(connection: &mut SqliteConnection) {
    use crate::schema::scores::dsl::scores;

    let results = scores
        .select(Score::as_select())
        .limit(100)
        .load(connection)
        .expect("Error loading scores");

    println!("Displaying {} scores", results.len());

    for score in results {
        score.display();
    }
}

pub fn main(import: bool) {
    let database_url = tilde("~/.local/share/thoth/db.sqlite");

    let connection = &mut SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"));

    run_migrations(connection);

    if import {
        let search: Vec<String> = SearchBuilder::default()
            .location("~")
            .ext("ly")
            .build()
            .collect();

        for path in search {
            let score = insert_score(connection, NewScore::from_file(&path));

            println!("{score:?}");
        }
    }

    show_scores(connection);
}
