use chrono::Utc;

use sea_orm::ActiveValue::{NotSet, Set};

use crate::models::user::*;
use _types::website::request::models::User;

impl From<User> for ActiveModel {
    fn from(info: User) -> Self {
        Self {
            id: NotSet,
            name: Set(info.name),
            password_hash: Set(info.password_hash),
            permission: Set(info.permission.into()),
            email: Set(info.email),
            extra_profile: Set(info.extra_profile),
        }
    }
}

impl From<Model> for User {
    fn from(val: Model) -> Self {
        User {
            id: Some(val.id),
            name: val.name,
            password_hash: val.password_hash,
            permission: val.permission.into(),
            email: val.email,
            extra_profile: val.extra_profile,
        }
    }
}
