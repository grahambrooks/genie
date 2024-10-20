use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use anyhow::Result;
use crate::adapters::Adapter;
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

impl Adapter for OllamaChat {
    async fn generate(&self, prompt: String) -> Result<String> {
        if prompt.is_empty() {
            return Err(anyhow::anyhow!("Prompt cannot be empty"));
        }
        let messages = expand_template(prompt, &DEFAULT_TEMPLATE);
        let connection = Ollama::new("http://localhost".to_string(), 11434);
        let model = self.model.to_string();
        let res = connection.generate(GenerationRequest::new(model, messages)).await;
        match res {
            Ok(response) => Ok(response.response),
            Err(e) => return Err(anyhow::anyhow!("Error calling Ollama: {}", e)),
        }
    }

    async fn list_models(&self) -> Result<()> {
        todo!()
    }

    async fn generate_images(&self, _prompt: String, _image_path: String) -> Result<()> {
        todo!()
    }
}
