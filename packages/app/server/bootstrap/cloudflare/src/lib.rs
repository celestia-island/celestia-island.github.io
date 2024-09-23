use anyhow::Result;

use axum::{body::Body, response::Response};
use tower_service::Service;
use worker::{event, Context, Env, HttpRequest};

use _server_database::init_tables;
use _server_functions::InitRouteEnvParams;
use _server_routes::RouteEnv;

// https://developers.cloudflare.com/workers/languages/rust

#[event(fetch)]
async fn fetch(req: HttpRequest, env: Env, _ctx: Context) -> Result<Response<Body>> {
    console_error_panic_hook::set_once();

    let state = RouteEnv::new(InitRouteEnvParams::Cloudflare(env)).await?;
    init_tables(&*state.sql).await?;

    Ok(_server_routes::router(state).await?.call(req).await?)
}
