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
    custom_user.ok_or_else(|| anyhow::anyhow!("custom表中没用该用户"))
}

pub async fn check_user_admin(uuid: &String) -> Result<sys_user::Model, anyhow::Error> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败"))?;
    let sys_user = sys_user::Entity::find_by_id(uuid).one(db).await?;
    sys_user.ok_or_else(|| anyhow::anyhow!("当前账号非管理员"))
}
