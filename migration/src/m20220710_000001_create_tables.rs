use sea_orm::Statement;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::ConnectionTrait;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220710_000001_create_tables"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
        CREATE TABLE `users` (
            uuid BLOB PRIMARY KEY NOT NULL,
            email TEXT UNIQUE NOT NULL,
            password TEXT NOT NULL,
            role TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
            updated_at TIMESTAMP DEFAULT current_timestamp NOT NULL
        );
        "#;
        let users_stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());

        manager.get_connection().execute(users_stmt).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
        DROP TABLE `users`;
        "#;
        let users_stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());

        manager.get_connection().execute(users_stmt).await?;

        Ok(())
    }
}
