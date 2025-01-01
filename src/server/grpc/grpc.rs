use tonic::{Request, Response, Status};

use crate::{config::AppConfig, utils::HttpClient, RedisClientExt, SimpleCache};
use std::sync::Arc;

// 引入生成的代码
pub mod proto {
    tonic::include_proto!("grpc.service");
}

use proto::{your_grpc_service_server::YourGrpcService, HelloRequest, HelloResponse};

// #[derive(Clone)]
pub struct YourGrpcServiceImpl {
    // // 允许 cache 字段未被读取
    #[allow(dead_code)]
    cache: SimpleCache,
    // 允许 config 字段未被读取
    #[allow(dead_code)]
    config: Arc<AppConfig>,
    // // 允许 http_repository 字段未被读取
    #[allow(dead_code)]
    http_repository: HttpClient,
}

impl YourGrpcServiceImpl {
    pub fn new(config: Arc<AppConfig>, cache: SimpleCache, http_repository: HttpClient) -> Self {
        Self {
            config,
            cache,
            http_repository,
        }
    }
}

#[tonic::async_trait]
impl YourGrpcService for YourGrpcServiceImpl {
    // 实现简单的 RPC 方法
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        // 添加详细日志
        tracing::info!("Received gRPC request: {:?}", request);

        let key = "key";
        let value = self
            .cache
            .get(key)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .unwrap_or_else(|| "default".to_string());
        tracing::debug!("Processing request for name: {}", value);

        let reply = HelloResponse {
            message: format!("Hello {}!", "world"),
        };

        tracing::info!("Sending response: {:?}", reply);
        Ok(Response::new(reply))
    }
}
