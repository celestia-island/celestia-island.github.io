use anyhow::Result;
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use std::{
    net::{IpAddr, Ipv4Addr},
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{middlewares::ExtractIP, RouteEnv};
use _server_functions::auth::register as do_register;
use _types::website::request::api::RegisterParams;

type RegisterItem = (IpAddr, DateTime<Utc>);
static REGISTER_LOG: Lazy<Arc<Mutex<Vec<RegisterItem>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

#[tracing::instrument(err(Debug))]
pub async fn register(
    ExtractIP(ip): ExtractIP,
    State(env): State<RouteEnv>,
    Json(info): Json<RegisterParams>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Write the register log
    let ip = ip.unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST));
    let now = Utc::now();
    REGISTER_LOG
        .lock()
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Cannot lock register log: {}", err),
            )
        })?
        .push((ip, now));

    // Clear the login log that is older than 1 day
    REGISTER_LOG
        .lock()
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Cannot lock register log: {}", err),
            )
        })?
        .retain(|(_, time)| now.signed_duration_since(*time).num_hours() < 24);

    // Check if the user is trying to login too frequently
    // Limit to 3 times per day
    let count = REGISTER_LOG
        .lock()
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Cannot lock login log: {}", err),
            )
        })?
        .iter()
        .filter(|(log_ip, time)| log_ip == &ip && now.signed_duration_since(*time).num_hours() < 24)
        .count();
    if count > 3 {
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            "Too many requests".to_string(),
        ));
    }

    let ret = do_register(env, info.name, info.password_raw, info.email)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Cannot login: {}", e),
            )
        })?;

    Ok(Json(ret))
}
