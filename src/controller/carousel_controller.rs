use crate::{
    dtos::{
        query_struct::DeleteUuid,
        sys_carousel_dto::{CreateCarouselRequest, QueryCarouselResponse},
    },
    middleware::*,
    services::sys_carousel_service,
    utils::{
        app_error::AppError,
        app_writer::{AppWriter, ErrorResponseBuilder},
    },
};
use salvo::{
    http::StatusCode,
    oapi::{endpoint, extract::JsonBody},
    prelude::Json,
    Depot, Request, Response, Writer,
};
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[endpoint(tags("删除轮播图"))]
pub async fn delete_carousel(img_uuid: JsonBody<DeleteUuid>, depot: &mut Depot) -> AppWriter<()> {
    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    if let Err(err) = jwt::parse_token(token) {
        return AppError::AnyHow(err).into();
    }
    let jwt_model = jwt::parse_token(token).unwrap();
    let token_uuid = jwt_model.user_id;
    let image = img_uuid.0.img_uuid;
    let _result = sys_carousel_service::delete_carousel(image.unwrap(), token_uuid).await;
    AppWriter(_result)
}

#[endpoint(tags("获取首页轮播图"))]
pub async fn all_carousel() -> AppWriter<Vec<QueryCarouselResponse>> {
    let carousel = sys_carousel_service::get_carousel().await;
    AppWriter(carousel)
}

#[endpoint(tags("创建轮播图"))]
pub async fn create_carousel(
    form_data: JsonBody<CreateCarouselRequest>,
    depot: &mut Depot,
    res: &mut Response,
) {
    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    if let Err(err) = jwt::parse_token(token) {
        return ErrorResponseBuilder::with_err(AppError::AnyHow(err)).into_response(res);
    }

    let jwt_model = jwt::parse_token(token).unwrap();
    let uuid = jwt_model.user_id;
    let _form_data = form_data.0;
    let _result = sys_carousel_service::create_carouwsel(_form_data, uuid).await;
    match _result {
        Ok(_) => {
            res.status_code(StatusCode::OK);
        }
        Err(err) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            return ErrorResponseBuilder::with_err(err).into_response(res);
        }
    }
}

#[endpoint(tags("上传轮播图"))]
pub async fn upload_carousel(req: &mut Request, res: &mut Response) {
    // 创建一个uploads目录，用于保存上传的文件
    let file = req.file("avatar").await;
    if let Some(file) = file {
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
                extension.to_str().unwrap_or("png")
            ));

            let info = if let Err(e) = std::fs::copy(file.path(), &dest) {
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                format!("file not found in request: {}", e)
            } else {
                format!("{:?}", dest)
            };

            let _result = sys_carousel_service::save_carsousel(
                dest.to_str().unwrap_or("").to_string(),
                file_name,
            )
            .await;
            match _result {
                Ok(_) => {
                    res.status_code(StatusCode::OK);
                    res.render(Json(info));
                }
                Err(e) => {
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.render(Json(e.to_string()));
                }
            }
            // res.render(Json(info));
        }
    } else {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Json("file not found in request"));
    }
}
