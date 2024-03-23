use futures::executor::block_on;
use sea_orm::Database;
use sea_orm_migration::prelude::*;
use shellexpand::tilde;

use crate::migrator::Migrator;

const DATABASE_PATH: &str = "~/.config/thoth/db.sqlite";

async fn run() -> Result<(), DbErr> {
    let db =
        Database::connect(format!("sqlite:{}?mode=rwc", tilde(DATABASE_PATH)))
            .await?;

    let schema_manager = SchemaManager::new(&db);

    Migrator::refresh(&db).await?;
    assert!(schema_manager.has_table("score").await?);

    Ok(())
}

pub fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
