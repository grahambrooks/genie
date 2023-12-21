use std::error::Error;
use std::io;
use std::io::Write;
use std::process::Command;

use async_trait::async_trait;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use termion::event::Key;
use termion::input::TermRead;

use crate::errors::GenieError;
use crate::expand_template;
use crate::messages::{CODE_TEMPLATE, DEFAULT_TEMPLATE, SHELL_TEMPLATE};
use crate::adaptors::ChatTrait;

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
        let messages = expand_template(prompt, &DEFAULT_TEMPLATE);
        let connection = Ollama::new("http://localhost".to_string(), 11434);
        let model = self.model.to_string();
        let res = connection.generate(GenerationRequest::new(model, messages)).await;
        if let Ok(res) = res {
            println!("{}", res.response);
        }
        Ok(())
    }

    async fn generate_code(&self, prompt: String) -> Result<(), Box<dyn Error>> {
        let connection = Ollama::new("http://localhost".to_string(), 11434);
        let model = self.model.to_string();
        let messages = expand_template(prompt, &CODE_TEMPLATE);
        println!("prompt: {}", messages);

        match connection.generate(GenerationRequest::new(model, messages)).await {
            Ok(res) => {
                println!("{}", res.response);
                Ok(())
            }
            Err(e) => Err(Box::new(GenieError::new(&format!("Error generating images: {}", e)))),
        }
    }

    async fn list_models(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    async fn generate_images(&self, _prompt: String) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    async fn shell(&self, prompt: String) -> Result<(), Box<dyn Error>> {
        let connection = Ollama::new("http://localhost".to_string(), 11434);
        let model = self.model.to_string();
        let messages = expand_template(prompt, &SHELL_TEMPLATE);
        println!("prompt: {}", messages);
        let res = connection.generate(GenerationRequest::new(model, messages)).await;
        if let Ok(res) = res {
            println!("{}", res.response);

            print!("Run these commands? (n/e): ");
            let _ = io::stdout().flush();
            if action() {
                exec(res.response).await.unwrap();
            }
        }
        Ok(())
    }
}

async fn exec(script: String) -> io::Result<()> {
    println!("executing: {}", script);
    // remove grave characters from the start and end of the script

    let s = script.trim_start_matches('`');

    let mut child = Command::new("zsh")
        .stdin(std::process::Stdio::piped())
        .spawn()?;


    {
        let stdin = child.stdin.as_mut().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to open stdin"))?;
        stdin.write_all(s.as_bytes())?;
    }

    let output = child.wait_with_output()?;

    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;
    Ok(())
}

#[allow(clippy::never_loop)]
fn action() -> bool {
    let stdin = io::stdin();
    for c in stdin.keys() {
        return matches!(c.unwrap(), Key::Char('e'));
    }
    false
}
