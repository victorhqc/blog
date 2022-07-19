use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket_cors::{Cors, CorsOptions};
use std::env;

pub fn init_cors() -> Cors {
    let allowed_origins = env::var("ALLOWED_ORIGINS").unwrap_or_else(|_| String::from(""));
    let allowed_origins = match allowed_origins.as_str() {
        "" => AllowedOrigins::all(),
        _ => {
            let origins = allowed_origins.split(',').collect::<Vec<&str>>();
            AllowedOrigins::some_exact(&origins)
        }
    };

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Failed to load CORS")
}
