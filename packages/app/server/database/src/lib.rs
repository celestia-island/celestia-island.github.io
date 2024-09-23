pub mod models;
pub mod types;

use anyhow::Result;

use sea_orm::{ConnectionTrait, DatabaseConnection};

#[allow(unused_imports)]
pub mod prelude {
    pub use crate::models::*;
    pub use crate::types::*;
}

macro_rules! create_table {
    ($db:path, $builder:path, $table:ident) => {
        $db.execute(
            $builder.build(
                ::sea_orm::Schema::new($builder)
                    .create_table_from_entity(models::$table::Entity)
                    .if_not_exists(),
            ),
        )
        .await?;
    };
}

pub async fn init_tables(db: &DatabaseConnection) -> Result<()> {
    let builder = db.get_database_backend();

    create_table!(db, builder, user);

    Ok(())
}
