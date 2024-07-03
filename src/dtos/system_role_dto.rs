use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SystemRole {
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub desc: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListSystemRoleRequest {}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListSystemRoleResponse {
    pub data: Vec<SystemRole>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PageSystemRoleRequest {
    pub page_index: i32,
    pub page_size: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PageSystemRoleResponse {
    pub data: Vec<SystemRole>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetSystemRoleRequest {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetSystemRoleResponse {
    pub data: SystemRole,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateSystemRoleRequest {
    pub name: String,
    pub r#type: String,
    pub desc: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateSystemRoleResponse {
    pub data: SystemRole,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateSystemRoleRequest {
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub desc: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateSystemRoleResponse {}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteSystemRoleRequest {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteSystemRoleResponse {}
