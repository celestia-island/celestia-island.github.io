use anyhow::Result;

use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder, QuerySelect};

use crate::RouteEnv;
use _server_database::models::*;

pub async fn select(env: RouteEnv, id: i64) -> Result<Option<user::Model>> {
    let ret = user::Entity::find_by_id(id).one(&**env.sql).await?;

    Ok(ret)
}

pub async fn count(env: RouteEnv) -> Result<u64> {
    let count = user::Entity::find().count(&**env.sql).await?;

    Ok(count)
}

pub async fn list(env: RouteEnv, offset: usize, limit: usize) -> Result<Vec<user::Model>> {
    let ret = user::Entity::find()
        .offset(offset as u64)
        .limit(limit as u64)
        .order_by_asc(user::Column::Id)
        .all(&**env.sql)
        .await?;

    Ok(ret)
}
