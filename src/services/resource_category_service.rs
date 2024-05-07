use crate::{
    app_writer::AppResult,
    dtos::sys_category_dto::QueryCategoryResponse,
    entities::{
        prelude::{SysResourceCategoty, SysUser},
        sys_resource_category, sys_user,
    },
    utils::db::DB,
};
use sea_orm::*;

pub async fn create_category(req: String, uuid: String) -> AppResult<()> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败"))?;
    let user_query = SysUser::find_by_id(uuid).one(db).await?;
    if user_query.is_none() {
        return Err(anyhow::anyhow!("该管理员不存在").into());
    }
    let user_model = user_query.clone().unwrap();

    sys_resource_category::ActiveModel {
        category_id: Set(0),
        category_name: Set(req),
        crate_user_name: Set(user_model.user_name),
    }
    .save(db)
    .await?;
    Ok(())
}

pub async fn get_all_category() -> AppResult<Vec<QueryCategoryResponse>> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败"))?;
    let _query = SysResourceCategoty::find().all(db).await?;
    let category_res = _query
        .into_iter()
        .map(|item| QueryCategoryResponse {
            category_id: item.category_id,
            category_name: item.category_name,
            crate_user_name: item.crate_user_name,
        })
        .collect::<Vec<_>>();
    Ok(category_res)
}

pub async fn delete_category(id: i32, uuid:String) -> AppResult<()> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败"))?;
    let user_query = SysUser::find_by_id(uuid).one(db).await?;
    if user_query.is_none() {
        return Err(anyhow::anyhow!("该管理员不存在").into());
    }
    let _query = SysResourceCategoty::find_by_id(id).one(db).await?;
    if _query.is_none() {
        return Err(anyhow::anyhow!("该分类不存在").into());
    }
    SysResourceCategoty::delete_by_id(id).exec(db).await?;
    Ok(())
}