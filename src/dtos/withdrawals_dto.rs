use salvo::prelude::ToSchema;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalsResponse {
    pub uuid: String,
    pub user_uuid: String,
    pub quantities: u64,
    pub arrive: u64,
    pub create_date: DateTime,
    pub tariff: u64,
    pub status: u32,
    pub succes_date: Option<DateTime>,
}
