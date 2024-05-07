//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "custom_orders")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub order_uuid: String,
    pub user_uuid: String,
    pub resource_uuid: String,
    pub resource_name: String,
    pub resource_category: String,
    pub resource_language: String,
    pub download_link: String,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub order_resource_price: Decimal,
    pub creation_date: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
