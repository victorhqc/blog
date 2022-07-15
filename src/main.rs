#[macro_use]
extern crate rocket;

mod authorization;
mod aws;
mod db;
mod graphql;
mod uploads;
mod user;
mod utils;

use crate::{
    authorization::enforcer::init_enforcer,
    aws::build_client,
    db::{build_pool, get_figment_before_build, Db},
    graphql::{
        context::{AWSContext, AppContext},
        export_sdl,
        loader::DataLoader as AppLoader,
        routes::{graphql_playground, graphql_query, graphql_request, graphql_request_multipart},
        ApiSchema, MutationRoot, QueryRoot,
    },
    utils::cors::init_cors,
};
use async_graphql::{dataloader::DataLoader, EmptySubscription, Schema};
use async_mutex::Mutex;
use dotenv::dotenv;
use migration::MigratorTrait;
use rocket::fairing::{self, AdHoc};
use rocket::serde::{json::Json, Serialize};
use rocket::{Build, Rocket};
use sea_orm_rocket::{Config, Connection, Database};
use std::{env, sync::Arc};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();

    let export_graphql_sdl = env::var("EXPORT_GRAPHQL_SDL").unwrap_or_else(|_| String::from("0"));

    let figment = get_figment_before_build();
    let config = figment.extract::<Config>().expect("Failed to load config");
    let pool = build_pool(&config).await.expect("Failed to init pool");
    let enforcer = init_enforcer().await.expect("Failed to init enforcer");
    let enforcer_api = init_enforcer().await.expect("Failed to init enforcer");
    let client = build_client().await.expect("Failed to init AWS Client");
    let aws = AWSContext::default()
        .await
        .expect("Failed to create AWSContext");

    let app_loader = AppLoader::new(&pool);

    let context = AppContext {
        enforcer: Arc::new(Mutex::new(enforcer)),
        aws,
    };

    let schema: ApiSchema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(context)
    .data(DataLoader::new(app_loader, async_std::task::spawn))
    .finish();

    if export_graphql_sdl == "1" || export_graphql_sdl == "true" {
        export_sdl(&schema).unwrap();
    }

    let _rocket = rocket::build()
        .manage(schema)
        .manage(client)
        .manage(enforcer_api)
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .attach(init_cors())
        .mount(
            "/",
            routes![
                index,
                graphql_query,
                graphql_request,
                graphql_request_multipart,
                graphql_playground,
            ],
        )
        .launch()
        .await?;

    Ok(())
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

#[get("/")]
async fn index(_conn: Connection<'_, Db>) -> Json<Hello> {
    Json(Hello {
        message: "Hello, this the API for my blog.".into(),
    })
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Hello {
    message: String,
}
