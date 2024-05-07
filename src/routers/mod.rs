// mod admin;
mod custom;
use salvo::prelude::Router;

pub fn router() -> Router {
    let router = Router::new()
    .push(Router::with_path("/custom/api").push(custom::api()));
    // .push(Router::with_path("/admin/api").push(admin::api()));

    router
}

pub fn router_whitelist() -> Vec<String>{
    let router_whitelist = vec![];

    router_whitelist
}