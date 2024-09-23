use anyhow::Result;

use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};

use crate::RouteEnv;
use tairitsu_database::prelude::*;

pub async fn wasm(
    State(state): State<RouteEnv>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    {
        let mut headers = HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, "application/wasm".parse().unwrap());

        Ok((
            headers,
            state
                .bucket
                .static_resources
                .get("app.wasm".to_string(), None)
                .await
                .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
                .ok_or((StatusCode::NOT_FOUND, "Not Found".to_string())),
        ))
    }
}

pub async fn js(State(state): State<RouteEnv>) -> Result<impl IntoResponse, (StatusCode, String)> {
    {
        let mut headers = HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, "text/javascript".parse().unwrap());

        Ok((
            headers,
            state
                .bucket
                .static_resources
                .get("app.js".to_string(), None)
                .await
                .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
                .ok_or((StatusCode::NOT_FOUND, "Not Found".to_string())),
        ))
    }
}

pub async fn route(env: RouteEnv) -> Result<Router> {
    let router = Router::new()
        .route("/app.wasm", get(wasm))
        .route("/app.js", get(js))
        .with_state(env);

    Ok(router)
}
