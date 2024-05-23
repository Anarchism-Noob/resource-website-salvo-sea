use crate::{
    controller::{
        carousel_controller::get_carousel,
        custom_controller::{
            get_captcha, get_orders, get_user_profile, post_login, post_register, put_buy_resource,
            put_change_password, put_change_profile, put_upload_avatar,
        },
        sys_resources_controller::{get_resource_detail_by_uuid, get_resource_list},
        website_controller::{get_custom_bg, get_website_profile},
    },
    middleware::{cors::cors_middleware, jwt_auth::jwt_auth_middleware},
};
use salvo::prelude::{CatchPanic, Logger, OpenApi, Router, SwaggerUi};

pub fn custom_api() -> Router {
    let mut no_auth_router = vec![
        Router::with_path("comm")
            // 用户登陆
            .push(
                Router::with_path("login")
                    .get(get_captcha)
                    .push(Router::with_path("get_login_bg").get(get_custom_bg))
                    .push(Router::with_path("loading").post(post_login)),
            )
            // 获取网站信息
            .push(Router::with_path("get_website").get(get_website_profile))
            .push(
                // 用户注册
                Router::with_path("register")
                    .get(get_captcha)
                    .push(Router::with_path("create").post(post_register)),
            ),
        // 首页
        Router::with_path("index")
            .push(
                Router::with_path("resources")
                    .get(get_resource_list)
                    .push(Router::with_path("<uuid>").get(get_resource_detail_by_uuid)),
            )
            .push(Router::with_path("carousel").get(get_carousel)),
    ];

    let _cors_handler = cors_middleware();

    let mut need_auth_routers = vec![Router::with_path("user")
        .push(
            Router::with_path("profile")
                .push(Router::with_path("view").get(get_user_profile))
                .push(Router::with_path("change_pwd").put(put_change_password))
                .push(Router::with_path("change_profile").put(put_change_profile))
                .push(Router::with_path("orders").get(get_orders))
                .push(Router::with_path("avatar").put(put_upload_avatar)),
        )
        .push(Router::with_path("resource/<uuid>").put(put_buy_resource))];

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
    router
}
