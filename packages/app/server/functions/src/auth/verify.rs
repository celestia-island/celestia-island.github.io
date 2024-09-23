use anyhow::{anyhow, ensure, Context, Result};

use jsonwebtoken::{decode, Validation};
use sea_orm::EntityTrait;

use super::{Claims, JWT_SECRET};
use crate::RouteEnv;
use _server_database::prelude::*;
use _types::website::response::api::UserInfo;
use tairitsu_database::prelude::*;

pub async fn verify(env: RouteEnv, token: String) -> Result<UserInfo> {
    let token_raw = token.clone();
    let token = decode::<Claims>(&token, &JWT_SECRET.decoding, &Validation::default())
        .context("Invalid token")?;

    let user = user::Entity::find_by_id(token.claims.user_id)
        .one(&**env.sql)
        .await?
        .ok_or(anyhow!("Cannot find the user"))?;

    let iat = token.claims.iat;
    let last_login = env
        .kv
        .token_expired
        .get(user.id.to_string())
        .await?
        .ok_or(anyhow!("Cannot find the last login time in the cache"))?;
    let last_login = chrono::DateTime::parse_from_rfc3339(&last_login)?.to_utc();
    let last_login = last_login - chrono::Duration::minutes(1);
    ensure!(iat >= last_login, "Token expired");

    Ok(UserInfo {
        token: token_raw,
        id: user.id,
        name: user.name,
        permission: user.permission.into(),
        last_login,
    })
}
