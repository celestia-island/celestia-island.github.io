use anyhow::Result;

use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use crate::RouteEnv;
use _server_functions::auth::verify;
use _types::website::response::api::AuthInfo;

pub struct ExtractAuthInfo(pub AuthInfo);

#[async_trait::async_trait]
impl FromRequestParts<RouteEnv> for ExtractAuthInfo {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        env: &RouteEnv,
    ) -> Result<Self, Self::Rejection> {
        match TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, env).await {
            Ok(bearer) => {
                let token = bearer.token().to_string();
                let info = verify(env.clone(), token).await.map_err(|err| {
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
                })?;

                Ok(Self(Some(info)))
            }
            Err(_) => Ok(Self(None)),
        }
    }
}
