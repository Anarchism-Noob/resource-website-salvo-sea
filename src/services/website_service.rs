use crate::{
    app_writer::AppResult,
    dtos::sys_website_dto::{WebSiteProfileResponse, WebSiteProfileUpdateRequest},
    entities::{
        prelude::{SysUser, SysWebsiteInfo}, sys_website_info,
    },
    utils::{db::DB},
};
use sea_orm::*;

pub async fn save_admin_bg(file_path: String, uuid: String) -> AppResult<()> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败")).unwrap();
    let user = SysUser::find_by_id(uuid).one(db).await?;
    if user.is_none() {
        return Err(anyhow::anyhow!("用户不存在").into());
    }
    // 更新网站信息
    let profile_query = SysWebsiteInfo::find()
        .filter(sys_website_info::Column::Id.eq(1))
        .one(db)
        .await?;
    let mut profile_res: sys_website_info::ActiveModel = profile_query.unwrap().into();
    profile_res.admin_login_img = Set(Some(file_path));
    profile_res.update(db).await?;
    Ok(())
}

pub async fn save_custom_bg(file_path: String, uuid: String) -> AppResult<()> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败")).unwrap();
    let user = SysUser::find_by_id(uuid).one(db).await?;
    if user.is_none() {
        return Err(anyhow::anyhow!("用户不存在").into());
    }
    // 更新网站信息
    let profile_query = SysWebsiteInfo::find()
        .filter(sys_website_info::Column::Id.eq(1))
        .one(db)
        .await?;
    let mut profile_res: sys_website_info::ActiveModel = profile_query.unwrap().into();
    profile_res.custom_login_img = Set(Some(file_path));
    profile_res.update(db).await?;
    Ok(())
}

pub async fn get_website_logo() -> AppResult<String> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败")).unwrap();
    // 查询网站信息
    let profile = SysWebsiteInfo::find()
        .filter(sys_website_info::Column::Id.eq(1))
        .one(db)
        .await?;
    let profile_res = profile.ok_or(anyhow::anyhow!("网站信息不存在"))?;
    Ok(profile_res.website_icon.unwrap())
}

pub async fn save_logo(file_path: String, uuid: String) -> AppResult<()> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败")).unwrap();
    let user = SysUser::find_by_id(uuid).one(db).await?;
    if user.is_none() {
        return Err(anyhow::anyhow!("用户不存在").into());
    }
    // 更新网站信息
    let profile_query = SysWebsiteInfo::find()
        .filter(sys_website_info::Column::Id.eq(1))
        .one(db)
        .await?;
    let mut profile_res: sys_website_info::ActiveModel = profile_query.unwrap().into();
    profile_res.website_icon = Set(Some(file_path));
    profile_res.update(db).await?;
    Ok(())
}

pub async fn get_website_info() -> AppResult<WebSiteProfileResponse> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败")).unwrap();
    // // 查询用户信息
    // let user = SysUser::find_by_id(uuid).one(db).await?;
    // let user_res = user.ok_or(anyhow::anyhow!("用户不存在"))?;
    // 查询网站信息
    let profile = SysWebsiteInfo::find()
        .filter(sys_website_info::Column::Id.eq(1))
        .one(db)
        .await?;
    let profile_res = profile.ok_or(anyhow::anyhow!("网站信息不存在"))?;
    Ok(WebSiteProfileResponse {
        website_name: Some(profile_res.name),
        version: profile_res.version,
        public_record: profile_res.public_record,
        website_record: profile_res.website_record,
        sys_kefu: profile_res.sys_kefu,
        website_logo: profile_res.website_icon,
        custom_login_bg: profile_res.custom_login_img,
        admin_login_bg: profile_res.admin_login_img,
    })
}

pub async fn update_website_info(
    req: WebSiteProfileUpdateRequest,
    uuid: String,
) -> AppResult<WebSiteProfileResponse> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败")).unwrap();
    let user = SysUser::find_by_id(uuid).one(db).await?;
    if user.is_none() {
        return Err(anyhow::anyhow!("用户不存在").into());
    };
    // 更新网站信息
    let profile_query = SysWebsiteInfo::find()
        .filter(sys_website_info::Column::Name.eq(req.website_name.clone()))
        .one(db)
        .await?;
    let mut profile_res: sys_website_info::ActiveModel = profile_query.unwrap().into();
    profile_res.name = Set(req.website_name.unwrap());
    profile_res.version = Set(req.version);
    profile_res.public_record = Set(req.public_record);
    profile_res.website_record = Set(req.website_record);
    profile_res.sys_kefu = Set(req.sys_kefu);
    let _redult = profile_res.update(db).await?;

    Ok(WebSiteProfileResponse {
        website_name: Some(_redult.name),
        version: _redult.version,
        public_record: _redult.public_record,
        website_record: _redult.website_record,
        sys_kefu: _redult.sys_kefu,
        website_logo: _redult.website_icon,
        custom_login_bg: _redult.custom_login_img,
        admin_login_bg: _redult.admin_login_img,
    })
}

// pub async fn change_custom_login_bg(file_path: String, uuid: String) -> AppResult<String> {
//     let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败")).unwrap();
//     let user = SysUser::find_by_id(uuid).one(db).await?;
//     let user_res = user.ok_or(anyhow::anyhow!("用户不存在"))?;
//     let profile = SysWebsiteInfo::find()
//         .filter(sys_website_info::Column::Id.eq(1))
//         .one(db)
//         .await?;
//     let profile_model = profile.ok_or(anyhow::anyhow!("网站信息不存在"))?;
//     let mut profile_res: sys_website_info::ActiveModel = profile_model.into();
//     // 更新网站信息
//     profile_res.custom_login_img = Set(Some(file_path.clone()));
//     profile_res.update(db).await?;
//     Ok(file_path)
// }
// pub async fn change_admin_login_bg(file_path: String, uuid: String) -> AppResult<String> {
//     let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败")).unwrap();
//     let user = SysUser::find_by_id(uuid).one(db).await?;
//     let user_res = user.ok_or(anyhow::anyhow!("用户不存在"))?;
//     let profile = SysWebsiteInfo::find()
//         .filter(sys_website_info::Column::Id.eq(1))
//         .one(db)
//         .await?;
//     let profile_model = profile.ok_or(anyhow::anyhow!("网站信息不存在"))?;
//     let mut profile_res: sys_website_info::ActiveModel = profile_model.into();
//     // 更新网站信息
//     profile_res.admin_login_img = Set(Some(file_path.clone()));
//     profile_res.update(db).await?;
//     Ok(file_path)
// }

pub async fn get_custom_bg() -> AppResult<String> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败")).unwrap();
    let profile = SysWebsiteInfo::find()
        .filter(sys_website_info::Column::Id.eq(1))
        .one(db)
        .await?;

    let profile_model = profile.ok_or(anyhow::anyhow!("网站信息不存在"))?;

    Ok(profile_model.custom_login_img.unwrap())
}

pub async fn get_admin_bg() -> AppResult<String> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败")).unwrap();
    let profile = SysWebsiteInfo::find()
        .filter(sys_website_info::Column::Id.eq(1))
        .one(db)
        .await?;

    let profile_model = profile.ok_or(anyhow::anyhow!("网站信息不存在"))?;

    Ok(profile_model.admin_login_img.unwrap())
}
