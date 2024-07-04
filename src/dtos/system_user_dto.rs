use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use crate::entities::{casbin_resource, system_role};

use super::{casbin_resource_dto, system_role_dto};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SystemUserDTO {
    pub id: u64,
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
    pub data: Vec<SystemUserDTO>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PageSystemUserRequest {
    pub page_index: u64,
    pub page_size: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PageSystemUserResponse {
    pub data: Vec<SystemUserDTO>,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetSystemUserRequest {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetSystemUserResponse {
    pub data: SystemUserDTO,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateSystemUserRequest {
    pub name: String,
    pub nick_name: String,
    pub email: String,
    pub avatar_url: String,
    pub casbin_role_ids: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateSystemUserResponse {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateSystemUserRequest {
    pub id: u64,
    pub name: String,
    pub nick_name: String,
    pub email: String,
    pub avatar_url: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateSystemUserResponse {}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteSystemUserRequest {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteSystemUserResponse {}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CurentUserRequest {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CurentUserResponse {
    pub data: SystemUserDTO,
    // pub roles: Vec<system_role_dto::SystemRoleDTO>,
    pub roles: Vec<String>,
    // pub resources: Vec<casbin_resource_dto::CasbinResourceDTO>,
    pub resources: Vec<Vec<String>>,
}
