use tonic::{Request, Response, Status};

use crate::{
    config::AppConfig,
    server::{dtos::ai_dto::ChatMessageDto, services::ai_services::DynOpenAisService},
    utils::HttpClient,
    RusCache,
};
use std::sync::Arc;

// 引入生成的代码
pub mod proto {
    tonic::include_proto!("grpc.service");
}

use proto::{user_grpc_service_server::UserGrpcService, UserRequest, UserResponse};

// #[derive(Clone)]
pub struct UserGrpcServiceImpl {
    // // 允许 cache 字段未被读取
    #[allow(dead_code)]
    cache: RusCache,
    // 允许 config 字段未被读取
    #[allow(dead_code)]
    config: Arc<AppConfig>,
    // // 允许 http_repository 字段未被读取
    #[allow(dead_code)]
    http_repository: HttpClient,
    openais: DynOpenAisService,
}

impl UserGrpcServiceImpl {
    pub fn new(
        config: Arc<AppConfig>,
        cache: RusCache,
        http_repository: HttpClient,
        openais: DynOpenAisService,
    ) -> Self {
        Self {
            config,
            cache,
            http_repository,
            openais,
        }
    }
}

#[tonic::async_trait]
impl UserGrpcService for UserGrpcServiceImpl {
    // 实现简单的 RPC 方法
    async fn say_user_hello(
        &self,
        request: Request<UserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        // 添加详细日志
        tracing::info!("Received gRPC request: {:?}", request);

        // 调用 AI 服务
        let chat_request = ChatMessageDto {
            chat_id: Some("test_chat".to_string()),
            message: Some(request.into_inner().request_msg),
        };

        let chat_result = self
            .openais
            .chat_message(chat_request)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let reply = UserResponse {
            message: chat_result.message,
        };

        tracing::info!("Sending response: {:?}", reply);
        Ok(Response::new(reply))
    }
}
