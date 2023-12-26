use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

use async_trait::async_trait;
use orca::llm::quantized::Quantized;
use orca::pipeline::Pipeline;
use orca::pipeline::simple::LLMPipeline;

use crate::adaptors::ChatTrait;
use crate::errors::GenieError;

pub(crate) struct EmbeddedChat {}

impl EmbeddedChat {
    pub(crate) fn new(_model: String) -> Self {
        Self {}
    }
}

impl Display for EmbeddedChat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "embedded")
    }
}

// implement ChatAdaptor for EmbeddedChat
// implement ChatTrait for EmbeddedChat

#[async_trait]
impl ChatTrait for EmbeddedChat {
    async fn prompt(&self, prompt: String) -> Result<(), Box<dyn Error>> {
        if prompt.is_empty() {
            return Err(Box::new(GenieError::new("Prompt cannot be empty")));
        }
        Ok(generate_code(prompt).await?)
    }

    async fn generate_code(&self, _prompt: String) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    async fn list_models(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    async fn generate_images(&self, _prompt: String) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    async fn shell(&self, _prompt: String) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

async fn generate_code(prompt: String) -> anyhow::Result<()> {
    env_logger::init();

    let model = Quantized::new()
        .with_sample_len(99)
        .load_model(orca::llm::quantized::Model::Mistral7bInstruct)
        .await
        .unwrap()
        // .with_model(Model::Mistral7bInstruct)
        // .load_model_from_path("../../models/mistral-7b-instruct-v0.1.Q4_K_S.gguf")?
        .build_model()?;

    let pipe = LLMPipeline::new(&model)
        .load_template("greet", &format!("{{#chat}}{{#user}}{}{{/user}}{{/chat}}", prompt))
        .unwrap();
    let result = pipe.execute("greet").await?;

    println!("{}", result.content());

    Ok(())
}
