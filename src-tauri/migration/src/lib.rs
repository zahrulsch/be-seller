pub use sea_orm_migration::prelude::*;

mod m20230123_102114_create_config_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230123_102114_create_config_table::Migration),
        ]
    }
}
