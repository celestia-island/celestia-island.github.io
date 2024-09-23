mod login;
mod refresh;
mod register;
mod verify;

pub use login::*;
pub use refresh::*;
pub use register::*;
pub use verify::*;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header};

use crate::RouteEnv;
use _server_database::models::user;
use tairitsu_database::prelude::*;

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

static JWT_SECRET: Lazy<Keys> = Lazy::new(|| {
    let str = "没有人的文明，毫无意义";
    Keys {
        encoding: EncodingKey::from_secret(str.as_bytes()),
        decoding: DecodingKey::from_secret(str.as_bytes()),
    }
});

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Claims {
    user_id: i64,
    #[serde(with = "jwt_numeric_date")]
    iat: DateTime<Utc>,
    #[serde(with = "jwt_numeric_date")]
    exp: DateTime<Utc>,
}

mod jwt_numeric_date {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = date.timestamp();
        serializer.serialize_i64(timestamp)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Utc.timestamp_opt(i64::deserialize(deserializer)?, 0)
            .single()
            .ok_or_else(|| serde::de::Error::custom("invalid Unix timestamp value"))
    }
}

pub async fn generate_token(env: RouteEnv, user: user::Model) -> Result<(String, DateTime<Utc>)> {
    let now = chrono::Utc::now();
    let claims = Claims {
        user_id: user.id,
        iat: now,
        exp: now + chrono::Duration::days(15),
    };

    env.kv
        .token_expired
        .set(user.id.to_string(), now.to_rfc3339())
        .await?;

    Ok((
        encode(&Header::default(), &claims, &JWT_SECRET.encoding)
            .context("Failed to encode token")?,
        now,
    ))
}
