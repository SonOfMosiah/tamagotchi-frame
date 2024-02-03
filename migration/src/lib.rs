pub use sea_orm_migration::prelude::*;
mod m20240203_045500_create_table_tamagotchi;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240203_045500_create_table_tamagotchi::Migration),
        ]
    }
}
