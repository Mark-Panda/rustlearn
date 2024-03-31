use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::openai::OpenAi;

impl OpenAi {
    pub fn into_dto(self) -> ResponseChatMessageDto {
        ResponseChatMessageDto {
            chat_id: self.chat_id,
            message: self.message,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct ChatMessageDto {
    #[validate(required, length(min = 1))]
    pub chat_id: Option<String>,
    #[validate(required, length(min = 1))]
    pub message: Option<String>,
}


#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ResponseChatMessageDto {
    pub chat_id: String,
    pub message: String,
}