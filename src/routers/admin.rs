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
        // 获取当前登录用户详细信息
        Router::with_path("/ordinary/manager/token_user/profile").get(get_token_profile),
        // 修改当前登录用户密码
        Router::with_path("/ordinary/manager/token_user/change_pwd").post(pchange_pwd),
        // 修改当前登录用户详细信息
        Router::with_path("/ordinary/manager/token_user/change_info").post(change_profile),
        // 提交取款申请
        Router::with_path("/ordinary/manager/token_user/withdrawl").post(put_withdraw),
        // 查看取款申请历史数据
        Router::with_path("/ordinary/manager/token_user/all_withdrawl").get(all_withdraw),
        // 当前登录账号能使用的菜单项
        // Router::with_path("/ordinary/token_user/menu").get(get_menu),
        // 管理员账号管理
        // 创建一个管理员账号
        Router::with_path("/super/manager/admin/create").post(post_register_admin),
        // 查看所有管理员账号
        Router::with_path("/super/manager/admin/all_admin").get(get_admin_list),
        // 禁用一个管理员账号
        Router::with_path("/super/manager/admin/disable_admin").post(disable_admin),
        // 启用一个管理员账号
        Router::with_path("/super/manager/admin/enable_admin").post(enable_admin),
        //Router::with_path("//manager/admin/delete_admin").delete(del_admin),
        // 用户账号管理
        // 查看所有用户
        Router::with_path("/admin/manager/custom/all_custom").get(get_custom_list),
        // 禁用一个用户
        Router::with_path("/admin/manager/custom/disable_custom").post(disable_custom),
        // 启用一个用户
        Router::with_path("/admin/manager/custom/enable_custom").post(enable_custom),
        // 手动充值
        Router::with_path("/admin/manager/custom/recharge").post(put_recharge),
        // 提款管理
        // 查看所有提款申请
        Router::with_path("/super/manager/unprocessed").get(all_unprocessed),
        // 处理提款申请
        Router::with_path("/super/manager/processed").post(put_process),
        // 资源管理
        // 根据条件查询
        Router::with_path("/ordinary/manager/resource/query_resource").get(query_resource),
        // 删除图片
        Router::with_path("/ordinary/manager/resource/remove/image").delete(delete_image),
        // 删除描述文件
        Router::with_path("/ordinary/manager/resource/remove/des_file").delete(delete_des_file),
        // 创建资源
        Router::with_path("/ordinary/manager/resource/create").post(create_resource),
        // 更改资源下载链接
        Router::with_path("/ordinary/manager/resource/change_link").post(put_change_link),
        // 获取所有语言
        Router::with_path("/ordinary/manager/resource/language/all_language").get(all_languages),
        // 创建语言
        Router::with_path("/ordinary/manager/resource/language/create").post(create_language),
        // 删除语言
        Router::with_path("/ordinary/manager/resource/language/remove").delete(delete_language),
        // 获取所有分类
        Router::with_path("/ordinary/manager/resource/category/all_category").get(all_categories),
        // 创建分类
        Router::with_path("/ordinary/manager/resource/category/create").post(create_category),
        // 删除分类
        Router::with_path("/ordinary/manager/resource/category/remove").delete(delete_category),
        // 网站管理
        // 更新网站信息
        Router::with_path("/admin/manager/website/update").post(update_website_profile),
        // 获取历史数据
        Router::with_path("/ordinary/manager/website/get_history").get(get_history_data),
        // 轮播图管理
        // 获取所有轮播图
        Router::with_path("/admin/manager/carousel/all_carousel").get(all_carousel),
        // 创建一个轮播图
        Router::with_path("/admin/manager/carousel/create").post(create_carousel),
        // 删除一个轮播图
        Router::with_path("/admin/manager/carousel/remove").delete(delete_carousel),
        // 查看轮播图详情
        //Router::with_path("/manager/carousel/detail").get(detail_carousel)
        // 上传统一管理
        // 上传资源图片
        Router::with_path("/upload/resource_image").post(put_upload_image),、
        // 上传轮播图
        Router::with_path("/upload/carousel_image").post(upload_carousel),
        // 上传资源描述文件
        Router::with_path("/upload/resource_des_file").post(upload_des_file),
        // 上传网站logo
        Router::with_path("/upload/website_logo").post(upload_website_logo),
        // 上传管理员登陆页面背景
        Router::with_path("/upload/admin_bg").post(upload_admin_bg),
        // 上传用户登录页面背景
        Router::with_path("/upload/custom_bg").post(upload_custom_bg),
        // 上传用户头像
        Router::with_path("/upload/token_user_avatar").post(upload_avatar),
    ];
    router
}

pub fn no_auth_system_api() -> Vec<Router> {
    let router = vec![
        // 用户登录
        Router::with_path("/comm/login").post(post_login),
        Router::with_path("/comm/get_bg").get(get_admin_bg),
        Router::with_path("/website").get(get_website_profile)
    ];
    router
}

