use crate::{
    app_writer::AppResult,
    dtos::sys_language_dto::QueryLanguageResponse,
    entities::{
        prelude::{SysResourceLanguage, SysUser},
        sys_resource_language,
    },
    utils::db::DB,
};
use sea_orm::*;

pub async fn create_language(req: String, uuid: String) -> AppResult<()> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败"))?;
    let user_query = SysUser::find_by_id(uuid).one(db).await?;
    let user = user_query.ok_or(anyhow::anyhow!("管理员不存在"))?;

    sys_resource_language::ActiveModel {
        language_id: Set(0),
        language_name: Set(req),
        create_user_name: Set(user.user_name),
    }
    .save(db)
    .await?;
    Ok(())
}

pub async fn get_language_list()-> AppResult<Vec<QueryLanguageResponse>>{

    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败"))?;
    let language_list = SysResourceLanguage::find()
        .all(db)
        .await?;
    let mut result = Vec::new();
    for language in language_list{
        result.push(QueryLanguageResponse{
            language_id: language.language_id,
            language_name: language.language_name,
            crate_user_name: language.create_user_name,
        })
    }
    Ok(result)
}

pub async fn del_language(id: i32, uuid: String)-> AppResult<()>{
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败"))?;
    let user_query = SysUser::find_by_id(uuid).one(db).await?;  
    if user_query.is_none(){
        return Err(anyhow::anyhow!("管理员不存在").into());
    }
    let language_query = SysResourceLanguage::find_by_id(id).one(db).await?;

    let _language = language_query.ok_or(anyhow::anyhow!("语言类型不存在"))?;
    SysResourceLanguage::delete_by_id(id).exec(db).await?;
    Ok(())
}