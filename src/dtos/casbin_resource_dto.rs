use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CasbinResource {
    pub id: String,
    pub name: String,
    pub resource_type: String,
    pub display_name: String,
    pub r#type: String,
    pub resource_id: String,
    pub create_time: String,
    pub update_time: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListCasbinResourceRequest {}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListCasbinResourceResponse {
    pub data: Vec<CasbinResource>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PageCasbinResourceRequest {
    pub page_index: i32,
    pub page_size: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PageCasbinResourceResponse {
    pub data: Vec<CasbinResource>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetCasbinResourceRequest {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetCasbinResourceResponse {
    pub data: Option<CasbinResource>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateCasbinResourceRequest {
    pub name: String,
    pub resource_type: String,
    pub display_name: String,
    pub r#type: String,
    pub resource_id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateCasbinResourceResponse {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateCasbinResourceRequest {
    pub id: String,
    pub name: Option<String>,
    pub resource_type: Option<String>,
    pub display_name: Option<String>,
    pub r#type: Option<String>,
    pub resource_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateCasbinResourceResponse {}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteCasbinResourceRequest {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteCasbinResourceResponse {}
