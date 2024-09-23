use anyhow::Result;

use axum::{
    http::{
        header::{HeaderValue, CONTENT_TYPE},
        HeaderMap, StatusCode,
    },
    response::IntoResponse,
};

use crate::middlewares::ExtractAuthInfo;
use _client_view::app::{App, AppStates};
use hikari_boot::DeclType;

pub async fn query(
    ExtractAuthInfo(auth): ExtractAuthInfo,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let title = "Not found".to_string();

    let ret =
        App::render_to_string_outside("".to_string(), "".to_string(), AppStates { title, auth })
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("text/html; charset=utf-8"),
    );
    Ok((headers, ret).into_response())
}
