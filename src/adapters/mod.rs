use anyhow::Result;

pub(crate) mod ollama;
pub(crate) mod openai;
pub(crate) mod github;

pub(crate) trait Adapter {
    async fn generate(&self, prompt: String) -> Result<String>;
    async fn list_models(&self) -> Result<()>;
    async fn generate_images(&self, prompt: String, image_path: String) -> Result<()>;
}
