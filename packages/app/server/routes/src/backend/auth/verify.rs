use anyhow::Result;

use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use crate::{middlewares::ExtractIP, RouteEnv};
use _server_functions::auth::verify as do_verify;

#[tracing::instrument(err(Debug))]
pub async fn verify(
    bearer: TypedHeader<Authorization<Bearer>>,
    ExtractIP(ip): ExtractIP,
    State(env): State<RouteEnv>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    do_verify(env, bearer.token().to_string())
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err)))?;

    Ok(())
}
