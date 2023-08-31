#![forbid(unsafe_code)]

use config::Config;
use services::website_crawl_service;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions,
};
use std::str::FromStr;
#[macro_use]
extern crate rocket;

mod config;
mod db;
mod models;
mod services;
mod util;

#[launch]
async fn rocket() -> _ {
    let config = Config::new().expect("Failed to load config");

    // Setup DB
    let mut pg_options =
        PgConnectOptions::from_str(&config.database_url).expect("Invalid database url provided");

    if !config.log_db_statements {
        pg_options = pg_options.disable_statement_logging();
    }

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(pg_options)
        .await
        .expect("Failed to connect to DB");

    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to run migrations");

    website_crawl_service::crawl_websites(&db_pool)
        .await
        .expect("Failed to crawl website");

    rocket::build()
}
