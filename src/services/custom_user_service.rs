use crate::{
    app_writer::AppResult,
    dtos::{
        custom_orders_dto::{CustomOrderCrateRequest, CustomOrderResponse},
        custom_user_dto::{
            BuyResourcetRequest, ChangePwdRequest, ChangeUserProfileRequest,
            CustomUserLoginRequest, CustomUserLoginResponse, CustomUserProfileResponse,
            CustomUserRegisterRequest, CustomUserResponse,
        },
    },
    entities::{
        custom_orders, custom_user,
        prelude::{CustomOrders, CustomUser, SysResources},
        sys_resources,
    },
    middleware::jwt::get_token,
    utils::{db::DB, rand_utils, redis_utils::*},
};
use chrono::{Local, NaiveDate, Utc};
use sea_orm::*;
use uuid::Uuid;

pub async fn save_avatar(
    avatar_path: String,
    uuid: String,
) -> AppResult<CustomUserProfileResponse> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败"))?;
    let user_query = CustomUser::find_by_id(uuid.clone()).one(db).await?;
    let mut user_model: custom_user::ActiveModel = user_query.unwrap().into();
    user_model.avatar_path = Set(avatar_path.clone());
    let model_res: custom_user::Model = user_model.update(db).await?;
    Ok(CustomUserProfileResponse {
        user_uuid: uuid,
        nick_name: model_res.nick_name,
        user_name: model_res.user_name,
        email: model_res.email,
        balance_usdt: model_res.balance_usdt,
        avatar_path: model_res.avatar_path,
    })
}

pub async fn list_orders(user_uuid: String) -> AppResult<Vec<CustomOrderResponse>> {
    let redis_pool = get_redis_pool().await;
    let mut redis_client = redis_pool.get().await.unwrap();
    let _data: String = redis_client
        .get("order_list")
        .await
        .unwrap_or("".to_string());
    let mut _result: Vec<CustomOrderResponse> = Vec::new();
    if _data.len() > 0 {
        _result = serde_json::from_str::<Vec<CustomOrderResponse>>(&_data).unwrap_or_default();
    }
    if _result.len() > 0 {
        return Ok(_result);
    }

    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败"))?;
    let vec_orders = CustomOrders::find().all(db).await?;
    let orders_res = vec_orders
        .into_iter()
        .map(|order| CustomOrderResponse {
            order_uuid: order.order_uuid,
            resource_uuid: order.resource_uuid,
            resource_name: order.resource_name,
            resource_category: order.resource_category,
            resource_language: order.resource_language,
            download_link: order.download_link,
            order_resource_price: order.order_resource_price,
            creation_date: order.creation_date,
        })
        .collect::<Vec<_>>();

    let _: () = redis_client
        .set(
            "order_list",
            serde_json::to_string(&orders_res).unwrap_or_default(),
        )
        .await
        .unwrap_or_default();

    Ok(orders_res)
}

pub async fn change_profile(
    form_data: ChangeUserProfileRequest,
    uuid: String,
) -> AppResult<CustomUserProfileResponse> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败"))?;
    let user_query = CustomUser::find_by_id(uuid.clone()).one(db).await?;
    if user_query.is_none() {
        return Err(anyhow::anyhow!("用户不存在").into());
    }
    let user = custom_user::ActiveModel {
        nick_name: Set(form_data.nick_name.clone()),
        email: Set(form_data.email.clone()),
        ..Default::default()
    }
    .update(db)
    .await?;
    Ok(CustomUserProfileResponse {
        user_uuid: user.user_uuid,
        nick_name: user.nick_name,
        user_name: user.user_name,
        email: user.email,
        balance_usdt: user.balance_usdt,
        avatar_path: user.avatar_path,
    })
}

pub async fn get_user_profile(user_uuid: String) -> AppResult<CustomUserProfileResponse> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败"))?;
    let user_query = CustomUser::find_by_id(user_uuid).one(db).await?;
    let user_res = user_query.unwrap();
    Ok(CustomUserProfileResponse {
        user_uuid: user_res.user_uuid,
        nick_name: user_res.nick_name,
        user_name: user_res.user_name,
        email: user_res.email,
        balance_usdt: user_res.balance_usdt,
        avatar_path: user_res.avatar_path,
    })
}

pub async fn buy_resource_request(
    req: BuyResourcetRequest,
    uuid: String,
) -> AppResult<CustomOrderResponse> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败"))?;
    let order_result = CustomOrders::find()
        .join_rev(
            JoinType::LeftJoin,
            custom_user::Entity::belongs_to(custom_orders::Entity)
                .from(custom_user::Column::UserUuid)
                .to(custom_orders::Column::UserUuid)
                .into(),
        )
        .filter(custom_user::Column::UserUuid.eq(uuid.clone()))
        .filter(custom_orders::Column::ResourceUuid.eq(req.resource_uuid.clone()))
        .all(db)
        .await
        .unwrap();
    if order_result.len() > 0 {
        return Err(anyhow::anyhow!("资源已购买").into());
    }
    let resource_result = SysResources::find()
        .filter(sys_resources::Column::ResourceUuid.eq(req.resource_uuid.clone()))
        .one(db)
        .await?;
    let user_result = CustomUser::find()
        .filter(custom_user::Column::UserUuid.eq(uuid.clone()))
        .one(db)
        .await?;
    let user_model = user_result.unwrap();
    let resource_model = resource_result.unwrap();

    if user_model.balance_usdt < resource_model.resource_price {
        return Err(anyhow::anyhow!("余额不足").into());
    }
    custom_orders::ActiveModel {
        order_uuid: Set(Uuid::new_v4().to_string()),
        user_uuid: Set(user_model.user_uuid),
        resource_uuid: Set(resource_model.resource_uuid),
        resource_name: Set(resource_model.resource_name),
        resource_category: Set(resource_model.category),
        resource_language: Set(resource_model.language),
        download_link: Set(resource_model.resource_link),
        order_resource_price: Set(resource_model.resource_price),
        creation_date: Set(Local::now().naive_local()),
    }
    .save(db)
    .await?;
    let order_query = CustomOrders::find()
        .filter(custom_orders::Column::UserUuid.eq(uuid))
        .filter(custom_orders::Column::ResourceUuid.eq(req.resource_uuid))
        .one(db)
        .await?;
    let order_res = order_query.unwrap();
    Ok(CustomOrderResponse {
        order_uuid: order_res.order_uuid.to_string(),
        resource_uuid: order_res.resource_uuid.to_string(),
        resource_name: order_res.resource_name.to_string(),
        resource_category: order_res.resource_category.to_string(),
        resource_language: order_res.resource_language.to_string(),
        download_link: order_res.download_link.to_string(),
        order_resource_price: order_res.order_resource_price,
        creation_date: order_res.creation_date,
    })
}

pub async fn change_password(req: ChangePwdRequest, uuid: String) -> AppResult<()> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败"))?;
    let user_query = CustomUser::find()
        .filter(custom_user::Column::UserUuid.eq(uuid.clone()))
        .one(db)
        .await?;
    if user_query.is_none() {
        return Err(anyhow::anyhow!("用户不存在").into());
    }
    let mut user: custom_user::ActiveModel = user_query.clone().unwrap().into();
    if let Err(_err) =
        rand_utils::verify_password(req.user_pwd.clone(), user_query.unwrap().clone().user_pwd)
            .await
    {
        return Err(anyhow::anyhow!("密码错误").into());
    }
    user.user_pwd = Set(rand_utils::hash_password(req.user_pwd.clone()).await?);
    user.update(db).await?;
    Ok(())
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

pub async fn registry(req: CustomUserRegisterRequest) -> AppResult<CustomUserResponse> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败"))?;
    custom_user::ActiveModel {
        user_uuid: Set(Uuid::new_v4().to_string()),
        nick_name: Set(req.nick_name.clone()),
        user_name: Set(req.user_name.clone()),
        user_pwd: Set(rand_utils::hash_password(req.user_pwd).await?),
        email: Set(Option::from(req.email.unwrap())),
        user_status: Set(0),
        balance_usdt: Set(Default::default()),
        registration_date: Set(Local::now().naive_local()),
        avatar_path: Set("../assets/uploads/avatar/default.png".to_string()),
    }
    .save(db)
    .await?;
    let user_query = CustomUser::find()
        .filter(custom_user::Column::UserName.eq(req.user_name.clone()))
        .one(db)
        .await?;
    let user_model = user_query.unwrap();
    Ok(CustomUserResponse {
        user_uuid: user_model.user_uuid,
        nick_name: user_model.nick_name.clone(),
        user_name: user_model.user_name.clone(),
    })
}

pub async fn login(req: CustomUserLoginRequest) -> AppResult<CustomUserLoginResponse> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败"))?;
    let user_query = CustomUser::find()
        .filter(custom_user::Column::UserName.eq(req.user_name))
        .one(db)
        .await?;
    if user_query.is_none() {
        return Err(anyhow::anyhow!("用户名错误").into());
    }
    let user = user_query.unwrap();
    let user_clone = user.clone();
    if rand_utils::verify_password(req.user_pwd.unwrap(), user_clone.user_pwd)
        .await
        .is_err()
    {
        return Err(anyhow::anyhow!("密码错误").into());
    }
    if user_clone.user_status != 0 {
        return Err(anyhow::anyhow!("用户已禁用").into());
    }
    let (token, exp) = get_token(
        user.user_name.clone(),
        user.user_uuid.clone(),
        Some(user.user_status),
        None,
    )?;

    let res = CustomUserLoginResponse {
        user_uuid: user.user_uuid,
        user_name: user.user_name,
        token,
        exp,
    };
    Ok(res)
}
