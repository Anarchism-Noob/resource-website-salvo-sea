use chrono::{DateTime, Utc};
use salvo::prelude::ToSchema;
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct SysResourceChangeLink {
    pub resource_link: String,
}

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct SysResourceList {
    pub resource_uuid: String,
    pub resource_name: String,
    pub resource_price: Decimal,
    pub category: String,
    pub language: String,
    pub resource_image: String,
}

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct SysResourceResponse {
    pub resource_uuid: String,
    pub resource_name: String,
    pub description: Option<String>,
    pub description_file_path: Option<String>,
    pub resource_price: u64,
    pub category: String,
    pub language: String,
    pub resource_link: String,
    pub create_user_name: String,
    pub resource_image: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct SysResourceCreateRequest {
    pub image_uuids: Vec<String>,
    pub resource_name: String,
    pub description: Option<String>,
    pub description_file_path: Option<String>,
    pub resource_price: u64,
    pub category: String,
    pub language: String,
    pub resource_link: String,
    pub create_date: DateTime<Utc>,
    pub create_user_name: String,
}
