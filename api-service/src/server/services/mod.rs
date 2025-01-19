use std::sync::Arc;

use tracing::info;

use self::ai_services::DynOpenAisService;
use crate::{
    config::AppConfig,
    server::{services::ai_services::OpenAisService, utils::jwt_utils::JwtTokenUtil},
    utils::HttpClient,
    OpenAiClient,
};
use rutils::{Database, RusCache};

use super::utils::jwt_utils::DynJwtUtil;

pub mod ai_services;

#[derive(Clone)]
pub struct Services {
    pub jwt_util: DynJwtUtil,       // 认证鉴权服务
    pub openais: DynOpenAisService, //openai服务
}

impl Services {
    pub fn new(
        db: Database,
        cache: RusCache,
        http_client: HttpClient,
        config: Arc<AppConfig>,
        ai_client: OpenAiClient,
    ) -> Self {
        info!("初始化实用服务...");
        let jwt_util = Arc::new(JwtTokenUtil::new(config.clone())) as DynJwtUtil;

        info!("实用服务已初始化，正在构建要素服务...");
        // dao层服务
        let repository = Arc::new(db);
        let cache_repository = Arc::new(cache);

        let openais = Arc::new(OpenAisService::new(
            config.clone(),
            repository.clone(),
            cache_repository.clone(),
            ai_client.clone(),
            http_client.clone(),
        )) as DynOpenAisService;

        Self { jwt_util, openais }
    }
}
