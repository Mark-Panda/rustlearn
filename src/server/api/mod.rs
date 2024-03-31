mod category_controller;
mod user_controller;
mod  ai_controller;

use axum::routing::*;

use self::{category_controller::CategoryController, user_controller::UserController, ai_controller::OpenAiController};

pub async fn health() -> &'static str {
    "🚀🚀🚀 Server Running"
}

pub fn app() -> Router {
    Router::new()
        .nest("/users", UserController::app())
        .nest("/categories", CategoryController::app())
        .nest("/ais", OpenAiController::app())
}
