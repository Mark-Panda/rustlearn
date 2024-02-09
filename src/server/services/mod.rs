use std::sync::Arc;

use tracing::info;

use crate::{
    config::AppConfig,
    database::Database,
    server::{
        services::{
            category_services::CategoriesService, session_services::SessionsService,
            user_services::UsersService,
        },
        utils::{
            argon_utils::{ArgonSecurityUtil, DynArgonUtil},
            jwt_utils::JwtTokenUtil,
        },
    },
};

use self::{
    category_services::DynCategoriesService, session_services::DynSessionsService,
    user_services::DynUsersService,
};

use super::utils::jwt_utils::DynJwtUtil;

pub mod category_services;
pub mod seed_services;
pub mod session_services;
pub mod user_services;

#[derive(Clone)]
pub struct Services {
    pub jwt_util: DynJwtUtil, // 认证鉴权服务
    pub users: DynUsersService, // 用户服务
    pub sessions: DynSessionsService, // session服务
    pub categories: DynCategoriesService, // 类别服务
}

impl Services {
    pub fn new(db: Database, config: Arc<AppConfig>) -> Self {
        info!("初始化实用服务...");
        let security_service = Arc::new(ArgonSecurityUtil::new(config.clone())) as DynArgonUtil;
        let jwt_util = Arc::new(JwtTokenUtil::new(config)) as DynJwtUtil;

        info!("实用服务已初始化，正在构建要素服务...");
        // dao层服务
        let repository = Arc::new(db);

        let sessions = Arc::new(SessionsService::new(repository.clone(), jwt_util.clone()))
            as DynSessionsService;

        let users = Arc::new(UsersService::new(
            repository.clone(),
            security_service,
            jwt_util.clone(),
            sessions.clone(),
        )) as DynUsersService;

        let categories =
            Arc::new(CategoriesService::new(repository.clone())) as DynCategoriesService;

        Self {
            jwt_util,
            users,
            sessions,
            categories,
        }
    }
}
