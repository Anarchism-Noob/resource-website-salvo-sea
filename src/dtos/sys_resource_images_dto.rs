use salvo::prelude::{ToSchema};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct SysResourceImageCreateRequest {
    pub id: i32,
    pub resource_uuid: i32,
    pub image_uuid: i32,
}
