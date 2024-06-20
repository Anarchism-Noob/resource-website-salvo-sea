use crate::{
    app_writer::{AppResult, AppWriter, ErrorResponseBuilder},
    dtos::{
        count_data_dto::CountDataResponse,
        custom_user_dto::{CustomUserProfileResponse, RechargeOfAdminRequest},
        query_struct::BodyStructOfDE,
        sys_menus_dto::MenuListResponse,
        sys_user_dto::{
            BaseResponse, ChangeAdminProfileRequest, ChangeAdminPwdRequest, SysLoginRequest, SysLoginResponse, SysUserCrateRequest, SysUserProfileResponse
        },
        withdrawals_dto::WithdrawalsResponse,
    },
    middleware::jwt::JwtClaims,
    services::{admin_user_service, sys_menu_service},
    utils::app_error::AppError,
};
use salvo::{
    http::StatusCode,
    oapi::{endpoint, extract::JsonBody},
    prelude::*,
    Request, Response, Writer,
};
use std::path::{Path, PathBuf};
use uuid::Uuid;


// 用于将解码后的认证数据插入到 depot 的键。
pub const JWT_AUTH_DATA_KEY: &str = "::salvo::Jwt_auth::auth_data";
// 用于将认证状态数据插入到 depot 的键。
pub const JWT_AUTH_STATE_KEY: &str = "::salvo::jwt_auth::auth_state";
// 用于将认证令牌数据插入到 depot 的键。
pub const JWT_AUTH_TOKEN_KEY: &str ="::salvo::jwt_auth::auth_token";
// 用于将认证错误插入到 depot 的键。
pub const JWT_AUTH_ERROR_KEY: &str ="::salvo::jwt_auth::auth_error";


#[endpoint(tags("获取菜单列表"))]
pub async fn get_menu(depot: &mut Depot) -> AppWriter<Vec<MenuListResponse>> {
    // dbg!(depot);
    let jwt_model = depot.jwt_auth_data::<JwtClaims>().unwrap();
    dbg!(jwt_model);
    let uuid = &jwt_model.claims.user_id;
    //
    match sys_menu_service::get_menu_list(uuid).await {
        Ok(result) => AppWriter(Ok(result)),
        Err(err) => {
            AppWriter(Err(err))
        }
    }
    // let _result = sys_menu_service::get_menu_list(uuid).await;
    // return AppWriter(_result);
}
 
#[endpoint(tags("获取历史数据"))]
pub async fn get_history_data(depot: &mut Depot) -> AppWriter<CountDataResponse> {
    // dbg!(depot);
    let jwt_model = depot.jwt_auth_data::<JwtClaims>().unwrap();
    dbg!(jwt_model);
    let uuid = &jwt_model.claims.user_id;
    // 
    match admin_user_service::get_history_data(uuid).await {
        Ok(result) => AppWriter(Ok(result)),
        Err(err) => {
            AppWriter(Err(err))
        }
    }
    // let _result = admin_user_service::get_history_data(uuid).await;
    // AppWriter(_result)
}

#[endpoint(tags("处理取款申请"))]
pub async fn put_process(req: JsonBody<String>, depot: &mut Depot) -> AppWriter<BaseResponse> {
    // dbg!(depot);
    let jwt_model = depot.jwt_auth_data::<JwtClaims>().unwrap();
    dbg!(jwt_model);
    let uuid = &jwt_model.claims.user_id;

    let withdraw_uuid = req.0;
    match admin_user_service::post_withdraw_process(withdraw_uuid, uuid).await {
        Ok(result) =>{ 
            let resp = BaseResponse {
                code: 200,
                msg: "处理成功".to_string(),
                data: None,
            };
            AppWriter(Ok(resp))
        },
        Err(err) => {
            // let resp = BaseResponse {
            //     code: 500,
            //     msg: "处理失败".to_string(),
            //     data: None,
            // };
            AppWriter(Err(err))
        }
    }
    // let _result = admin_user_service::post_withdraw_process(withdraw_uuid, uuid).await;
    // AppWriter(_result)
}

#[endpoint(tags("获取未处理的取款记录"))]
pub async fn all_unprocessed(depot: &mut Depot) -> AppWriter<Vec<WithdrawalsResponse>> {
    // dbg!(depot);
    let jwt_model = depot.jwt_auth_data::<JwtClaims>().unwrap();
    dbg!(jwt_model);
    let uuid = &jwt_model.claims.user_id;
    //
    let _result = admin_user_service::get_withdrawals_list_unprocessed(uuid).await;
    AppWriter(_result)
}

#[endpoint(tags("获取当前用户的取款记录"))]
pub async fn all_withdraw(depot: &mut Depot) -> AppWriter<Vec<WithdrawalsResponse>> {
    // dbg!(depot);
    let jwt_model = depot.jwt_auth_data::<JwtClaims>().unwrap();
    dbg!(jwt_model);
    let uuid = &jwt_model.claims.user_id;
    //
    let _result = admin_user_service::get_withdrawals_list(uuid).await;
    AppWriter(_result)
}

#[endpoint(tags("取款申请"))]
pub async fn put_withdraw(req: JsonBody<u64>, depot: &mut Depot) -> AppWriter<()> {
    // dbg!(depot);
    let jwt_model = depot.jwt_auth_data::<JwtClaims>().unwrap();
    dbg!(jwt_model);
    let uuid = &jwt_model.claims.user_id;
    // 
    let _result = admin_user_service::post_withdrawals(req.0, uuid).await;
    AppWriter(_result)
}

#[endpoint(tags("手动充值"))]
pub async fn put_recharge(
    form_data: JsonBody<RechargeOfAdminRequest>,
    depot: &mut Depot,
) -> AppWriter<()> {
    // dbg!(depot);
    let jwt_model = depot.jwt_auth_data::<JwtClaims>().unwrap();
    dbg!(jwt_model);
    let uuid = &jwt_model.claims.user_id;
    //
    let get_request = form_data.0;
    let result = admin_user_service::recharge_for_custom(get_request, uuid).await;
    AppWriter(result)
}

#[endpoint(tags("禁用admin账号"))]
pub async fn disable_admin(
    admin_uuid: JsonBody<BodyStructOfDE>,
    depot: &mut Depot,
) -> AppWriter<()> {
    // dbg!(depot);
    let jwt_model = depot.jwt_auth_data::<JwtClaims>().unwrap();
    dbg!(jwt_model);
    let uuid = &jwt_model.claims.user_id;
    // 获取被禁用账号的uuid
    let disable_uuid = admin_uuid.d_e_uuid.clone();
    let _result = admin_user_service::disable_admin_user(disable_uuid.unwrap(), uuid).await;
    AppWriter(_result)
}

#[endpoint(tags("启用admin账号"))]
pub async fn enable_admin(
    admin_uuid: JsonBody<BodyStructOfDE>,
    depot: &mut Depot,
) -> AppWriter<()> {
    // dbg!(depot);
    let jwt_model = depot.jwt_auth_data::<JwtClaims>().unwrap();
    dbg!(jwt_model);
    let uuid = &jwt_model.claims.user_id;
    // 获取被启用用账号的uuid
    let enable_uuid = admin_uuid.d_e_uuid.clone();

    let _result = admin_user_service::enable_admin_user(enable_uuid.unwrap(), uuid).await;
    AppWriter(_result)
}

#[endpoint(tags("禁用custom账号"))]
pub async fn disable_custom(
    custom_uuid: JsonBody<BodyStructOfDE>,
    depot: &mut Depot,
) -> AppWriter<()> {
    // dbg!(depot);
    let jwt_model = depot.jwt_auth_data::<JwtClaims>().unwrap();
    dbg!(jwt_model);
    let uuid = &jwt_model.claims.user_id;
    //
    let disable_uuid = custom_uuid.d_e_uuid.clone();

    let _result = admin_user_service::disable_custom_user(disable_uuid.unwrap(), uuid).await;
    AppWriter(_result)
}

#[endpoint(tags("启用custom账号"))]
pub async fn enable_custom(
    custom_uuid: JsonBody<BodyStructOfDE>,
    depot: &mut Depot,
) -> AppWriter<()> {
    // dbg!(depot);
    let jwt_model = depot.jwt_auth_data::<JwtClaims>().unwrap();
    dbg!(jwt_model);
    let uuid = &jwt_model.claims.user_id;
    //
    let enable_uuid = custom_uuid.d_e_uuid.clone();

    let _result = admin_user_service::enable_custom_user(enable_uuid.unwrap(), uuid).await;
    AppWriter(_result)
}

#[endpoint(tags("获取管理员列表"))]
pub async fn get_admin_list(depot: &mut Depot) -> AppWriter<Vec<SysUserProfileResponse>> {
    // dbg!(depot);
    let jwt_model = depot.jwt_auth_data::<JwtClaims>().unwrap();
    dbg!(jwt_model);
    let uuid = &jwt_model.claims.user_id;
    //
    let _result = admin_user_service::list_admin_user(uuid).await;
    AppWriter(_result)
}

#[endpoint(tags("获取用户列表"))]
pub async fn get_custom_list(depot: &mut Depot) -> AppWriter<Vec<CustomUserProfileResponse>> {
    // dbg!(depot);
    let jwt_model = depot.jwt_auth_data::<JwtClaims>().unwrap();
    dbg!(jwt_model);
    let uuid = &jwt_model.claims.user_id;
    //
    let _result = admin_user_service::list_custom_user(uuid).await;
    AppWriter(_result)
}

#[endpoint(tags("更改当前用户信息"))]
pub async fn change_profile(
    form_data: JsonBody<ChangeAdminProfileRequest>,
    depot: &mut Depot,
) -> AppWriter<SysUserProfileResponse> {
    // dbg!(depot);
    let jwt_model = depot.jwt_auth_data::<JwtClaims>().unwrap();
    dbg!(jwt_model);
    let uuid = &jwt_model.claims.user_id;
    //

    let get_request = form_data.0;
    let result = admin_user_service::change_profile(get_request, uuid).await;
    AppWriter(result)
}

// 获取用户详细信息
#[endpoint(tags("获取当前用户详情"))]
pub async fn get_token_profile(depot: &mut Depot) -> AppWriter<SysUserProfileResponse> {
    // dbg!(depot);
    let jwt_model = depot.jwt_auth_data::<JwtClaims>().unwrap();
    dbg!(jwt_model);
    let uuid = &jwt_model.claims.user_id;
    //
    let _result = admin_user_service::get_admin_profile(uuid).await;
    AppWriter(_result)
}

// 头像上传功能
#[endpoint(tags("将头像保存到服务器"))]
pub async fn upload_avatar(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    // dbg!(depot);
    let jwt_model = depot.jwt_auth_data::<JwtClaims>().unwrap();
    dbg!(jwt_model);
    let uuid = &jwt_model.claims.user_id;
    //

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
                res.status_code(StatusCode::OK);
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
pub async fn pchange_pwd(
    req: JsonBody<ChangeAdminPwdRequest>,
    depot: &mut Depot,
    res: &mut Response,
) {
    let jwt_model = depot.jwt_auth_data::<JwtClaims>().unwrap();
    dbg!(jwt_model);
    let uuid = &jwt_model.claims.user_id;
    //

    let user_req = req.0;
    let result = admin_user_service::change_admin_password(user_req, uuid).await;
    match result {
        Ok(_data) => {
            res.remove_cookie("jwt_token");
            res.render(Json(_data))
        }
        Err(err) => ErrorResponseBuilder::with_err(err).into_response(res),
    }
}

// #[endpoint(tags("忘记密码"))]
// pub async fn put_reset_password(req: JsonBody<ResetAdminPwdRequest>, res: &mut Response) {
//     let _result = admin_user_service::reset_admin_password(req.0).await;
//     match _result {
//         Ok(_data) => {
//             res.remove_cookie("jwt_token");
//         }
//     }
// }

#[endpoint(tags("管理员登录"))]
pub async fn post_login(
    form_data: JsonBody<SysLoginRequest>,
    // res: &mut Response
) -> AppWriter<SysLoginResponse> {
    let form_data = form_data.0;
    println!("用户：{}正在登录", form_data.user_name.clone());
    let result: AppResult<SysLoginResponse> = admin_user_service::login(form_data).await;
    match result {
        Ok(data) => {
            return AppWriter(Ok(data));
            // let jwt_token = data.token.clone();
            // let cookie = Cookie::build(("jwt_token", jwt_token))
            //     .path("/")
            //     .http_only(true)
            //     .build();
            // res.add_cookie(cookie);
            // res.render(Json(data))
        }
        Err(err) => {
            return AppWriter(Err(AppError::AnyHow(err.into())));
            // ErrorResponseBuilder::with_err(err).into_response(res);
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
    let jwt_model = depot.jwt_auth_data::<JwtClaims>().unwrap();
    dbg!(jwt_model);
    let uuid = &jwt_model.claims.user_id;
    //

    if let Err(err) = admin_user_service::check_user_name(_model.user_name.clone()).await {
        return ErrorResponseBuilder::with_err(AppError::AnyHow(err.into())).into_response(res);
    }
    if let Err(err) = admin_user_service::create_admin_user(_model, uuid).await {
        return ErrorResponseBuilder::with_err(AppError::AnyHow(err.into())).into_response(res);
    }
    res.render(Json("success".to_string()));
}
