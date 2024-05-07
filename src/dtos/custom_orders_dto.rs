use salvo::prelude::ToSchema;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct CustomOrderCrateRequest {
    pub order_uuid: String,
    pub user_uuid: String,
    pub resource_uuid: String,
    pub resource_name: String,
    pub resource_category: String,
    pub resource_language: String,
    pub download_link: String,
    pub order_resource_price: u64,
    pub creation_date: DateTime,
}

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema, Default, Clone)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct CustomOrderResponse {
    pub order_uuid: String,
    pub resource_uuid: String,
    pub resource_name: String,
    pub resource_category: String,
    pub resource_language: String,
    pub download_link: String,
    pub order_resource_price: u64,
    pub creation_date: DateTime,
}
