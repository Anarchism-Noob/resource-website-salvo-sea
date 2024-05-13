use crate::{
    dtos::sys_language_dto::QueryLanguageResponse,
    middleware::*,
    services::resource_language_service,
    utils::{
        app_error::AppError,
        app_writer::AppWriter,
    },
};
use salvo::{
    oapi::{
        endpoint,
        extract::PathParam,
    },
    Depot, Writer,
};

#[endpoint(tags("获取开发语言列表"))]
pub async fn get_dev_languages() -> AppWriter<Vec<QueryLanguageResponse>> {
    let _redult = resource_language_service::get_language_list().await;
    AppWriter(_redult)
}

#[endpoint(tags("新增开发语言项"))]
pub async fn post_create_language(req: PathParam<String>, depot: &mut Depot) -> AppWriter<()> {
    // 获取token
    let token = depot.get::<&str>("jwt_token").copied().unwrap();
    //判断token是否可用
    if let Err(err) = jwt::parse_token(token) {
        return AppError::AnyHow(err).into();
    }
    // 解析token
    let jwt_model = jwt::parse_token(token).unwrap();

    // 获取用户id
    let user_id = jwt_model.user_id;

    let _redult = resource_language_service::create_language(req.0, user_id).await;
    AppWriter(_redult)
}

#[endpoint(tags("删除开发语言项"))]
pub async fn delete_language(req: PathParam<i32>, depot: &mut Depot) -> AppWriter<()> {
    // 获取token
    let token = depot.get::<&str>("jwt_token").copied().unwrap();
    //判断token是否可用
    if let Err(err) = jwt::parse_token(token) {
        return AppError::AnyHow(err).into();
    }
    // 解析token
    let jwt_model = jwt::parse_token(token).unwrap();

    // 获取用户id
    let user_id = jwt_model.user_id;

    let _redult = resource_language_service::del_language(req.0, user_id).await;
    AppWriter(_redult)
}
