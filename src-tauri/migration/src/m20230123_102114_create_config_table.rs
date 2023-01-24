use sea_orm_migration::{prelude::*, sea_orm::ConnectionTrait};
use sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
        CREATE TABLE IF NOT EXISTS config (
            id bigserial PRIMARY KEY,
            name varchar(100) not null,
            created_at timestamp with time zone default current_timestamp,
            seller_types json not null default '[]',
            price_ranges json not null default '[]',
            minimum_star int not null default 4,
            minimum_sold int not null default 10,
            minimum_stock int not null default 1,
            sort_by varchar(100) not null  default 'relevancy',
            shippings json not null default '[]',
            exclude_cod bool not null default false,
            ban_keywords json not null default '[]',
            cities json not null default '[]'
        )
        "#;

        let statement = Statement::from_string(manager.get_database_backend(), sql.to_string());
        manager.get_connection().execute(statement).await.map(|_| ())
    }
    
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE config";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_string());
        manager.get_connection().execute(statement).await.map(|_| ())
    }
}

