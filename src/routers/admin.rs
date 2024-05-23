use crate::{
    controller::{
        carousel_controller::{
            delete_carousel, get_carousel, upload_carousel, create_carousel,
        },
        resource_category_controller::{create_category, delete_category, all_categories},
        resource_language_controller::{delete_language, all_languages, create_language},
        sys_resources_controller::{
            delete_des_file, delete_image, get_resource_list, create_resource,
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
            get_admin_bg, get_custom_bg, get_website_profile, update_website_profile,
            upload_admin_bg, upload_custom_bg, upload_website_logo,
        },
    },
    middleware::{cors::cors_middleware, jwt_auth::jwt_auth_middleware},
};
use salvo::prelude::{CatchPanic, Logger, OpenApi, Router, SwaggerUi};

pub fn system_api() -> Router {
    let router= Router::new()
        .hoop(Logger::new())
        .hoop(CatchPanic::new())
        .push(
            Router::new()
                // 登录
                .push(Router::with_path("/comm/login").post(post_login))
                // 获取登录页面背景图
                .push(Router::with_path("/comm/get_bg").get(get_admin_bg))
                // 获取网站信息
                .push(Router::with_path("/website").get(get_website_profile))
        )
        .push(
            Router::new()
                .hoop(jwt_auth_middleware)
                // 获取当前登录用户的详细信息
                .push(Router::with_path("/token_user/profile").get(get_token_profile))
                .push(Router::with_path("/token_user/change_pwd").post(pchange_pwd))
                .push(Router::with_path("/token_user/change_info").post(change_profile))
                .push(Router::with_path("/token_user/upload/avatar").post(upload_avatar))
                .push(Router::with_path("/token_user/withdrawl").post(put_withdraw))
                .push(Router::with_path("/token_user/all_withdrawl").get(all_withdraw))
                // 创建一个管理员账号
                .push(Router::with_path("/manager/admin/create").post(post_register_admin))
                .push(Router::with_path("/manager/admin/gey_history").get(get_history_data))
                .push(Router::with_path("/manager/admin/all_admin").get(get_admin_list))
                .push(Router::with_path("/manager/admin/disable_admin").post(disable_admin))
                .push(Router::with_path("/manager/admin/enable_admin").post(enable_admin))
                // .push(Router::with_path("//manager/admin/delete_admin").delete(del_admin))
                .push(Router::with_path("/manager/custom/all_custom").get(get_custom_list))
                .push(Router::with_path("/manager/custom/disable_custom").post(disable_custom))
                .push(Router::with_path("/manager/custom/enable_custom").post(enable_custom))
                .push(Router::with_path("/manager/custom/recharge").post(put_recharge))
                .push(Router::with_path("/manager/unprocessed").get(all_unprocessed))
                .push(Router::with_path("/manager/processed").post(put_process))
                .push(Router::with_path("/manager/resource/upload/image").post(put_upload_image))
                .push(Router::with_path("/manager/resource/upload/des_file").post(upload_des_file))
                .push(Router::with_path("/manager/resource/remove/image").delete(delete_image))
                .push(Router::with_path("/manager/resource/remove/des_file").delete(delete_des_file))
                .push(Router::with_path("/manager/resource/create").post(create_resource))
                .push(Router::with_path("/manager/resource/change_link").post(put_change_link))
                .push(Router::with_path("/manager/resource/language/all_language").get(all_languages))
                .push(Router::with_path("/manager/resource/language/create").post(create_language))
                .push(Router::with_path("/manager/resource/language/remove").delete(delete_language))
                .push(Router::with_path("/manager/resource/category/all_category").get(all_categories))
                .push(Router::with_path("/manager/resource/category/create").post(create_category))
                .push(Router::with_path("/manager/resource/category/remove").delete(delete_category))
                .push(Router::with_path("/manager/website/upload/logo").post(upload_website_logo))
                .push(Router::with_path("/manager/website/upload/admin_bg").post(upload_admin_bg))
                .push(Router::with_path("/manager/website/upload/custom_bg").post(upload_custom_bg))
                .push(Router::with_path("/manager/website/update").post(update_website_profile))
                .push(Router::with_path("/manager/carousel/upload").post(upload_carousel))
                .push(Router::with_path("/manager/carousel/create").post(create_carousel))
                .push(Router::with_path("/manager/carousel/remove").delete(delete_carousel))
             // .push(Router::with_path("/manager/carousel/detail").get(detail_carousel))
        );


    // let mut no_auth_router = vec![
    //     Router::with_path("comm")
    //         .push(Router::with_path("get_admin_bg").get(get_admin_bg))
    //         .push(Router::with_path("login").post(post_login)),
    //     Router::with_path("manager").push(
    //         Router::with_path("website")
    //             .push(Router::with_path("get_profile").get(get_website_profile))
    //             .push(Router::with_path("get_custom_bg").get(get_custom_bg)),
    //     ),
    // ];
    //
    // let mut need_auth_router = vec![
    //     Router::with_path("manager")
    //         .push(
    //             // 管理员账号管理
    //             Router::with_path("admin")
    //                 .push(Router::with_path("get_history_data").get(get_history_data))
    //                 .push(Router::with_path("create").post(post_register_admin))
    //                 .push(Router::with_path("get_admin_list").get(get_admin_list))
    //                 .push(Router::with_path("disable_admin").put(disable_admin))
    //                 .push(Router::with_path("enable_admin").put(enable_admin)),
    //         )
    //         .push(
    //             // 用户账号管理
    //             Router::with_path("custom")
    //                 .push(Router::with_path("get_custom_list").get(get_custom_list))
    //                 .push(Router::with_path("disable_custom").put(disable_custom))
    //                 .push(Router::with_path("enable_custom").put(enable_custom))
    //                 .push(Router::with_path("recharge").put(put_recharge)),
    //         )
    //         .push(
    //             // 处理取款申请
    //             Router::with_path("unprocessed")
    //                 .push(Router::with_path("get_unprocessed").get(get_withdraw_list_unprocessed))
    //                 .push(Router::with_path("process").put(put_withdraw_process)),
    //         )
    //         .push(
    //             // 资源管理
    //             Router::with_path("resource")
    //                 .push(
    //                     Router::with_path("upload")
    //                         .push(Router::with_path("image").post(put_upload_image))
    //                         .push(Router::with_path("des").post(put_upload_description)),
    //                 )
    //                 .push(
    //                     Router::with_path("remove")
    //                         .push(Router::with_path("image").delete(delete_image))
    //                         .push(Router::with_path("des_file").delete(delete_des_file)),
    //                 )
    //                 .push(Router::with_path("create").post(post_create_resource))
    //                 .push(Router::with_path("<uuid>").put(put_change_link))
    //                 .push(Router::with_path("query_resource").get(get_resource_list))
    //                 .push(
    //                     // 语言管理
    //                     Router::with_path("language")
    //                         .push(Router::with_path("create").post(post_create_language))
    //                         .push(Router::with_path("del").delete(delete_language))
    //                         .push(Router::with_path("get_languages").get(get_dev_languages)),
    //                 )
    //                 .push(
    //                     // 分类管理
    //                     Router::with_path("category")
    //                         .push(Router::with_path("create").post(create_category))
    //                         .push(Router::with_path("del").delete(delete_category))
    //                         .push(Router::with_path("get_categories").get(get_category_list)),
    //                 ),
    //         )
    //         .push(
    //             // 网站信息
    //             Router::with_path("website")
    //                 .push(Router::with_path("update").post(update_website_profile))
    //                 .push(
    //                     Router::with_path("upload")
    //                         .push(Router::with_path("logo").post(upload_website_logo))
    //                         .push(Router::with_path("admin_bg").post(upload_admin_bg))
    //                         .push(Router::with_path("custom_bg").post(upload_custom_bg))
    //                         .push(Router::with_path("carousel").post(post_upload_carousel)),
    //                 )
    //                 .push(
    //                     // 轮播图
    //                     Router::with_path("carousel")
    //                         .push(Router::with_path("create").put(put_create_carousel))
    //                         .push(Router::with_path("get_carousels").get(get_carousel))
    //                         .push(Router::with_path("delete").delete(delete_carousel)),
    //                 ),
    //         ),
    //     // 当前账号管理
    //     Router::with_path("token_admin")
    //         .push(Router::with_path("profile").get(get_token_profile))
    //         .push(Router::with_path("change_pwd").put(put_change_password))
    //         .push(Router::with_path("update_profile").put(put_change_profile))
    //         .push(Router::with_path("upload_avatar").put(put_upload_avatar))
    //         .push(
    //             Router::with_path("withdrwal")
    //                 .push(Router::with_path("claimable ").post(put_withdraw))
    //                 .push(Router::with_path("list").get(get_withdraw_list)),
    //         ),
    // ];
    // let router = Router::new()
    //     // .hoop(_cors_handler)
    //     .hoop(Logger::new())
    //     .hoop(CatchPanic::new())
    //     .append(&mut no_auth_router)
    //     .push(
    //         Router::new()
    //             .hoop(jwt_auth_middleware)
    //             .append(&mut need_auth_router),
    //     );
    router
}
