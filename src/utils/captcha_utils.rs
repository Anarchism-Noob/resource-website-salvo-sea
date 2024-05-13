use anyhow::Error;
use captcha_rs::CaptchaBuilder;
use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::utils::{
    app_writer::AppResult,
    redis_utils::get_redis_pool,
};
use redis::AsyncCommands;


#[derive(Serialize, Deserialize, Clone, Debug, Default, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CaptchaImage{
    pub captcha_enable: bool,
    pub captcha_uuid: String,
    pub captcha_image: String,
}

pub async fn generate_captcha(captcha_type: &str) -> AppResult<CaptchaImage> {
    let captcha = CaptchaBuilder::new()
        .length(5)
        .width(130)
        .height(40)
        .dark_mode(false)
        .complexity(1)
        .compression(40)
        .build();
    let rand_string = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();
    let redis_pool = get_redis_pool().await;
    let mut redis_conn = redis_pool.get().await.expect("");
    
    let redis_key = format!("captcha: {}: {}", captcha_type, rand_string);
    
    let _: () = redis_conn
        .set_ex(redis_key, captcha.text.clone(), 300)
        .await
        .expect("验证码生成失败");
    
    Ok(CaptchaImage {
        captcha_enable: true,
        captcha_uuid: rand_string,
        captcha_image: format!("data:image/*;base64,{}", captcha.to_base64()),
    })
}

pub async fn varify_captcha(
    captcha_type:String, 
    captcha_uuid: String,
    code: String,
) -> AppResult<()> {
    let redis_pool = get_redis_pool().await;
    let  mut redis_conn = redis_pool.get().await.expect("");
    
    let redis_key = format!("Captcha: : {}  : {}", captcha_type, captcha_uuid);
    
    let second: i64 = redis_conn
        .ttl(redis_key.clone())
        .await
        .expect("验证码失效");
    
    if  second > 0 { 
        if second > 180 {
            let value:Option<String> = redis_conn
                .get(redis_key.clone())
                .await
                .expect("验证码失效");
            if value.is_none() { 
                return Err(Error::msg("验证码错误").into());
            }
            if code != value.unwrap_or_default() { 
                return Err(Error::msg("验证码错误").into());
            }
        }else { 
            return Err(Error::msg("验证码已过期").into());
        }
    }else { return Err(Error::msg("验证码错误").into()) }
    Ok(())
}