use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20230903_000001_create_record_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Bakery table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Zhouyi::Table)
                    .col(
                        ColumnDef::new(Zhouyi::Email)
                            .string()
                            .not_null()
                            // .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Zhouyi::Pwd).string().not_null())
                    .col(ColumnDef::new(Zhouyi::ActivationState).string().not_null())
                    .col(ColumnDef::new(Zhouyi::UserType).string().not_null())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Bakery table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Zhouyi::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Record {
    Id,
    YaoId,
    YaoxangId,
    Inps,
    Time,
    Location,
    Analysis,
    UserType,
}