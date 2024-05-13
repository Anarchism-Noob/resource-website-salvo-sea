use salvo::prelude::ToSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// 网站信息
#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct WebSiteProfileUpdateRequest {
    pub website_name: Option<String>,
    pub version: Option<String>,
    pub public_record: Option<String>,
    pub website_record: Option<String>,
    pub sys_kefu: Option<String>,
}

#[derive(Serialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct WebSiteProfileResponse {
    pub website_name: Option<String>,
    pub version: Option<String>,
    pub public_record: Option<String>,
    pub website_record: Option<String>,
    pub sys_kefu: Option<String>,
    pub website_logo: Option<String>,
    pub custom_login_bg: Option<String>,
    pub admin_login_bg: Option<String>,
}
