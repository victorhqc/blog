use async_trait::async_trait;
use rocket::figment::{providers::Serialized, Figment};
use sea_orm::ConnectOptions;
use sea_orm_rocket::{Config, Database};
use std::{env, str, time::Duration};

#[derive(Database, Debug)]
#[database("blog_api")]
pub struct Db(SeaOrmPool);

#[derive(Debug, Clone)]
pub struct SeaOrmPool {
    pub conn: sea_orm::DatabaseConnection,
}

#[async_trait]
impl sea_orm_rocket::Pool for SeaOrmPool {
    type Error = sea_orm::DbErr;

    type Connection = sea_orm::DatabaseConnection;

    async fn init(figment: &Figment) -> Result<Self, Self::Error> {
        let config = figment.extract::<Config>().unwrap();
        let pool = build_pool(&config).await?;

        Ok(pool)
    }

    fn borrow(&self) -> &Self::Connection {
        &self.conn
    }
}

pub async fn build_pool(config: &Config) -> Result<SeaOrmPool, sea_orm::DbErr> {
    let mut options: ConnectOptions = config.url.clone().into();
    options
        .max_connections(config.max_connections as u32)
        .min_connections(config.min_connections.unwrap_or_default())
        .connect_timeout(Duration::from_secs(config.connect_timeout));
    if let Some(idle_timeout) = config.idle_timeout {
        options.idle_timeout(Duration::from_secs(idle_timeout));
    }
    let conn = sea_orm::Database::connect(options).await?;

    Ok(SeaOrmPool { conn })
}

// Code gotten from:
// https://github.com/SeaQL/sea-orm/blob/f58d890df5bac944d288a3deb8f96d807d27b54b/sea-orm-rocket/lib/src/database.rs
// This is a workaround in order to build the pool before rocket does.
pub fn get_figment_before_build() -> Figment {
    let default_config = rocket::Config::default();
    let workers: usize = default_config.workers;
    let url = env::var("DATABASE_URL").expect("DATABASE_URL IS NOT SET");

    Figment::from(default_config)
        .merge(Serialized::defaults(rocket::Config::default()))
        .merge(Serialized::default("databases.blog_api.url", url))
        .focus("databases.blog_api")
        .merge(Serialized::default("max_connections", workers * 4))
        .merge(Serialized::default("connect_timeout", 5))
}
