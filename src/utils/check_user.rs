use crate::{
    entities::{
        custom_user,
        sys_user,
    },
    utils::db::DB,
};
use sea_orm::*;

pub async fn check_user_custom(uuid: &String) -> Result<custom_user::Model, anyhow::Error> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败"))?;
    let custom_user = custom_user::Entity::find_by_id(uuid).one(db).await?;
    if custom_user.is_none() {
        return Err(anyhow::anyhow!("当前账号非普通用户"));
    }
    let custom_res = custom_user.unwrap();

    Ok(custom_res)
}

pub async fn check_user_admin(uuid: &String) -> Result<sys_user::Model, anyhow::Error> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败"))?;
    let sys_user = sys_user::Entity::find_by_id(uuid).one(db).await?;
    if sys_user.is_none() {
        return Err(anyhow::anyhow!("当前账号非管理员"));
    }
    let sys_res = sys_user.unwrap();

    Ok(sys_res)
}
