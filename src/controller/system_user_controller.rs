use salvo::{oapi::extract::JsonBody, prelude::*, Response};

use crate::{
    cerror::ERR_SYSTEM_INTERNAL,
    common::{self, error, resolve_code_error, success},
    dtos::system_user_dto::{
        CreateSystemUserRequest, DeleteSystemUserRequest, GetSystemUserRequest,
        ListSystemUserRequest, PageSystemUserRequest, UpdateSystemUserRequest,
    },
    services::system_user_service,
};

#[endpoint(tags("用户管理"))]
pub async fn list_system_user(
    request: JsonBody<ListSystemUserRequest>,
    depot: &mut Depot,
    response: &mut Response,
) {
    response.render(Json("success".to_string()));
}

#[endpoint(tags("用户管理"))]
pub async fn page_system_user(
    request: JsonBody<PageSystemUserRequest>,
    depot: &mut Depot,
    response: &mut Response,
) {
    response.render(Json("success".to_string()));
}

#[endpoint(tags("用户管理"))]
pub async fn get_system_user(
    request: JsonBody<GetSystemUserRequest>,
    depot: &mut Depot,
    response: &mut Response,
) {
    resolve_code_error(response, || system_user_service::get_system_user(request.0));
    // system_user_service::get_system_user(request.0)
    //     .await
    //     .map(|data| success(response, data))
    //     .map_err(|err| error(response, err))
    //     .map_or_else(|f| f, f)
}

#[endpoint(tags("用户管理"))]
pub async fn create_system_user(
    request: JsonBody<CreateSystemUserRequest>,
    depot: &mut Depot,
    response: &mut Response,
) {
    response.render(Json("success".to_string()));
}

#[endpoint(tags("用户管理"))]
pub async fn update_system_user(
    request: JsonBody<UpdateSystemUserRequest>,
    depot: &mut Depot,
    response: &mut Response,
) {
    response.render(Json("success".to_string()));
}

#[endpoint(tags("用户管理"))]
pub async fn delete_system_user(
    request: JsonBody<DeleteSystemUserRequest>,
    depot: &mut Depot,
    response: &mut Response,
) {
    response.render(Json("success".to_string()));
}
