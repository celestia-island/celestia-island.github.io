use anyhow::Result;

use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{backend::models::PageArgs, RouteEnv};
use _server_functions::models::user::{count as do_count, list as do_list, select as do_select};

#[tracing::instrument(err(Debug))]
pub async fn select(
    State(env): State<RouteEnv>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let ret = do_select(env, id)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Not found".to_string()))?;
    Ok(Json(ret))
}

#[tracing::instrument(err(Debug))]
pub async fn count(State(env): State<RouteEnv>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let ret = do_count(env)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    Ok(Json(ret))
}

#[tracing::instrument(err(Debug))]
pub async fn list(
    State(env): State<RouteEnv>,
    args: Query<PageArgs>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let ret = do_list(env, args.offset.unwrap_or(0), args.limit.unwrap_or(10))
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    Ok(Json(ret))
}
