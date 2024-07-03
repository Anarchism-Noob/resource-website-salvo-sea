use salvo::{oapi::extract::JsonBody, prelude::*, Response};

use crate::dtos::casbin_resource_dto::{
    CreateCasbinResourceRequest, DeleteCasbinResourceRequest, GetCasbinResourceRequest,
    ListCasbinResourceRequest, PageCasbinResourceRequest, UpdateCasbinResourceRequest,
};

#[endpoint(tags("权限资源管理"))]
pub async fn list_casbin_resource(
    request: JsonBody<ListCasbinResourceRequest>,
    depot: &mut Depot,
    response: &mut Response,
) {
    response.render(Json("succes".to_string()));
}

#[endpoint(tags("权限资源管理"))]
pub async fn page_casbin_resource(
    request: JsonBody<PageCasbinResourceRequest>,
    depot: &mut Depot,
    response: &mut Response,
) {
    response.render(Json("success".to_string()));
}

#[endpoint(tags("权限资源管理"))]
pub async fn get_casbin_resource(
    request: JsonBody<GetCasbinResourceRequest>,
    depot: &mut Depot,
    response: &mut Response,
) {
    response.render(Json("success".to_string()));
}

#[endpoint(tags("权限资源管理"))]
pub async fn create_casbin_resource(
    request: JsonBody<CreateCasbinResourceRequest>,
    depot: &mut Depot,
    response: &mut Response,
) {
    response.render(Json("success".to_string()));
}

#[endpoint(tags("权限资源管理"))]
pub async fn update_casbin_resource(
    request: JsonBody<UpdateCasbinResourceRequest>,
    depot: &mut Depot,
    response: &mut Response,
) {
    response.render(Json("success".to_string()));
}

#[endpoint(tags("权限资源管理"))]
pub async fn delete_casbin_resource(
    request: JsonBody<DeleteCasbinResourceRequest>,
    depot: &mut Depot,
    response: &mut Response,
) {
    response.render(Json("success".to_string()));
}
