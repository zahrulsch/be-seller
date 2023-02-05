pub use sea_orm_migration::prelude::*;

mod m20230123_102114_create_config_table;
mod m20230205_121924_collection_table;
mod m20230205_125159_collection_product_table;



pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230123_102114_create_config_table::Migration),
            Box::new(m20230205_121924_collection_table::Migration),
            Box::new(m20230205_125159_collection_product_table::Migration),
        ]
    }
}
