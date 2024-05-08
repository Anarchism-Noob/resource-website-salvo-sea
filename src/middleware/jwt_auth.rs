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
            ctrl.skip_rest();
            return;
        }
    };

    let token = match depot.get::<&str>("jwt-token") {
        Ok(token) => token.to_string(),
        Err(_) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.body("JWT token not found in Depot".into());
            ctrl.skip_rest();
            return;
        }
    };

    let jwt_claims = match jwt::parse_token(&token) {
        Ok(claims) => claims,
        Err(err) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.body(err.to_string().into());
            ctrl.skip_rest();
            return;
        }
    };

    let uri: Uri = match req.uri().to_string().parse() {
        Ok(uri) => uri,
        Err(err) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.body(err.to_string().into());
            ctrl.skip_rest();
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
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.body(err.to_string().into());
                    ctrl.skip_rest();
                    return;
                }
            };
            if !is_admin_route(&uri.path()) {
                // 如果访问的路径不在 admin 路由中，则拒绝访问
                res.status_code(StatusCode::UNAUTHORIZED);
                res.body("越权访问或路径不存在".to_string().into());
                ctrl.skip_rest();
            }
            // if uri.path() == "/listCustom" {
            //     if 
            // }
        }
        None => {
            // 查看custom表中是否有这个用户
            let check_result = match check_user_custom(&jwt_claims.user_id).await {
                    Ok(custom_res) => 
                        // 如果用户存在，则返回结构体
                         custom_res,
                    
                    Err(err) => {
                        // 如果用户不存在，则进行错误处理
                        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                        res.body(err.to_string().into());
                        ctrl.skip_rest();
                        return;
                    }
            };
            if !is_custom_route(&uri.path()) {
                // 如果访问的路径不在 custom 路由中，则拒绝访问
                res.status_code(StatusCode::UNAUTHORIZED);
                res.body("越权访问或路径不存在".to_string().into());
                ctrl.skip_rest();
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