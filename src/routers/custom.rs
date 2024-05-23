use crate::controller::{
        carousel_controller::all_carousel,
        custom_controller::{
            get_captcha, get_orders, get_user_profile, post_login, post_register, put_buy_resource,
            put_change_password, put_change_profile, put_upload_avatar,
        },
        sys_resources_controller::{get_resource_detail_by_uuid, query_resource},
        website_controller::{get_custom_bg, get_website_profile},
    };
use salvo::prelude::Router;

pub fn auth_custom_api() -> Vec<Router> {
    let router = vec![
        // 需要认证的路由
        Router::with_path("/user/info").get(get_user_profile),
        Router::with_path("/user/change_pwd").post(put_change_password),
        Router::with_path("/user/change_info").post(put_change_profile),
        Router::with_path("/user/upload_avatar").post(put_upload_avatar),
        Router::with_path("/user/orders").get(get_orders),
        Router::with_path("/resource/buy").post(put_buy_resource),
    ];
    router
}

pub fn no_auth_custom_api() -> Vec<Router> {
    let router = vec![
        // 不需要认证的路由
        Router::with_path("/comm/login").post(post_login),
        Router::with_path("/comm/get_bg").get(get_custom_bg),
        Router::with_path("/comm/get_website").get(get_website_profile),
        Router::with_path("/comm/register").post(post_register),
        Router::with_path("/comm/captcha").get(get_captcha),
        Router::with_path("/index/resources").get(query_resource),
        Router::with_path("/index/carousel").get(all_carousel),
        Router::with_path("/resource/<uuid>").get(get_resource_detail_by_uuid),
    ];
    router
}
