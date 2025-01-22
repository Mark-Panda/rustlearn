use axum::extract::Json;
use axum::routing::post;
use axum::{Extension, Router};
use tracing::info;

use crate::dtos::ai_dto::{ChatMessageDto, ResponseChatMessageDto};
use crate::server::extractors::ValidatedJson;
use crate::server::services::Services;
use rutils::AppResult;
pub struct OpenAiController;

impl OpenAiController {
    pub fn app() -> Router {
        Router::new()
            .route("/chat", post(Self::chat_message_endpoint))
            .route("/multiple", post(Self::multiple_message_chat))
    }

    pub async fn chat_message_endpoint(
        Extension(services): Extension<Services>,
        ValidatedJson(request): ValidatedJson<ChatMessageDto>,
    ) -> AppResult<Json<ResponseChatMessageDto>> {
        info!(
            "recieved request to create chat {:?}/{:?}",
            request.chat_id.as_ref().unwrap(),
            request.message.as_ref().unwrap()
        );

        let created_user = services.openais.chat_message(request).await?;

        Ok(Json(created_user))
    }

    pub async fn multiple_message_chat(
        Extension(services): Extension<Services>,
        ValidatedJson(request): ValidatedJson<ChatMessageDto>,
    ) -> AppResult<String> {
        info!(
            "recieved request to create chat {:?}/{:?}",
            request.chat_id.as_ref().unwrap(),
            request.message.as_ref().unwrap()
        );

        let result = services.openais.multiple_insert_message().await?;

        Ok(result)
    }
}
