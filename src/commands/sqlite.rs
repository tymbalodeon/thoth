use diesel::RunQueryDsl;
use diesel::SelectableHelper;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{
    embed_migrations, EmbeddedMigrations, MigrationHarness,
};
use shellexpand::tilde;

use crate::models::{NewScore, Score};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

type DB = diesel::sqlite::Sqlite;

fn run_migrations(connection: &mut impl MigrationHarness<DB>) {
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Could not run migrations");
}

pub fn main() {
    let database_url = tilde("~/.local/share/thoth/db.sqlite");

    let connection = &mut SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    run_migrations(connection);

    use crate::schema::scores;

    let new_score = NewScore {
        title: "Title",
        composer: "Composer",
    };

    diesel::insert_into(scores::table)
        .values(&new_score)
        .returning(Score::as_returning())
        .get_result(connection)
        .expect("Error saving new post");
}
