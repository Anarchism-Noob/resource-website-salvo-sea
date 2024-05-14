use crate::{
    dtos::{
        custom_orders_dto::CustomOrderResponse,
        custom_user_dto::{
            BuyResourcetRequest, ChangePwdRequest, ChangeUserProfileRequest,
            CustomUserLoginRequest, CustomUserLoginResponse, CustomUserProfileResponse,
            CustomUserRegisterRequest, CustomUserResponse,
        },
    },
    middleware::*,
    services::custom_user_service,
    utils::{
        app_error::AppError,
        app_writer::{AppResult, AppWriter, ErrorResponseBuilder},
        captcha_utils::{generate_captcha, varify_captcha, CaptchaImage},
    },
};
use salvo::{
    http::{cookie::Cookie, StatusCode},
    oapi::{endpoint, extract::JsonBody},
    prelude::Json,
    Depot, Request, Response, Writer,
};
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[endpoint(tags("获取历史订单",))]
pub async fn get_orders(depot: &mut Depot) -> AppWriter<Vec<CustomOrderResponse>> {
    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    if let Err(err) = jwt::parse_token(token) {
        return AppError::AnyHow(err).into();
    }

    let jwt_model = jwt::parse_token(token).unwrap();
    let uuid = jwt_model.user_id;

    let result = custom_user_service::list_orders(uuid).await;
    AppWriter(result)
}

#[endpoint{tags ("购买资源"), }]
pub async fn put_buy_resource(
    form_data: JsonBody<BuyResourcetRequest>,
    depot: &mut Depot,
) -> AppWriter<CustomOrderResponse> {
    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    if let Err(err) = jwt::parse_token(token) {
        return AppError::AnyHow(err).into();
    }

    let jwt_model = jwt::parse_token(token).unwrap();
    let uuid = jwt_model.user_id;

    let model = form_data.0;
    let result = custom_user_service::buy_resource_request(model, uuid).await;
    AppWriter(result)
}

#[endpoint(tags("更改用户信息"))]
pub async fn put_change_profile(
    form_data: JsonBody<ChangeUserProfileRequest>,
    depot: &mut Depot,
) -> AppWriter<CustomUserProfileResponse> {
    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    if let Err(err) = jwt::parse_token(token) {
        return AppError::AnyHow(err).into();
    }

    let jwt_model = jwt::parse_token(token).unwrap();
    let uuid = jwt_model.user_id;

    let get_request = form_data.0;
    let result = custom_user_service::change_profile(get_request, uuid).await;
    AppWriter(result)
}

// 获取用户详细信息
#[endpoint(tags("获取用户详情"))]
pub async fn get_user_profile(depot: &mut Depot) -> AppWriter<CustomUserProfileResponse> {
    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    if let Err(err) = jwt::parse_token(token) {
        return AppError::AnyHow(err).into();
    }

    let jwt_model = jwt::parse_token(token).unwrap();
    let uuid = jwt_model.user_id;
    let _result = custom_user_service::get_user_profile(uuid).await;
    AppWriter(_result)
}

// 头像上传功能
#[endpoint(tags("将头像保存到服务器"))]
pub async fn put_upload_avatar(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    if let Err(err) = jwt::parse_token(token) {
        return ErrorResponseBuilder::with_err(AppError::AnyHow(err)).into_response(res);
    }

    let jwt_model = jwt::parse_token(token).unwrap();
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

            let info = if let Err(e) = std::fs::copy(file.path(), &dest) {
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                format!("file not found in request: {}", e)
            } else {
                format!("{:?}", dest)
            };

            let _result =
                custom_user_service::save_avatar(dest.to_str().unwrap_or("").to_string(), uuid)
                    .await;
            res.render(Json(info));
        }
    } else {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Json("file not found in request"));
    }
}

#[endpoint(tags("获取验证码"))]
pub async fn get_captcha(req: &mut Request, res: &mut Response) {
    // 从查询参数中获取验证码类型
    let captcha_type = req.query::<String>("captchaType").unwrap_or_default();
    // 生成验证码
    let captcha_result: AppResult<CaptchaImage> = generate_captcha(&captcha_type).await;
    match captcha_result {
        Ok(captcha) => {
            res.render(Json(captcha));
        }
        Err(err) => {
            ErrorResponseBuilder::with_err(err).into_response(res);
        }
    }
}

#[endpoint(tags("用户登录"))]
pub async fn post_login(form_data: JsonBody<CustomUserLoginRequest>, res: &mut Response) {
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

        let result: AppResult<CustomUserLoginResponse> =
            custom_user_service::login(form_data.0).await;
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

#[endpoint(tags("更改密码"),
parameters(
    ("user_uuid", description = "用户uuid"),
))]
pub async fn put_change_password(
    req: JsonBody<ChangePwdRequest>,
    depot: &mut Depot,
    res: &mut Response,
) {
    let token = depot.get::<&str>("jwt_token").copied().unwrap();

    if let Err(err) = jwt::parse_token(token) {
        return ErrorResponseBuilder::with_err(AppError::AnyHow(err)).into_response(res);
    }
    let jwt_model = jwt::parse_token(token).unwrap();
    let uuid = jwt_model.user_id;

    let user_req = req.0;
    let result = custom_user_service::change_password(user_req, uuid).await;
    match result {
        Ok(_data) => {
            res.remove_cookie("jwt_token");
        }
        Err(err) => ErrorResponseBuilder::with_err(err).into_response(res),
    }
}

#[endpoint(tags("创建用户"))]
pub async fn post_register(
    new_user: JsonBody<CustomUserRegisterRequest>,
) -> AppWriter<CustomUserResponse> {
    let captcha_str = new_user.code.clone().unwrap_or_default();
    let captcha_uuid = new_user.captcha_uuid.clone().unwrap_or_default();

    match varify_captcha("registry".to_string(), captcha_uuid, captcha_str).await {
        Err(err) => err.into(),
        Ok(_) => {
            let user_model = new_user.0;
            if let Err(err) =
                custom_user_service::check_user_name(user_model.user_name.clone()).await
            {
                return err.into();
            }
            let result = custom_user_service::registry(user_model).await;
            AppWriter(result)
        }
    }
}
