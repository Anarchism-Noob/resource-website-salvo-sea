use salvo::{prelude::*, Response};
use tracing::error;

use crate::{
    cerror::{ERR_REQUEST_PARAM_ERROR, ERR_REQUEST_PARAM_INVALID},
    common::{failed, resolve_code_error},
    constant::{DEFAULT_PAGE_INDEX, DEFAULT_PAGE_SIZE},
    dtos::system_user_dto::{
        CreateSystemUserRequest, DeleteSystemUserRequest, GetSystemUserRequest,
        ListSystemUserRequest, PageSystemUserRequest, UpdateSystemUserRequest,
    },
    services::system_user_service,
};

#[endpoint(tags("用户管理"))]
pub async fn list_system_user(request: &mut Request, response: &mut Response) {
    match request.parse_queries::<ListSystemUserRequest>() {
        Err(err) => {
            error!("parse param err: {:?}", err);
            failed(response, ERR_REQUEST_PARAM_INVALID);
        }
        Ok(request) => {
            resolve_code_error(response, || system_user_service::list_system_user(request)).await;
        }
    }
}

#[endpoint(tags("用户管理"))]
pub async fn page_system_user(request: &mut Request, response: &mut Response) {
    match request.parse_queries::<PageSystemUserRequest>() {
        Err(err) => {
            error!("parse param err: {:?}", err);
            failed(response, ERR_REQUEST_PARAM_INVALID);
        }
        Ok(mut request) => {
            if !request.page_index.is_positive() {
                request.page_index = DEFAULT_PAGE_INDEX;
            }

            if !request.page_size.is_positive() {
                request.page_size = DEFAULT_PAGE_SIZE;
            }

            resolve_code_error(response, || system_user_service::page_system_user(request)).await;
        }
    }
}

#[endpoint(tags("用户管理"))]
pub async fn get_system_user(request: &mut Request, response: &mut Response) {
    match request.parse_queries::<GetSystemUserRequest>() {
        Err(err) => {
            error!("parse param err: {:?}", err);
            failed(response, ERR_REQUEST_PARAM_INVALID);
        }
        Ok(request) => {
            if request.id.trim().is_empty() {
                error!("id is empty");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            resolve_code_error(response, || system_user_service::get_system_user(request)).await;
        }
    }
}

#[endpoint(tags("用户管理"))]
pub async fn create_system_user(request: &mut Request, response: &mut Response) {
    match request.parse_body::<CreateSystemUserRequest>().await {
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

            if request.nick_name.trim().is_empty() {
                error!("nick_name is empty");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            if request.email.trim().is_empty() {
                error!("email is empty");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            if request.avatar_url.trim().is_empty() {
                error!("avatar_url is empty");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            resolve_code_error(response, || {
                system_user_service::create_system_user(request)
            })
            .await;
        }
    }
}

#[endpoint(tags("用户管理"))]
pub async fn update_system_user(request: &mut Request, response: &mut Response) {
    match request.parse_body::<UpdateSystemUserRequest>().await {
        Err(err) => {
            error!("parse param err: {:?}", err);
            failed(response, ERR_REQUEST_PARAM_INVALID);
        }
        Ok(request) => {
            if request.id.trim().is_empty() {
                error!("id is empty");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            if request.name.trim().is_empty() {
                error!("name is empty");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            if request.nick_name.trim().is_empty() {
                error!("nick_name is empty");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            if request.email.trim().is_empty() {
                error!("email is empty");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            if request.avatar_url.trim().is_empty() {
                error!("avatar_url is empty");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            resolve_code_error(response, || {
                system_user_service::update_system_user(request)
            })
            .await;
        }
    }
}

#[endpoint(tags("用户管理"))]
pub async fn delete_system_user(request: &mut Request, response: &mut Response) {
    match request.parse_body::<DeleteSystemUserRequest>().await {
        Err(err) => {
            error!("parse param err: {:?}", err);
            failed(response, ERR_REQUEST_PARAM_INVALID);
        }
        Ok(request) => {
            if request.id.trim().is_empty() {
                error!("id is empty");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            resolve_code_error(response, || {
                system_user_service::delete_system_user(request)
            })
            .await;
        }
    }
}
