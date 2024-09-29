pub mod auth;
pub mod configs;
pub mod models;

use anyhow::Result;
use std::sync::Arc;

use sea_orm::DatabaseConnection;

use tairitsu_database::prelude::*;

#[derive(Clone)]
pub struct RouteEnv {
    pub sql: Arc<Box<DatabaseConnection>>,
    pub kv: RouteEnvKV,
    pub bucket: RouteEnvBucket,
}

impl std::fmt::Debug for RouteEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RouteEnv").finish()
    }
}

#[derive(Clone)]
pub struct RouteEnvKV {
    pub token_expired: Arc<Box<ProxyKV>>,
    pub global_config: Arc<Box<ProxyKV>>,
}

#[derive(Clone)]
pub struct RouteEnvBucket {
    pub static_resources: Arc<Box<ProxyBucket>>,
}

#[derive(Clone)]
pub enum InitRouteEnvParams {
    Cloudflare(worker::Env),
    Native,
    WASI,
}

cfg_if::cfg_if! {
    if #[cfg(feature = "cloudflare")]{
        impl RouteEnv {
            pub async fn new(param: InitRouteEnvParams) -> Result<Self> {
                match param {
                    InitRouteEnvParams::Cloudflare(env) => {
                        Ok(Self {
                            sql: Arc::new(
                                init_db(InitSQLParams::Cloudflare {
                                    env: Arc::new(env.clone()),
                                    name: "site".to_string(),
                                })
                                .await?,
                            ),
                            kv: RouteEnvKV {
                                token_expired: Arc::new(
                                    init_kv(InitKVParams::Cloudflare {
                                        env: Arc::new(env.clone()),
                                        name: "token-expired".to_string(),
                                    })
                                    .await?,
                                ),
                                global_config: Arc::new(
                                    init_kv(InitKVParams::Cloudflare {
                                        env: Arc::new(env.clone()),
                                        name: "global-config".to_string(),
                                    })
                                    .await?,
                                ),
                            },
                            bucket: RouteEnvBucket {
                                static_resources: Arc::new(
                                    init_bucket(InitBucketParams::Cloudflare {
                                        env: Arc::new(env.clone()),
                                        bucket_name: "static".to_string(),
                                        multipart_kv_name: "global-config".to_string(),
                                    })
                                    .await?,
                                ),
                            },
                        })
                    }
                    _ => unreachable!("Only allow one platform at a time"),
                }
            }
        }
    } else if #[cfg(feature = "native")]{
        impl RouteEnv {
            pub async fn new(_param: InitRouteEnvParams) -> Result<Self> {
                    let mut pwd = std::env::current_dir().map_err(
                        |err| anyhow::anyhow!("Failed to get current directory: {}", err)
                    )?;
                    pwd.push("target/cache");
                    std::fs::create_dir_all(&pwd).map_err(
                        |err| anyhow::anyhow!("Failed to create directory: {}", err)
                    )?;

                Ok(Self {
                    sql: Arc::new(init_db(InitSQLParams::Native {
                        url: {
                            let mut pwd = std::env::current_dir().map_err(
                                |err| anyhow::anyhow!("Failed to get current directory: {}", err)
                            )?;
                            pwd.push("target/cache/site.db");
                            format!("sqlite://{}?mode=rwc", pwd.to_string_lossy())
                        }
                    }).await?),
                    kv: RouteEnvKV {
                        token_expired: Arc::new(
                            init_kv(InitKVParams::Native {
                                path: {
                                    let mut pwd = std::env::current_dir().map_err(
                                        |err| anyhow::anyhow!("Failed to get current directory: {}", err)
                                    )?;
                                    pwd.push("target/cache/token-expired.db");
                                    pwd.to_string_lossy().to_string()
                                }
                            })
                            .await?,
                        ),
                        global_config: Arc::new(
                            init_kv(InitKVParams::Native {
                                path: {
                                    let mut pwd = std::env::current_dir().map_err(
                                        |err| anyhow::anyhow!("Failed to get current directory: {}", err)
                                    )?;
                                    pwd.push("target/cache/global-config.db");
                                    pwd.to_string_lossy().to_string()
                                }
                            })
                            .await?,
                        ),
                    },
                    bucket: RouteEnvBucket {
                        static_resources: Arc::new(
                            init_bucket(InitBucketParams::Native {
                                path: {
                                    let mut pwd = std::env::current_dir().map_err(
                                        |err| anyhow::anyhow!("Failed to get current directory: {}", err)
                                    )?;
                                    pwd.push("target/cache/static-resources");

                                    std::fs::create_dir_all(&pwd).map_err(
                                        |err| anyhow::anyhow!("Failed to create directory: {}", err)
                                    )?;

                                    pwd.to_string_lossy().to_string()
                                }
                            })
                            .await?,
                        ),
                    },
                })
            }
        }
    } else if #[cfg(feature = "wasi")] {
        impl RouteEnv {
            pub async fn new(_param: InitRouteEnvParams) -> Result<Self> {
                Ok(Self {
                    sql: Arc::new(init_db(InitSQLParams::WASI).await?),
                    kv: RouteEnvKV {
                        token_expired: Arc::new(
                            init_kv(InitKVParams::WASI {
                                name: "token-expired".to_string()
                            }).await?,
                        ),
                        global_config: Arc::new(
                            init_kv(InitKVParams::WASI {
                                name: "global-config".to_string()
                            }).await?,
                        ),
                    },
                    bucket: RouteEnvBucket {
                        static_resources: Arc::new(
                            init_bucket(InitBucketParams::WASI {
                                name: "static".to_string()
                            }).await?,
                        ),
                    },
                })
            }
        }
    }
}
