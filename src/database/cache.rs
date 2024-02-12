
use redis::{aio::Connection, cluster::ClusterClient, cluster_async::ClusterConnection, Client};
use anyhow::{Context, Result};

pub struct ClusterCache {
    pub client: ClusterConnection,
}

impl ClusterCache  {
    pub async fn connect(connection_string: &str) -> Result<Self> {
        let nodes = vec![connection_string];
        let client = ClusterClient::new(nodes).unwrap();
        let cache = client.get_async_connection().await.context("error while initializing the cache connection ")?;
        Ok(Self { client: cache })
    }
}

pub struct SimpleCache {
    pub client: Connection,
}

impl SimpleCache  {
    pub async fn connect(connection_string: &str) -> Result<Self> {
        let client = Client::open(connection_string)?;
        let cache = client.get_async_connection().await.context("error while initializing the cache connection ")?;
        Ok(Self { client: cache })
    }
}