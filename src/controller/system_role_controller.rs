use salvo::{http::request, prelude::*, Response};
use tracing::error;

use crate::{
    cerror::{ERR_REQUEST_PARAM_ERROR, ERR_REQUEST_PARAM_INVALID},
    common::{failed, resolve_code_error},
    constant::DEFAULT_PAGE_SIZE,
    dtos::system_role_dto::{
        CreateSystemRoleRequest, DeleteSystemRoleRequest, GetSystemRoleRequest,
        ListSystemRoleRequest, PageSystemRoleRequest, UpdateSystemRoleRequest,
    },
    services::system_role_service,
};

#[endpoint(tags("角色管理"))]
pub async fn list_system_role(request: &mut Request, response: &mut Response) {
    match request.parse_queries::<ListSystemRoleRequest>() {
        Err(err) => {
            error!("parse param err: {:?}", err);
            failed(response, ERR_REQUEST_PARAM_INVALID);
        }
        Ok(request) => {
            resolve_code_error(response, || system_role_service::list_system_role(request)).await;
        }
    }
}

#[endpoint(tags("角色管理"))]
pub async fn page_system_role(request: &mut Request, response: &mut Response) {
    match request.parse_queries::<PageSystemRoleRequest>() {
        Err(err) => {
            error!("parse param err: {:?}", err);
            failed(response, ERR_REQUEST_PARAM_INVALID);
        }
        Ok(mut request) => {
            if request.page_size == 0 {
                request.page_size = DEFAULT_PAGE_SIZE;
            }

            resolve_code_error(response, || system_role_service::page_system_role(request)).await;
        }
    }
}

#[endpoint(tags("角色管理"))]
pub async fn get_system_role(request: &mut Request, response: &mut Response) {
    match request.parse_queries::<GetSystemRoleRequest>() {
        Err(err) => {
            error!("parse param err: {:?}", err);
            failed(response, ERR_REQUEST_PARAM_INVALID);
        }
        Ok(request) => {
            if request.id == 0 {
                error!("id is 0");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            resolve_code_error(response, || system_role_service::get_system_role(request)).await;
        }
    }
}

#[endpoint(tags("角色管理"))]
pub async fn create_system_role(request: &mut Request, response: &mut Response) {
    match request.parse_body::<CreateSystemRoleRequest>().await {
        Err(err) => {
            error!("parse param err: {:?}", err);
            failed(response, ERR_REQUEST_PARAM_INVALID);
        }
        Ok(request) => {
            if request.name.trim().is_empty() {
                error!("name is empty");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            if request.code.trim().is_empty() {
                error!("code is empty");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            if request.desc.trim().is_empty() {
                error!("desc is empty");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            resolve_code_error(response, || {
                system_role_service::create_system_role(request)
            })
            .await;
        }
    }
}

#[endpoint(tags("角色管理"))]
pub async fn update_system_role(request: &mut Request, response: &mut Response) {
    match request.parse_body::<UpdateSystemRoleRequest>().await {
        Err(err) => {
            error!("parse param err: {:?}", err);
            failed(response, ERR_REQUEST_PARAM_INVALID);
        }
        Ok(request) => {
            if request.id == 0 {
                error!("id is 0");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            if request.name.trim().is_empty() {
                error!("name is empty");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            if request.code.trim().is_empty() {
                error!("code is empty");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            if request.desc.trim().is_empty() {
                error!("desc is empty");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            resolve_code_error(response, || {
                system_role_service::update_system_role(request)
            })
            .await;
        }
    }
}

#[endpoint(tags("角色管理"))]
pub async fn delete_system_role(request: &mut Request, response: &mut Response) {
    match request.parse_body::<DeleteSystemRoleRequest>().await {
        Err(err) => {
            error!("parse param err: {:?}", err);
            failed(response, ERR_REQUEST_PARAM_INVALID);
        }
        Ok(request) => {
            if request.id == 0 {
                error!("id is 0");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            resolve_code_error(response, || {
                system_role_service::delete_system_role(request)
            })
            .await;
        }
    }
}
