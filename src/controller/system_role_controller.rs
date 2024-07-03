use salvo::{oapi::extract::JsonBody, prelude::*, Response};

use crate::dtos::system_role_dto::{
    CreateSystemRoleRequest, DeleteSystemRoleRequest, GetSystemRoleRequest, ListSystemRoleRequest,
    PageSystemRoleRequest, UpdateSystemRoleRequest,
};

#[endpoint(tags("角色管理"))]
pub async fn list_system_role(
    request: JsonBody<ListSystemRoleRequest>,
    response: &mut Response,
    depot: &mut Depot,
) {
    response.render(Json("success".to_string()));
}

#[endpoint(tags("角色管理"))]
pub async fn page_system_role(
    request: JsonBody<PageSystemRoleRequest>,
    response: &mut Response,
    depot: &mut Depot,
) {
    response.render(Json("success".to_string()));
}

#[endpoint(tags("角色管理"))]
pub async fn get_system_role(
    request: JsonBody<GetSystemRoleRequest>,
    response: &mut Response,
    depot: &mut Depot,
) {
    response.render(Json("success".to_string()));
}

#[endpoint(tags("角色管理"))]
pub async fn create_system_role(
    request: JsonBody<CreateSystemRoleRequest>,
    response: &mut Response,
    depot: &mut Depot,
) {
    response.render(Json("success".to_string()));
}

#[endpoint(tags("角色管理"))]
pub async fn update_system_role(
    request: JsonBody<UpdateSystemRoleRequest>,
    response: &mut Response,
    depot: &mut Depot,
) {
    response.render(Json("success".to_string()));
}

#[endpoint(tags("角色管理"))]
pub async fn delete_system_role(
    request: JsonBody<DeleteSystemRoleRequest>,
    response: &mut Response,
    depot: &mut Depot,
) {
    response.render(Json("success".to_string()));
}
