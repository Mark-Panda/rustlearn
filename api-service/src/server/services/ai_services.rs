use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
};
use std::sync::Arc;

use crate::{
    config::AppConfig,
    dtos::ai_dto::{ChatMessageDto, ResponseChatMessageDto},
    openai::DynOpenAisRepository,
    utils::HttpClient,
    OpenAiClient,
};
use async_trait::async_trait;

use rutils::{AppResult, DynRedisClientExt, Error};
/// A reference counter for our user service allows us safely pass instances user utils
/// around which themselves depend on the user repostiory, and ultimately, our Posgres connection pool.
pub type DynOpenAisService = Arc<dyn OpenAisServiceTrait + Send + Sync>;

#[async_trait]
pub trait OpenAisServiceTrait {
    async fn chat_message(&self, request: ChatMessageDto) -> AppResult<ResponseChatMessageDto>;
}

#[derive(Clone)]
pub struct OpenAisService {
    repository: DynOpenAisRepository,
    // 允许 cache 字段未被读取
    #[allow(dead_code)]
    cache: DynRedisClientExt,
    // 允许 config 字段未被读取
    #[allow(dead_code)]
    config: Arc<AppConfig>,
    // 允许 ai_client 字段未被读取
    #[allow(dead_code)]
    ai: OpenAiClient,
    // 允许 http_repository 字段未被读取
    #[allow(dead_code)]
    http_repository: HttpClient,
}

impl OpenAisService {
    pub fn new(
        config: Arc<AppConfig>,
        repository: DynOpenAisRepository,
        cache: DynRedisClientExt,
        ai: OpenAiClient,
        http_repository: HttpClient,
    ) -> Self {
        Self {
            config,
            repository,
            cache,
            ai,
            http_repository,
        }
    }
}

#[async_trait]
impl OpenAisServiceTrait for OpenAisService {
    async fn chat_message(&self, request: ChatMessageDto) -> AppResult<ResponseChatMessageDto> {
        let chat_id = request.chat_id.unwrap();
        let req_message = request.message.unwrap();
        let request = CreateChatCompletionRequestArgs::default()
            .max_tokens(512u16)
            .model("gpt-3.5-turbo")
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content("You are a helpful assistant.")
                    .build()
                    .unwrap()
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content("Who won the world series in 2020?")
                    .build()
                    .unwrap()
                    .into(),
                ChatCompletionRequestAssistantMessageArgs::default()
                    .content("The Los Angeles Dodgers won the World Series in 2020.")
                    .build()
                    .unwrap()
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content("Where was it played?")
                    .build()
                    .unwrap()
                    .into(),
            ])
            .build()
            .unwrap();
        println!("消息{}", serde_json::to_string(&request).unwrap());

        let ai_res = self.ai.client.chat().create(request).await;
        match ai_res {
            Ok(success_value) => {
                // 处理成功的情况
                println!("Success: {:?}", success_value);
                // 继续你的逻辑
                for choice in success_value.choices {
                    println!(
                        "{}: Role: {}  Content: {:?}",
                        choice.index, choice.message.role, choice.message.content
                    );
                }
            }
            Err(error) => {
                // 处理错误的情况
                println!("Error: {:?}", error);
                // 可以决定如何处理错误，例如返回、重试或记录错误等
                return Err(Error::ObjectConflict(format!("openai请求错误")));
            }
        }

        let create_chat_result = self
            .repository
            .create_openai(&chat_id, &req_message)
            .await?;
        // TODO: 不能引用use anyhow::Ok; 否则报错，用的是AppResult
        Ok(create_chat_result.into_dto())
    }
}
