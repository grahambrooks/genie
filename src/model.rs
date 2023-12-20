use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use crate::adaptors;

pub(crate) struct Model {
    protocol: String,
    model_name: String,
}

#[async_trait]
pub(crate) trait ChatTrait {
    async fn prompt(&self, prompt: String) -> Result<(), Box<dyn Error>>;
    async fn list_models(&self) -> Result<(), Box<dyn Error>>;
    async fn generate_images(&self, prompt: String) -> Result<(), Box<dyn Error>>;
}

impl Display for Model {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.protocol, self.model_name)
    }
}

impl Model {
    #[allow(dead_code)]
    pub(crate) fn new(protocol: &'static str, model_name: &'static str) -> Self {
        Self {
            protocol: protocol.to_string(),
            model_name: model_name.to_string(),
        }
    }
    pub(crate) fn from_string(spec: &str) -> Self {
        let mut split_spec = spec.split("::");
        let protocol = split_spec.next().unwrap().to_string();
        let model_name = split_spec.next().unwrap().to_string();
        Self {
            protocol,
            model_name,
        }
    }

    pub(crate) fn chat_adaptor(&self) -> Box<dyn ChatTrait> {
        // return an adaptor based on the protocol
        match self.protocol.as_str() {
            "ollama" => Box::new(adaptors::ollama::OllamaChat::new(self.model_name.clone())),
            "embedded" => Box::new(adaptors::embedded::EmbeddedChat::new(self.model_name.clone())),
            _ => Box::new(adaptors::openai::OpenAIGPTChat::new(self.model_name.clone()))
        }
    }
}