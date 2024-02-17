use std::sync::Arc;
use async_trait::async_trait;
use redis::{ AsyncCommands, Client, RedisError};
use anyhow::Result;
use tracing::info;
use crate::{error::AppResult, server::error::Error};

pub type RedisClient = Client;

#[derive(Debug, Clone)]
pub struct SimpleCache {
    pub client: RedisClient,
}

impl SimpleCache  {
    pub async fn connect(connection_string: &str) -> Result<Self> {
        let client = Client::open(connection_string)?;
        Ok(Self { client })
    }
}
pub type DynRedisClientExt = Arc<dyn RedisClientExt + Send + Sync>;

#[async_trait]
pub trait RedisClientExt{
  async fn ping(&self) -> AppResult<String>;
  async fn set(&self, key: &str, value: &str, expire: u64) -> Result<(), RedisError>;
  async fn exist(&self, key: &str) -> Result<bool, RedisError>;
  async fn get(&self, key: &str) -> Result<Option<String>, RedisError>;
  async fn del(&self, key: &str) -> Result<bool, RedisError>;
  async fn ttl(&self, key: &str) -> Result<i64, RedisError>;
}

#[async_trait]
impl RedisClientExt for SimpleCache {
  async fn ping(&self) -> AppResult<String> {
    let mut conn = self.client.get_async_connection().await.unwrap();
    let value: Option<String> = redis::cmd("PING").query_async(&mut conn).await.unwrap();
    info!("ping redis server");
    if let Some(string) = value {
      Ok(string)
    }else {
      info!("ping redis server type no fond");
      return Err(Error::NotFound(String::from("user email does not exist")));
    }
    
  }

  async fn set(&self, key: &str, value: &str, expire: u64) -> Result<(), RedisError> {
    let mut conn = self.client.get_async_connection().await?;
    conn.set_ex(key, value, expire).await?;
    // let msg: String = redis::cmd("SET")
    //   .arg(&[key, value])
    //   .query_async(&mut conn)
    //   .await?;
    // let msg: i32 = redis::cmd("EXPIRE")
    //   .arg(&[key, &expire.as_secs().to_string()])
    //   .query_async(&mut conn)
    //   .await?;
    Ok(())
  }

  async fn exist(&self, key: &str) -> Result<bool, RedisError> {
    let mut conn = self.client.get_async_connection().await?;
    let value: bool = redis::cmd("EXISTS").arg(key).query_async(&mut conn).await?;
    info!("check key exists: {key}");
    Ok(value)
  }

  async fn get(&self, key: &str) -> Result<Option<String>, RedisError> {
    let mut conn = self.client.get_async_connection().await?;
    let value: Option<String> = redis::cmd("GET").arg(key).query_async(&mut conn).await?;
    info!("get value: {key}");
    Ok(value)
  }

  async fn del(&self, key: &str) -> Result<bool, RedisError> {
    let mut conn = self.client.get_async_connection().await?;
    let value: i32 = redis::cmd("DEL").arg(key).query_async(&mut conn).await?;
    info!("delete value: {key}");
    Ok(value == 1)
  }
  async fn ttl(&self, key: &str) -> Result<i64, RedisError> {
    let mut conn = self.client.get_async_connection().await?;
    let value: i64 = redis::cmd("TTL").arg(key).query_async(&mut conn).await?;
    info!("get TTL value: {key}");
    Ok(value)
  }
}

