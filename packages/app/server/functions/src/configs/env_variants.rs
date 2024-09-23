use anyhow::{anyhow, Result};

use crate::RouteEnv;
use _types::website::configs::env_variants::{EnvVariants, PREFIX};
use tairitsu_database::prelude::*;

pub async fn init(env: RouteEnv) -> Result<()> {
    let value = EnvVariants::BackendDynamicEntry(uuid::Uuid::new_v4().to_string());
    if select(env.clone(), value.to_string()).await.is_err() {
        update(env.clone(), value.to_string(), value).await?;
    }

    Ok(())
}

pub async fn select(env: RouteEnv, key: String) -> Result<EnvVariants> {
    if let Some(value) = env
        .kv
        .global_config
        .get(format!("{}::{}", PREFIX, key))
        .await?
    {
        let value: EnvVariants = serde_json::from_str(&value)?;
        if value.to_string() != key {
            return Err(anyhow!("Invalid key"));
        }

        Ok(value)
    } else {
        Err(anyhow!("Variant not found"))
    }
}

pub async fn update(env: RouteEnv, key: String, value: EnvVariants) -> Result<()> {
    if key != value.to_string() {
        return Err(anyhow!("Invalid key"));
    }

    env.kv
        .global_config
        .set(
            format!("{}::{}", PREFIX, key),
            serde_json::to_string(&value)?,
        )
        .await?;

    Ok(())
}
