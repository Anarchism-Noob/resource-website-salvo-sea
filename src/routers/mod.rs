mod admin;
mod custom;

use self::admin::{auth_system_api, no_auth_system_api};
use self::custom::{auth_custom_api, no_auth_custom_api};
use crate::controller::system_user_controller::{create_system_user, get_system_user};
use crate::middleware::jwt::jwt_middleware;
use crate::middleware::jwt_auth::jwt_auth_middleware;
use crate::services::admin_user_service::create_admin_user;
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
            no_auth_router_system_temp.push(Router::with_path("/api/system").push(sub_router));
    }
    // for sub_router in no_auth_system_api() {
    //     system_router = system_router.push(Router::with_path("/api/system").push(sub_router));
    // }
    for sub_router in auth_system_api() {
        auth_router_system_temp =
            auth_router_system_temp.push(Router::with_path("/api/system").push(sub_router));
    }
    // 客户端添加API路由
    for sub_router in no_auth_custom_api() {
        no_auth_router_client_temp =
            no_auth_router_client_temp.push(Router::with_path("/api/custom").push(sub_router));
    }
    // for sub_router in no_auth_custom_api() {
    //     client_router = client_router.push(Router::with_path("/api/custom").push(sub_router));
    // }
    for sub_router in auth_custom_api() {
        auth_router_client_temp =
            auth_router_client_temp.push(Router::with_path("/api/custom").push(sub_router));
    }
    // for sub_router in auth_custom_api() {
    //     client_router = client_router.push(Router::with_path("/api/custom").push(sub_router));
    // }

    // 创建并拼接API文档路由

    system_router = system_router.unshift(no_auth_router_system_temp).push(
        Router::new()
            .hoop(jwt_middleware()) //
            .hoop(jwt_auth_middleware) //api验证
            .push(auth_router_system_temp),
    );
    // .unshift(system_doc.into_router("/system-doc/openapi.json"))
    // .unshift(
    //     SwaggerUi::new("/system-doc/openapi.json")
    //             .title("RSWS System API")
    //             .into_router("/api/system/swagger-ui"),
    // );

    let system_doc = OpenApi::new("RSWS System API", "0.1.1").merge_router(&system_router);
    // println!("System API JSON:{:?}", system_doc);

    client_router = client_router.unshift(no_auth_router_client_temp).push(
        Router::new()
            .push(auth_router_client_temp)
            .hoop(jwt_auth_middleware),
    );
    // .unshift(custom_doc.into_router("/custom-doc/openapi.json"))
    // .unshift(
    //     SwaggerUi::new("/custom-doc/openapi.json")
    //             .title("RSWS Client API")
    //             .into_router("/api/custom/swagger-ui"),
    // );

    let custom_doc = OpenApi::new("RSWS Client API", "0.1.1").merge_router(&client_router);
    // println!("Custom API JSON:{:?}", custom_doc);

    router = router
        .hoop(Logger::new())
        .hoop(CatchPanic::new())
        // .hoop(cors_middleware())
        .unshift(client_router)
        .push(custom_doc.into_router("/custom-doc/openapi.json"))
        .push(
            SwaggerUi::new("/custom-doc/openapi.json")
                .title("RSWS Client API")
                .into_router("/api/custom/swagger-ui"),
        )
        .unshift(system_router)
        .unshift(
            Router::new()
                .path("/api/v1")
                .push(Router::with_path("/system-user/get").get(get_system_user)),
        )
        .unshift(
            Router::new()
                .path("/api/v1")
                .push(Router::with_path("/system-user/create").post(create_system_user)),
        )
        .push(system_doc.into_router("/system-doc/openapi.json"))
        .push(
            SwaggerUi::new("/system-doc/openapi.json")
                .title("RSWS System API")
                .into_router("/api/system/swagger-ui"),
        );

    router
}
