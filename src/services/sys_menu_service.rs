use crate::{
    app_writer::AppResult,
    dtos::sys_menus_dto::MenuListResponse,
    entities::{
        prelude::{SysUser, SysMenus},
        sys_user, sys_menus,
    },
    utils::db::DB,
};
use sea_orm::*;
use uuid::Uuid;

pub async fn get_menu_list(uuid: &String) -> AppResult<Vec<MenuListResponse>> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败"))?;
    
    // 查询用户菜单
    let user_query = SysUser::find_by_id(uuid).one(db).await?;
    let user_model = match user_query {
        Some(user) => user,
        None => return Err(anyhow::anyhow!("用户未找到").into()),
    };

    let menu_list = if user_model.role == 0 {
        SysMenus::find().all(db).await?
    } else if user_model.role == 1 {
        SysMenus::find()
            .filter(sys_menus::Column::FRole.eq(0))
            .all(db).await?
    } else {
        SysMenus::find()
            .filter(sys_menus::Column::FRole.eq(0))
            .filter(sys_menus::Column::UserRole.eq(user_model.role))
            .all(db).await?
    };
    let menu_res = menu_list
    .into_iter()
    .map(|menu| MenuListResponse {
        menu_name: menu.menu_name,
        menu_url: menu.menu_url,
    })
    .collect::<Vec<_>>();

    Ok(menu_res)
}
