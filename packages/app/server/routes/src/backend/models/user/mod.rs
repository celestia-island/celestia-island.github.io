mod edit;
mod view;

use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};

use crate::RouteEnv;

pub async fn route(env: RouteEnv) -> Result<Router> {
    let router = Router::new()
        .route("/select/:id", get(view::select))
        .route("/count", get(view::count))
        .route("/list", get(view::list))
        .route("/insert", post(edit::insert))
        .route("/update/:id", post(edit::update))
        .route("/delete/:id", post(edit::delete))
        .with_state(env);

    Ok(router)
}
