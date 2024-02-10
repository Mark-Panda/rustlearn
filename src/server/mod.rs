mod api;
pub mod dtos;
pub mod error;
pub mod extractors;
pub mod services;
pub mod utils;
use tokio::signal::unix::{signal, SignalKind};
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Ok};
use axum::extract::{MatchedPath, Request};
use axum::http::HeaderValue;
use axum::middleware::{self, Next};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Extension;
use axum::{error_handling::HandleErrorLayer, http::StatusCode, BoxError, Json, Router};
use lazy_static::lazy_static;
use serde_json::json;
use tokio::time::Instant;
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::{cors::Any, cors::CorsLayer, trace::TraceLayer};
use tracing::{debug, info};

use crate::config::AppConfig;
use crate::database::Database;
use crate::server::services::seed_services::SeedService;
use crate::server::services::Services;

lazy_static! {
    static ref HTTP_TIMEOUT: u64 = 30;
    static ref EXPONENTIAL_SECONDS: &'static [f64] =
        &[0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,];
}

pub struct ApplicationServer;

impl ApplicationServer {
    pub async fn serve(config: Arc<AppConfig>, db: Database) -> anyhow::Result<()> {
        let services = Services::new(db, config.clone());

        if config.seed {
            // TODO: 创建测试数据
            info!("seeding enabled, creating test data...");
            SeedService::new(services.clone())
                .seed()
                .await
                .expect("unexpected error occurred while seeding application data");
        }

        let cors_origin = &config.cors_origin;

        let cors = CorsLayer::new()
            .allow_origin(cors_origin.parse::<HeaderValue>().unwrap())
            .allow_methods(Any)
            .allow_headers(Any);

        // TODO: 中间件链接https://docs.rs/axum/latest/axum/middleware/index.html#commonly-used-middleware  https://docs.rs/axum/latest/axum/middleware/index.html#applying-middleware
        // axum 使用tower-http 实现中间件https://docs.rs/tower-http/0.5.0/tower_http/cors/index.html
        let router = Router::new()
            .nest("/api/v1", api::app()) // nest路由组
            .route("/", get(api::health))
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http()) // 高级跟踪和日志
                    .layer(HandleErrorLayer::new(Self::handle_timeout_error))
                    .timeout(Duration::from_secs(*HTTP_TIMEOUT)) // 超时处理
                    .layer(cors) // 跨域
                    .layer(Extension(services)) // 扩展服务
                    .layer(BufferLayer::new(1024)) // buffer限制
                    .layer(RateLimitLayer::new(5, Duration::from_secs(1))), // 请求限流
            )
            .route_layer(middleware::from_fn(Self::track_metrics)); // 请求扩展将状态从中间件传递到处理程序

        // 404处理
        let router = router.fallback(Self::handle_404);

        let port = config.port;
        let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));

        info!("🚀 Server has launched on https://{addr}");
        debug!("routes initialized, listening on port {}", port);
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, router.into_make_service())
            .with_graceful_shutdown(Self::shutdown_signal())
            .await
            .context("error while starting API server")?;

        Ok(())
    }

    /// TODO: axum常用中间件链接 Adds a custom handler for tower's `TimeoutLayer`, see https://docs.rs/axum/latest/axum/middleware/index.html#commonly-used-middleware.
    async fn handle_timeout_error(err: BoxError) -> (StatusCode, Json<serde_json::Value>) {
        if err.is::<tower::timeout::error::Elapsed>() {
            (
                StatusCode::REQUEST_TIMEOUT,
                Json(json!({
                    "error":
                        format!(
                            "request took longer than the configured {} second timeout",
                            *HTTP_TIMEOUT
                        )
                })),
            )
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("unhandled internal error: {}", err)
                })),
            )
        }
    }

    async fn track_metrics(request: Request, next: Next) -> impl IntoResponse {
        let path = if let Some(matched_path) = request.extensions().get::<MatchedPath>() {
            matched_path.as_str().to_owned()
        } else {
            request.uri().path().to_owned()
        };

        let start = Instant::now();
        let method = request.method().clone();
        let response = next.run(request).await;
        let latency = start.elapsed().as_secs_f64();
        let status = response.status().as_u16().to_string();

        let labels = [
            ("method", method.to_string()),
            ("path", path),
            ("status", status),
            ("latency", latency.to_string()),
        ];

        metrics::counter!("http_requests_total", &labels);
        metrics::histogram!("http_requests_duration_seconds", &labels);

        response
    }

    /// Tokio signal handler that will wait for a user to press CTRL+C.
    /// We use this in our hyper `Server` method `with_graceful_shutdown`.
    async fn shutdown_signal() {
        // An infinite stream of hangup signals.
        let mut stream = signal(SignalKind::hangup()).expect("expect tokio signal SIGHUP");

        // Print whenever a HUP signal is received
        loop {
            stream.recv().await;
            println!("signal shutdown");
        }
    }

    async fn handle_404() -> impl IntoResponse {
        (
            StatusCode::NOT_FOUND,
            axum::response::Json(serde_json::json!({
            "errors":{
            "message": vec!(String::from("The requested resource does not exist on this server!")),}
            })),
        )
    }
}
