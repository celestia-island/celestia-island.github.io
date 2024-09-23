use anyhow::{anyhow, Result};

#[cfg(target_arch = "wasm32")]
use gloo::storage::{LocalStorage, Storage as _};
use reqwest::Client;

use crate::app::get_host;
use _types::website::{request::api::RegisterParams, response::api::UserInfo};

pub async fn register(name: String, password_raw: String, email: String) -> Result<UserInfo> {
    let res = Client::new()
        .post(format!("{}/api/auth/register", get_host()?))
        .json(&RegisterParams {
            name,
            password_raw,
            email,
        })
        .send()
        .await?;

    if res.status().is_success() {
        let ret: UserInfo = res.json().await?;
        #[cfg(target_arch = "wasm32")]
        LocalStorage::set("auth", ret.clone()).unwrap();

        Ok(ret)
    } else {
        Err(anyhow!("{} - {}", res.status(), res.text().await?))
    }
}
