use eyre::Result;
use sea_orm::{ActiveValue, Database, DatabaseConnection};
use sea_orm::{EntityTrait, InsertResult};
use sea_orm_migration::prelude::*;
use shellexpand::tilde;

use crate::entities::prelude::*;
use crate::entities::score::{self, ActiveModel};
use crate::migrator::Migrator;

const DATABASE_PATH: &str = "~/.local/share/thoth/db.sqlite";

async fn migrate(db: &DatabaseConnection) -> Result<(), DbErr> {
    let schema_manager = SchemaManager::new(db);

    Migrator::up(db, None).await?;

    assert!(schema_manager.has_table("score").await?);

    Ok(())
}

async fn insert(
    db: &DatabaseConnection,
) -> Result<InsertResult<ActiveModel>, DbErr> {
    let score = score::ActiveModel {
        title: ActiveValue::Set("Title".to_owned()),
        composer: ActiveValue::Set("Composer".to_owned()),
        ..Default::default()
    };

    Score::insert(score).exec(db).await
}

pub async fn main() {
    let Ok(db) =
        Database::connect(format!("sqlite:{}?mode=rwc", tilde(DATABASE_PATH)))
            .await
    else {
        return;
    };

    if let Err(err) = migrate(&db).await {
        panic!("{}", err);
    }

    let result = insert(&db).await;

    let _ = dbg!(result);
}
