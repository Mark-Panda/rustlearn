mod ai_controller;

use axum::routing::*;

use self::ai_controller::OpenAiController;

pub async fn health() -> &'static str {
    "🚀🚀🚀 Server Running"
}

pub fn app() -> Router {
    Router::new().nest("/ais", OpenAiController::app())
}
