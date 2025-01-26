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

    /// Inject Nacos configuration into environment variables
    pub async fn inject_nacos_config(&self, data_id: &str, group: &str) -> anyhow::Result<()> {
        let key = ConfigKey::new(data_id, group, "" /*tenant_id*/);
        let nacos_config = self
            .client
            .get_config(&key)
            .await
            .context("failed to get config from nacos")?;
        println!("直接打印配置内容: {:?}", nacos_config);

        let config: serde_json::Value = serde_json::from_str(&nacos_config)?;
        if let serde_json::Value::Object(map) = config {
            for (key, value) in map {
                let value_str = match value {
                    serde_json::Value::String(s) => s,
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    _ => continue, // 跳过其他类型
                };
                println!("nacos_key: {}, nacos_value: {}", key, value_str);
                env::set_var(key, value_str);
            }
        }
        Ok(())
    }
}
