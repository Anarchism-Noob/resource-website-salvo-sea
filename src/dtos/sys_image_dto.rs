use chrono::{DateTime, Utc};
use salvo::prelude::{Extractible, ToSchema};
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct SysImageCrateRequest {
    pub image_uuid: String,
    pub image_name: String,
    pub image_path: String,
    pub description: Option<String>,
    pub usage_location: Option<String>,
}
