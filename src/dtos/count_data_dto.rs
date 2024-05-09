use salvo::prelude::ToSchema;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct CountDataResponse {
    pub count_deal: u64,
    pub count_recharge: u64,
    pub count_withdraw: u64,
    pub count_custom: u64,
}
