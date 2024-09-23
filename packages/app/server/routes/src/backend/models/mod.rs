mod user;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    Router,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use _server_functions::auth::verify;

use crate::RouteEnv;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageArgs {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

async fn auth_middleware(
    bearer: TypedHeader<Authorization<Bearer>>,
    State(env): State<RouteEnv>,
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    if let Err(err) = verify(env, bearer.token().to_string()).await {
        return Err((StatusCode::UNAUTHORIZED, format!("Unauthorized: {}", err)));
    }

    Ok(next.run(request).await)
}

pub async fn route(env: RouteEnv) -> Result<Router> {
    let router = Router::new()
        .nest("/user", user::route(env.clone()).await?)
        .layer(middleware::from_fn_with_state(env.clone(), auth_middleware));

    Ok(router)
}
