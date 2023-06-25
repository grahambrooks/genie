use futures::StreamExt;

use crate::completion::{CompletionRequest, StreamedResponse};

pub(crate) struct Client {
    client: reqwest::Client,
    token: String,
}

pub const GPT_3_5_TURBO: &str = "gpt-3.5-turbo";

impl Client {
    pub(crate) fn new(bearer_token: String) -> Client {
        Client {
            client: reqwest::Client::new(),
            token: bearer_token,
        }
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

                            if line.starts_with("data:") {
                                let data = line.strip_prefix("data:").unwrap().to_string();
                                match StreamedResponse::from_string(data) {
                                    Ok(a) => callback(&a),
                                    Err(e) => println!("Error decoding line {}", e),
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


