use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tamagotchi")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub fid: i64,
    pub color: String,
    pub option: i8,
    pub created_at: u64,
    pub last_interaction: u64,
    pub health: i8,
    pub hunger: i8,
    pub sleepiness: i8,
    pub dirtiness: i8,
    pub happiness: i8,
    pub version: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}