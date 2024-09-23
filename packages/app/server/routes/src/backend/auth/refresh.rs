use anyhow::Result;

use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use crate::{middlewares::ExtractIP, RouteEnv};
use _server_functions::auth::refresh as do_refresh;

#[tracing::instrument(err(Debug))]
pub async fn refresh(
    bearer: TypedHeader<Authorization<Bearer>>,
    ExtractIP(ip): ExtractIP,
    State(env): State<RouteEnv>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let ret = do_refresh(env, bearer.token().to_string())
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Cannot refresh: {}", err),
            )
        })?;

    Ok(Json(ret))
}
