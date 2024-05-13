use crate::{
    app_writer::AppResult,
    config::CFG,
    dtos::{
        count_data_dto::CountDataResponse,
        custom_user_dto::{CustomUserProfileResponse, RechargeOfAdminRequest},
        sys_user_dto::{
            ChangeAdminProfileRequest, ChangeAdminPwdRequest, SysLoginRequest, SysLoginResponse,
            SysUserCrateRequest, SysUserProfileResponse,
        },
        withdrawals_dto::WithdrawalsResponse,
    },
    entities::{
        count_data, custom_recharge, custom_user,
        prelude::{CountData, CustomRecharge, CustomUser, SysUser, Withdrawals},
        sys_user, withdrawals,
    },
    middleware::jwt::get_token,
    utils::{db::DB, rand_utils, redis_utils::*},
};
use chrono::{Local, Utc};
use sea_orm::*;
use serde_json;
use uuid::Uuid;

pub async fn super_admin_init() {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    // 查看是否已经初始化过
    let data_query = SysUser::find()
        .filter(sys_user::Column::UserName.eq("superadmin"))
        .one(db)
        .await;
    match data_query {
        Ok(Some(_model)) => {
            
            println!("超级管理员已初始化");
        }
        Ok(None) => {
            let user_pwd = "waqu2024".to_string();
            let hashed_pwd = match rand_utils::hash_password(user_pwd).await {
                Ok(pwd) => pwd,
                Err(err) => {
                    panic!("密码哈希失败: {:?}", err);
                }
            };
            // 创建超级管理员数据对象
            let new_super_admin = sys_user::ActiveModel {
                user_uuid: Set(Uuid::new_v4().to_string()),
                nick_name: Set("超级管理员".to_string()),
                user_name: Set("superadmin".to_string()),
                user_pwd: Set(hashed_pwd),
                balance: Set(0),
                liaison: Set("/t.me/bitpieok".to_string()),
                user_status: Set(0),
                role: Set(0),
                avatar_path: Set("../assets/avatar/default.png".to_string()),
            };

            // 创建超级管理员
            let _result = SysUser::insert(new_super_admin).exec(db).await;
            match _result {
                Ok(_) => println!("超级管理员初始化成功"),
                Err(err) => {
                    eprintln!("超级管理员初始化失败: {:?}", err);
                }
            }
        }
        Err(err) => {
            println!("数据库查询失败: {:?}", err)
        }
    }
}

// 获取计数数据
pub async fn get_history_data(_uuid: String) -> AppResult<CountDataResponse> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let admin_query = SysUser::find_by_id(&_uuid).one(db).await?;
    if admin_query.is_none() {
        return Err(anyhow::anyhow!("用户不存在").into());
    }
    let data_query = CountData::find().one(db).await?;
    let data_model = data_query.clone().unwrap();
    // 查询用户数量
    let user_query = CustomUser::find().all(db).await?;
    let count_custom: u64 = user_query.len().try_into().unwrap();
    // 判断计数数据中的数据是否正确
    if data_model.count_deal != count_custom {
        let mut data_model: count_data::ActiveModel = data_query.clone().unwrap().into();
        data_model.count_deal = Set(count_custom);
        data_model.update(db).await?;
    }
    Ok(CountDataResponse {
        count_deal: data_model.count_deal,
        count_recharge: data_model.count_recharge,
        count_withdraw: data_model.count_withdraw,
        count_custom: data_model.count_custom,
    })
}

// 处理取款申请
pub async fn post_withdraw_process(withdrawals_uuid: String, uuid: String) -> AppResult<()> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    // 查找未处理的取款记录
    let withdrawals_query = withdrawals::Entity::find_by_id(&withdrawals_uuid)
        .one(db)
        .await?;
    let count_quantities = withdrawals_query.clone().unwrap().quantities;
    let mut withdrawals_model: withdrawals::ActiveModel = withdrawals_query.clone().unwrap().into();
    withdrawals_model.status = Set(0);
    withdrawals_model.succes_date = Set(Some(Utc::now().naive_utc()));
    withdrawals_model.update(db).await?;
    // 将扣除的手续费存入superadmin的余额
    // 查询superadmin
    let super_admin_query = SysUser::find_by_id(&uuid).one(db).await?;
    // 判断是否是超级管理员
    if super_admin_query.clone().unwrap().role != 0 {
        return Err(anyhow::anyhow!("没有权限").into());
    }
    // 更新superadmin的余额
    let mut super_admin_model: sys_user::ActiveModel = super_admin_query.clone().unwrap().into();
    let super_admin_balance =
        super_admin_query.clone().unwrap().balance + withdrawals_query.unwrap().tariff;
    super_admin_model.balance = Set(super_admin_balance);
    super_admin_model.update(db).await?;
    // 更新取款金额计数
    let count_data_query = CountData::find().one(db).await?;
    let mut count_data_model: count_data::ActiveModel = count_data_query.clone().unwrap().into();
    count_data_model.count_withdraw =
        Set(count_data_query.unwrap().count_withdraw + count_quantities);
    count_data_model.update(db).await?;
    Ok(())
}

// 获取未处理的取款记录
pub async fn get_withdrawals_list_unprocessed(
    _uuid: String,
) -> AppResult<Vec<WithdrawalsResponse>> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    //获取当前用户信息
    let admin_query = SysUser::find_by_id(&_uuid).one(db).await?;
    if admin_query.is_none() {
        return Err(anyhow::anyhow!("用户不存在").into());
    } else if admin_query.clone().unwrap().role != 0 {
        return Err(anyhow::anyhow!("没有权限").into());
    }
    let query = withdrawals::Entity::find()
        .filter(withdrawals::Column::Status.eq(1))
        .all(db)
        .await?;
    let res = query
        .into_iter()
        .map(|item| WithdrawalsResponse {
            uuid: item.uuid,
            user_uuid: item.user_uuid,
            quantities: item.quantities,
            arrive: item.arrive,
            create_date: item.create_date,
            tariff: item.tariff,
            status: item.status,
            succes_date: item.succes_date,
        })
        .collect::<Vec<_>>();
    Ok(res)
}

// 获取当前用户的取款记录
pub async fn get_withdrawals_list(uuid: String) -> AppResult<Vec<WithdrawalsResponse>> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let query = Withdrawals::find()
        .filter(withdrawals::Column::UserUuid.eq(uuid))
        .all(db)
        .await;
    match query {
        Ok(res) => {
            let res = res
                .into_iter()
                .map(|item| WithdrawalsResponse {
                    uuid: item.uuid,
                    user_uuid: item.user_uuid,
                    quantities: item.quantities,
                    arrive: item.arrive,
                    create_date: item.create_date,
                    tariff: item.tariff,
                    status: item.status,
                    succes_date: item.succes_date,
                })
                .collect::<Vec<_>>();
            Ok(res)
        }
        Err(e) => Err(anyhow::anyhow!("{}", e).into()),
    }
}

// 取款申请
pub async fn post_withdrawals(req: u64, uuid: String) -> AppResult<()> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let admin_query = SysUser::find_by_id(&uuid).one(db).await?;
    let admin_model = admin_query.clone().unwrap();
    if admin_model.role == 0 {
        return Err(anyhow::anyhow!("超级管理员没有取款功能").into());
    }
    let tariff: f64;
    if req > 1000 {
        tariff = CFG.tariff.tariff_1000;
    } else if req > 100 {
        tariff = CFG.tariff.tariff_100;
    } else {
        tariff = 0.0;
    }

    let mut change_model: sys_user::ActiveModel = admin_model.clone().into();
    // 扣除取款金额
    // 判断取款数额是否超过余额
    if admin_model.balance < req {
        return Err(anyhow::anyhow!("余额不足").into());
    }
    let balance = admin_model.balance - req;
    // 计算手续费，向上取整
    let tariff_to = ((req as f64) * tariff).ceil() as u64;
    // 计算到账金额
    let aarrive = req - tariff_to;
    change_model.balance = Set(balance);
    change_model.update(db).await?;

    // 创建取款记录对象
    let withdrawal_model = withdrawals::ActiveModel {
        uuid: Set(Uuid::new_v4().to_string()),
        user_uuid: Set(uuid),
        quantities: Set(req),
        arrive: Set(aarrive),
        create_date: Set(Local::now().naive_utc()),
        tariff: Set(tariff_to),
        status: Set(1),
        ..Default::default()
    };
    // 保存取款记录
    let _result = Withdrawals::insert(withdrawal_model).exec(db).await?;
    Ok(())
    // withdrawals::ActiveModel {
    //     uuid: Set(Uuid::new_v4().to_string()),
    //     user_uuid: Set(uuid),
    //     quantities: Set(req),
    //     arrive: Set(aarrive),
    //     create_date: Set(Local::now().naive_utc()),
    //     tariff: Set(tariff_to.clone()),
    //     status: Set(1),
    //     ..Default::default()
    // }
    // .save(db)
    // .await?;
    // Ok(())
}

// 手动充值
pub async fn recharge_for_custom(
    from_data: RechargeOfAdminRequest,
    admin_uuid: String,
) -> AppResult<()> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let admin_query = SysUser::find_by_id(admin_uuid).one(db).await?;
    let _admin_model = admin_query.clone().ok_or(anyhow::anyhow!("用户不存在"));
    let custom_query = CustomUser::find_by_id(&from_data.user_uuid).one(db).await?;
    let usdt = from_data.balance_usdt + custom_query.clone().unwrap().balance_usdt;
    let mut custom_model: custom_user::ActiveModel = custom_query.clone().unwrap().into();
    custom_model.balance_usdt = Set(usdt);
    custom_model.update(db).await?;
    let transaction_id = format!("{}:{}", admin_query.unwrap().user_uuid, Uuid::new_v4(),);
    // 创建充值记录
    let new_recharge = custom_recharge::ActiveModel {
        record_uuid: Set(Uuid::new_v4().to_string()),
        user_uuid: Set(from_data.user_uuid),
        recharge_amount: Set(from_data.balance_usdt),
        payment_channel: Set("线下充值".to_string()),
        recharge_date: Set(Local::now().naive_utc()),
        recharge_status: Set(2),
        transaction_id: Set(transaction_id),
        ..Default::default()
    };
    // 保存充值记录
    let _result = CustomRecharge::insert(new_recharge).exec(db).await;
    // 确认插入是否成功
    if let Err(err) = &_result {
        return Err(anyhow::anyhow!("充值失败: {}", err).into());
    }
    // 更新充值金额计数
    let count_data_query = CountData::find().one(db).await?;
    // 检查查询结果是否为空
    let count_data_model = match count_data_query {
        Some(model) => model,
        None => {
            // 如果查询结果为空，返回错误
            return Err(anyhow::anyhow!("未找到充值计数信息").into());
        }
    };
    // 更新计数信息
    let updated_count = count_data_model.count_recharge + from_data.balance_usdt;
    let mut count_data_update: count_data::ActiveModel = count_data_model.into();
    count_data_update.count_recharge = Set(updated_count);
    count_data_update.update(db).await?;
    // 返回成功
    Ok(())
}

// 禁用管理员账号
pub async fn disable_admin_user(admin_uuid: String, token_uuid: String) -> AppResult<()> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let depot_query = SysUser::find_by_id(token_uuid).one(db).await?;
    let admin_query = SysUser::find_by_id(admin_uuid).one(db).await?;

    // 检查是否找到对应的用户
    let depot_model = depot_query.ok_or_else(|| anyhow::anyhow!("未找到对应的用户"))?;
    let admin_model = admin_query.ok_or_else(|| anyhow::anyhow!("未找到对应的管理员"))?;

    // 检查权限：只有管理员账号可以执行该操作
    if depot_model.role != 0 {
        return Err(anyhow::anyhow!("无权限").into());
    }

    // 检查是否是超级管理员账号
    if admin_model.role == 0 {
        return Err(anyhow::anyhow!("超级管理员不可禁用").into());
    }

    // 更新管理员账号状态为禁用
    let mut admin_model: sys_user::ActiveModel = admin_model.into();
    admin_model.user_status = Set(1);
    admin_model.update(db).await?;
    Ok(())
}

// 解禁管理员账号
pub async fn enable_admin_user(admin_uuid: String, token_uuid: String) -> AppResult<()> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let depot_query = SysUser::find_by_id(token_uuid).one(db).await?;
    let admin_query = SysUser::find_by_id(admin_uuid).one(db).await?;

    // 检查是否找到对应的用户
    let depot_model = depot_query.ok_or_else(|| anyhow::anyhow!("未找到对应的用户"))?;
    let admin_model = admin_query.ok_or_else(|| anyhow::anyhow!("未找到对应的管理员"))?;

    // 检查权限：只有管理员账号可以执行该操作
    if depot_model.role != 0 {
        return Err(anyhow::anyhow!("无权限").into());
    }

    // 更新管理员账号状态为启用
    let mut admin_model: sys_user::ActiveModel = admin_model.into();
    admin_model.user_status = Set(0);
    let _result = admin_model.update(db).await;
    // 错误处理
    if let Err(err) = _result {
        return Err(anyhow::anyhow!("解禁失败: {}", err).into());
    }

    Ok(())
}
// 禁用自定义用户
pub async fn disable_custom_user(custom_uuid: String, admin_uuid: String) -> AppResult<()> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    // 检查管理员权限
    let admin_query = SysUser::find_by_id(admin_uuid).one(db).await?;
    let admin_model = admin_query.ok_or_else(|| anyhow::anyhow!("未找到对应的管理员"))?;
    if admin_model.role > 1 {
        return Err(anyhow::anyhow!("无权限").into());
    }

    // 禁用自定义用户
    let custom_query = CustomUser::find_by_id(custom_uuid).one(db).await?;
    let custom_model = custom_query.ok_or_else(|| anyhow::anyhow!("未找到对应的自定义用户"))?;
    let mut custom_model: custom_user::ActiveModel = custom_model.into();
    custom_model.user_status = Set(1);
    // 更新自定义用户状态
    let _result = custom_model.update(db).await;
    //错误处理
    if let Err(err) = _result {
        return Err(anyhow::anyhow!("禁用失败: {}", err).into());
    }

    Ok(())
}

// 解禁自定义用户
pub async fn enable_custom_user(custom_uuid: String, admin_uuid: String) -> AppResult<()> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let admin_query = SysUser::find_by_id(admin_uuid).one(db).await?;
    if admin_query.unwrap().role > 1 {
        return Err(anyhow::anyhow!("无权限").into());
    }
    let custom_query = CustomUser::find_by_id(custom_uuid).one(db).await?;
    let mut custom_model: custom_user::ActiveModel = custom_query.unwrap().clone().into();
    custom_model.user_status = Set(0);
    // 更新自定义用户状态
    let _result = custom_model.update(db).await;
    // 错误处理
    if let Err(err) = _result {
        return Err(anyhow::anyhow!("解禁失败: {}", err).into());
    }

    Ok(())
}

// 更改当前用户信息
pub async fn change_profile(
    form_data: ChangeAdminProfileRequest,
    uuid: String,
) -> AppResult<SysUserProfileResponse> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    // 查询当前用户信息
    let user_query = SysUser::find_by_id(uuid).one(db).await?;
    let mut user_model: sys_user::ActiveModel = user_query.unwrap().clone().into();
    // 更改用户信息
    user_model.nick_name = Set(form_data.nick_name);
    user_model.liaison = Set(form_data.liaison);
    let _result = user_model.update(db).await;
    // 错误处理
    if let Err(err) = _result {
        return Err(anyhow::anyhow!("更改用户信息失败: {}", err).into());
    }
    // 查询更改后的信息
    let user_query = SysUser::find_by_id(_result.unwrap().user_uuid)
        .one(db)
        .await?;
    let user_res = user_query.unwrap();
    Ok(SysUserProfileResponse {
        user_uuid: user_res.user_uuid,
        nick_name: user_res.nick_name,
        user_name: user_res.user_name,
        role: user_res.role,
        liaison: user_res.liaison,
        balance: user_res.balance,
        avatar_path: user_res.avatar_path,
    })
}

// 保存头像
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
        liaison: user_res.liaison,
        balance: user_res.balance,
        avatar_path: user_res.avatar_path,
    })
}

// 校验用户名是否存在
pub async fn check_user_name(req: String) -> AppResult<()> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库连接失败"))?;
    let user_query = CustomUser::find()
        .filter(custom_user::Column::UserName.eq(req))
        .one(db)
        .await?;
    if user_query.is_some() {
        return Err(anyhow::anyhow!("用户名已存在").into());
    }
    Ok(())
}

// 更改当前用户密码
pub async fn change_admin_password(
    form_data: ChangeAdminPwdRequest,
    uuid: String,
) -> AppResult<()> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    let user_query = SysUser::find_by_id(uuid).one(db).await?;
    let mut user_model: sys_user::ActiveModel = user_query.clone().unwrap().into();
    if let Err(_err) =
        rand_utils::verify_password(form_data.old_pwd, user_query.unwrap().clone().user_pwd).await
    {
        return Err(anyhow::anyhow!("密码错误").into());
    }
    user_model.user_pwd = Set(rand_utils::hash_password(form_data.new_pwd).await.unwrap());
    user_model.update(db).await?;
    Ok(())
}

// 创建管理员或挂售个商
pub async fn create_admin_user(from_data: SysUserCrateRequest, uuid: String) -> AppResult<()> {
    let db = DB.get().ok_or("数据库连接失败").unwrap();
    if from_data.role == 0 {
        return Err(anyhow::anyhow!("超级管理员无法创建其他超级管理员").into());
    }
    let user_query = SysUser::find_by_id(uuid).one(db).await?;
    if user_query.unwrap().role != 0 {
        return Err(anyhow::anyhow!("无权限").into());
    }
    // 创建管理员对象
    let new_admin = sys_user::ActiveModel {
        user_uuid: Set(Uuid::new_v4().to_string()),
        nick_name: Set(from_data.nick_name),
        user_name: Set(from_data.user_name.clone()),
        role: Set(from_data.role),
        liaison: Set(from_data.liaison),
        balance: Set(Default::default()),
        user_status: Set(0),
        user_pwd: Set(rand_utils::hash_password(from_data.user_pwd).await.unwrap()),
        avatar_path: Set("../assets/avatar/default.png".to_string()),
    };
    // 将 数据保存到数据库
    let user_res = SysUser::insert(new_admin).exec(db).await;
    match user_res {
        Ok(_) => Ok(()),
        Err(err) => Err(anyhow::anyhow!("创建管理员失败:{}", err).into()),
    }
}
// 登陆
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
        Some(user_model.role),
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
    let user_query = match SysUser::find()
        .filter(sys_user::Column::UserUuid.eq(user_uuid))
        .one(db)
        .await
    {
        Ok(query) => query,
        Err(err) => return Err(anyhow::anyhow!(err).into()),
    };

    let user_model = match user_query {
        Some(model) => model,
        None => return Err(anyhow::anyhow!("用户不存在").into()),
    };
    Ok(SysUserProfileResponse {
        user_uuid: user_model.user_uuid,
        nick_name: user_model.nick_name,
        user_name: user_model.user_name,
        role: user_model.role,
        liaison: user_model.liaison,
        balance: user_model.balance,
        avatar_path: user_model.avatar_path,
    })
}

// 查看自定义用户列表
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
    if !_data.is_empty() {
        _result =
            serde_json::from_str::<Vec<CustomUserProfileResponse>>(&_data).unwrap_or_default();
    }
    if !_result.is_empty() {
        return Ok(_result);
    }

    let custom_query = CustomUser::find().all(db).await?;
    let custom_res = custom_query
        .into_iter()
        .map(|x| CustomUserProfileResponse {
            user_uuid: x.user_uuid,
            nick_name: x.nick_name,
            user_name: x.user_name,
            balance_usdt: x.balance_usdt.into(),
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
    if user_model.role != 0 {
        return Err(anyhow::anyhow!("无权限").into());
    }

    let redis_pool = get_redis_pool().await;
    let mut redis_client = redis_pool.get().await.unwrap();
    let _data: String = redis_client
        .get("admin_user_list")
        .await
        .unwrap_or("".to_string());
    let mut _result: Vec<SysUserProfileResponse> = Vec::new();
    if !_data.is_empty() {
        _result = serde_json::from_str::<Vec<SysUserProfileResponse>>(&_data).unwrap_or_default();
    }
    if !_result.is_empty() {
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
            liaison: x.liaison,
            balance: x.balance,
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
