use crate::{
    controller::{
        carousel_controller::{
            delete_carousel, get_carousel, post_upload_carousel, put_create_carousel,
        },
        resource_category_controller::{create_category, delete_category, get_category_list},
        resource_language_controller::{delete_language, get_dev_languages, post_create_language},
        sys_resources_controller::{
            delete_image, get_resource_list, post_create_resource, put_change_link,
            put_upload_description, put_upload_image,
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
    middleware::{cors::cors_middleware, jwt_auth::jwt_auth_middleware},
};
use salvo::prelude::{CatchPanic, Logger, OpenApi, Router, SwaggerUi};

pub fn api() -> Router {
    
    let _cors_handler = cors_middleware();
    
    let mut no_auth_router = vec![
        Router::with_path("comm")
        .push(
            Router::with_path("get_captcha").get(get_captcha),
        )
        .push(
            Router::with_path("get_admin_bg").get(get_admin_bg),
        )
        .push(
            Router::with_path("login").get(post_login),
        ),
        Router::with_path("system")
        .push(
            Router::with_path("manager")
            .push(
                Router::with_path("website")
                    .push(
                        Router::with_path("get_profile").get(get_website_profile)
                    ),
            ),
        ),
    ];

    let mut need_auth_router = vec![Router::with_path("system")
        .push(
            Router::with_path("manager")
                .push(
                    // 管理员账号管理
                    Router::with_path("admin")
                        .push(Router::with_path("get_history_data").get(get_history_data))
                        .push(Router::with_path("create").post(post_register_admin))
                        .push(Router::with_path("get_admin_list").get(get_admin_list))
                        .push(Router::with_path("disable_admin").put(disable_admin))
                        .push(Router::with_path("enable_admin").put(enable_admin)),
                )
                .push(
                    // 用户账号管理
                    Router::with_path("custom")
                        .push(Router::with_path("disable_custom").put(disable_custom))
                        .push(Router::with_path("enable_custom").put(enable_custom))
                        .push(Router::with_path("recharge").put(put_recharge)),
                )
                .push(
                    // 处理取款申请
                    Router::with_path("unprocessed")
                        .push(
                            Router::with_path("get_unprocessed").get(get_withdraw_list_unprocessed),
                        )
                        .push(Router::with_path("process").put(put_withdraw_process)),
                )
                .push(
                    // 资源管理
                    Router::with_path("resource")
                        .push(Router::with_path("create").post(post_create_resource))
                        .push(Router::with_path("upload/image").post(put_upload_image))
                        .push(Router::with_path("upload/des").post(put_upload_description))
                        .push(Router::with_path("image/<uuid>").delete(delete_image))
                        .push(Router::with_path("<uuid>").put(put_change_link))
                        .push(Router::with_path("get_all").get(get_resource_list))
                        .push(
                            // 语言管理
                            Router::with_path("language")
                                .push(Router::with_path("create").post(post_create_language))
                                .push(Router::with_path("<id>").delete(delete_category))
                                .push(Router::with_path("get_languages").get(get_dev_languages)),
                        )
                        .push(
                            // 分类管理
                            Router::with_path("category")
                                .push(Router::with_path("create").post(create_category))
                                .push(Router::with_path("<id>").delete(delete_category))
                                .push(Router::with_path("get_categories").get(get_category_list)),
                        ),
                )
                .push(
                    // 网站信息
                    Router::with_path("website")
                        .push(Router::with_path("update").put(update_website_profile))
                        .push(Router::with_path("upload_logo").put(upload_website_logo))
                        .push(Router::with_path("upload_admin_bg").post(upload_admin_bg))
                        .push(Router::with_path("upload_custom_bg").post(upload_custom_bg))
                        .push(
                            // 轮播图
                            Router::with_path("carousel").push(
                                Router::with_path("create")
                                    .post(post_upload_carousel)
                                    .put(put_create_carousel),
                            ),
                        ),
                ),
        )
        .push(
            // 用户账号管理
            Router::with_path("token_admin")
                .push(Router::with_path("profile").get(get_token_profile))
                .push(Router::with_path("change_password").put(put_change_password))
                .push(Router::with_path("change_profile").put(put_change_profile))
                .push(Router::with_path("upload_avatar").put(put_upload_avatar))
                .push(
                    Router::with_path("withdrwal")
                        .get(get_withdraw_list)
                        .put(put_withdraw),
                ),
        )];
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
