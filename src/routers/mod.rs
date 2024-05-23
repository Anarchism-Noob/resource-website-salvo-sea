// mod admin;
mod admin;
mod custom;

use salvo::oapi::OpenApi;
use salvo::prelude::{Router, SwaggerUi};
use crate::routers::admin::system_api;
use crate::routers::custom::custom_api;

pub fn get_client_router() -> Vec<Router>{
    vec![custom_api()]
}

pub fn get_system_router() -> Vec<Router>{
    vec![system_api()]
}

pub fn router() -> Router {
    // 创建新的Router实例
    let mut router = Router::new();
    let mut client_router = Router::new();
    let mut system_router = Router::new();

    //使用extend将子路由添加到主路由
    for sub_router in get_client_router() {
        router = router.push(Router::with_path("/custom/api").push(sub_router));
    }

    for sub_router in get_system_router() {
        router = router.push(Router::with_path("/system/api").push(sub_router));
    }

    for sub_router in get_client_router() {
        client_router = client_router.push(Router::with_path("/custom/api").push(sub_router));
    }

    for sub_router in get_system_router() {
        system_router = system_router.push(Router::with_path("/system/api").push(sub_router));
    }

    let custom_doc = OpenApi::new("RSWS Client API", "0.1.1").merge_router(&client_router);

    router = router
        .unshift(custom_doc.into_router("/openapi.json"))
        .unshift(
            SwaggerUi::new("/openapi.json")
                .title("RSWS Client API")
                .into_router("swagger-ui")
        );

    let system_doc = OpenApi::new("RSWS System API", "0.1.1").merge_router(&system_router);

    router = router
        .unshift(system_doc.into_router("/openapi.json"))
        .unshift(
            SwaggerUi::new("/openapi.json")
                .title("RSWS System API")
                .into_router("swagger-ui")
        );
    router
}
