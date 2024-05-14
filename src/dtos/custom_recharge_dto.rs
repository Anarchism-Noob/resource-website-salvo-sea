use chrono::{DateTime, Utc};
use salvo::prelude::ToSchema;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct CustomRechargeRecordsCreateRequest {
    pub record_uuid: String,
    pub user_uuid: String,
    pub recharge_amount: u64,
    pub payment_channel: String,
    pub recharge_status: u32,
    pub recharge_date: DateTime<Utc>,
    pub transaction_id: String,
    pub remark: Option<String>,
}
