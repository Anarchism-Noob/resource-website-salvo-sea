mod admin;
mod custom;

use self::admin::{auth_system_api, no_auth_system_api};
use self::custom::{auth_custom_api, no_auth_custom_api};
use crate::middleware::cors::cors_middleware;
use crate::middleware::jwt_auth::jwt_auth_middleware;
use salvo::{
    oapi::OpenApi,
    prelude::{CatchPanic, Logger, Router, SwaggerUi},
};

pub fn router() -> Router {
    // 创建新的Router实例
    let mut router = Router::new();
    let mut no_auth_router_client_temp = Router::new();
    let mut no_auth_router_system_temp = Router::new();
    let mut auth_router_client_temp = Router::new();
    let mut auth_router_system_temp = Router::new();
    let mut client_router = Router::new();
    let mut system_router = Router::new();

    // 管理端添加API路由
    for sub_router in no_auth_system_api() {
        no_auth_router_system_temp =
            no_auth_router_system_temp.push(Router::with_path("/system/api").push(sub_router));
    }
    for sub_router in no_auth_system_api() {
        system_router = system_router.push(Router::with_path("/system/api").push(sub_router));
    }
    for sub_router in auth_system_api() {
        auth_router_system_temp =
            auth_router_system_temp.push(Router::with_path("/system/api").push(sub_router));
    }
    for sub_router in auth_system_api() {
        system_router = system_router.push(Router::with_path("/system/api").push(sub_router));
    }
    // 客户端添加API路由
    for sub_router in no_auth_custom_api() {
        no_auth_router_client_temp =
            no_auth_router_client_temp.push(Router::with_path("/custom/api").push(sub_router));
    }
    for sub_router in no_auth_custom_api() {
        client_router = client_router.push(Router::with_path("/custom/api").push(sub_router));
    }
    for sub_router in auth_custom_api() {
        auth_router_client_temp =
            auth_router_client_temp.push(Router::with_path("/custom/api").push(sub_router));
    }
    for sub_router in auth_custom_api() {
        client_router = client_router.push(Router::with_path("/custom/api").push(sub_router));
    }

    // // 创建k跨域处理中间件
    let _cors_middleware = cors_middleware();

    // 创建并拼接API文档路由
    let custom_doc = OpenApi::new("RSWS Client API", "0.1.1").merge_router(&client_router);
    // println!("Custom API JSON:{:?}", custom_doc);
    let system_doc = OpenApi::new("RSWS System API", "0.1.1").merge_router(&system_router);
    // println!("System API JSON:{:?}", system_doc);

    router = router
        .hoop(Logger::new())
        .hoop(CatchPanic::new())
        .hoop(_cors_middleware)
        .unshift(
            Router::new()
                .unshift(no_auth_router_client_temp)
                .push(
                    Router::new()
                        .hoop(jwt_auth_middleware)
                        .push(auth_router_client_temp),
                )
                .push(custom_doc.into_router("/custom-doc/openapi.json"))
                .push(
                    SwaggerUi::new("/custom-doc/openapi.json")
                        .title("RSWS Client API")
                        .into_router("/custom-api"),
                ),
        )
        .unshift(
            Router::new()
                .unshift(no_auth_router_system_temp)
                .push(
                    Router::new()
                        .hoop(jwt_auth_middleware)
                        .push(auth_router_system_temp),
                )
                .unshift(system_doc.into_router("/system-doc/openapi.json"))
                .unshift(
                    SwaggerUi::new("/system-doc/openapi.json")
                        .title("RSWS System API")
                        .into_router("/system-api"),
                ),
        );

    router
}
