use tonic::{Request, Response, Status};

use crate::config::AppConfig;
use std::sync::Arc;

// 引入生成的代码
pub mod proto {
    tonic::include_proto!("grpc.service");
}

use proto::{
    hello_world_grpc_service_server::HelloWorldGrpcService, HelloWorldRequest, HelloWorldResponse,
};

// #[derive(Clone)]
pub struct HelloWorldGrpcServiceImpl {
    // // 允许 cache 字段未被读取
    // #[allow(dead_code)]
    // cache: SimpleCache,
    // 允许 config 字段未被读取
    #[allow(dead_code)]
    config: Arc<AppConfig>,
    // // 允许 http_repository 字段未被读取
    // #[allow(dead_code)]
    // http_repository: HttpClient,
}

impl HelloWorldGrpcServiceImpl {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self {
            config,
            // cache,
            // http_repository,
        }
    }
}

#[tonic::async_trait]
impl HelloWorldGrpcService for HelloWorldGrpcServiceImpl {
    // 实现简单的 RPC 方法
    async fn say_hello_world(
        &self,
        request: Request<HelloWorldRequest>,
    ) -> Result<Response<HelloWorldResponse>, Status> {
        // 添加详细日志
        tracing::info!("Received gRPC request: {:?}", request);

        let name = request.into_inner().name;
        tracing::debug!("Processing request for name: {}", name);

        let reply = HelloWorldResponse {
            message: format!("Hello {}!", name),
        };

        tracing::info!("Sending response: {:?}", reply);
        Ok(Response::new(reply))
    }
}
