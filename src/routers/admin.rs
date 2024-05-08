use crate::{
    controller::{
        system_controller::{
            post_register_admin, post_login, get_captcha, put_change_password,put_upload_avatar, 
            get_user_profile, put_change_profile, get_custom_list, get_admin_list, disable_custom,
            disable_admin, 
            // enable_custom, enable_admin, get_custom_detail, get_admin_detail, 
            put_recharge, put_withdraw, get_withdraw_list, get_withdraw_list_unprocessed,
            put_withdraw_process,
        },
        sys_resources_controller::{
            put_upload_image,post_create_resource, put_change_link
        },
        resource_category_controller::{
            get_category_list, create_category, delete_category
        },
        resource_language_controller::{
            get_dev_languages, post_create_language, delete_language
        },
        carousel_controller::{
            get_carousel, put_create_carousel, post_upload_carousel, delete_carousel
        },
        website_controller::{
            update_website_profile, get_website_profile, upload_website_logo, 
            get_website_logo, upload_admin_bg, get_admin_bg, upload_custom_bg, get_custom_bg, 
        }
    },
    middleware::{
        cors::cors_middleware, jwt, jwt_auth
    },
};
use salvo::prelude::{CatchPanic, Logger, OpenApi, SwaggerUi, Router};

use super::router;

pub fn api() -> Router {
    let mut no_auth_router = vec![
        Router::with_path("/login").get(post_login),
        Router::with_path("/captcha").get(get_captcha),
        Router::with_path("/website/profile").get(get_website_profile),
        Router::with_path("/logo").get(get_website_logo),
        Router::with_path("/login/bg").get(get_admin_bg),
        Router::with_path("/custom/bg").get(get_custom_bg),
        Router::with_path("/dev/languages").get(get_dev_languages),
        Router::with_path("/category").get(get_category_list),
        Router::with_path("/carousel").get(get_carousel),
    ];

    let _cors_handler = cors_middleware();

    let mut need_auth_router = vec![
        Router::with_path("/create/admin").post(post_register_admin),
        Router::with_path("/profile/<uuid>")
        .get(get_user_profile)
        .put(put_change_password)
        .put(put_change_profile)
        .put(put_upload_avatar),
        Router::with_path("/resource/create").post(post_create_resource),
    ];
    router()
}