use crate::{
    dtos::{
        query_struct::{BodyStructCreateLanguage, DeleteId},
        sys_language_dto::QueryLanguageResponse,
    },
    middleware::*,
    services::resource_language_service,
    utils::{app_error::AppError, app_writer::AppWriter},
};
use salvo::{
    oapi::{
        endpoint,
        extract::JsonBody,
    },
    Depot, Writer,
};

#[endpoint(tags("获取开发语言列表"))]
pub async fn all_languages() -> AppWriter<Vec<QueryLanguageResponse>> {
    let _redult = resource_language_service::get_language_list().await;
    AppWriter(_redult)
}

#[endpoint(tags("新增开发语言项"))]
pub async fn create_language(
    req: JsonBody<BodyStructCreateLanguage>,
    depot: &mut Depot,
) -> AppWriter<()> {
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
    let language = req.0.language.clone();
    let _redult = resource_language_service::create_language(language.unwrap(), user_id).await;
    AppWriter(_redult)
}

#[endpoint(tags("删除开发语言项"))]
pub async fn delete_language(del: JsonBody<DeleteId>, depot: &mut Depot) -> AppWriter<()> {
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
    let language_id = del.0.c_l_id.clone();

    let _redult = resource_language_service::del_language(language_id.unwrap(), user_id).await;
    AppWriter(_redult)
}
