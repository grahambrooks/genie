use std::error::Error;

use async_trait::async_trait;

pub(crate) mod ollama;
pub(crate) mod openai;

#[async_trait]
pub(crate) trait Adapter {
    async fn call(&self, prompt: String) -> Result<String, Box<dyn Error>>;
    async fn prompt(&self, prompt: String) -> Result<(), Box<dyn Error>>;
    async fn generate_code(&self, prompt: String) -> Result<(), Box<dyn Error>>;
    async fn list_models(&self) -> Result<(), Box<dyn Error>>;
    async fn generate_images(&self, prompt: String) -> Result<(), Box<dyn Error>>;

    async fn shell(&self, prompt: String) -> Result<(), Box<dyn Error>>;
}
