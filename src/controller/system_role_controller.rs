use salvo::{oapi::extract::JsonBody, prelude::*, Response};

use crate::dtos::system_role_dto::{
    CreateSystemRoleRequest, DeleteSystemRoleRequest, GetSystemRoleRequest, ListSystemRoleRequest,
    PageSystemRoleRequest, UpdateSystemRoleRequest,
};

#[endpoint(tags("角色管理"))]
pub async fn list_system_role(
    request: JsonBody<ListSystemRoleRequest>,
    depot: &mut Depot,
    response: &mut Response,
) {
    response.render(Json("success".to_string()));
}

#[endpoint(tags("角色管理"))]
pub async fn page_system_role(
    request: JsonBody<PageSystemRoleRequest>,
    depot: &mut Depot,
    response: &mut Response,
) {
    response.render(Json("success".to_string()));
}

#[endpoint(tags("角色管理"))]
pub async fn get_system_role(
    request: JsonBody<GetSystemRoleRequest>,
    depot: &mut Depot,
    response: &mut Response,
) {
    response.render(Json("success".to_string()));
}

#[endpoint(tags("角色管理"))]
pub async fn create_system_role(
    request: JsonBody<CreateSystemRoleRequest>,
    depot: &mut Depot,
    response: &mut Response,
) {
    response.render(Json("success".to_string()));
}

#[endpoint(tags("角色管理"))]
pub async fn update_system_role(
    request: JsonBody<UpdateSystemRoleRequest>,
    depot: &mut Depot,
    response: &mut Response,
) {
    response.render(Json("success".to_string()));
}

#[endpoint(tags("角色管理"))]
pub async fn delete_system_role(
    request: JsonBody<DeleteSystemRoleRequest>,
    depot: &mut Depot,
    response: &mut Response,
) {
    response.render(Json("success".to_string()));
}
