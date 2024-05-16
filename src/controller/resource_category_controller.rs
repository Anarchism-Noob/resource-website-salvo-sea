use crate::{
    dtos::sys_category_dto::QueryCategoryResponse,
    middleware::*,
    services::resource_category_service,
    utils::{app_error::AppError, app_writer::AppWriter},
};
use salvo::{
    oapi::{
        endpoint,
        extract::{JsonBody, PathParam},
    },
    Depot, Writer,
};

#[endpoint(tags("获取资源分类列表"))]
pub async fn get_category_list() -> AppWriter<Vec<QueryCategoryResponse>> {
    let _result = resource_category_service::get_all_category().await;
    AppWriter(_result)
}

#[endpoint(tags("创建资源分类"))]
pub async fn create_category(req: JsonBody<String>, depot: &mut Depot) -> AppWriter<()> {
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

    let _result = resource_category_service::create_category(req.0, user_id).await;
    AppWriter(_result)
}

#[endpoint(tags("删除资源分类"))]
pub async fn delete_category(req: PathParam<u32>, depot: &mut Depot) -> AppWriter<()> {
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
    let category = req.0.try_into().unwrap();

    let _result = resource_category_service::delete_category(category, user_id).await;
    AppWriter(_result)
}
