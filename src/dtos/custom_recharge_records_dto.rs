use salvo::prelude::{ToSchema, Extractible};
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all= "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct CustomRechargeRecordsCreateRequest {
    pub record_uuid: String,
    pub user_uuid: String,
    pub recharge_amount: Decimal,
    pub payment_channel: String,
    pub recharge_status: i32,
    pub recharge_date: DateTime<Utc>,
    pub transaction_id: String,
    pub remark: Option<String>,    
}