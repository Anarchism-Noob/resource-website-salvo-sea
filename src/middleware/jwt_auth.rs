use crate::utils::check_user::{check_user_admin, check_user_custom};
use crate::{app_error::AppError, config::CFG, middleware::jwt};
use anyhow::Result;
use jsonwebtoken::{decode, Algorithm, DecodingKey, EncodingKey, Validation};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use salvo::jwt_auth::{ConstDecoder, CookieFinder, HeaderFinder, QueryFinder};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::{Duration, OffsetDateTime};

pub async fn jwt_auth_middleware(
    req: &mut Request,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
    depot: &mut Depot,
) {
    println!("jwt_auth_middleware--> load");

    let item = req.parse_headers::<HashMap<String, String>>().unwrap();

    let token = depot.get::<&str>("jwt-token").copied().unwrap();

    let jwt_model = jwt::parse_token(&token).unwrap();

    let check_custom = check_user_custom(jwt_model.user_id.clone()).await;
    let check_admin = check_user_admin(jwt_model.user_id.clone()).await;
}
