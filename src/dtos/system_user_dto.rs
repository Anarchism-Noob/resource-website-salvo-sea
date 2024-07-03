use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SystemUser {
    pub id: String,
    pub name: String,
    pub nick_name: String,
    pub email: String,
    pub status: String,
    pub avatar_url: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListSystemUserRequest {}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListSystemUserResponse {
    pub data: Vec<SystemUser>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PageSystemUserRequest {
    pub page_index: i32,
    pub page_size: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PageSystemUserResponse {
    pub data: Vec<SystemUser>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetSystemUserRequest {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetSystemUserResponse {
    pub data: SystemUser,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateSystemUserRequest {
    pub name: String,
    pub nick_name: String,
    pub email: String,
    pub avatar_url: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateSystemUserResponse {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateSystemUserRequest {
    pub id: String,
    pub name: String,
    pub nick_name: String,
    pub email: String,
    pub avatar_url: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateSystemUserResponse {}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteSystemUserRequest {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteSystemUserResponse {}
