mod backend;
mod frontend;
mod middlewares;

use anyhow::Result;

use axum::{
    extract::DefaultBodyLimit,
    middleware::{from_extractor, from_extractor_with_state},
    routing::get,
    Router,
};

use _server_functions::configs::env_variants;
pub use _server_functions::RouteEnv;

pub async fn router(env: RouteEnv) -> Result<Router> {
    env_variants::init(env.clone()).await?;

    let ret = Router::new()
        .nest("/", frontend::route(env.clone()).await?)
        .nest("/api", backend::route(env.clone()).await?)
        .fallback(get(frontend::pages::not_found::query).with_state(env.clone()))
        .layer(from_extractor_with_state::<middlewares::ExtractAuthInfo, _>(env.clone()))
        .layer(from_extractor::<middlewares::ExtractIP>())
        .layer(DefaultBodyLimit::max(1024 * 1024 * 16)); // 16 MiB

    Ok(ret)
}
