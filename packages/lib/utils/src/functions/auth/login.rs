use anyhow::{anyhow, Result};

use gloo::storage::{LocalStorage, Storage as _};
use reqwest::Client;

use crate::app::get_host;
use _types::website::{request::api::LoginInfo, response::api::UserInfo};

pub async fn login(email: String, password_raw: String) -> Result<UserInfo> {
    let res = Client::new()
        .post(format!("{}/api/auth/login", get_host()?))
        .json(&LoginInfo {
            email,
            password_raw,
        })
        .send()
        .await?;

    if res.status().is_success() {
        let ret: UserInfo = res.json().await?;
        LocalStorage::set("auth", ret.clone()).unwrap();

        Ok(ret)
    } else {
        Err(anyhow!("{} - {}", res.status(), res.text().await?))
    }
}
