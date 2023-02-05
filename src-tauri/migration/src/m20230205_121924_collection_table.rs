use sea_orm_migration::{prelude::*, sea_orm::{Statement, ConnectionTrait}};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            CREATE TABLE IF NOT EXISTS collection (
                id serial,
                name varchar(100) not null,
                created_at timestamp with time zone default current_timestamp,
                config_id bigserial,
                primary key(id),
                constraint fk_config_id
                    foreign key(config_id)
                        references config(id)
                        on delete set null
            )
        "#;
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_string());
        manager.get_connection().execute(statement).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
       let sql = r#"
            DROP TABLE collection
       "#;
       let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
       manager.get_connection().execute(statement).await.map(|_| ()) 
    }
}

