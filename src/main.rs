use crate::{
    middleware::{cors::cors_middleware, handle_404::handle_404},
    routers::router,
    services::admin_user_service::super_admin_init,
    utils::{app_error, app_writer, db::init_db_conn, redis_utils::get_redis_connection},
};
use config::{CERT_KEY, CFG};
use salvo::{
    catcher::Catcher,
    conn::rustls::{Keycert, RustlsConfig},
    prelude::*,
    server::ServerHandle,
};
use tokio::signal;
use tracing::info;
mod config;
pub mod controller;
mod dtos;
mod entities;
mod middleware;
mod routers;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    get_redis_connection().await;
    init_db_conn().await;

    //At the same time, logs are only output to the terminal or file
    let _guard = clia_tracing_config::build()
        .filter_level("debug")
        .with_ansi(CFG.log.with_ansi)
        .to_stdout(CFG.log.to_stdout)
        .directory(&CFG.log.directory)
        .file_name(&CFG.log.file_name)
        .rolling(&CFG.log.rolling)
        .init();
    super_admin_init().await;
    tracing::info!("log level: {}", &CFG.log.filter_level);
    //
    // init_db_conn().await;
    let router = router();
    let service: Service = router.into();
    let service = service.catcher(Catcher::default().hoop(handle_404)); //.hoop(_cors_handler).hoop(handle_404));
    println!("🌪️ {} is starting ", &CFG.server.name);
    println!("🔄 listen on {}", &CFG.server.address);
    let _cors_handler = cors_middleware();
    match CFG.server.ssl {
        true => {
            println!("🔒 SSL is enabled");
            println!(
                "📖 System Open API Page: https://{}/system/api/swagger-ui",
                &CFG.server.address.replace("0.0.0.0", "127.0.0.1")
            );
            println!(
                "📖 Custom Open API Page: https://{}/custom/api/swagger-ui",
                &CFG.server.address.replace("0.0.0.0", "127.0.0.1")
            );
            let config = RustlsConfig::new(
                Keycert::new()
                    .cert(CERT_KEY.cert.clone())
                    .key(CERT_KEY.key.clone()),
            );
            let listener = TcpListener::new(&CFG.server.address).rustls(config.clone());

            //HTTP
            let acceptor = QuinnListener::new(config, &CFG.server.address)
                .join(listener)
                .bind()
                .await;

            let server = Server::new(acceptor);
            let handle = server.handle();
            tokio::spawn(shutdown_signal(handle));
            server.serve(service).await;
        }
        false => {
            println!("🔓 SSL is disabled");
            println!(
                "📖 System Open API Page: http://{}/system/api/swagger-ui",
                &CFG.server.address.replace("0.0.0.0", "127.0.0.1")
            );
            println!(
                "📖 Custom Open API Page: http://{}/custom/api/swagger-ui",
                &CFG.server.address.replace("0.0.0.0", "127.0.0.1")
            );
            let acceptor = TcpListener::new(&CFG.server.address).bind().await;
            let server = Server::new(acceptor);
            let handle = server.handle();
            tokio::spawn(shutdown_signal(handle));
            server.serve(service).await;
        }
    }
}

async fn shutdown_signal(handle: ServerHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("ctrl_c signal received"),
        _ = terminate => info!("terminate signal received"),
    }
    handle.stop_graceful(std::time::Duration::from_secs(60));
}
