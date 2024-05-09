use crate::{
    config::CFG,
    middleware::jwt,
    utils::{
        app_error::AppError,
        check_user::{check_user_admin, check_user_custom},
    },
};
use anyhow::Result;
use jsonwebtoken::{decode, Algorithm, DecodingKey, EncodingKey, Validation};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use salvo::{
    hyper::Uri,
    jwt_auth::{ConstDecoder, CookieFinder, HeaderFinder, QueryFinder},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::{Duration, OffsetDateTime};

#[handler]
pub async fn jwt_auth_middleware(
    req: &mut Request,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
    depot: &mut Depot,
) {
    println!("jwt_auth_middleware--> load");

    let item = match req.parse_headers::<HashMap<String, String>>() {
        Ok(item) => item,
        Err(err) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.body(err.to_string().into());
            return;
        }
    };

    let token = match depot.get::<&str>("jwt-token") {
        Ok(token) => token.to_string(),
        Err(_) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.body("JWT token not found in Depot".into());
            return;
        }
    };

    let jwt_claims = match jwt::parse_token(&token) {
        Ok(claims) => claims,
        Err(err) => {
             handle_internal_server_error(res, StatusCode::INTERNAL_SERVER_ERROR, err.to_string());
            return;
        }
    };

    let uri: Uri = match req.uri().to_string().parse() {
        Ok(uri) => uri,
        Err(err) => {
             handle_internal_server_error(res, StatusCode::INTERNAL_SERVER_ERROR, err.to_string());
            return;
        }
    };

    //判断token是cusotm还是admin
    match jwt_claims.role {
        Some(role) => {
            // 查看admin表中是否有这个用户
            let check_result = match check_user_admin(&jwt_claims.user_id).await {
                Ok(admin_res) => admin_res,
                Err(err) => {
                    handle_unauthorized_access(res, ctrl);
                    return;
                }
            };
            if !is_admin_route(&uri.path()) {
                // 如果访问的路径不在 admin 路由中，则拒绝访问
                handle_unauthorized_access(res, ctrl);
                return;
            }
            if is_super_admin_route(&uri.path()) {
                // 如果访问的路径是超级管理员路由，则检查用户是否是超级管理员
                if check_result.role != 0 {
                    handle_unauthorized_access(res, ctrl);
                    return;
                }
            }
        }
        None => {
            // 查看custom表中是否有这个用户
            let check_result = match check_user_custom(&jwt_claims.user_id).await {
                // 如果用户存在，则返回结构体
                Ok(custom_res) => custom_res,
                Err(err) => {
                    // 如果用户不存在，则进行错误处理
                    handle_unauthorized_access(res, ctrl);
                    return;
                }
            };
            if !is_custom_route(&uri.path()) {
                // 如果访问的路径不在 custom 路由中，则拒绝访问
                handle_unauthorized_access(res, ctrl);
                return;
            }
        }
    }
}

fn is_custom_route(uri_path: &str) -> bool {
    uri_path.starts_with("/custom/api/")
}

fn is_admin_route(uri_path: &str) -> bool {
    uri_path.starts_with("/admin/api/")
}

fn is_super_admin_route(uri_path: &str) -> bool {
    uri_path.starts_with("/admin/api/manager/")
}

fn handle_internal_server_error(res: &mut Response, status_code: StatusCode, message: String) {
    res.status_code(status_code);
    res.body(message.into());
}

fn handle_unauthorized_access(res: &mut Response, ctrl: &mut FlowCtrl) {
    res.status_code(StatusCode::UNAUTHORIZED);
    res.body("越权访问或路径不存在".to_string().into());
    ctrl.skip_rest();
}