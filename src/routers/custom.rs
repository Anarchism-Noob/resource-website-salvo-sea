use crate::controller::custom_controller::{
    get_captcha,
    post_login,
    post_register,
    put_change_password, //get_register_captcha
};
use crate::middleware::{cors::cors_middleware, jwt::jwt_middleware};
use salvo::{
    prelude::{CatchPanic, Logger, OpenApi, SwaggerUi},
    Router,
};

pub fn api() -> Router {
    let mut no_auth_router = vec![
        Router::with_path("/custom/login").post(post_login),
        Router::with_path("/custom/register").post(post_register),
    ];

    let _cors_handler = cors_middleware();

    let mut need_auth_routers = vec![Router::with_path("/custom/changePwd")
        .push(Router::with_path("<uuid>").put(put_change_password))];

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
