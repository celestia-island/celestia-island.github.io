use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::website::request::models::Permission;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserInfo {
    pub token: String,
    pub id: i64,
    pub name: String,
    pub permission: Permission,
    pub last_login: DateTime<Utc>,
}

pub type AuthInfo = Option<UserInfo>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserInfoDraft {
    pub id: i64,
    pub name: String,
    pub permission: Permission,
}
