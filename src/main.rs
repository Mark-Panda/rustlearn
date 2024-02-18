use anyhow::Context;
use clap::Parser;
use dotenvy::dotenv;
use std::sync::Arc;
use system_test::{AppConfig, ApplicationServer, Database, Logger, SimpleCache};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let config = Arc::new(AppConfig::parse());

    let _guard = Logger::init(config.cargo_env);

    info!("environment loaded and configuration parsed, initializing Postgres connection...");
    let db = Database::connect(&config.database_url)
        .await
        .expect("could not initialize the database connection pool");

    let cache = SimpleCache::connect(&config.cache_url)
        .await
        .expect("could not initialize the cache connection ");

    ApplicationServer::serve(config, db, cache)
        .await
        .context("could not initialize application routes")?;

    Ok(())
}
