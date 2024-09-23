use anyhow::Result;

use sea_orm::{ActiveModelTrait, EntityTrait};

use crate::RouteEnv;
use _server_database::prelude::*;
use _types::website::request::models::User as VO;

pub async fn insert(env: RouteEnv, vo: VO) -> Result<user::Model> {
    let vo: user::ActiveModel = vo.into();
    let ret: user::Model = vo.insert(&**env.sql).await?;

    Ok(ret)
}

pub async fn update(env: RouteEnv, id: i64, vo: VO) -> Result<user::Model> {
    let mut vo: user::ActiveModel = vo.into();
    vo.set(user::Column::Id, id.into());

    let ret: user::Model = vo.update(&**env.sql).await?;

    Ok(ret)
}

pub async fn delete(env: RouteEnv, id: i64) -> Result<()> {
    user::Entity::delete_by_id(id).exec(&**env.sql).await?;

    Ok(())
}
