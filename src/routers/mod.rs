// mod admin;
mod admin;
mod custom;
use salvo::prelude::Router;

pub fn router() -> Router {
    Router::new()
        .push(Router::with_path("/custom/api/").push(custom::api()))
        .push(Router::with_path("/system/api").push(admin::api()))
}

// pub fn router_whitelist() -> Vec<String> {
//     let router_whitelist = vec![];
//     router_whitelist
// }
