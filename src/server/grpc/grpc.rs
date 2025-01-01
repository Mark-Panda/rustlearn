use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use crate::{config::AppConfig, utils::HttpClient, SimpleCache};
use std::sync::Arc;

// 引入生成的代码
pub mod proto {
    tonic::include_proto!("grpc.service");
}

use proto::{your_grpc_service_server::YourGrpcService, HelloRequest, HelloResponse};

// #[derive(Clone)]
pub struct YourGrpcServiceImpl {
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

impl YourGrpcServiceImpl {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self {
            config,
            // cache,
            // http_repository,
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

        let name = request.into_inner().name;
        tracing::debug!("Processing request for name: {}", name);

        let reply = HelloResponse {
            message: format!("Hello {}!", name),
        };

        tracing::info!("Sending response: {:?}", reply);
        Ok(Response::new(reply))
    }

    // 实现服务端流式 RPC
    type ServerStreamStream = ReceiverStream<Result<HelloResponse, Status>>;

    async fn server_stream(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<Self::ServerStreamStream>, Status> {
        let name = request.into_inner().name;

        let (tx, rx) = mpsc::channel(4);

        // 在新的任务中发送多个响应
        tokio::spawn(async move {
            for i in 0..3 {
                let reply = HelloResponse {
                    message: format!("Hello {} number {}", name, i),
                };

                tx.send(Ok(reply)).await.unwrap();
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    // 实现客户端流式 RPC
    async fn client_stream(
        &self,
        request: Request<tonic::Streaming<HelloRequest>>,
    ) -> Result<Response<HelloResponse>, Status> {
        let mut stream = request.into_inner();
        let mut messages = Vec::new();

        // 接收所有客户端消息
        while let Some(req) = stream.message().await? {
            messages.push(req.name);
        }

        let reply = HelloResponse {
            message: format!("Received names: {}", messages.join(", ")),
        };

        Ok(Response::new(reply))
    }

    // 实现双向流式 RPC
    type BidirectionalStreamStream = ReceiverStream<Result<HelloResponse, Status>>;

    async fn bidirectional_stream(
        &self,
        request: Request<tonic::Streaming<HelloRequest>>,
    ) -> Result<Response<Self::BidirectionalStreamStream>, Status> {
        let mut stream = request.into_inner();
        let (tx, rx) = mpsc::channel(4);

        // 在新的任务中处理双向流
        tokio::spawn(async move {
            while let Some(req) = stream.message().await.unwrap() {
                let reply = HelloResponse {
                    message: format!("Echo: {}", req.name),
                };

                tx.send(Ok(reply)).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
