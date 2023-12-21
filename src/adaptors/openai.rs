use std::error::Error;

use async_openai::Client;
use async_openai::types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs};
use async_trait::async_trait;

use crate::{expand_template, images, read_stdin};
use crate::errors::GenieError;
use crate::messages::CODE_TEMPLATE;
use crate::adaptors::ChatTrait;

pub(crate) struct OpenAIGPTChat {
    model: String,
}

impl OpenAIGPTChat {
    pub(crate) fn new(model: String) -> Self {
        Self {
            model: model.to_string(),
        }
    }
}

#[async_trait]
impl ChatTrait for OpenAIGPTChat {
    async fn prompt(&self, user_prompt: String) -> Result<(), Box<dyn Error>> {
        let connection = Client::new();

        let mut prompt = user_prompt.clone();
        prompt.push_str(read_stdin().as_str());

        let messages = expand_template(prompt, &CODE_TEMPLATE);

        let request = CreateChatCompletionRequestArgs::default()
            .model(self.model.clone())
            .max_tokens(512u16)
            .messages([ChatCompletionRequestUserMessageArgs::default()
                .content(messages)
                .build()?
                .into()])
            .build()?;

        let result = connection.chat().create(request).await;

        match result {
            Ok(response) => {
                response.choices.iter().for_each(|chat_choice| {
                    if let Some(ref content) = chat_choice.message.content {
                        println!("{}", content);
                    }
                });
                Ok(())
            },
            Err(e) => Err(Box::new(GenieError::new(&format!("Error generating images: {}", e)))),
        }
    }

    async fn generate_code(&self, prompt: String) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    async fn list_models(&self) -> Result<(), Box<dyn Error>> {
        let client = Client::new();

        let model_list = client.models().list().await?;


        for model in model_list.data {
            if model.id == self.model {
                println!("* {}", model.id, );
                continue;
            }
            println!("  {}", model.id, );
        }
        Ok(())
    }

    async fn generate_images(&self, prompt: String) -> Result<(), Box<dyn Error>> {
        let connection = Client::new();
        match images::generator(connection)
            .count(images::IMAGE_COUNT)
            .size(images::IMAGE_SIZE)
            .path(images::SAVE_PATH)
            .generate(prompt).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(GenieError::new(&format!("Error generating images: {}", e)))),
        }
    }

    async fn shell(&self, _prompt: String) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
