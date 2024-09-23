mod login;
pub mod not_found;
mod personal;
mod portal;
mod register;

use anyhow::Result;

use axum::{routing::get, Router};

use crate::RouteEnv;

pub async fn route(env: RouteEnv) -> Result<Router> {
    let router = Router::new()
        .route("/", get(portal::query))
        .route("/personal", get(personal::query))
        .route("/login", get(login::query))
        .route("/register", get(register::query))
        .with_state(env);

    Ok(router)
}
