use anyhow::{Context, Result};
use nacos_rust_client::client::config_client::{ConfigClient, ConfigKey};
use nacos_rust_client::client::{AuthInfo, ClientBuilder};
use std::env;
use std::sync::Arc;
/// Nacos client wrapper

pub struct NacosClient {
    client: Arc<ConfigClient>,
}

impl NacosClient {
    /// Create a new Nacos client from environment variables
    pub async fn new() -> Result<Self> {
        // 从环境变量获取 Nacos 配置
        let nacos_addr =
            env::var("NACOS_ADDR").context("NACOS_ADDR environment variable not set")?;

        let tenant = "public".to_owned(); //default teant

        //let auth_info = Some(AuthInfo::new("nacos","nacos"));
        // let auth_info = None;
        let auth_info = match (env::var("NACOS_USERNAME"), env::var("NACOS_PASSWORD")) {
            (Ok(username), Ok(password)) => Some(AuthInfo::new(&username, &password)),
            _ => None,
        };
        // 初始化 Nacos 客户端配置
        let client = ClientBuilder::new()
            .set_endpoint_addrs(&nacos_addr)
            .set_auth_info(auth_info)
            .set_tenant(tenant)
            .set_use_grpc(true)
            .build_config_client();
        let client = Self { client };
        Ok(client)
    }

    /// Get configuration from Nacos
    pub async fn get_config(&self, data_id: &str, group: &str) -> Result<String> {
        let key = ConfigKey::new(data_id, group, "" /*tenant_id*/);
        let v = self
            .client
            .get_config(&key)
            .await
            .context("failed to get config from nacos")?;
        Ok(v)
    }
}
