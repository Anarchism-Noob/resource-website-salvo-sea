use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "custom_orders")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub uuid: String,
    pub user_uuid: String,
    pub quantities: u64,
    pub arrive: u64,
    pub create_date: DateTime,
    pub tariff: u64,
    pub status: u32,
    pub succes_date: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
