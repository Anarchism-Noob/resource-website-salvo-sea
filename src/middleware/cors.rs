use crate::config::CFG;
use salvo::cors::{AllowHeaders, AllowMethods, AllowOrigin, Cors, CorsHandler};
use tracing::info;

pub fn cors_middleware() -> CorsHandler {
    info!("加载跨域中间件");
    Cors::new()
        .allow_origin(AllowOrigin::any())
        // .allow_origin(&CFG.server.cors_allow_origin)
        .allow_methods(AllowMethods::any())
        .allow_headers(AllowHeaders::any())
        .into_handler()
}
