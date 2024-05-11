use salvo::prelude::{ToSchema};
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct QueryCarouselResponse {
    pub id: i32,
    pub image_uuid: String,
    pub carousel_url: String,
    pub image_to_url: Option<String>,
}

#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct CreateCarouselRequest {
    pub image_uuid: String,
    pub image_to_url: Option<String>,
    pub image_to_description: Option<String>,
}
