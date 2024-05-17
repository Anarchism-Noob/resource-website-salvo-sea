use crate::{
    app_writer::{AppWriter, ErrorResponseBuilder},
    dtos::{
        query_struct::{DeleteUuid, PathFilterStruct, QueryPageStruct, QueryParamsStruct},
        sys_resources_dto::{
            SysResourceChangeLink, SysResourceCreateRequest, SysResourceList, SysResourceResponse,
        },
    },
    middleware::jwt,
    services::sys_resource_service,
    utils::app_error::AppError,
};
use salvo::{
    http::StatusCode,
    oapi::{
        endpoint,
        extract::JsonBody,
    },
    prelude::Json,
    Depot, Request, Response, Writer,
};
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[endpoint(tags("删除页面展示图"))]
pub async fn delete_image(resource_img: JsonBody<DeleteUuid>, depot: &mut Depot) -> AppWriter<()> {
    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    if let Err(err) = jwt::parse_token(token) {
        return AppWriter(Err(err.into()));
    }
    let jwt_model = jwt::parse_token(token).unwrap();
    let uuid = jwt_model.user_id;
    let image_uuid = resource_img.into_inner().img_uuid.clone();
    let result = sys_resource_service::delete_image(image_uuid.unwrap(), uuid).await;
    AppWriter(result)
}

#[endpoint(tags("根据uuid获取资源详情"))]
pub async fn get_resource_detail_by_uuid(
    resource: PathFilterStruct,
    depot: &mut Depot,
) -> AppWriter<SysResourceResponse> {
    // 从url获取resource_uuid
    let resource_uuid = resource.resource.clone();

    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    let jwt_model = match jwt::parse_token(token) {
        Ok(jwt_model) => jwt_model,
        Err(err) => return AppWriter(Err(err.into())),
    };

    // 获取用户id
    let uuid = jwt_model.user_id;
    let role: Option<u32> = jwt_model.role;

    let result =
        sys_resource_service::get_resource_detail_by_uuid(resource_uuid.unwrap(), Some(uuid), role)
            .await;
    AppWriter(result)
}

#[endpoint(tags("根据QueryParams获取资源列表"))]
pub async fn get_resource_list(
    query_parament: QueryParamsStruct,
    page_s: JsonBody<QueryPageStruct>,
) -> AppWriter<Vec<SysResourceList>> {
    // 从请求中获取分页参数
    // let query_params: QueryParamsStruct = query_parament;
    let language = query_parament.language;
    let category = query_parament.category;
    let page = page_s.0.page.unwrap_or_default();
    let page_size = page_s.0.page_size.unwrap_or_default();
    // 若同时存在
    if language.is_some() && category.is_some() {
        match sys_resource_service::get_resources_by_category_and_language(
            category.unwrap(),
            language.unwrap(),
            page,
            page_size,
        )
        .await
        {
            Ok(result) => AppWriter(Ok(result)),
            Err(err) => AppWriter(Err(err)),
        }
    } else
    // 若存在category
    if category.is_some() {
        match sys_resource_service::get_resources_of_category(category.unwrap(), page, page_size)
            .await
        {
            Ok(result) => AppWriter(Ok(result)),
            Err(err) => AppWriter(Err(err)),
        }
    } else
    // 若存在language
    if language.is_some() {
        match sys_resource_service::get_resours_of_language(language.unwrap(), page, page_size)
            .await
        {
            Ok(result) => AppWriter(Ok(result)),
            Err(err) => AppWriter(Err(err)),
        }
    } else {
        // 返回默认列表
        match sys_resource_service::get_resource_list(page, page_size).await {
            Ok(result) => AppWriter(Ok(result)),
            Err(err) => AppWriter(Err(err)),
        }
    }
}

#[endpoint(tags("更改下载链接"))]
pub async fn put_change_link(
    query_param: PathFilterStruct,
    form_data: JsonBody<SysResourceChangeLink>,
    depot: &mut Depot,
    res: &mut Response,
) {
    let new_link = form_data.0.resource_link;
    let resource_uuid = query_param.resource;
    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    let jwt_model = match jwt::parse_token(token) {
        Ok(jwt_model) => jwt_model,
        Err(err) => {
            return ErrorResponseBuilder::with_err(AppError::AnyHow(err)).into_response(res)
        }
    };

    // 获取用户id
    let uuid = jwt_model.user_id;
    // 更改下载链接
    if let Err(_err) =
        sys_resource_service::change_resource_link(new_link.clone(), resource_uuid.unwrap(), uuid).await
    {
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
    }
    res.render(Json(format!("改资源的下载链接已更改为：{}", new_link)));
}

#[endpoint(tags("新建源码包"))]
pub async fn post_create_resource(
    form_data: JsonBody<SysResourceCreateRequest>,
    depot: &mut Depot,
    res: &mut Response,
) {
    let form_data = form_data.0;

    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    if let Err(err) = jwt::parse_token(token) {
        return ErrorResponseBuilder::with_err(AppError::AnyHow(err)).into_response(res);
    }

    let jwt_model = jwt::parse_token(token).unwrap();
    let uuid = jwt_model.user_id;

    if let Err(_err) = sys_resource_service::create_resource(form_data, uuid).await {
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
    }
    res.status_code(StatusCode::CREATED);
}

#[endpoint(tags("上传描述文件"))]
pub async fn put_upload_description(req: &mut Request, res: &mut Response) {
    let file = req.file("description").await;
    if let Some(file) = file {
        let mime = file.content_type().unwrap().to_string();
        if mime.starts_with("text/") {
            let file_name = Uuid::new_v4().to_string();
            let mut dest = PathBuf::from("../assets/uploads/description/");

            // 提取原始文件名和扩展名
            let original_file_name = file.name().unwrap_or("file");
            let extension = match Path::new(original_file_name).extension() {
                Some(extension) => extension.to_string_lossy().to_lowercase(),
                None => return,
            };
            // 判断上传的描述文件类型是否为.md或.txt
            if !extension.eq("md") || !extension.eq("txt") {
                res.status_code(StatusCode::BAD_REQUEST);
                res.render(Json("文件类型错误，请上传.md或.txt文件"));
            }

            // 构建新的文件名（保留原始文件的扩展名）
            dest.push(format!("{}.{}", file_name, extension));

            // 保存文件
            let info = if let Err(e) = std::fs::copy(file.path(), &dest) {
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                format!("file not found in request: {}", e)
            } else {
                res.status_code(StatusCode::OK);
                format!("{:?}", dest)
            };

            res.render(Json(info));
        }
    } else {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Json("file not found in request"));
    }
}

//上传资源截图
#[endpoint(tags("上传图片"))]
pub async fn put_upload_image(req: &mut Request, res: &mut Response) {
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

                if let Err(e) = std::fs::copy(file.path(), &dest) {
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.render(Json(format!("file not found in request: {}", e)));
                } else {
                    msgs.push((dest.to_string_lossy().to_string(), file_name.clone()));
                }
            }
        }
        let _resulr = sys_resource_service::save_resource_image(msgs.clone());
        if let Err(e) = _resulr.await {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(format!("file not found in request: {}", e)));
        };
        res.render(Json(&msgs));
    } else {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Json("file not found in request"));
    }
}
