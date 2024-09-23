use serde::{Deserialize, Serialize};

pub const PASSWORD_SPECIAL_CHARS_PATTERN: &'static str =
    r#"^(?![a-zA-Z]+$)(?!\d+$)(?![^\da-zA-Z\s]+$).{8,}$"#;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeNameInfo {
    pub new_name: String,
    pub password_raw: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeEmailInfo {
    pub new_email: String,
    pub password_raw: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangePasswordInfo {
    pub old_password: String,
    pub new_password: String,
    pub confirm_password: String,
}
