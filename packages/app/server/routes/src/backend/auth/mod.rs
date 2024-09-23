mod login;
mod refresh;
mod register;
mod verify;

use anyhow::Result;

use axum::{
    routing::{get, post},
    Router,
};

use crate::RouteEnv;

pub async fn route(env: RouteEnv) -> Result<Router> {
    let router = Router::new()
        .route("/login", post(login::login))
        .route("/refresh", get(refresh::refresh))
        .route("/verify", get(verify::verify))
        .route("/register", post(register::register))
        .with_state(env);

    Ok(router)
}
