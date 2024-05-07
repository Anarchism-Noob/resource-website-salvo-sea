use chrono::{DateTime, Utc};
use salvo::prelude::{Extractible, ToSchema};
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct ChangeAdminProfileRequest {
    pub nick_name: String,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct ChangeAdminPwdRequest {
    pub user_uuid: String,
    pub old_pwd: String,
    pub new_pwd: String,
}

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct SysUserCrateRequest {
    pub nick_name: String,
    pub user_name: String,
    pub user_pwd: String,
    pub email: Option<String>,
    pub role: i32,
    pub avatar_path: String,
}

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct SysUserProfileResponse {
    pub user_uuid: String,
    pub nick_name: String,
    pub user_name: String,
    pub email: Option<String>,
    pub role: i32,
    pub avatar_path: String,
}

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct SysLoginRequest {
    pub code: Option<String>,
    pub captcha_uuid: Option<String>,
    pub user_name: String,
    pub user_pwd: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
// #[salvo(schema(rename_all="PascalCase"))]
pub struct SysLoginResponse {
    pub user_uuid: String,
    pub user_name: String,
    pub token: String,
    pub exp: i64,
}
