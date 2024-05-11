use crate::{
    controller::{
        carousel_controller::get_carousel,
        custom_controller::{
            get_captcha, get_orders, get_user_profile, post_login, post_register, put_buy_resource,
            put_change_password, put_change_profile, put_upload_avatar,
        },
        sys_resources_controller::{
            get_resource_detail_by_uuid, get_resource_list, get_resource_list_of_language,
            get_resources_of_category, get_resources_of_category_and_language,
        },
        website_controller::{get_website_profile},
    },
    middleware::{cors::cors_middleware, jwt_auth::{jwt_auth_middleware}},
};
use salvo::prelude::{CatchPanic, Logger, OpenApi, Router, SwaggerUi};

pub fn api() -> Router {
    let mut no_auth_router = vec![
        Router::with_path("/captcha").get(get_captcha),
        Router::with_path("/website").get(get_website_profile),
        // 用户登陆
        Router::with_path("/login").post(post_login),
        // 用户注册
        Router::with_path("/register").post(post_register),
        // 首页
        Router::with_path("/index")
        .get(get_resource_list)
        .get(get_carousel)
        .push(Router::with_path("/<language>").get(get_resource_list_of_language))
        .push(Router::with_path("/<category>").get(get_resources_of_category))
        .push(Router::with_path("/<language>/<category>").get(get_resources_of_category_and_language))
        .push(Router::with_path("/<uuid>").get(get_resource_detail_by_uuid)),
    ];

    let _cors_handler = cors_middleware();

    let mut need_auth_routers = vec![
        Router::with_path("/orders/<uuid>").get(get_orders),
        Router::with_path("/resource/<uuid>").put(put_buy_resource),
        Router::with_path("/Custom/<uuid>")
        .get(get_user_profile)
        .put(put_change_password)
        .put(put_change_profile)
        .put(put_upload_avatar),
        
    ];

    let router = Router::new()
        .hoop(_cors_handler)
        .hoop(Logger::new())
        .hoop(CatchPanic::new())
        .get(get_captcha)
        .append(&mut no_auth_router)
        .push(
            Router::new()
                .append(&mut need_auth_routers)
                .hoop(jwt_auth_middleware),
        );
    let doc = OpenApi::new("Resource WebSite API", "0.1.1").merge_router(&router);
    router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"))
}
