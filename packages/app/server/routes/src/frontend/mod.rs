pub mod pages;
pub mod r#static;

use anyhow::Result;

use axum::Router;

use crate::RouteEnv;

pub async fn route(env: RouteEnv) -> Result<Router> {
    let router = Router::new()
        .nest("/", pages::route(env.clone()).await?)
        .nest("/", r#static::route(env.clone()).await?);

    Ok(router)
}
