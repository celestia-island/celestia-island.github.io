mod app;

use anyhow::Result;

use axum::Router;

use crate::RouteEnv;

pub async fn route(env: RouteEnv) -> Result<Router> {
    let router = Router::new().nest("/", app::route(env.clone()).await?);

    Ok(router)
}
