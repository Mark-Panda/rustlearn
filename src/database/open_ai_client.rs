use std::sync::Arc;

use anyhow::Result;
use async_openai::{config::OpenAIConfig, types::{ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs}, Client};

/// OpenAI API client wrapper
#[derive( Clone)]
pub struct OpenAiClient {
    // 使用 Arc 来共享 Client 实例
    pub client: Arc<Client<OpenAIConfig>>,
}

impl OpenAiClient {
    pub async fn connect() -> Result<Self> {
        // 创建 Client 实例
        let client = Client::new();

        let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a helpful assistant.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("Who won the world series in 2020?")
                .build()?
                .into(),
            ChatCompletionRequestAssistantMessageArgs::default()
                .content("The Los Angeles Dodgers won the World Series in 2020.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("Where was it played?")
                .build()?
                .into(),
        ])
        .build()?;
    println!("消息{}", serde_json::to_string(&request).unwrap());

    let response = client.chat().create(request).await?;
    println!("\nResponse:\n");
    for choice in response.choices {
        println!(
            "{}: Role: {}  Content: {:?}",
            choice.index, choice.message.role, choice.message.content
        );
    }
        // 使用 Arc 包装 Client 以便可以共享
        Ok(Self {
            client: Arc::new(client),
        })
    }
}



// use anyhow::Result;
// use openai_api_rs::v1::api::Client;
// use std::sync::Arc;
// use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
// use openai_api_rs::v1::common::GPT3_5_TURBO;

// /// OpenAI API client wrapper
// #[derive( Clone)]
// pub struct OpenAiClient {
//     // 使用 Arc 来共享 Client 实例
//     pub client: Arc<Client>,
// }

// impl OpenAiClient {
//     pub async fn connect(connection_string: &str) -> Result<Self> {
//         // 创建 Client 实例
//         let client = Client::new(connection_string.to_string());

//         let req = ChatCompletionRequest::new(
//             GPT3_5_TURBO.to_string(),
//             vec![chat_completion::ChatCompletionMessage {
//                 role: chat_completion::MessageRole::user,
//                 content: chat_completion::Content::Text(String::from("What is Bitcoin?")),
//                 name: None,
//             }],
//         );
//         let result = client.chat_completion(req)?;
//         println!("错误{:?}", result);

//         // 使用 Arc 包装 Client 以便可以共享
//         Ok(Self {
//             client: Arc::new(client),
//         })
//     }
// }
