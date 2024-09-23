use anyhow::{anyhow, Result};

use gloo::storage::{LocalStorage, Storage as _};
use reqwest::Client;

use _utils::app::get_host;
use _types::website::{
    request::api::{ChangeEmailInfo, ChangeNameInfo, ChangePasswordInfo},
    response::api::UserInfo,
};

pub async fn update_personal_name(new_name: String, password_raw: String) -> Result<()> {
    match LocalStorage::get::<UserInfo>("auth") {
        Ok(info) => {
            let res = Client::new()
                .post(format!(
                    "{}/api/secure/update_personal_info/name",
                    get_host()?,
                ))
                .json(&ChangeNameInfo {
                    new_name,
                    password_raw,
                })
                .bearer_auth(info.token)
                .send()
                .await?;

            if res.status().is_success() {
                Ok(())
            } else {
                Err(anyhow!("{} - {}", res.status(), res.text().await?))
            }
        }
        _ => Err(anyhow!("No token found")),
    }
}

pub async fn update_personal_email(new_email: String, password_raw: String) -> Result<()> {
    match LocalStorage::get::<UserInfo>("auth") {
        Ok(info) => {
            let res = Client::new()
                .post(format!(
                    "{}/api/secure/update_personal_info/email",
                    get_host()?,
                ))
                .json(&ChangeEmailInfo {
                    new_email,
                    password_raw,
                })
                .bearer_auth(info.token)
                .send()
                .await?;

            if res.status().is_success() {
                Ok(())
            } else {
                Err(anyhow!("{} - {}", res.status(), res.text().await?))
            }
        }
        _ => Err(anyhow!("No token found")),
    }
}

pub async fn update_personal_password(
    old_password: String,
    new_password: String,
    confirm_password: String,
) -> Result<()> {
    match LocalStorage::get::<UserInfo>("auth") {
        Ok(info) => {
            let res = Client::new()
                .post(format!(
                    "{}/api/secure/update_personal_info/password",
                    get_host()?,
                ))
                .json(&ChangePasswordInfo {
                    old_password,
                    new_password,
                    confirm_password,
                })
                .bearer_auth(info.token)
                .send()
                .await?;

            if res.status().is_success() {
                Ok(())
            } else {
                Err(anyhow!("{} - {}", res.status(), res.text().await?))
            }
        }
        _ => Err(anyhow!("No token found")),
    }
}
