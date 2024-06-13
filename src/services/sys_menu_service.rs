use crate::{
    app_writer::AppResult,
    dtos::sys_menus_dto::MenuListResponse,
    entities::{
        prelude::{SysUser, SysMenus},
        sys_menus,
    },
    utils::db::DB,
};
use sea_orm::*;

pub async fn get_menu_list(uuid: &String) -> AppResult<Vec<MenuListResponse>> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败"))?;
    
    // 查询用户菜单
    let user_query = SysUser::find_by_id(uuid).one(db).await?;
    let user_model = match user_query.clone() {
        Some(user) => user,
        None => return Err(anyhow::anyhow!("用户未找到").into()),
    };
    dbg!(user_query);

    let menu_list = match user_model.role
    {
        0 => {
        SysMenus::find().all(db).await?
        }
        1 => {
        SysMenus::find()
            .filter(sys_menus::Column::FRole.eq(0))
            .all(db).await?
        } 
        2 => {
        SysMenus::find()
            .filter(sys_menus::Column::FRole.eq(0))
            .filter(sys_menus::Column::UserRole.eq(user_model.role))
            .all(db).await?
        }
        _ => todo!()
    };
    let menu_res = menu_list
    .into_iter()
    .map(|menu| MenuListResponse {
        menu_name: menu.menu_name,
        menu_url: menu.menu_url,
    })
    .collect::<Vec<_>>();
    dbg!(menu_res.clone());
    Ok(menu_res)
}
