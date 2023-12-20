use std::error::Error;
use async_trait::async_trait;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use crate::model::ChatTrait;

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

#[async_trait]
impl ChatTrait for OllamaChat {
    async fn prompt(&self, prompt: String) -> Result<(), Box<dyn Error>> {
// For custom values:
        let ollama = Ollama::new("http://localhost".to_string(), 11434);

        let model = self.model.to_string();

        let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

        // let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

        if let Ok(res) = res {
            println!("{}", res.response);
        }
        Ok(())
    }

    async fn list_models(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    async fn generate_images(&self, prompt: String) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
