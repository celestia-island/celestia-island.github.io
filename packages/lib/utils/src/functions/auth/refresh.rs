use anyhow::{anyhow, Result};

use reqwest::Client;

use crate::app::get_host;
use _types::website::response::api::UserInfo;

pub async fn refresh(old_token: UserInfo) -> Result<UserInfo> {
    let res = Client::new()
        .get(format!("{}/api/auth/refresh", get_host()?))
        .bearer_auth(old_token.token)
        .send()
        .await?;

    if res.status().is_success() {
        Ok(res.json().await?)
    } else {
        Err(anyhow!("{} - {}", res.status(), res.text().await?))
    }
}
