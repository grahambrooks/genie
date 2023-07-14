use futures::StreamExt;
use serde_json::Value;

use crate::completion::{CompletionRequest, StreamedResponse};

pub(crate) struct Client {
    client: reqwest::Client,
    token: String,
}

pub const GPT_3_5_TURBO: &str = "gpt-3.5-turbo";

// #[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Model {
    pub id: String,
}

impl Model {
    fn from_json(json: &Value) -> Model {
        Model {
            id: match json["id"] {
                Value::Null => "undefined".to_string(),
                _ => json["id"].as_str().unwrap().to_string(),
            },
        }
    }
}

impl Client {
    pub(crate) fn new(bearer_token: String) -> Client {
        Client {
            client: reqwest::Client::new(),
            token: bearer_token,
        }
    }

    pub(crate) async fn list_models(&self) -> Vec<Model> {
        let response = self.client.get("https://api.openai.com/v1/engines")
            .bearer_auth(self.token.clone())
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap();

        let mut models: Vec<Model> = Vec::new();
        for model in response["data"].as_array().unwrap() {
            models.push(Model::from_json(model));
        }

        models
    }


    #[allow(dead_code)]
    pub(crate) async fn call(&self, request: &CompletionRequest, callback: fn(String)) {
        let mut stream = self.client.post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(self.token.clone())
            .json(request)
            .send()
            .await
            .unwrap()
            .bytes_stream();

        let mut buffer = Vec::new();
        while let Some(item) = stream.next().await {
            match item {
                Ok(item) =>
                    for byte in item {
                        if byte == b'\n' {
                            callback(String::from_utf8(buffer.clone()).unwrap());
                            buffer.clear();
                        } else {
                            buffer.push(byte);
                        }
                    }
                Err(e) => println!("Error: {}", e),
            };
        }
    }

    pub(crate) async fn call_streamed_response(&self, request: &CompletionRequest, callback: fn(&StreamedResponse)) {
        let mut stream = self.client.post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(self.token.clone())
            .json(request)
            .send()
            .await
            .unwrap()
            .bytes_stream();

        let mut buffer = Vec::new();
        while let Some(item) = stream.next().await {
            match item {
                Ok(item) =>
                    for byte in item {
                        if byte == b'\n' {
                            let line = String::from_utf8(buffer.clone()).unwrap();
                            // println!("Line: {}", line);
                            if line.starts_with("data: ") {
                                let data = line.strip_prefix("data: ").unwrap().to_string();
                                if !data.starts_with("[DONE]") {
                                    match StreamedResponse::from_string(data) {
                                        Ok(a) => callback(&a),
                                        Err(e) => println!("Error decoding line {}", e),
                                    }
                                }
                            }
                            buffer.clear();
                        } else {
                            buffer.push(byte);
                        }
                    }
                Err(e) => println!("Error: {}", e),
            };
        }
    }
}

// Tests for Client
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client() {
        let c = Client::new("a test token".to_string());
        assert_eq!(c.token, "a test token");
    }
}


