use crate::{
    controller::{
        carousel_controller::{
            delete_carousel, get_carousel, post_upload_carousel, put_create_carousel,
        },
        resource_category_controller::{create_category, delete_category, get_category_list},
        resource_language_controller::{delete_language, get_dev_languages, post_create_language},
        sys_resources_controller::{
            delete_image, post_create_resource, put_change_link, put_upload_description,
            put_upload_image,
        },
        system_controller::{
            disable_admin, disable_custom, enable_admin, enable_custom, get_admin_list,
            get_captcha, get_custom_list, get_history_data, get_token_profile, get_withdraw_list,
            get_withdraw_list_unprocessed, post_login, post_register_admin, put_change_password,
            put_change_profile, put_recharge, put_upload_avatar, put_withdraw,
            put_withdraw_process,
        },
        website_controller::{
            get_admin_bg, get_custom_bg, get_website_logo, get_website_profile,
            update_website_profile, upload_admin_bg, upload_custom_bg, upload_website_logo,
        },
    },
    middleware::{
        cors::cors_middleware,
        jwt,
        jwt_auth::{self, jwt_auth_middleware},
    },
};
use salvo::prelude::{CatchPanic, Logger, OpenApi, Router, SwaggerUi};

use super::router;

pub fn api() -> Router {
    let mut no_auth_router = vec![
        // 管理员登陆
        Router::with_path("/login")
            .get(post_login)
            .get(get_captcha)
            .get(get_admin_bg),
        // 获取网站信息
        Router::with_path("/website")
            .get(get_website_logo)
            .get(get_website_profile)
            .push(Router::with_path("/carousel").get(get_carousel)),
        Router::with_path("/custom/bg").get(get_custom_bg),
        // 获取资源的分类和开发语言
        Router::with_path("/resource")
            .get(get_dev_languages)
            .get(get_category_list),
    ];

    let _cors_handler = cors_middleware();

    let mut need_auth_router = vec![
        Router::with_path("/create").post(post_register_admin),
        //账号侧管理
        Router::with_path("/manager")
            .get(get_history_data)
            // 管理员账号管理
            .push(
                Router::with_path("/admin").get(get_admin_list).push(
                    Router::with_path("/<uuid>")
                        .put(disable_admin)
                        .put(enable_admin),
                ),
            )
            // 处理取款申请
            .push(
                Router::with_path("/unprocessed")
                    .get(get_withdraw_list_unprocessed)
                    .push(Router::with_path("/<uuid>").put(put_withdraw_process)),
            )
            // 用户账号管理
            .push(
                Router::with_path("/custom").get(get_custom_list).push(
                    Router::with_path("/<uuid>")
                        .put(disable_custom)
                        .put(put_recharge)
                        .put(enable_custom),
                ),
            ),
        // 当前登陆用户管理
        Router::with_path("/admin/<uuid>")
            .get(get_token_profile)
            .get(get_withdraw_list)
            .put(put_withdraw)
            .put(put_change_password)
            .put(put_change_profile)
            .put(put_upload_avatar),
        // 资源管理
        Router::with_path("/resource")
            .post(post_create_resource)
            .push(Router::with_path("/upload/image").post(put_upload_image))
            .push(Router::with_path("/upload/des").post(put_upload_description))
            .push(Router::with_path("/delete/image/<uuid>").delete(delete_image)),
        Router::with_path("/resource/<uuid>").put(put_change_link),
        // 语言管理
        Router::with_path("/language")
            .post(post_create_language)
            .push(Router::with_path("/<id>").delete(delete_category)),
        // 分类管理
        Router::with_path("/category")
            .post(create_category)
            .push(Router::with_path("/<id>").delete(delete_category)),
        // 轮播图
        Router::with_path("/carousel/create")
            .post(post_upload_carousel)
            .post(put_create_carousel),
        Router::with_path("/carousel/<id>").delete(delete_carousel),
        // 网站信息
        Router::with_path("/website")
            .put(update_website_profile)
            .post(upload_website_logo)
            .post(upload_admin_bg)
            .post(upload_custom_bg),
    ];
    let router = Router::new()
        .hoop(_cors_handler)
        .hoop(Logger::new())
        .hoop(CatchPanic::new())
        .append(&mut no_auth_router)
        .push(
            Router::new()
                .hoop(jwt_auth_middleware)
                .append(&mut need_auth_router),
        );
    let doc = OpenApi::new("Resource Management API", "0.1.1").merge_router(&router);
    router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"))
}
