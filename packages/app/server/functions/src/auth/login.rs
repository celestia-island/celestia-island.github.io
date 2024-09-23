use anyhow::{anyhow, ensure, Result};

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use super::generate_token;
use crate::RouteEnv;
use _server_database::prelude::*;
use _types::website::response::api::UserInfo;
use _utils::app::bcrypt::verify_hash;

pub async fn login(env: RouteEnv, email: String, password_hash: String) -> Result<UserInfo> {
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .one(&**env.sql)
        .await?
        .ok_or(anyhow!("Cannot find the user"))?;

    ensure!(
        verify_hash(password_hash, user.password_hash.clone())?,
        "Wrong password"
    );

    let (token, last_login) = generate_token(env.clone(), user.clone()).await?;

    Ok(UserInfo {
        token,
        id: user.id,
        name: user.name,
        permission: user.permission.into(),
        last_login,
    })
}
