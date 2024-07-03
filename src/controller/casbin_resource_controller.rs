use salvo::{oapi::extract::JsonBody, prelude::*, Response};

use crate::dtos::casbin_resource_dto::{
    CreateCasbinResourceRequest, DeleteCasbinResourceRequest, GetCasbinResourceRequest,
    ListCasbinResourceRequest, PageCasbinResourceRequest, UpdateCasbinResourceRequest,
};

#[endpoint(tags("权限资源管理"))]
pub async fn list_casbin_resource(
    request: JsonBody<ListCasbinResourceRequest>,
    response: &mut Response,
    depot: &mut Depot,
) {
    response.render(Json("succes".to_string()));
}

#[endpoint(tags("权限资源管理"))]
pub async fn page_casbin_resource(
    request: JsonBody<PageCasbinResourceRequest>,
    response: &mut Response,
    depot: &mut Depot,
) {
    response.render(Json("success".to_string()));
}

#[endpoint(tags("权限资源管理"))]
pub async fn get_casbin_resource(
    request: JsonBody<GetCasbinResourceRequest>,
    response: &mut Response,
    depot: &mut Depot,
) {
    response.render(Json("success".to_string()));
}

#[endpoint(tags("权限资源管理"))]
pub async fn create_casbin_resource(
    request: JsonBody<CreateCasbinResourceRequest>,
    response: &mut Response,
    depot: &mut Depot,
) {
    response.render(Json("success".to_string()));
}

#[endpoint(tags("权限资源管理"))]
pub async fn update_casbin_resource(
    request: JsonBody<UpdateCasbinResourceRequest>,
    response: &mut Response,
    depot: &mut Depot,
) {
    response.render(Json("success".to_string()));
}

#[endpoint(tags("权限资源管理"))]
pub async fn delete_casbin_resource(
    request: JsonBody<DeleteCasbinResourceRequest>,
    response: &mut Response,
    depot: &mut Depot,
) {
    response.render(Json("success".to_string()));
}
