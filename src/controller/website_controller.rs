use crate::{
    dtos::sys_website_dto::{WebSiteProfileResponse, WebSiteProfileUpdateRequest},
    middleware::*,
    services::website_service,
    utils::{
        app_error::AppError,
        app_writer::{AppResult, AppWriter, ErrorResponseBuilder},
    },
};
use salvo::{
    http::{cookie::Cookie, ParseError, StatusCode},
    oapi::{
        endpoint,
        extract::{JsonBody, PathParam},
    },
    prelude::Json,
    Depot, Request, Response, Writer,
};
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};
use uuid::Uuid;

#[endpoint(tags("获取管理员登陆页面背景"))]
pub async fn get_admin_bg() -> AppWriter<String> {
    if let Err(err) = website_service::get_admin_bg().await {
        return err.into();
    }
    let result = website_service::get_admin_bg().await.unwrap();
    AppWriter(Ok(result))
}

#[endpoint(tags("上传管理员登陆背景"))]
pub async fn upload_admin_bg(mut req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    if let Err(err) = jwt::parse_token(&token) {
        return ErrorResponseBuilder::with_err(AppError::AnyHow(err)).into_response(res);
    }

    let jwt_model = jwt::parse_token(&token).unwrap();
    let uuid = jwt_model.user_id;

    // 创建一个uploads目录，用于保存上传的文件
    let file = req.file("admin_bg").await;
    if let Some(file) = file {
        let mime = file.content_type().unwrap().to_string();
        if mime.starts_with("image/") {
            let file_name = Uuid::new_v4().to_string();
            let mut dest = PathBuf::from("../assets/uploads/sysImage/");

            // 提取原始文件名和扩展名
            let original_file_name = file.name().unwrap_or("file");
            let extension = Path::new(original_file_name)
                .extension()
                .unwrap_or_default();

            // 构建新的文件名（保留原始文件的扩展名）
            dest.push(format!(
                "{}.{}",
                file_name,
                extension.to_str().unwrap_or("png")
            ));

            let info = if let Err(e) = std::fs::copy(&file.path(), &dest) {
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                format!("file not found in request: {}", e)
            } else {
                format!("{:?}", dest)
            };

            let _result =
                website_service::save_admin_bg(dest.to_str().unwrap_or("").to_string(), uuid).await;
            res.render(Json(info));
        }
    } else {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Json("file not found in request"));
    }
}

#[endpoint(tags("获取客户登录页面背景图"))]
pub async fn get_custom_bg() -> AppWriter<String> {
    if let Err(err) = website_service::get_custom_bg().await {
        return err.into();
    }
    let result = website_service::get_custom_bg().await.unwrap();
    AppWriter(Ok(result))
}

#[endpoint(tags("上传客户登录页面背景图"))]
pub async fn upload_custom_bg(mut req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    if let Err(err) = jwt::parse_token(&token) {
        return ErrorResponseBuilder::with_err(AppError::AnyHow(err)).into_response(res);
    }

    let jwt_model = jwt::parse_token(&token).unwrap();
    let uuid = jwt_model.user_id;

    // 创建一个uploads目录，用于保存上传的文件
    let file = req.file("custom_bg").await;
    if let Some(file) = file {
        let mime = file.content_type().unwrap().to_string();
        if mime.starts_with("image/") {
            let file_name = Uuid::new_v4().to_string();
            let mut dest = PathBuf::from("../assets/uploads/sysImage/");

            // 提取原始文件名和扩展名
            let original_file_name = file.name().unwrap_or("file");
            let extension = Path::new(original_file_name)
                .extension()
                .unwrap_or_default();

            // 构建新的文件名（保留原始文件的扩展名）
            dest.push(format!(
                "{}.{}",
                file_name,
                extension.to_str().unwrap_or("png")
            ));

            let info = if let Err(e) = std::fs::copy(&file.path(), &dest) {
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                format!("file not found in request: {}", e)
            } else {
                format!("{:?}", dest)
            };

            let _result =
                website_service::save_custom_bg(dest.to_str().unwrap_or("").to_string(), uuid)
                    .await;
            res.render(Json(info));
        }
    } else {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Json("file not found in request"));
    }
}

#[endpoint(tags("获取网站logo"))]
pub async fn get_website_logo() -> AppWriter<String> {
    if let Err(err) = website_service::get_website_logo().await {
        return err.into();
    }
    let result = website_service::get_website_logo().await.unwrap();
    AppWriter(Ok(result))
}

#[endpoint(tags("上传网站logo"))]
pub async fn upload_website_logo(mut req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    if let Err(err) = jwt::parse_token(&token) {
        return ErrorResponseBuilder::with_err(AppError::AnyHow(err)).into_response(res);
    }

    let jwt_model = jwt::parse_token(&token).unwrap();
    let uuid = jwt_model.user_id;

    // 创建一个uploads目录，用于保存上传的文件
    let file = req.file("website_logo").await;
    if let Some(file) = file {
        let mime = file.content_type().unwrap().to_string();
        if mime.starts_with("image/") {
            let file_name = Uuid::new_v4().to_string();
            let mut dest = PathBuf::from("../assets/uploads/sysImage/");

            // 提取原始文件名和扩展名
            let original_file_name = file.name().unwrap_or("file");
            let extension = Path::new(original_file_name)
                .extension()
                .unwrap_or_default();

            // 构建新的文件名（保留原始文件的扩展名）
            dest.push(format!(
                "{}.{}",
                file_name,
                extension.to_str().unwrap_or("png")
            ));

            let info = if let Err(e) = std::fs::copy(&file.path(), &dest) {
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                format!("file not found in request: {}", e)
            } else {
                format!("{:?}", dest)
            };

            let _result =
                website_service::save_logo(dest.to_str().unwrap_or("").to_string(), uuid).await;
            res.render(Json(info));
        }
    } else {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Json("file not found in request"));
    }
}

#[endpoint(tags("获取网站信息"))]
pub async fn get_website_profile() -> AppWriter<WebSiteProfileResponse> {
    // // 获取token
    // let token = depot.get::<&str>("jwt_token").copied().unwrap();
    // //判断token是否可用
    // if let Err(err) = jwt::parse_token(&token) {
    //     return AppError::AnyHow(err).into();
    // }
    // // 解析token
    // let jwt_model = jwt::parse_token(&token).unwrap();

    // // 获取用户id
    // let user_id = jwt_model.user_id;

    // 获取网站信息
    let website_info = website_service::get_website_info().await;

    // 返回网站信息
    AppWriter(website_info)
}

#[endpoint(tags("更新网站信息"))]
pub async fn update_website_profile(
    form_data: JsonBody<WebSiteProfileUpdateRequest>,
    depot: &mut Depot,
) -> AppWriter<WebSiteProfileResponse> {
    // 获取token
    let token = depot.get::<&str>("jwt_token").copied().unwrap();
    //判断token是否可用
    if let Err(err) = jwt::parse_token(&token) {
        return AppError::AnyHow(err).into();
    }
    // 解析token
    let jwt_model = jwt::parse_token(&token).unwrap();

    // 获取用户id
    let user_id = jwt_model.user_id;

    // 更新网站信息
    let _result = website_service::update_website_info(form_data.0, user_id).await;
    AppWriter(_result)
}
