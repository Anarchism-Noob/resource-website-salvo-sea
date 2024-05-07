use chrono::{DateTime, Utc};
use salvo::{
    http::*,
    prelude::{Extractible, ToSchema},
};
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct RechargeOfAdminRequest {
    pub user_uuid: String,
    pub balance_usdt: Decimal,
}

#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct ChangePwdRequest {
    pub old_pwd: String,
    pub user_pwd: String,
}

#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct BuyResourcetRequest {
    pub resource_uuid: String,
}

#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct ChangeUserProfileRequest {
    pub nick_name: String,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct CustomUserProfileResponse {
    pub user_uuid: String,
    pub nick_name: String,
    pub user_name: String,
    pub email: Option<String>,
    pub balance_usdt: Decimal,
    pub avatar_path: String,
}

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct CustomUserRegisterRequest {
    pub code: Option<String>,
    pub captcha_uuid: Option<String>,
    pub nick_name: String,
    pub user_name: String,
    pub user_pwd: String,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Extractible, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
// 注册或更改密码成功后的返回
pub struct CustomUserResponse {
    pub user_uuid: String,
    pub nick_name: String,
    pub user_name: String,
}

#[derive(Debug, Deserialize, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
// #[salvo(schema(rename_all="PascalCase"))]
pub struct CustomUserLoginRequest {
    pub code: Option<String>,
    pub captcha_uuid: Option<String>,
    pub user_name: Option<String>,
    pub user_pwd: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema, Default)]
#[salvo(schema(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
// #[salvo(schema(rename_all="PascalCase"))]
pub struct CustomUserLoginResponse {
    pub user_uuid: String,
    pub user_name: String,
    pub token: String,
    pub exp: i64,
}
