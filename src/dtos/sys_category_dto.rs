use salvo::prelude::ToSchema;
use serde::Serialize;
use validator::Validate;

#[derive(Serialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct QueryCategoryResponse {
    pub category_id: i32,
    pub category_name: String,
    pub crate_user_name: String,
}
