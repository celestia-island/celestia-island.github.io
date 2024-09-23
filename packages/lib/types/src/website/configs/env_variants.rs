use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

pub const PREFIX: &str = "env_variants";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display, EnumString, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum EnvVariants {
    MaintenanceMode(bool),
    CloudflareUserId(String),
    CloudflareStreamDomain(String),
    CloudflareStreamToken(String),
    CloudflareStreamSigningKey { id: String, pem: Vec<u8> },
    BackendDynamicEntry(String),
    TagGroupList(Vec<String>),
}
