use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_resource_categoty")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_role: u32,
    pub f_role: Option<u32>,
    pub menu_name: String,
    pub menu_url: String,
    
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}