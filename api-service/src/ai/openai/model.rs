use async_trait::async_trait;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
use std::{sync::Arc, time::SystemTime};
use uuid::{uuid, Uuid};

#[derive(FromRow, Debug)]
pub struct OpenAi {
    pub id: Uuid,
    // TODO: 如果字段为空的情况 都得Option转
    pub chat_id: String,
    pub message: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    // 因为deleted_at不是必填项 所以需要用Option转一下，不然会报错
    pub deleted_at: Option<OffsetDateTime>,
}

impl Default for OpenAi {
    fn default() -> Self {
        OpenAi {
            id: uuid!("f3f898aa-ffa3-4b58-91b0-612a1c801a5e"),
            chat_id: String::from("121211312"),
            message: String::from("你是GPT多少版本"),
            created_at: OffsetDateTime::from(SystemTime::now()),
            updated_at: OffsetDateTime::from(SystemTime::now()),
            deleted_at: None,
        }
    }
}

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynOpenAisRepository = Arc<dyn OpenAisRepository + Send + Sync>;

#[async_trait]
pub trait OpenAisRepository {
    async fn create_openai(&self, chat_id: &str, message: &str) -> anyhow::Result<OpenAi>;
    async fn get_message_by_chat_id(&self, chat_id: &str) -> anyhow::Result<Option<OpenAi>>;
}
