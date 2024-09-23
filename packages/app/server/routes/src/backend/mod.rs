mod auth;
mod configs;
mod models;

use anyhow::Result;
use axum::Router;

use crate::RouteEnv;

pub async fn route(env: RouteEnv) -> Result<Router> {
    let router = Router::new()
        .nest("/auth", auth::route(env.clone()).await?)
        .nest("/configs", configs::route(env.clone()).await?)
        .nest("/", models::route(env.clone()).await?);

    Ok(router)
}
