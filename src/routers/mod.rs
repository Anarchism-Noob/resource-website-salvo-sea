mod admin;
mod custom;

use self::admin::{auth_system_api, no_auth_system_api};
use self::custom::{auth_custom_api, no_auth_custom_api};
use crate::controller::casbin_resource_controller::{
    create_casbin_resource, delete_casbin_resource, get_casbin_resource, list_casbin_resource,
    page_casbin_resource, update_casbin_resource,
};
use crate::controller::system_role_controller::{
    create_system_role, delete_system_role, get_system_role, page_system_role, update_system_role,
};
use crate::controller::system_user_controller::{
    create_system_user, delete_system_user, get_system_user, list_system_user, page_system_user,
    update_system_user,
};
use crate::middleware::jwt::jwt_middleware;
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
                // system user router
                .push(Router::with_path("/system-user/list").get(list_system_user))
                .push(Router::with_path("/system-user/page").get(page_system_user))
                .push(Router::with_path("/system-user/get").get(get_system_user))
                .push(Router::with_path("/system-user/create").post(create_system_user))
                .push(Router::with_path("/system-user/update").put(update_system_user))
                .push(Router::with_path("/system-user/delete").delete(delete_system_user))
                // system role router
                .push(Router::with_path("/system-role/list").get(page_system_user))
                .push(Router::with_path("/system-role/page").get(page_system_role))
                .push(Router::with_path("/system-role/get").get(get_system_role))
                .push(Router::with_path("/system-role/create").post(create_system_role))
                .push(Router::with_path("/system-role/update").put(update_system_role))
                .push(Router::with_path("/system-role/delete").delete(delete_system_role))
                // casbin resource router
                .push(Router::with_path("/casbin-resource/list").get(list_casbin_resource))
                .push(Router::with_path("/casbin-resource/page").get(page_casbin_resource))
                .push(Router::with_path("/casbin-resource/get").get(get_casbin_resource))
                .push(Router::with_path("/casbin-resource/create").post(create_casbin_resource))
                .push(Router::with_path("/casbin-resource/update").put(update_casbin_resource))
                .push(Router::with_path("/casbin-resource/delete").delete(delete_casbin_resource)),
        )
        .push(system_doc.into_router("/system-doc/openapi.json"))
        .push(
            SwaggerUi::new("/system-doc/openapi.json")
                .title("RSWS System API")
                .into_router("/api/system/swagger-ui"),
        );

    router
}
