use crate::{
    app_writer::{AppResult, AppWriter, ErrorResponseBuilder},
    dtos::{
        custom_orders_dto::CustomOrderResponse,
        custom_user_dto::{CustomUserProfileResponse, RechargeOfAdminRequest},
        sys_user_dto::{
            ChangeAdminProfileRequest, ChangeAdminPwdRequest, SysLoginRequest, SysLoginResponse,
            SysUserCrateRequest, SysUserProfileResponse,
        },
        withdrawals_dto::WithdrawalsResponse,
    },
    middleware::jwt::{self, JwtClaims},
    services::admin_user_service,
    utils::{
        app_error::AppError,
        captcha_utils::{generate_captcha, varify_captcha, CaptchaImage},
        redis_utils::get_redis_pool,
    },
};
use salvo::{
    http::{cookie::Cookie, ParseError, StatusCode},
    oapi::{
        endpoint,
        extract::{JsonBody, PathParam},
    },
    prelude::*,
    Request, Response, Writer,
};
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};
use uuid::Uuid;

#[endpoint(tags("处理取款申请"))]
pub async fn put_withdraw_process(req: JsonBody<String>, depot: &mut Depot) -> AppWriter<()> {
    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    if let Err(err) = jwt::parse_token(&token) {
        return AppError::AnyHow(err).into();
    }
    let jwt_model = jwt::parse_token(&token).unwrap();
    let uuid = jwt_model.user_id;
    let _result = admin_user_service::post_withdraw_process(req.0, uuid).await;
    AppWriter(_result)
}

#[endpoint(tags("获取未处理的取款记录"))]
pub async fn get_withdraw_list_unprocessed(
    depot: &mut Depot,
) -> AppWriter<Vec<WithdrawalsResponse>> {
    let token = depot.get::<&str>("jwt_token").copied().unwrap();
    if let Err(err) = jwt::parse_token(&token) {
        return AppError::AnyHow(err).into();
    }
    let jwt_model = jwt::parse_token(&token).unwrap();
    let uuid = jwt_model.user_id;
    let _result = admin_user_service::get_withdrawals_list_unprocessed(uuid).await;
    AppWriter(_result)
}

#[endpoint(tags("获取当前用户的取款记录"))]
pub async fn get_withdraw_list(depot: &mut Depot) -> AppWriter<Vec<WithdrawalsResponse>> {
    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    if let Err(err) = jwt::parse_token(&token) {
        return AppError::AnyHow(err).into();
    }
    let jwt_model = jwt::parse_token(&token).unwrap();
    let uuid = jwt_model.user_id;
    let _result = admin_user_service::get_withdrawals_list(uuid).await;
    AppWriter(_result)
}

#[endpoint(tags("取款申请"))]
pub async fn put_withdraw(req: JsonBody<u64>, depot: &mut Depot) -> AppWriter<()> {
    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    if let Err(err) = jwt::parse_token(&token) {
        return AppError::AnyHow(err).into();
    }
    let jwt_model = jwt::parse_token(&token).unwrap();
    let uuid = jwt_model.user_id;
    let _result = admin_user_service::post_withdrawals(req.0, uuid).await;
    AppWriter(_result)
}

#[endpoint(tags("手动充值"))]
pub async fn put_recharge(
    form_data: JsonBody<RechargeOfAdminRequest>,
    depot: &mut Depot,
) -> AppWriter<()> {
    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    if let Err(err) = jwt::parse_token(&token) {
        return AppError::AnyHow(err).into();
    }
    let jwt_model = jwt::parse_token(&token).unwrap();
    let uuid = jwt_model.user_id;
    let get_request = form_data.0;
    let result = admin_user_service::recharge_for_custom(get_request, uuid).await;
    AppWriter(result)
}

#[endpoint(tags("禁用admin账号"))]
pub async fn disable_admin(admin_uuid: PathParam<String>, depot: &mut Depot) -> AppWriter<()> {
    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    if let Err(err) = jwt::parse_token(&token) {
        return AppError::AnyHow(err).into();
    }
    let jwt_model = jwt::parse_token(&token).unwrap();
    let uuid = jwt_model.user_id;
    let _result = admin_user_service::disable_admin_user(admin_uuid.0, uuid).await;
    AppWriter(_result)
}

#[endpoint(tags("禁用custom账号"))]
pub async fn disable_custom(custom_uuid: PathParam<String>, depot: &mut Depot) -> AppWriter<()> {
    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    if let Err(err) = jwt::parse_token(&token) {
        return AppError::AnyHow(err).into();
    }

    let jwt_model = jwt::parse_token(&token).unwrap();
    let uuid = jwt_model.user_id;
    let _result = admin_user_service::disable_custom_user(custom_uuid.0, uuid).await;
    AppWriter(_result)
}

#[endpoint(tags("获取管理员列表"))]
pub async fn get_admin_list(depot: &mut Depot) -> AppWriter<Vec<SysUserProfileResponse>> {
    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    if let Err(err) = jwt::parse_token(&token) {
        return AppError::AnyHow(err).into();
    }

    let jwt_model = jwt::parse_token(&token).unwrap();
    let uuid = jwt_model.user_id;
    let _result = admin_user_service::list_admin_user(uuid).await;
    AppWriter(_result)
}

#[endpoint(tags("获取用户列表"))]
pub async fn get_custom_list(depot: &mut Depot) -> AppWriter<Vec<CustomUserProfileResponse>> {
    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    if let Err(err) = jwt::parse_token(&token) {
        return AppError::AnyHow(err).into();
    }

    let jwt_model = jwt::parse_token(&token).unwrap();
    let uuid = jwt_model.user_id;
    let _result = admin_user_service::list_custom_user(uuid).await;
    AppWriter(_result)
}

#[endpoint(tags("更改当前用户信息"))]
pub async fn put_change_profile(
    form_data: JsonBody<ChangeAdminProfileRequest>,
    depot: &mut Depot,
) -> AppWriter<SysUserProfileResponse> {
    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    if let Err(err) = jwt::parse_token(&token) {
        return AppError::AnyHow(err).into();
    }

    let jwt_model = jwt::parse_token(&token).unwrap();
    let uuid = jwt_model.user_id;

    let get_request = form_data.0;
    let result = admin_user_service::change_profile(get_request, uuid).await;
    AppWriter(result)
}

// 获取用户详细信息
#[endpoint(tags("获取当前用户详情"))]
pub async fn get_user_profile(depot: &mut Depot) -> AppWriter<SysUserProfileResponse> {
    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    if let Err(err) = jwt::parse_token(&token) {
        return AppError::AnyHow(err).into();
    }

    let jwt_model = jwt::parse_token(&token).unwrap();
    let uuid = jwt_model.user_id;
    let _result = admin_user_service::get_admin_profile(uuid).await;
    AppWriter(_result)
}

// 头像上传功能
#[endpoint(tags("将头像保存到服务器"))]
pub async fn put_upload_avatar(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    if let Err(err) = jwt::parse_token(&token) {
        return ErrorResponseBuilder::with_err(AppError::AnyHow(err)).into_response(res);
    }

    let jwt_model = jwt::parse_token(&token).unwrap();
    let uuid = jwt_model.user_id;

    // 创建一个uploads目录，用于保存上传的文件
    let file = req.file("avatar").await;
    if let Some(file) = file {
        let mime = file.content_type().unwrap().to_string();
        if mime.starts_with("image/") {
            let file_name = Uuid::new_v4().to_string();
            let mut dest = PathBuf::from("../assets/uploads/avatar/");

            // 提取原始文件名和扩展名
            let original_file_name = file.name().unwrap_or("file");
            let extension = Path::new(original_file_name)
                .extension()
                .unwrap_or_default();

            // 构建新的文件名（保留原始文件的扩展名）
            dest.push(format!(
                "{}.{}",
                file_name,
                extension.to_str().unwrap_or("png")
            ));

            let info = if let Err(e) = std::fs::copy(&file.path(), &dest) {
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                format!("file not found in request: {}", e)
            } else {
                format!("{:?}", dest)
            };

            let _result =
                admin_user_service::save_avatar(dest.to_str().unwrap_or("").to_string(), uuid)
                    .await;
            res.render(Json(info));
        }
    } else {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Json("file not found in request"));
    }
}

#[endpoint(tags("更改当前用户密码"))]
pub async fn put_change_password(
    req: JsonBody<ChangeAdminPwdRequest>,
    depot: &mut Depot,
    res: &mut Response,
) {
    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    if let Err(err) = jwt::parse_token(&token) {
        return ErrorResponseBuilder::with_err(AppError::AnyHow(err)).into_response(res);
    }
    let jwt_model = jwt::parse_token(&token).unwrap();
    let uuid = jwt_model.user_id;

    let user_req = req.0;
    let result = admin_user_service::change_admin_password(user_req, uuid).await;
    match result {
        Ok(_data) => {
            res.remove_cookie("jwt_token");
        }
        Err(err) => ErrorResponseBuilder::with_err(err).into_response(res),
    }
}

#[endpoint(tags("获取验证码"))]
pub async fn get_captcha(req: &mut Request, res: &mut Response) {
    // 生成验证码
    let captcha_result: AppResult<CaptchaImage> =
        generate_captcha(&req.param::<String>("captchaType").unwrap_or_default()).await;
    match captcha_result {
        Ok(captcha) => {
            res.render(Json(captcha));
        }
        Err(err) => {
            ErrorResponseBuilder::with_err(err).into_response(res);
        }
    }
}

#[endpoint(tags("管理员登录"))]
pub async fn post_login(form_data: JsonBody<SysLoginRequest>, res: &mut Response) {
    if let Some(captcha_str) = form_data.code.clone() {
        if let Err(err) = varify_captcha(
            "login".to_string(),
            form_data.captcha_uuid.clone().unwrap(),
            captcha_str.clone(),
        )
        .await
        {
            return ErrorResponseBuilder::with_err(err).into_response(res);
        }

        let result: AppResult<SysLoginResponse> = admin_user_service::login(form_data.0).await;
        match result {
            Ok(data) => {
                let jwt_token = data.token.clone();
                let cookie = Cookie::build(("jwt_token", jwt_token))
                    .path("/")
                    .http_only(true)
                    .build();
                res.add_cookie(cookie);
            }
            Err(err) => ErrorResponseBuilder::with_err(err).into_response(res),
        }
    }
}

#[endpoint(tags("创建管理员"))]
pub async fn post_register_admin(
    form_data: JsonBody<SysUserCrateRequest>,
    depot: &mut Depot,
    res: &mut Response,
) {
    let _model = form_data.0;
    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    if let Err(err) = jwt::parse_token(&token) {
        return ErrorResponseBuilder::with_err(AppError::AnyHow(err.into())).into_response(res);
    }

    let jwt_model = jwt::parse_token(&token).unwrap();
    let uuid = jwt_model.user_id;

    if let Err(err) = admin_user_service::check_user_name(_model.user_name.clone()).await {
        return ErrorResponseBuilder::with_err(AppError::AnyHow(err.into())).into_response(res);
    }
    if let Err(err) = admin_user_service::create_admin_user(_model, uuid).await {
        return ErrorResponseBuilder::with_err(AppError::AnyHow(err.into())).into_response(res);
    }
    res.render(Json("success".to_string()));
}
