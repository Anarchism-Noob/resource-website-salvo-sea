use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CasbinResourceDTO {
    pub id: u64,
    pub name: String,
    pub resource_type: String,
    pub display_name: String,
    pub r#type: String,
    pub resource_id: u64,
    pub parent_id: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListCasbinResourceRequest {}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListCasbinResourceResponse {
    pub data: Vec<CasbinResourceDTO>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PageCasbinResourceRequest {
    pub page_index: u64,
    pub page_size: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PageCasbinResourceResponse {
    pub data: Vec<CasbinResourceDTO>,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetCasbinResourceRequest {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetCasbinResourceResponse {
    pub data: CasbinResourceDTO,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateCasbinResourceRequest {
    pub name: String,
    pub resource_type: String,
    pub display_name: String,
    pub r#type: String,
    pub resource_id: u64,
    pub parent_id: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateCasbinResourceResponse {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateCasbinResourceRequest {
    pub id: u64,
    pub name: String,
    pub resource_type: String,
    pub display_name: String,
    pub resource_id: u64,
    pub r#type: String,
    pub parent_id: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateCasbinResourceResponse {}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteCasbinResourceRequest {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteCasbinResourceResponse {}
