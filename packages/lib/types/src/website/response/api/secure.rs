use serde::{Deserialize, Serialize};

use crate::website::request::models::Permission;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonalInfo {
    pub name: String,
    pub email: String,
    pub permission: Permission,
}
