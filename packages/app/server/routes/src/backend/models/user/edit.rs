use anyhow::Result;

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{middlewares::ExtractAuthInfo, RouteEnv};
use _server_functions::models::user::{
    delete as do_delete, insert as do_insert, update as do_update,
};
use _types::website::request::models::User as VO;

#[tracing::instrument(err(Debug))]
pub async fn insert(
    ExtractAuthInfo(info): ExtractAuthInfo,
    State(env): State<RouteEnv>,
    Json(vo): Json<VO>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !info
        .ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "No permission".to_string(),
        ))?
        .permission
        .is_root()
    {
        return Err((StatusCode::FORBIDDEN, "No permission".to_string()));
    }

    let ret = do_insert(env, vo)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    Ok(Json(ret))
}

#[tracing::instrument(err(Debug))]
pub async fn update(
    ExtractAuthInfo(info): ExtractAuthInfo,
    State(env): State<RouteEnv>,
    Path(id): Path<i64>,
    Json(vo): Json<VO>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !info
        .ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "No permission".to_string(),
        ))?
        .permission
        .is_root()
    {
        return Err((StatusCode::FORBIDDEN, "No permission".to_string()));
    }

    let ret = do_update(env, id, vo)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    Ok(Json(ret))
}

#[tracing::instrument(err(Debug))]
pub async fn delete(
    ExtractAuthInfo(info): ExtractAuthInfo,
    State(env): State<RouteEnv>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !info
        .ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "No permission".to_string(),
        ))?
        .permission
        .is_root()
    {
        return Err((StatusCode::FORBIDDEN, "No permission".to_string()));
    }

    do_delete(env, id)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    Ok(Json(()))
}
