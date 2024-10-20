use crate::adapters::Adapter;
use crate::errors::GenieError;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use dotenv::var;
use anyhow::Result;

pub(crate) struct GitHubChat {
    model: String,
}

impl GitHubChat {
    pub(crate) fn new(model: String) -> Self {
        Self {
            model: model.to_string(),
        }
    }
}


#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Message {
    content: String,
    role: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Choice {
    finish_reason: String,
    index: u32,
    logprobs: Option<serde_json::Value>,
    message: Message,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Usage {
    completion_tokens: u32,
    prompt_tokens: u32,
    total_tokens: u32,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct ChatCompletion {
    choices: Vec<Choice>,
    created: u64,
    id: String,
    model: String,
    object: String,
    system_fingerprint: String,
    usage: Usage,
}
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct ErrorResponse {
    code: String,
    message: String,
    details: Option<serde_json::Value>,
}
#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum ApiResponse {
    Success(ChatCompletion),
    Error { error: ErrorResponse },
}

impl Adapter for GitHubChat {
    async fn generate(&self, prompt: String) -> Result<String> {
        let github_token = var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");

        let client = Client::new();
        let response = client
            .post("https://models.inference.ai.azure.com/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", github_token))
            .json(&json!({
            "messages": [
                {
                    "role": "system",
                    "content": ""
                },
                {
                    "role": "user",
                    "content": prompt,
                }
            ],
            "model": self.model,
            "temperature": 1,
            "max_tokens": 4096,
            "top_p": 1
        }))
            .send()
            .await?;


        let response_text = response.text().await?;
        let api_response: ApiResponse = serde_json::from_str(response_text.as_str())?;
        match api_response {
            ApiResponse::Success(chat_completion) => {
                Ok(chat_completion.choices.iter().map(|choice| choice.message.content.clone()).collect::<Vec<String>>().join("\n"))
            }
            ApiResponse::Error { error } => {
                Err(anyhow::anyhow!("Error: {} - {}", error.code, error.message))
            }
        }
    }

    async fn list_models(&self) -> Result<()> {
        todo!()
    }

    async fn generate_images(&self, _prompt: String, _image_path: String) -> Result<()> {
        todo!()
    }
}
