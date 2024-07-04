use salvo::{prelude::*, Response};
use tracing::error;

use crate::{
    cerror::{ERR_REQUEST_PARAM_ERROR, ERR_REQUEST_PARAM_INVALID},
    common::{failed, resolve_code_error},
    constant::DEFAULT_PAGE_SIZE,
    dtos::casbin_resource_dto::{
        CreateCasbinResourceRequest, DeleteCasbinResourceRequest, GetCasbinResourceRequest,
        ListCasbinResourceRequest, PageCasbinResourceRequest, UpdateCasbinResourceRequest,
    },
    services::casbin_resource_service,
};

#[endpoint(tags("权限资源管理"))]
pub async fn list_casbin_resource(request: &mut Request, response: &mut Response) {
    match request.parse_queries::<ListCasbinResourceRequest>() {
        Err(err) => {
            error!("parse param err: {:?}", err);
            failed(response, ERR_REQUEST_PARAM_INVALID);
        }
        Ok(request) => {
            resolve_code_error(response, || {
                casbin_resource_service::list_casbin_resource(request)
            })
            .await;
        }
    }
}

#[endpoint(tags("权限资源管理"))]
pub async fn page_casbin_resource(request: &mut Request, response: &mut Response) {
    match request.parse_queries::<PageCasbinResourceRequest>() {
        Err(err) => {
            error!("parse param err: {:?}", err);
            failed(response, ERR_REQUEST_PARAM_INVALID);
        }
        Ok(mut request) => {
            if request.page_size == 0 {
                request.page_size = DEFAULT_PAGE_SIZE;
            }

            resolve_code_error(response, || {
                casbin_resource_service::page_casbin_resource(request)
            })
            .await;
        }
    }
}

#[endpoint(tags("权限资源管理"))]
pub async fn get_casbin_resource(request: &mut Request, response: &mut Response) {
    match request.parse_queries::<GetCasbinResourceRequest>() {
        Err(err) => {
            error!("parse param err: {:?}", err);
            failed(response, ERR_REQUEST_PARAM_INVALID);
        }
        Ok(request) => {
            if request.id==0 {
                error!("id is 0");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            resolve_code_error(response, || {
                casbin_resource_service::get_casbin_resource(request)
            })
            .await;
        }
    }
}

#[endpoint(tags("权限资源管理"))]
pub async fn create_casbin_resource(request: &mut Request, response: &mut Response) {
    match request.parse_body::<CreateCasbinResourceRequest>().await {
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

            if request.resource_type.trim().is_empty() {
                error!("resource_type is empty");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            if request.display_name.trim().is_empty() {
                error!("display_name is empty");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            if request.r#type.trim().is_empty() {
                error!("type is empty");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            if request.resource_id==0 {
                error!("resource_id is 0");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            resolve_code_error(response, || {
                casbin_resource_service::create_casbin_resource(request)
            })
            .await;
        }
    }
}

#[endpoint(tags("权限资源管理"))]
pub async fn update_casbin_resource(request: &mut Request, response: &mut Response) {
    match request.parse_body::<UpdateCasbinResourceRequest>().await {
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

            if request.resource_type.trim().is_empty() {
                error!("resource_type is empty");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            if request.display_name.trim().is_empty() {
                error!("display_name is empty");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            if request.r#type.trim().is_empty() {
                error!("type is empty");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            if request.resource_id == 0 {
                error!("resource_id is 0");
                failed(response, ERR_REQUEST_PARAM_ERROR);
                return;
            }

            resolve_code_error(response, || {
                casbin_resource_service::update_casbin_resource(request)
            })
            .await;
        }
    }
}

#[endpoint(tags("权限资源管理"))]
pub async fn delete_casbin_resource(request: &mut Request, response: &mut Response) {
    match request.parse_body::<DeleteCasbinResourceRequest>().await {
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
                casbin_resource_service::delete_casbin_resource(request)
            })
            .await;
        }
    }
}
