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
        website_controller::{get_custom_bg, get_website_logo, get_website_profile},
    },
    middleware::{cors::cors_middleware, jwt, jwt_auth},
};
use salvo::prelude::{CatchPanic, Logger, OpenApi, Router, SwaggerUi};

pub fn api() -> Router {
    let mut no_auth_router = vec![
        Router::with_path("/login").post(post_login),
        Router::with_path("/register").post(post_register),
        Router::with_path("/captcha").get(get_captcha),
        Router::with_path("/index").get(get_resource_list),
        Router::with_path("/resource/<language>").get(get_resource_list_of_language),
        Router::with_path("/resource/<category>").get(get_resources_of_category),
        Router::with_path("/resource/<language>/<category>")
            .get(get_resources_of_category_and_language),
        Router::with_path("/resource/<uuid>").get(get_resource_detail_by_uuid),
        Router::with_path("/carousel").get(get_carousel),
        Router::with_path("/website/profile").get(get_website_profile),
        Router::with_path("/logo").get(get_website_logo),
        Router::with_path("/login/bg").get(get_custom_bg),
    ];

    let _cors_handler = cors_middleware();

    let mut need_auth_routers = vec![
        Router::with_path("/orders/<uuid>").get(get_orders),
        Router::with_path("/resource/<uuid>").put(put_buy_resource),
        Router::with_path("/profile/<uuid>")
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
                .hoop(jwt::jwt_middleware()),
        );
    let doc = OpenApi::new("Resource WebSite API", "0.1.1").merge_router(&router);
    router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"))
}
