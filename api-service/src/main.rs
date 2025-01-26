use anyhow::Context;
use api_service::{cron::CronJobs, AppConfig, ApplicationServer, Logger, OpenAiClient};
use clap::Parser;
use dotenvy::dotenv;
use rutils::{Database, NacosClient, RCache};
use std::sync::Arc;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let config = Arc::new(AppConfig::parse());
    // 初始化日志
    let _guard = Logger::init(config.cargo_env);

    let nacos = NacosClient::new().await?;
    nacos.inject_nacos_config("test", "DEFAULT_GROUP").await?;

    info!("environment loaded and configuration parsed, initializing Postgres connection...");
    let db = Database::connect(&config.database_url)
        .await
        .expect("could not initialize the database connection pool");

    let ai_client = OpenAiClient::connect()
        .await
        .expect("could not initialize the openai connection ");

    let cache = RCache::connect(&config.cache_url)
        .await
        .expect("could not initialize the cache connection ");

    // 初始化并启动定时任务
    let cron_jobs = CronJobs::new().await?;
    cron_jobs.setup().await?;

    ApplicationServer::serve(config, db, cache, ai_client)
        .await
        .context("could not initialize application routes")?;

    Ok(())
}
