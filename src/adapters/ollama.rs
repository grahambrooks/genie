use std::error::Error;

use async_trait::async_trait;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;

use crate::adapters::Adapter;
use crate::errors::GenieError;
use crate::expand_template;
use crate::messages::DEFAULT_TEMPLATE;

pub(crate) struct OllamaChat {
    model: String,
}

impl OllamaChat {
    pub(crate) fn new(model: String) -> Self {
        Self {
            model: model.to_string(),
        }
    }
}

impl std::fmt::Display for OllamaChat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ollama::{}", self.model)
    }
}

#[async_trait]
impl Adapter for OllamaChat {
    async fn generate(&self, prompt: String) -> Result<String, Box<dyn Error>> {
        if prompt.is_empty() {
            return Err(Box::new(GenieError::new("Prompt cannot be empty")));
        }
        let messages = expand_template(prompt, &DEFAULT_TEMPLATE);
        let connection = Ollama::new("http://localhost".to_string(), 11434);
        let model = self.model.to_string();
        let res = connection.generate(GenerationRequest::new(model, messages)).await;
        match res {
            Ok(response) => Ok(response.response),
            Err(e) => return Err(Box::new(GenieError::new(&format!("Error calling Ollama: {}", e)))),
        }
    }

    async fn list_models(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    async fn generate_images(&self, _prompt: String, _image_path: String) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
