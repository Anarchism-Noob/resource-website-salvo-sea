use salvo::prelude::ToSchema;
use serde::Serialize;
use validator::Validate;

#[derive(Serialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct MenuListResponse {
    pub menu_name: String,
    pub menu_url: String,
}
