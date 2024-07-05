use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SystemRoleDTO {
    pub id: u64,
    pub name: String,
    pub code: String,
    pub desc: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListSystemRoleRequest {}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListSystemRoleResponse {
    pub data: Vec<SystemRoleDTO>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PageSystemRoleRequest {
    pub page_index: u64,
    pub page_size: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PageSystemRoleResponse {
    pub data: Vec<SystemRoleDTO>,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetSystemRoleRequest {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetSystemRoleResponse {
    pub data: SystemRoleDTO,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateSystemRoleRequest {
    pub name: String,
    pub code: String,
    pub desc: String,
    pub casbin_resource_ids: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateSystemRoleResponse {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateSystemRoleRequest {
    pub id: u64,
    pub name: String,
    pub code: String,
    pub desc: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateSystemRoleResponse {}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteSystemRoleRequest {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteSystemRoleResponse {}
