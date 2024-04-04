use std::sync::Arc;

use anyhow::Result;
use async_openai::{config::OpenAIConfig, Client};

/// OpenAI API client wrapper
#[derive(Clone)]
pub struct OpenAiClient {
    // 使用 Arc 来共享 Client 实例
    pub client: Arc<Client<OpenAIConfig>>,
}

impl OpenAiClient {
    pub async fn connect() -> Result<Self> {
        // 创建 Client 实例
        let client = Client::new();
        
        // 使用 Arc 包装 Client 以便可以共享
        Ok(Self {
            client: Arc::new(client),
        })
    }
}

