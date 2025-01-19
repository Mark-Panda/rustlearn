use anyhow::Context;
use async_trait::async_trait;
use sqlx::query_as;

use rutils::Database;

use super::{OpenAi, OpenAisRepository};

#[async_trait]
impl OpenAisRepository for Database {
    async fn create_openai(&self, chat_id: &str, message: &str) -> anyhow::Result<OpenAi> {
        query_as!(
            OpenAi,
            r#"
        insert into openais (created_at, updated_at, chat_id, message)
        values (current_timestamp, current_timestamp, $1::varchar, $2::varchar)
        returning *
            "#,
            chat_id,
            message
        )
        .fetch_one(&self.pool)
        .await
        .context("an unexpected error occured while creating the openais")
    }

    async fn get_message_by_chat_id(&self, chat_id: &str) -> anyhow::Result<Option<OpenAi>> {
        query_as!(
            OpenAi,
            r#"
            select *
            from openais
            where chat_id = $1::varchar
            "#,
            chat_id,
        )
        .fetch_optional(&self.pool)
        .await
        .context("unexpected error while querying for openais by chat_id")
    }
}
