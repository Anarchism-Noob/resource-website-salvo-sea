use crate::utils::check_user::{check_user_admin, check_user_custom};
use salvo::prelude::*;
use crate::config::CFG;
use crate::middleware::jwt::JwtClaims;

#[handler]
pub async fn jwt_auth_middleware(
    req: &mut Request,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
    depot: &mut Depot,
) {
    println!("加载jwt鉴权中间件");
    if let Ok(user) = depot.get::<JwtClaims>(&CFG.jwt.jwt_secret){
        if let Ok(admin_user) = check_user_admin(&user.user_id).await {
            // 超级管理员直接放行
            if admin_user.role == 0 {
                ctrl.call_next(req, depot, res).await;
            }else if admin_user.role == 1{
                //非超级管理员的管理员账号不能访问的url白名单
                let restricted_urls = [
                    // 禁用或启用管理员账号
                    "/create",
                    "/disable_admin",
                    "/enable_admin",
                    "/all_admin",
                    // 获取和处理取款申请
                    "/process",
                    "/get_unprocessed",
                    //删除资源
                    "/del_resource"
                ];
                if restricted_urls.iter().any(|url| req.uri().path().starts_with(url)) {
                    res.status_code(StatusCode::FORBIDDEN);
                    res.render("Access forbidden for this URL.");
                    return;
                }else{
                    ctrl.call_next(req, depot, res).await;
                }
            }else {
                // ctrl.call_next(req, depot, res).await;
                res.status_code(StatusCode::FORBIDDEN);
                res.render("Access forbidden for this URL.");
            }
        }else if let Ok(_custom_user) = check_user_custom(&user.user_id).await {
            // 普通用户只能访问/custom/api/**下的url
            if req.uri().path().starts_with("/custom/api/"){
                ctrl.call_next(req, depot, res).await;
                return;
            }else{
                res.status_code(StatusCode::FORBIDDEN);
                res.render("Access forbidden for this URL.");
                return;
            }
        }else{
            //表中没有查到该用户的数据，拒绝访问
            res.status_code(StatusCode::UNAUTHORIZED);
            res.render("Access forbidden for this URL.");
            return;
        }
    }else{
        // 没有找到JWT声明，拒绝访问
        res.render(
            StatusError::unauthorized()
                .brief("you need to login first or the api permission is set wrong"),
        );
        ctrl.skip_rest();
    }
    // let _item = match req.parse_headers::<HashMap<String, String>>() {
    //     Ok(item) => item,
    //     Err(err) => {
    //         res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
    //         res.body(err.to_string().into());
    //         return;
    //     }
    // };
    //
    // let token = match depot.get::<&str>("jwt-token") {
    //     Ok(token) => token.to_string(),
    //     Err(_) => {
    //         res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
    //         res.body("JWT token not found in Depot".into());
    //         return;
    //     }
    // };
    //
    // let jwt_claims = match jwt::parse_token(&token) {
    //     Ok(claims) => claims,
    //     Err(err) => {
    //         handle_internal_server_error(res, StatusCode::INTERNAL_SERVER_ERROR, err.to_string());
    //         return;
    //     }
    // };
    //
    // let uri: Uri = match req.uri().to_string().parse() {
    //     Ok(uri) => uri,
    //     Err(err) => {
    //         handle_internal_server_error(res, StatusCode::INTERNAL_SERVER_ERROR, err.to_string());
    //         return;
    //     }
    // };
    //
    // //判断token是custom还是admin
    // match jwt_claims.role {
    //     Some(_role) => {
    //         // 查看admin表中是否有这个用户
    //         let check_result = match check_user_admin(&jwt_claims.user_id).await {
    //             Ok(admin_res) => admin_res,
    //             Err(_err) => {
    //                 handle_unauthorized_access(res, ctrl);
    //                 return;
    //             }
    //         };
    //         if !is_admin_route(uri.path()) {
    //             // 如果访问的路径不在 admin 路由中，则拒绝访问
    //             handle_unauthorized_access(res, ctrl);
    //             return;
    //         }
    //         if is_super_admin_route(uri.path()) {
    //             // 如果访问的路径是超级管理员路由，则检查用户是否是超级管理员
    //             if check_result.role != 0 {
    //                 handle_unauthorized_access(res, ctrl);
    //                 return;
    //             }
    //         }
    //     }
    //     None => {
    //         // 查看custom表中是否有这个用户
    //         let _check_result = match check_user_custom(&jwt_claims.user_id).await {
    //             // 如果用户存在，则返回结构体
    //             Ok(custom_res) => custom_res,
    //             Err(_err) => {
    //                 // 如果用户不存在，则进行错误处理
    //                 handle_unauthorized_access(res, ctrl);
    //                 return;
    //             }
    //         };
    //         if !is_custom_route(uri.path()) {
    //             // 如果访问的路径不在 custom 路由中，则拒绝访问
    //             handle_unauthorized_access(res, ctrl);
    //             return;
    //         }
    //     }
    // }
}
// fn is_custom_route(uri_path: &str) -> bool {
//     uri_path.starts_with("/custom/api/")
// }
//
// fn is_admin_route(uri_path: &str) -> bool {
//     uri_path.starts_with("/admin/api/")
// }
//
// fn is_super_admin_route(uri_path: &str) -> bool {
//     uri_path.starts_with("/admin/api/manager/")
// }
//
// fn handle_internal_server_error(res: &mut Response, status_code: StatusCode, message: String) {
//     res.status_code(status_code);
//     res.body(message.into());
// }
//
// fn handle_unauthorized_access(res: &mut Response, ctrl: &mut FlowCtrl) {
//     res.status_code(StatusCode::UNAUTHORIZED);
//     res.body("越权访问或路径不存在".to_string().into());
//     ctrl.skip_rest();
// }
