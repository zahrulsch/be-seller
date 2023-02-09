use sea_orm_migration::{prelude::*, sea_orm::{Statement, ConnectionTrait}};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            CREATE TABLE IF NOT EXISTS collection_product (
                item_id bigint not null,
                shop_id bigint not null,
                url text not null,
                name text not null,
                thumbnail text,
                view bigint,
                sold bigint,
                price_min bigint,
                price_max bigint,
                collection_id serial,
                primary key(item_id, collection_id),
                constraint fk_collection_id
                    foreign key(collection_id)
                        references collection(id)
                            on delete cascade
                            on update cascade
            )
        "#;

        let statement = Statement::from_string(manager.get_database_backend(), sql.to_string());
        manager.get_connection().execute(statement).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            DROP TABLE collection_product
        "#;
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_string());
        manager.get_connection().execute(statement).await.map(|_| ())
    }
}

