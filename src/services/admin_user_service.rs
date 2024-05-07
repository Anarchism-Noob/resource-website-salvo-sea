use crate::{
    app_writer::AppResult,
    dtos::{
        custom_user_dto::{CustomUserProfileResponse, RechargeOfAdminRequest},
        sys_resources_dto::{SysResourceCreateRequest, SysResourceResponse},
        sys_user_dto::{
            ChangeAdminProfileRequest, ChangeAdminPwdRequest, SysLoginRequest, SysLoginResponse,
            SysUserCrateRequest, SysUserProfileResponse,
        },
    },
    entities::{
        custom_orders, custom_recharge_records, custom_user,
        prelude::{CustomOrders, CustomUser, SysImage, SysResources, SysUser},
        sys_image, sys_resource_images, sys_resources, sys_user,
    },
    middleware::jwt::get_token,
    utils::{db::DB, rand_utils, redis_utils::*},
};
use chrono::{Local, NaiveDate, Utc};
use redis::{Client, RedisResult};
use sea_orm::{prelude::Decimal, *};
use serde_json;
use uuid::Uuid;

pub async fn super_admin_init() {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let user_pwd = "waqu2024".to_string();
    sys_user::ActiveModel {
        user_uuid: Set(Uuid::new_v4().to_string()),
        nick_name: Set("超级管理员".to_string()),
        user_name: Set("superadmin".to_string()),
        user_pwd: Set(rand_utils::hash_password(user_pwd).await.unwrap()),
        email: Set(Option::from("slurredforgfun@gmail.com".to_string())),
        user_status: Set(0),
        role: Set(0),
        avatar_path: Set("../assets/avatar/default.png".to_string()),
    }
    .save(db)
    .await
    .unwrap();
}

pub async fn recharge_for_custom(
    from_data: RechargeOfAdminRequest,
    admin_uuid: String,
) -> AppResult<()> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let admin_query = SysUser::find_by_id(admin_uuid).one(db).await?;
    let admin_model = admin_query.clone().ok_or(anyhow::anyhow!("用户不存在"));
    let custom_query = CustomUser::find_by_id(&from_data.user_uuid).one(db).await?;
    let usdt = from_data.balance_usdt + custom_query.clone().unwrap().balance_usdt;
    let mut custom_model: custom_user::ActiveModel = custom_query.clone().unwrap().into();
    custom_model.balance_usdt = Set(usdt);
    custom_model.update(db).await?;
    let transaction_id = format!(
        "{}:{}",
        admin_query.unwrap().user_uuid,
        Uuid::new_v4().to_string()
    );
    custom_recharge_records::ActiveModel {
        record_uuid: Set(Uuid::new_v4().to_string()),
        user_uuid: Set(from_data.user_uuid),
        recharge_amount: Set(from_data.balance_usdt),
        payment_channel: Set("线下充值".to_string()),
        recharge_date: Set(Local::now().naive_utc()),
        recharge_status: Set(2),
        transaction_id: Set(transaction_id),
        ..Default::default()
    }
    .save(db)
    .await?;
    Ok(())
}

pub async fn disable_admin_user(admin_uuid: String, uuid: String) -> AppResult<()> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let depot_query = SysUser::find_by_id(uuid).one(db).await?;
    if depot_query.clone().unwrap().role != 0 {
        return Err(anyhow::anyhow!("无权限").into());
    }
    let admin_query = SysUser::find_by_id(admin_uuid).one(db).await?;
    if admin_query.clone().unwrap().role == 0 {
        return Err(anyhow::anyhow!("超级管理员不可禁用").into());
    }
    let mut admin_model: sys_user::ActiveModel = admin_query.unwrap().clone().into();
    admin_model.user_status = Set(1);
    admin_model.update(db).await?;
    Ok(())
}

pub async fn disable_custom_user(custom_uuid: String, admin_uuid: String) -> AppResult<()> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let admin_query = SysUser::find_by_id(admin_uuid).one(db).await?;
    if admin_query.unwrap().role.clone() > 1 {
        return Err(anyhow::anyhow!("无权限").into());
    }
    let custom_query = CustomUser::find_by_id(custom_uuid).one(db).await?;
    let mut custom_model: custom_user::ActiveModel = custom_query.unwrap().clone().into();
    custom_model.user_status = Set(1);
    custom_model.update(db).await?;
    Ok(())
}

pub async fn change_profile(
    form_data: ChangeAdminProfileRequest,
    uuid: String,
) -> AppResult<SysUserProfileResponse> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let res = sys_user::ActiveModel {
        nick_name: Set(form_data.nick_name),
        email: Set(form_data.email),
        ..Default::default()
    }
    .update(db)
    .await?;
    Ok(SysUserProfileResponse {
        user_uuid: res.user_uuid,
        nick_name: res.nick_name,
        user_name: res.user_name,
        role: res.role,
        email: res.email,
        avatar_path: res.avatar_path,
    })
}

pub async fn get_user_profile(uuid: String) -> AppResult<SysUserProfileResponse> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let user_query = SysUser::find()
        .filter(sys_user::Column::UserUuid.eq(uuid))
        .one(db)
        .await?;
    let user_model = user_query.clone().unwrap();
    Ok(SysUserProfileResponse {
        user_uuid: user_model.user_uuid,
        nick_name: user_model.nick_name,
        user_name: user_model.user_name,
        email: user_model.email,
        role: user_model.role,
        avatar_path: user_model.avatar_path,
    })
}

pub async fn save_avatar(avatar_path: String, uuid: String) -> AppResult<SysUserProfileResponse> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let user_query = SysUser::find_by_id(uuid).one(db).await?;
    let mut user_model: sys_user::ActiveModel = user_query.unwrap().clone().into();
    user_model.avatar_path = Set(avatar_path);
    let user_res = user_model.update(db).await?;
    Ok(SysUserProfileResponse {
        user_uuid: user_res.user_uuid,
        nick_name: user_res.nick_name,
        user_name: user_res.user_name,
        role: user_res.role,
        email: user_res.email,
        avatar_path: user_res.avatar_path,
    })
}

pub async fn check_user_name(req: String) -> AppResult<()> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败"))?;
    let user_query = CustomUser::find()
        .filter(custom_user::Column::UserName.eq(req))
        .one(db)
        .await?;
    if user_query != None {
        return Err(anyhow::anyhow!("用户名已存在").into());
    }
    Ok(())
}

pub async fn change_admin_password(
    form_data: ChangeAdminPwdRequest,
    uuid: String,
) -> AppResult<()> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let user_query = SysUser::find_by_id(uuid).one(db).await?;
    let mut user_model: sys_user::ActiveModel = user_query.clone().unwrap().into();
    if let Err(err) =
        rand_utils::verify_password(form_data.old_pwd, user_query.unwrap().clone().user_pwd).await
    {
        return Err(anyhow::anyhow!("密码错误").into());
    }
    user_model.user_pwd = Set(rand_utils::hash_password(form_data.new_pwd).await.unwrap());
    user_model.update(db).await?;
    Ok(())
}

pub async fn create_admin_user(from_data: SysUserCrateRequest, uuid: String) -> AppResult<()> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    if from_data.role == 0 {
        return Err(anyhow::anyhow!("超级管理员无法创建其他超级管理员").into());
    }
    let user_query = SysUser::find_by_id(uuid).one(db).await?;
    if user_query.unwrap().role != 0 {
        return Err(anyhow::anyhow!("无权限").into());
    }
    sys_user::ActiveModel {
        user_uuid: Set(Uuid::new_v4().to_string()),
        nick_name: Set(from_data.nick_name),
        user_name: Set(from_data.user_name.clone()),
        role: Set(from_data.role),
        email: Set(from_data.email),
        user_status: Set(0),
        user_pwd: Set(rand_utils::hash_password(from_data.user_pwd).await.unwrap()),
        avatar_path: Set("../assets/avatar/default.png".to_string()),
    }
    .save(db)
    .await?;
    Ok(())
}

pub async fn login(form_data: SysLoginRequest) -> AppResult<SysLoginResponse> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let user_query = SysUser::find()
        .filter(sys_user::Column::UserName.eq(form_data.user_name))
        .one(db)
        .await?;
    let user_model = user_query.clone().unwrap();
    let user_model_clone = user_model.clone();
    if user_query.is_none() {
        return Err(anyhow::anyhow!("用户名错误").into());
    }
    if rand_utils::verify_password(form_data.user_pwd, user_model_clone.user_pwd)
        .await
        .is_err()
    {
        return Err(anyhow::anyhow!("密码错误").into());
    }
    if user_model.user_status != 0 {
        return Err(anyhow::anyhow!("用户被禁用").into());
    }

    let (token, exp) = get_token(
        user_model.user_name.clone(),
        user_model.user_uuid.clone(),
        Some(user_model.user_status),
        Some(user_model.role.clone()),
    )?;

    Ok(SysLoginResponse {
        user_uuid: user_model.user_uuid,
        user_name: user_model.user_name,
        token,
        exp,
    })
}
// 查看当前用户详情
pub async fn get_admin_profile(user_uuid: String) -> AppResult<SysUserProfileResponse> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let user_query = SysUser::find()
        .filter(sys_user::Column::UserUuid.eq(user_uuid))
        .one(db)
        .await?;
    let user_model = user_query.clone().unwrap();
    Ok(SysUserProfileResponse {
        user_uuid: user_model.user_uuid,
        nick_name: user_model.nick_name,
        user_name: user_model.user_name,
        role: user_model.role,
        email: user_model.email,
        avatar_path: user_model.avatar_path,
    })
}

pub async fn list_custom_user(uuid: String) -> AppResult<Vec<CustomUserProfileResponse>> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let user_query = SysUser::find()
        .filter(sys_user::Column::UserUuid.eq(uuid))
        .one(db)
        .await?;
    let user_model = user_query.clone().unwrap();
    if user_model.role != 0 && user_model.role != 1 {
        return Err(anyhow::anyhow!("无权限").into());
    }

    let redis_pool = get_redis_pool().await;
    let mut redis_client = redis_pool.get().await.unwrap();
    let _data: String = redis_client
        .get("custom_user_list")
        .await
        .unwrap_or("".to_string());
    let mut _result: Vec<CustomUserProfileResponse> = Vec::new();
    if _data.len() > 0 {
        _result =
            serde_json::from_str::<Vec<CustomUserProfileResponse>>(&_data).unwrap_or_default();
    }
    if _result.len() > 0 {
        return Ok(_result);
    }

    let custom_query = CustomUser::find().all(db).await?;
    let custom_res = custom_query
        .into_iter()
        .map(|x| CustomUserProfileResponse {
            user_uuid: x.user_uuid,
            nick_name: x.nick_name,
            user_name: x.user_name,
            balance_usdt: x.balance_usdt,
            email: x.email,
            avatar_path: x.avatar_path,
        })
        .collect::<Vec<_>>();

    let _: () = redis_client
        .set(
            "custom_user_list",
            serde_json::to_string(&custom_res).unwrap_or("".to_string()),
        )
        .await
        .unwrap_or_default();

    Ok(custom_res)
}

pub async fn list_admin_user(uuid: String) -> AppResult<Vec<SysUserProfileResponse>> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let user_query = SysUser::find()
        .filter(sys_user::Column::UserUuid.eq(uuid))
        .one(db)
        .await?;
    let user_model = user_query.clone().unwrap();
    if user_model.role.clone() != 0 {
        return Err(anyhow::anyhow!("无权限").into());
    }

    let redis_pool = get_redis_pool().await;
    let mut redis_client = redis_pool.get().await.unwrap();
    let _data: String = redis_client
        .get("admin_user_list")
        .await
        .unwrap_or("".to_string());
    let mut _result: Vec<SysUserProfileResponse> = Vec::new();
    if _data.len() > 0 {
        _result = serde_json::from_str::<Vec<SysUserProfileResponse>>(&_data).unwrap_or_default();
    }
    if _result.len() > 0 {
        return Ok(_result);
    }

    let admin_query = SysUser::find().all(db).await?;
    let admin_res = admin_query
        .into_iter()
        .map(|x| SysUserProfileResponse {
            user_uuid: x.user_uuid,
            nick_name: x.nick_name,
            user_name: x.user_name,
            role: x.role,
            email: x.email,
            avatar_path: x.avatar_path,
        })
        .collect::<Vec<_>>();

    let _: () = redis_client
        .set(
            "custom_user_list",
            serde_json::to_string(&admin_res).unwrap_or("".to_string()),
        )
        .await
        .unwrap_or_default();

    Ok(admin_res)
}
