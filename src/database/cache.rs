use crate::{error::AppResult, server::error::Error};
use anyhow::Result;
use async_trait::async_trait;
use redis::{AsyncCommands, Client, RedisError};
use std::sync::Arc;
use tracing::info;

pub type RedisClient = Client;

#[derive(Debug, Clone)]
pub struct SimpleCache {
    pub client: RedisClient,
}

impl SimpleCache {
    pub async fn connect(connection_string: &str) -> Result<Self> {
        let client = Client::open(connection_string)?;
        Ok(Self { client })
    }
}
pub type DynRedisClientExt = Arc<dyn RedisClientExt + Send + Sync>;

#[async_trait]
pub trait RedisClientExt {
    async fn ping(&self) -> AppResult<String>;
    /// Redis SET 命令用于设置给定 key 的值。如果 key 已经存储其他值， SET 就覆写旧值，且无视类型。
    async fn set(&self, key: &str, value: &str) -> Result<String, RedisError>;
    async fn exist(&self, key: &str) -> Result<bool, RedisError>;
    /// Redis Get 命令用于获取指定 key 的值。如果 key 不存在，返回 nil 。如果key 储存的值不是字符串类型，返回一个错误。
    async fn get(&self, key: &str) -> Result<Option<String>, RedisError>;
    /// Redis DEL 命令用于删除已存在的键。不存在的 key 会被忽略。
    async fn del(&self, key: &str) -> Result<bool, RedisError>;
    /// Redis TTL 命令以秒为单位返回 key 的剩余过期时间。
    async fn ttl(&self, key: &str) -> Result<i64, RedisError>;
    /// setex Redis Setex 命令为指定的 key 设置值及其过期时间。如果 key 已经存在， SETEX 命令将会替换旧的值。 设置成功时返回 OK 。
    async fn set_ex(&self, key: &str, value: &str, expire: u64) -> Result<String, RedisError>;
}

#[async_trait]
impl RedisClientExt for SimpleCache {
    async fn ping(&self) -> AppResult<String> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .unwrap();
        let value: Option<String> = redis::cmd("PING").query_async(&mut conn).await.unwrap();
        info!("ping redis server");
        if let Some(string) = value {
            Ok(string)
        } else {
            info!("ping redis server type no fond");
            return Err(Error::NotFound(String::from(
                "ping redis server type no fond",
            )));
        }
    }

    async fn set(&self, key: &str, value: &str) -> Result<String, RedisError> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let msg: String = redis::cmd("SET")
            .arg(&[key, value])
            .query_async(&mut conn)
            .await?;
        info!("set key: {key}, {msg}");
        Ok(msg)
    }

    async fn exist(&self, key: &str) -> Result<bool, RedisError> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let value: bool = redis::cmd("EXISTS").arg(key).query_async(&mut conn).await?;
        info!("check key exists: {key}");
        Ok(value)
    }

    async fn get(&self, key: &str) -> Result<Option<String>, RedisError> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let value: Option<String> = redis::cmd("GET").arg(key).query_async(&mut conn).await?;
        info!("get value: {key}");
        Ok(value)
    }

    async fn del(&self, key: &str) -> Result<bool, RedisError> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let value: i32 = redis::cmd("DEL").arg(key).query_async(&mut conn).await?;
        info!("delete value: {key}");
        Ok(value == 1)
    }
    async fn ttl(&self, key: &str) -> Result<i64, RedisError> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let value: i64 = redis::cmd("TTL").arg(key).query_async(&mut conn).await?;
        info!("get TTL value: {key}");
        Ok(value)
    }

    async fn set_ex(&self, key: &str, value: &str, expire: u64) -> Result<String, RedisError> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let result: String = redis::cmd("SETEX")
            .arg(key)
            .arg(expire)
            .arg(value)
            .query_async(&mut conn)
            .await?;
        info!("setex key: {key}, {result}");
        Ok(result)
    }
}
