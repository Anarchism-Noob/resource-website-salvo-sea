use crate::controller::{
    custom_controller::{
        get_captcha, get_orders, get_user_profile, post_login, post_register, put_buy_resource,
        put_change_password, put_change_profile, put_upload_avatar,
    },
    sys_resources_controller::{
        get_resource_detail_by_uuid, get_resource_list, get_resource_list_of_language,
        get_resources_of_category, get_resources_of_category_and_language,
    },
};
use crate::middleware::{cors::cors_middleware, jwt::jwt_middleware};
use salvo::{
    prelude::{CatchPanic, Logger, OpenApi, SwaggerUi},
    Router,
};

pub fn api() -> Router {
    let mut no_auth_router = vec![
        Router::with_path("/login").post(post_login),
        Router::with_path("/register").post(post_register),
        Router::with_path("/captcha").get(get_captcha),
        Router::with_path("/resourceList").get(get_resource_list),
        Router::with_path("/resource/language/").get(get_resource_list_of_language),
        Router::with_path("/resource/category/").get(get_resources_of_category),
        Router::with_path("/resource/categoryAndLanguage/")
            .get(get_resources_of_category_and_language),
        Router::with_path("/resource/<uuid>").get(get_resource_detail_by_uuid),
    ];

    let _cors_handler = cors_middleware();

    let mut need_auth_routers = vec![Router::with_path("/home")
        .push(Router::with_path("/orders/<uuid>").get(get_orders))
        .push(Router::with_path("/profile/<uuid>").get(get_user_profile))
        .push(Router::with_path("/byResource/<uuid>").put(put_buy_resource))
        .push(Router::with_path("/upload/<uuid>").put(put_upload_avatar))
        .push(Router::with_path("/changeProfile/<uuid>").put(put_change_profile))
        .push(Router::with_path("/changePwd/<uuid>").put(put_change_password))];

    let router = Router::new()
        .hoop(Logger::new())
        .hoop(CatchPanic::new())
        .get(get_captcha)
        .append(&mut no_auth_router)
        .push(
            Router::new()
                .append(&mut need_auth_routers)
                .hoop(jwt_middleware()),
        );
    let doc = OpenApi::new("Resource WebSite API", "0.1.1").merge_router(&router);
    router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"))
}
