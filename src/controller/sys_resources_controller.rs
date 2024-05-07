use crate::{
    app_writer::{AppResult, AppWriter, ErrorResponseBuilder},
    dtos::{
        sys_image_dto::SysImageCrateRequest,
        sys_resources_dto::{
            PaginationParams, SysResourceChangeLink, SysResourceCreateRequest, SysResourceList,
            SysResourceResponse,
        },
    },
    middleware::jwt,
    services::sys_resource_service,
    utils::{
        app_error::AppError,
        captcha_utils::{generate_captcha, varify_captcha, CaptchaImage},
        redis_utils::get_redis_pool,
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


#[endpoint(tags("删除页面截图"))]
pub async fn delete_image(
    req: PathParam<String>,
    depot: &mut Depot,
) -> AppWriter<()> {
    let token = depot.get::<&str>("jwt-token").copied().unwrap();

    if let Err(err) = jwt::parse_token(&token) {
        return AppWriter(Err(err.into()));
    }
    let jwt_model = jwt::parse_token(&token).unwrap();
    let uuid = jwt_model.user_id;
    let image_uuid = req.0;
    let result = sys_resource_service::delete_image(image_uuid, uuid).await;
    AppWriter(result)
}


#[endpoint(tags("根据uuid获取资源详情"))]
pub async fn get_resource_detail_by_uuid(
    req: &mut Request,
    depot: &mut Depot,
) -> AppWriter<SysResourceResponse> {
    let resource_uuid = req.param("uuid").unwrap();

    let token = depot.get::<&str>("jwt-token").copied().unwrap();

    if let Err(err) = jwt::parse_token(&token) {
        return AppWriter(Err(err.into()));
    }
    let jwt_model = jwt::parse_token(&token).unwrap();
    let uuid = jwt_model.user_id;

    let result = sys_resource_service::get_resource_detail_by_uuid(resource_uuid, uuid).await;
    AppWriter(result)
}

#[endpoint(tags("根据类型和语言获取资源列表"))]
pub async fn get_resources_of_category_and_language(
    path_param: JsonBody<PaginationParams>,
) -> AppWriter<Vec<SysResourceList>> {
    let page_query = path_param.0;
    let page = page_query.page.unwrap_or(1);
    let page_size = page_query.page_size.unwrap_or(49);
    let category = page_query.category.unwrap();
    let language = page_query.language.unwrap();
    match sys_resource_service::get_resources_by_category_and_language(
        category, language, page, page_size,
    )
    .await
    {
        Ok(result) => AppWriter(Ok(result)),
        Err(err) => AppWriter(Err(err.into())),
    }
}

#[endpoint(tags("根据类型获取资源列表"))]
pub async fn get_resources_of_category(
    path_param: JsonBody<PaginationParams>,
) -> AppWriter<Vec<SysResourceList>> {
    let page_query = path_param.0;
    let page = page_query.page.unwrap_or(1);
    let page_size = page_query.page_size.unwrap_or(49);
    let category = page_query.category.unwrap();
    match sys_resource_service::get_resources_of_category(category, page, page_size).await {
        Ok(result) => AppWriter(Ok(result)),
        Err(err) => AppWriter(Err(err.into())),
    }
}

#[endpoint(tags("根据语言获取资源列表"))]
pub async fn get_resource_list_by_language(
    page_query: JsonBody<PaginationParams>,
) -> AppWriter<Vec<SysResourceList>> {
    let page_query = page_query.0;
    let page = page_query.page.unwrap_or(1);
    let page_size = page_query.page_size.unwrap_or(49);
    let language = page_query.language.unwrap();
    match sys_resource_service::get_resours_of_language(language, page, page_size).await {
        Ok(result) => AppWriter(Ok(result)),
        Err(err) => AppWriter(Err(err.into())),
    }
}

#[endpoint(tags("获取资源列表"))]
pub async fn get_resource_list(
    page_query: JsonBody<PaginationParams>,
) -> AppWriter<Vec<SysResourceList>> {
    let page_query = page_query.0;
    let page = page_query.page.unwrap_or(1);
    let page_size = page_query.page_size.unwrap_or(49);
    match sys_resource_service::get_resource_list(page, page_size).await {
        Ok(result) => AppWriter(Ok(result)),
        Err(err) => AppWriter(Err(err.into())),
    }
}

#[endpoint(tags("更改下载链接"))]
pub async fn post_change_link(form_data: JsonBody<SysResourceChangeLink>, res: &mut Response) {
    let cloned_form_data = form_data.0;
    let resource_link = cloned_form_data.resource_link.clone();
    if let Err(_err) = sys_resource_service::change_resource_link(cloned_form_data).await {
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
    }
    res.render(Json(format!("改资源的下载链接已更改为{}", resource_link)));
}

#[endpoint(tags("新建源码包"))]
pub async fn post_create_resource(
    form_data: JsonBody<SysResourceCreateRequest>,
    depot: &mut Depot,
    res: &mut Response,
) {
    let form_data = form_data.0;

    let token = depot.get::<&str>("jwt-token").copied().unwrap();

    if let Err(err) = jwt::parse_token(&token) {
        return ErrorResponseBuilder::with_err(AppError::AnyHow(err)).into_response(res);
    }

    let jwt_model = jwt::parse_token(&token).unwrap();
    let uuid = jwt_model.user_id;

    if let Err(_err) = sys_resource_service::create_resource(form_data, uuid).await {
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
    }
    res.status_code(StatusCode::CREATED);
}

//上传资源截图
#[endpoint(tags("上传图片"))]
pub async fn post_upload_avatar(req: &mut Request, res: &mut Response) {
    let files = req.files("avatar").await;
    if let Some(files) = files {
        let mut msgs: Vec<(String, String)> = Vec::with_capacity(files.len());
        for file in files {
            let mime = file.content_type().unwrap().to_string();
            if mime.starts_with("image/") {
                let file_name = Uuid::new_v4().to_string();
                let mut dest = PathBuf::from("../assets/uploads/avatar/");

                // 提取原始文件名和扩展名
                let original_file_name = file.name().unwrap_or("file");
                let extension = Path::new(original_file_name)
                    .extension()
                    .unwrap_or_default();

                // 构建新的文件名（保留原始文件的扩展名）
                dest.push(format!(
                    "{}.{}",
                    file_name,
                    extension.to_str().unwrap_or("jpg")
                ));

                if let Err(e) = std::fs::copy(&file.path(), &dest) {
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.render(Json(format!("file not found in request: {}", e)));
                } else {
                    msgs.push((dest.to_string_lossy().to_string(), file_name.clone()));
                }
            }
        }
        sys_resource_service::save_resource_image(msgs);
        // res.render(Json(msgs));
    } else {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Json("file not found in request"));
    }
}
