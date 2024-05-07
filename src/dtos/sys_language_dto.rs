use salvo::prelude::{Extractible, ToSchema};
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct QueryLanguageResponse {
    pub language_id: i32,
    pub language_name: String,
    pub crate_user_name: String,
}
