use anyhow::{anyhow, Result};

use reqwest::Client;

use crate::app::get_host;
use _types::website::response::api::UserInfo;

pub async fn verify(token: UserInfo) -> Result<UserInfo> {
    let res = Client::new()
        .get(format!("{}/api/auth/verify", get_host()?))
        .bearer_auth(token.token.clone())
        .send()
        .await?;

    if res.status().is_success() {
        Ok(token)
    } else {
        Err(anyhow!("{} - {}", res.status(), res.text().await?))
    }
}
