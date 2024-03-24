use diesel::{
    Connection, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection,
};
use diesel_migrations::{
    embed_migrations, EmbeddedMigrations, MigrationHarness,
};
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
    title: &String,
    composer: &String,
) {
    use crate::schema::scores;

    diesel::insert_into(scores::table)
        .values(&NewScore {
            title: Some(title),
            composer: Some(composer),
            ..NewScore::default()
        })
        .returning(Score::as_returning())
        .get_result(connection)
        .expect("Error saving new post");
}

fn show_scores(connection: &mut SqliteConnection) {
    use crate::schema::scores::dsl::scores;

    let results = scores
        .select(Score::as_select())
        .load(connection)
        .expect("Error loading scores");

    println!("Displaying {} scores", results.len());

    for score in results {
        println!(
            "Title: {:?}, Composer: {:?}",
            score.title.unwrap(),
            score.composer.unwrap()
        );
    }
}

pub fn main(title: &Option<String>, composer: &Option<String>) {
    let database_url = tilde("~/.local/share/thoth/db.sqlite");

    let connection = &mut SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"));

    run_migrations(connection);

    if title.is_some() && composer.is_some() {
        insert_score(
            connection,
            title.as_ref().unwrap(),
            composer.as_ref().unwrap(),
        );
    }

    show_scores(connection);
}
