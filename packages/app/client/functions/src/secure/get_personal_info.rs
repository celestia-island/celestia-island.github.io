use anyhow::{anyhow, Result};

use gloo::storage::{LocalStorage, Storage as _};
use reqwest::Client;

use _utils::app::get_host;
use _types::website::response::api::{PersonalInfo, UserInfo};

pub async fn get_personal_info() -> Result<PersonalInfo> {
    match LocalStorage::get::<UserInfo>("auth") {
        Ok(info) => {
            let res = Client::new()
                .get(format!("{}/api/secure/get_personal_info", get_host()?))
                .bearer_auth(info.token)
                .send()
                .await?;

            if res.status().is_success() {
                let ret: PersonalInfo = res.json().await?;

                Ok(ret)
            } else {
                Err(anyhow!("{} - {}", res.status(), res.text().await?))
            }
        }
        _ => Err(anyhow!("No token found")),
    }
}
