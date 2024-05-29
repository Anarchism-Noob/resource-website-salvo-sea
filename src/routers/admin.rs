use crate::controller::{
        carousel_controller::{
            delete_carousel, all_carousel, upload_carousel, create_carousel,
        },
        resource_category_controller::{create_category, delete_category, all_categories},
        resource_language_controller::{delete_language, all_languages, create_language},
        sys_resources_controller::{
            delete_des_file, delete_image, query_resource, create_resource,
            put_change_link, upload_des_file, put_upload_image,
        },
        system_controller::{
            disable_admin, disable_custom, enable_admin, enable_custom, get_admin_list,
            get_custom_list, get_history_data, get_token_profile, all_withdraw,
            all_unprocessed, post_login, post_register_admin, pchange_pwd,
            change_profile, put_recharge, upload_avatar, put_withdraw,
            put_process,
        },
        website_controller::{
            get_admin_bg, get_website_profile, update_website_profile,
            upload_admin_bg, upload_custom_bg, upload_website_logo,
        },
    };
use salvo::prelude::Router;

pub fn auth_system_api() -> Vec<Router> {
    let router= vec![
        // 获取当前登录用户的详细信息
        Router::with_path("/token_user/profile").get(get_token_profile),
        Router::with_path("/token_user/change_pwd").post(pchange_pwd),
        Router::with_path("/token_user/change_info").post(change_profile),
        Router::with_path("/token_user/withdrawl").post(put_withdraw),
        Router::with_path("/token_user/all_withdrawl").get(all_withdraw),
        // 创建一个管理员账号
        Router::with_path("/manager/admin/create").post(post_register_admin),
        Router::with_path("/manager/admin/get_history").get(get_history_data),
        Router::with_path("/manager/admin/all_admin").get(get_admin_list),
        Router::with_path("/manager/admin/disable_admin").post(disable_admin),
        Router::with_path("/manager/admin/enable_admin").post(enable_admin),
        //Router::with_path("//manager/admin/delete_admin").delete(del_admin),
        // 用户管理
        Router::with_path("/manager/custom/all_custom").get(get_custom_list),
        Router::with_path("/manager/custom/disable_custom").post(disable_custom),
        Router::with_path("/manager/custom/enable_custom").post(enable_custom),
        Router::with_path("/manager/custom/recharge").post(put_recharge),
        // 提款管理
        Router::with_path("/manager/unprocessed").get(all_unprocessed),
        Router::with_path("/manager/processed").post(put_process),
        // 资源管理
        Router::with_path("/manager/resource/query_resource").get(query_resource),
        Router::with_path("/manager/resource/remove/image").delete(delete_image),
        Router::with_path("/manager/resource/remove/des_file").delete(delete_des_file),
        Router::with_path("/manager/resource/create").post(create_resource),
        Router::with_path("/manager/resource/change_link").post(put_change_link),
        Router::with_path("/manager/resource/language/all_language").get(all_languages),
        Router::with_path("/manager/resource/language/create").post(create_language),
        Router::with_path("/manager/resource/language/remove").delete(delete_language),
        Router::with_path("/manager/resource/category/all_category").get(all_categories),
        Router::with_path("/manager/resource/category/create").post(create_category),
        Router::with_path("/manager/resource/category/remove").delete(delete_category),
        // 网站管理
        Router::with_path("/manager/website/update").post(update_website_profile),
        // 轮播图管理
        Router::with_path("/manager/carousel/all_carousel").get(all_carousel),
        Router::with_path("/manager/carousel/create").post(create_carousel),
        Router::with_path("/manager/carousel/remove").delete(delete_carousel),
        //Router::with_path("/manager/carousel/detail").get(detail_carousel)
        // 上传统一管理
        Router::with_path("/upload/resource_image").post(put_upload_image),
        Router::with_path("/upload/carousel_image").post(upload_carousel),
        Router::with_path("/upload/resource_des_file").post(upload_des_file),
        Router::with_path("/upload/website_logo").post(upload_website_logo),
        Router::with_path("/upload/admin_bg").post(upload_admin_bg),
        Router::with_path("/upload/custom_bg").post(upload_custom_bg),
        Router::with_path("/upload/token_user_avatar").post(upload_avatar),
    ];
    router
}

pub fn no_auth_system_api() -> Vec<Router> {
    let router = vec![
        Router::with_path("/comm/login").post(post_login),
        Router::with_path("/comm/get_bg").get(get_admin_bg),
        Router::with_path("/website").get(get_website_profile)
    ];
    router
}

