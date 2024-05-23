use salvo::prelude::ToSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct ChangeAdminProfileRequest {
    pub nick_name: String,
    pub liaison: String,
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
    pub liaison: String,
    pub role: u32,
    pub avatar_path: String,
}

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct SysUserProfileResponse {
    pub user_uuid: String,
    pub nick_name: String,
    pub user_name: String,
    pub liaison: String,
    pub balance: u64,
    pub role: u32,
    pub avatar_path: String,
}

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct SysLoginRequest {
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
