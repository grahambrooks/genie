use std::error::Error;

use async_openai::Client;
use async_openai::types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs, CreateImageRequestArgs, ImageSize, ResponseFormat};
use async_trait::async_trait;
use log::info;

use crate::adapters::Adapter;
use crate::errors::GenieError;

const IMAGE_COUNT: u8 = 2;
const IMAGE_SIZE: ImageSize = ImageSize::S1024x1024;


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

impl std::fmt::Display for OpenAIGPTChat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "openai::{}", self.model)
    }
}

#[async_trait]
impl Adapter for OpenAIGPTChat {
    async fn generate(&self, message: String) -> Result<String, Box<dyn Error>> {
        let connection = Client::new();

        let request = CreateChatCompletionRequestArgs::default()
            .model(self.model.clone())
            .max_tokens(512u16)
            .messages([ChatCompletionRequestUserMessageArgs::default()
                .content(message)
                .build()?
                .into()])
            .build()?;

        let result = connection.chat().create(request).await;

        match result {
            Ok(response) => {
                let mut result_text = String::new();

                response.choices.iter().for_each(|chat_choice| {
                    if let Some(ref content) = chat_choice.message.content {
                        result_text.push_str(content);
                    }
                });

                Ok(result_text)
            }
            Err(e) => Err(Box::new(GenieError::new(&format!("Error generating images: {}", e)))),
        }
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

    async fn generate_images(&self, prompt: String, image_path: String) -> Result<(), Box<dyn Error>> {
        if prompt.is_empty() {
            return Err(Box::new(GenieError::new("Prompt cannot be empty")));
        }
        let connection = Client::new();


        let request = CreateImageRequestArgs::default()
            .prompt(prompt)
            .n(IMAGE_COUNT)
            .response_format(ResponseFormat::Url)
            .size(IMAGE_SIZE)
            .user("async-openai")
            .build()?;

        let response = connection.images().create(request).await?;

        let paths = response.save(image_path).await?;

        for path in &paths {
            info!("Image file path: {}", path.display());
        }

        Ok(())
    }
}
