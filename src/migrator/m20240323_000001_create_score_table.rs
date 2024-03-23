use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20220602_000001_create_bakery_table"
    }
}

#[derive(Iden)]
pub enum Score {
    Table,
    Id,
    Title,
    Composer,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Score::Table)
                    .col(
                        ColumnDef::new(Score::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Score::Title).string().not_null())
                    .col(ColumnDef::new(Score::Composer).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Score::Table).to_owned())
            .await
    }
}
