//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "custom_recharge_records")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub record_uuid: String,
    pub user_uuid: String,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub recharge_amount: u64,
    pub payment_channel: String,
    pub recharge_status: u32,
    pub recharge_date: DateTime,
    pub transaction_id: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub remark: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::custom_user::Entity",
        from = "Column::UserUuid",
        to = "super::custom_user::Column::UserUuid",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    CustomUser,
}

impl Related<super::custom_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CustomUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}