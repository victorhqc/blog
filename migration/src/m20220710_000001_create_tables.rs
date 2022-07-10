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
        let backend = manager.get_database_backend();
        let conn = manager.get_connection();

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
        let users = Statement::from_string(backend, sql.to_owned());

        let sql = r#"
        CREATE TABLE `tags` (
            uuid BLOB PRIMARY KEY NOT NULL,
            name TEXT UNIQUE NOT NULL,
            created_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
            updated_at TIMESTAMP DEFAULT current_timestamp NOT NULL
        );
        "#;
        let tags = Statement::from_string(backend, sql.to_owned());

        let sql = r#"
        CREATE TABLE `posts` (
            uuid BLOB PRIMARY KEY NOT NULL,
            status TEXT NOT NULL,
            title TEXT UNIQUE NOT NULL,
            raw TEXT NOT NULL,
            html TEXT NOT NULL,
            created_by BLOB NOT NULL,
            created_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
            updated_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
            FOREIGN KEY (created_by)
            REFERENCES users (uuid)
                ON DELETE CASCADE
                ON UPDATE CASCADE
        );
        "#;
        let posts = Statement::from_string(backend, sql.to_owned());

        let sql = r#"
        CREATE TABLE `post_tags` (
            uuid BLOB PRIMARY KEY NOT NULL,
            post_uuid BLOB NOT NULL,
            tag_uuid BLOB NOT NULL,
            FOREIGN KEY (post_uuid)
            REFERENCES posts (uuid)
                ON DELETE CASCADE
                ON UPDATE CASCADE,
            FOREIGN KEY (tag_uuid)
            REFERENCES tags (uuid)
                ON DELETE CASCADE
                ON UPDATE CASCADE
        );
        "#;
        let post_tags = Statement::from_string(backend, sql.to_owned());

        conn.execute(users).await?;
        conn.execute(tags).await?;
        conn.execute(posts).await?;
        conn.execute(post_tags).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let backend = manager.get_database_backend();
        let conn = manager.get_connection();

        let sql = r#"
        DROP TABLE `post_tags`;
        "#;
        let post_tags = Statement::from_string(backend, sql.to_owned());

        let sql = r#"
        DROP TABLE `posts`;
        "#;
        let posts = Statement::from_string(backend, sql.to_owned());

        let sql = r#"
        DROP TABLE `tags`;
        "#;
        let tags = Statement::from_string(backend, sql.to_owned());

        let sql = r#"
        DROP TABLE `users`;
        "#;
        let users = Statement::from_string(backend, sql.to_owned());

        conn.execute(post_tags).await?;
        conn.execute(posts).await?;
        conn.execute(tags).await?;
        conn.execute(users).await?;

        Ok(())
    }
}
