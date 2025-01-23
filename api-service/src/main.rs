use anyhow::Context;
use api_service::{AppConfig, ApplicationServer, Logger, OpenAiClient};
use clap::Parser;
use dotenvy::dotenv;
use rutils::Scheduler;
use rutils::{Database, RCache};
use std::sync::Arc;
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

    let ai_client = OpenAiClient::connect()
        .await
        .expect("could not initialize the openai connection ");

    let cache = RCache::connect(&config.cache_url)
        .await
        .expect("could not initialize the cache connection ");

    // 创建调度器实例
    let scheduler = Scheduler::new().await?;
    // 添加定时任务（每分钟执行一次）
    scheduler
        .add_job("1/10 * * * * *", || async {
            println!("执行定时任务");
        })
        .await?;
    // 启动调度器
    scheduler.start().await?;

    ApplicationServer::serve(config, db, cache, ai_client)
        .await
        .context("could not initialize application routes")?;

    Ok(())
}
