use _types::website::configs::env_variants::EnvVariants;
use anyhow::Result;

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use crate::{middlewares::ExtractAuthInfo, RouteEnv};
use _server_functions::configs::env_variants::{select as do_select, update as do_update};

#[tracing::instrument(err(Debug))]
pub async fn select(
    ExtractAuthInfo(auth): ExtractAuthInfo,
    State(env): State<RouteEnv>,
    Path(key): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Some(auth) = auth {
        if !auth.permission.is_root() {
            return Err((StatusCode::FORBIDDEN, "Permission denied".to_string()));
        }

        let ret = do_select(env.clone(), key).await.map_err(|err| {
            (
                StatusCode::BAD_REQUEST,
                format!("Failed to select EnvVariants: {:?}", err),
            )
        })?;

        Ok(Json(ret))
    } else {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
    }
}

#[tracing::instrument(err(Debug))]
pub async fn update(
    ExtractAuthInfo(auth): ExtractAuthInfo,
    State(env): State<RouteEnv>,
    Path(key): Path<String>,
    Json(value): Json<EnvVariants>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Some(auth) = auth {
        if !auth.permission.is_root() {
            return Err((StatusCode::FORBIDDEN, "Permission denied".to_string()));
        }

        do_update(env.clone(), key, value).await.map_err(|err| {
            (
                StatusCode::BAD_REQUEST,
                format!("Failed to update EnvVariants: {:?}", err),
            )
        })?;

        Ok(())
    } else {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
    }
}

pub async fn route(env: RouteEnv) -> Result<Router> {
    let router = Router::new()
        .route("/:key", get(select))
        .route("/:key", post(update))
        .with_state(env);

    Ok(router)
}
