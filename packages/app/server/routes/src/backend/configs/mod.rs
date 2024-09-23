mod env_variants;

use anyhow::Result;
use axum::Router;

use crate::RouteEnv;

pub async fn route(env: RouteEnv) -> Result<Router> {
    let router = Router::new().nest("/env_variants", env_variants::route(env.clone()).await?);

    Ok(router)
}
