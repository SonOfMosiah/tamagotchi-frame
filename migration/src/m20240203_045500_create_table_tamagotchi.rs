use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tamagotchi::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tamagotchi::Fid)
                            .integer()
                            .not_null()
                            .primary_key(), // Note: auto_increment is removed as per schema definition
                    )
                    .col(ColumnDef::new(Tamagotchi::Color).string().not_null())
                    .col(ColumnDef::new(Tamagotchi::Option).tiny_integer().not_null())
                    .col(ColumnDef::new(Tamagotchi::CreatedAt).big_integer().not_null())
                    .col(ColumnDef::new(Tamagotchi::LastInteraction).big_integer().not_null())
                    .col(ColumnDef::new(Tamagotchi::Health).tiny_integer().not_null())
                    .col(ColumnDef::new(Tamagotchi::Hunger).tiny_integer().not_null())
                    .col(ColumnDef::new(Tamagotchi::Sleepiness).tiny_integer().not_null())
                    .col(ColumnDef::new(Tamagotchi::Dirtiness).tiny_integer().not_null())
                    .col(ColumnDef::new(Tamagotchi::Happiness).tiny_integer().not_null())
                    .col(ColumnDef::new(Tamagotchi::Version).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tamagotchi::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Tamagotchi {
    Table,
    Fid,
    Color,
    Option,
    CreatedAt,
    LastInteraction,
    Health,
    Hunger,
    Sleepiness,
    Dirtiness,
    Happiness,
    Version,
}
