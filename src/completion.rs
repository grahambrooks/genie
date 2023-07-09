use serde::{Deserialize, Serialize};

use crate::client::{Client, GPT_3_5_TURBO};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CompletionMessage {
    role: String,
    content: String,
}

impl CompletionMessage {
    pub(crate) fn new(role: String, content: String) -> CompletionMessage {
        CompletionMessage {
            role: role.to_string(),
            content: content.to_string(),
        }
    }
    pub(crate) fn from_str(role: &str, content: &str) -> CompletionMessage {
        CompletionMessage {
            role: role.to_string(),
            content: content.to_string(),
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Delta {
    pub role: Option<String>,
    pub content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Choice {
    index: u32,
    finish_reason: Option<String>,
    pub delta: Delta,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct StreamedResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
}

impl StreamedResponse {
    pub(crate) fn from_string(line: String) -> serde_json::Result<StreamedResponse> {
        return serde_json::from_str(line.as_str());
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CompletionRequest {
    // The model to use for completion.
    model: String,
    // The suffix that comes after a completion of inserted text.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    suffix: Option<String>,
    messages: Vec<CompletionMessage>,
    // The maximum number of tokens to generate in the completion.
    //    The token count of your prompt plus max_tokens cannot exceed the model's context length. Example Python code for counting tokens.
    temperature: f32,
    //    We generally recommend altering this or top_p but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    max_tokens: Option<u32>,
    // An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    //    We generally recommend altering this or temperature but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    top_p: Option<f32>,
    // How many completions to generate for each prompt.
    //    Note: Because this parameter generates many completions, it can quickly consume your token quota. Use carefully and ensure that you have reasonable settings for max_tokens and stop.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    n: Option<u32>,
    // Whether to stream back partial progress. If set, tokens will be sent as data-only server-sent events as they become available, with the stream terminated by a data: [DONE] message. Example Python code.
    stream: bool,
    // Include the log probabilities on the logprobs most likely tokens, as well the chosen tokens. For example, if logprobs is 5, the API will return a list of the 5 most likely tokens. The API will always return the logprob of the sampled token, so there may be up to logprobs+1 elements in the response.
    //    The maximum value for logprobs is 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    logprobs: Option<u32>,
    // Echo back the prompt in addition to the completion
    // echo: bool,
    // Up to 4 sequences where the API will stop generating further tokens. The returned text will not contain the stop sequence.
    //    string or array
    //    Optional
    //    presence_penalty
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    stop: Option<String>,
    // Number between 0 and 1 that penalizes new tokens based on whether they appear in the text so far. Increases the model's likelihood to talk about new topics.
    //    See more information about frequency and presence penalties.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    number: Option<f32>,
    // Number between 0 and 1 that penalizes new tokens based on their existing frequency in the text so far. Decreases the model's likelihood to repeat the same line verbatim.
    //    See more information about frequency and presence penalties.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    frequency_penalty: Option<f32>,
    // Generates best_of completions server-side and returns the "best" (the one with the highest log probability per token). Results cannot be streamed.
    //    When used with n, best_of controls the number of candidate completions and n specifies how many to return – best_of must be greater than n.
    //
    //    Note: Because this parameter generates many completions, it can quickly consume your token quota. Use carefully and ensure that you have reasonable settings for max_tokens and stop.
    //    logit_bias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    best_of: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    user: Option<String>,  // A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. Learn more.
}

impl CompletionRequest {
    #[allow(dead_code)]
    pub(crate) async fn call(&self, client: Client, callback: fn(String)) {
        client.call(self, callback).await
    }

    pub(crate) async fn call_streamed_response(&self, client: Client, callback: fn(&StreamedResponse)) {
        client.call_streamed_response(self, callback).await
    }

    pub(crate) fn temperature(mut self, temp: f32) -> CompletionRequest {
        self.temperature = temp;
        self
    }
    pub(crate) fn model(mut self, model: &str) -> CompletionRequest {
        self.model = model.to_string();
        self
    }
    pub(crate) fn stream(mut self) -> CompletionRequest {
        self.stream = true;
        self
    }
}


pub(crate) fn request(messages: Vec<&CompletionMessage>) -> CompletionRequest {
    CompletionRequest {
        messages: messages
            .iter()
            .map(|c| CompletionMessage::new(c.role.to_string(), c.content.to_string()))
            .collect::<Vec<CompletionMessage>>(),
        model: GPT_3_5_TURBO.to_string(),
        temperature: 0.5,
        suffix: None,
        max_tokens: None,
        top_p: None,
        n: None,
        stream: false,
        logprobs: None,
        // echo: false,
        stop: None,
        number: None,
        frequency_penalty: None,
        best_of: None,
        user: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_message_new() {
        let role = "role".to_string();
        let content = "content".to_string();
        let completion_message = CompletionMessage::new(role, content);
        assert_eq!(completion_message.role, "role");
        assert_eq!(completion_message.content, "content");
    }

    #[test]
    fn test_completion_message_from_str() {
        let role = "role";
        let content = "content";
        let completion_message = CompletionMessage::from_str(role, content);
        assert_eq!(completion_message.role, "role");
        assert_eq!(completion_message.content, "content");
    }

    #[test]
    fn test_request() {
        let c = request(vec![]);
        assert_eq!(c.model, GPT_3_5_TURBO.to_string());
        assert_eq!(c.suffix, None);
        assert_eq!(c.max_tokens, None);
        assert_eq!(c.top_p, None);
        assert_eq!(c.n, None);
        assert!(!c.stream);
        assert_eq!(c.logprobs, None);
        assert_eq!(c.stop, None);
        assert_eq!(c.number, None);
        assert_eq!(c.frequency_penalty, None);
        assert_eq!(c.best_of, None);
        assert_eq!(c.user, None);
    }

    #[test]
    fn test_setting_temperature() {
        let r = request(vec![])
            .temperature(0.1);
        assert_eq!(r.temperature, 0.1);
    }

    #[test]
    fn test_setting_model() {
        let r = request(vec![])
            .model("model");
        assert_eq!(r.model, "model");
    }

    #[test]
    fn test_setting_stream() {
        let r = request(vec![])
            .stream();
        assert!(r.stream);
    }

    #[test]
    fn test_completion_is_serializable() {
        let r = request([&CompletionMessage::from_str("user", "content")].to_vec());
        let json = serde_json::to_string(&r);
        assert!(json.unwrap().eq(r#"{"model":"gpt-3.5-turbo","messages":[{"role":"user","content":"content"}],"temperature":0.5,"stream":false}"#));
    }
}
