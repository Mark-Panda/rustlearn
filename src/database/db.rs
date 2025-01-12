use anyhow::{Context, Ok};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub type ConnectionPool = Pool<Postgres>;

/// Postgres database
#[derive(Debug, Clone)]
pub struct Database {
    pub pool: ConnectionPool,
}

impl Database {
    pub async fn connect(connection_string: &str) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection_string)
            .await
            .context("error while initializing the database connection pool")?;
        Ok(Self { pool })
    }
}
