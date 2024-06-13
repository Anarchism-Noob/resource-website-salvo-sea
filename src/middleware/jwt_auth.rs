use crate::dtos::sys_user_dto::BaseResponse;
use crate::middleware::jwt::JwtClaims;
use crate::utils::check_user::{check_user_admin, check_user_custom};
use salvo::prelude::*;

#[handler]
pub async fn jwt_auth_middleware(
    req: &mut Request,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
    depot: &mut Depot,
) {
    println!("加载jwt鉴权中间件");
    match depot.jwt_auth_state() {
        // 请求中包含jwt
        JwtAuthState::Authorized => {
            let jwt = depot.jwt_auth_data::<JwtClaims>().unwrap();
            println!("jwt_auth_data: {:?}", jwt);
            if let Ok(admin_user) = check_user_admin(&jwt.claims.user_id).await {
                // 超级管理员直接放行
                match admin_user.role{
                    0 => {
                        ctrl.call_next(req, depot, res).await;
                    } 
                    1 => {
                        //非超级管理员的管理员账号不能访问的url
                        if !req.uri().path().starts_with("/admin")
                            && !req.uri().path().starts_with("/ordinary")
                        {
                            let base_res = BaseResponse {
                                code: 403,
                                msg: "未授权访问1-1".to_string(),
                                data: None,
                            };
                            res.render(Json(base_res));
                            return;
                        } else {
                            ctrl.call_next(req, depot, res).await;
                        }
                    } 
                    2 => {
                        if !req.uri().path().starts_with("/ordinary") {
                            let base_res = BaseResponse {
                                code: 403,
                                msg: "未授权访问1-2".to_string(),
                                data: None,
                            };
                            res.render(Json(base_res));
                            return;
                        } else {
                            ctrl.call_next(req, depot, res).await;
                        }
                    }
                    _ => {
                        let base_res = BaseResponse {
                            code: 403,
                            msg: "未授权访问1-3".to_string(),
                            data: None,
                        };
                        res.render(Json(base_res));
                        return;
                    }
                }
            } else if let Ok(_custom_user) = check_user_custom(&jwt.claims.user_id).await {
                // 普通用户只能访问/custom/api/**下的url
                if req.uri().path().starts_with("/custom") {
                    ctrl.call_next(req, depot, res).await;
                } else {
                    let base_res = BaseResponse {
                        code: 403,
                        msg: "未授权访问2".to_string(),
                        data: None,
                    };
                    res.render(Json(base_res));
                    return;
                }
            } else {
                //表中没有查到该用户的数据，拒绝访问
                let base_res = BaseResponse {
                    code: 403,
                    msg: "请求异常，数据库中查无此人".to_string(),
                    data: None,
                };
                res.render(Json(base_res));
                return;
            }
        }
        // 请求中没有jwt
        JwtAuthState::Unauthorized => {
            let base_res = BaseResponse {
                code: 401,
                msg: "请登录！".to_string(),
                data: None,
            };
            res.render(Json(base_res));
        }
        // 请求的jwt异常
        JwtAuthState::Forbidden => {
            let base_res = BaseResponse {
                code: 403,
                msg: "Token异常.".to_string(),
                data: None,
            };
            res.render(Json(base_res));
        }
    }
   
}

// async fn find_token(request: &mut Request) -> Option<String, String> {
//     if let Some(Ok(auth)) = request.headers().get("authorization").map(|auth| auth.to_str()) {
//         if auth.starts_with("Bearer") {
//             let token_key = match auth.split_once(' ').map(|(_, token)| token.to_string())
//             {
//                 Some(token) => token,
//                 None => return None,
//             };
//             // 计算token_and_token_id的长度与scru128::new_string().len()的长度之差
//             let cut = token_and_token_id.len() - scru128::new_string().len();
//             // 截取token_and_token_id的前cut个字符并将其转换为字符串
//             let t_v = token_and_token_id[0..cut].to_string();
//             // 截取token_and_token_id的后cut个字符并将其转换为字符串
//             let t_id = token_and_token_id[cut..].to_string();
//             // 返回令牌ID和令牌值
//             Some((t_id, t_v))
//         }else {
//             None
//         }
//     }else {
//         None
//     }
// }