use anyhow::Result;

use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
};

use super::generate_token;
use crate::RouteEnv;
use _server_database::prelude::*;
use _types::website::{request::models::Permission, response::api::UserInfo};
use _utils::app::bcrypt::generate_hash;

pub async fn register(
    env: RouteEnv,
    user_name: String,
    password_raw: String,
    email: String,
) -> Result<UserInfo> {
    let password_hash = generate_hash(password_raw)?;

    let vo = user::ActiveModel {
        id: NotSet,
        name: Set(user_name.clone()),
        password_hash: Set(password_hash.clone()),
        email: Set(email.clone()),
        permission: Set(Permission::ROOT_USER.to_string()),
        extra_profile: NotSet,
    };
    let user: user::Model = vo.insert(&**env.sql).await?;

    let (token, last_login) = generate_token(env.clone(), user.clone()).await?;

    Ok(UserInfo {
        token,
        id: user.id,
        name: user.name,
        permission: user.permission.into(),
        last_login,
    })
}
