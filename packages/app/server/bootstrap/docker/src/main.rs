use anyhow::Result;
use log::info;
use std::net::SocketAddr;

use axum::serve;
use tokio::net::TcpListener;

use _server_functions::InitRouteEnvParams;
use _server_routes::RouteEnv;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    #[cfg(not(debug_assertions))]
    {
        use std::path::Path;
        use tracing_subscriber::fmt::writer::MakeWriterExt;

        let file_appender = tracing_appender::rolling::daily("/var/log/server-logs", "log");
        let std_out = std::io::stdout.with_max_level(tracing::Level::INFO);
        tracing_subscriber::fmt()
            .with_writer(std_out.and(file_appender))
            .init();
    }

    #[cfg(debug_assertions)]
    {
        tracing_subscriber::fmt()
            .with_writer(std::io::stdout)
            .with_max_level(tracing::Level::INFO)
            .with_ansi(true)
            .init();
    }

    info!("Server initializing");

    let route_env = RouteEnv::new(InitRouteEnvParams::Native).await?;

    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8787);

    let router = _server_routes::router(route_env)
        .await?
        .into_make_service_with_connect_info::<SocketAddr>();

    info!("Site will run on port {port}");

    let listener = TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .expect("Failed to bind");
    serve(listener, router).await?;

    Ok(())
}
