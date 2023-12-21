use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

use async_trait::async_trait;
use orca::llm::quantized::Quantized;
use orca::pipeline::Pipeline;
use orca::pipeline::simple::LLMPipeline;

use crate::adaptors::ChatTrait;

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
    async fn prompt(&self, _prompt: String) -> Result<(), Box<dyn Error>> {
        match generate_code().await {
            Ok(_) => Ok(()),
            Err(_e) => Ok(()),
        }
    }

    async fn generate_code(&self, prompt: String) -> Result<(), Box<dyn Error>> {
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

async fn generate_code() -> anyhow::Result<()> {
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
        .load_template("greet", "{{#chat}}{{#user}}fn fib(n: int32){{/user}}{{/chat}}")
        .unwrap();
    let result = pipe.execute("greet").await?;

    println!("{}", result.content());

    Ok(())
}
