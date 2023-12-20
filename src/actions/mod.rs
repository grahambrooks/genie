use std::fmt::{Debug, Display};

use async_trait::async_trait;

pub(crate) mod embedded;
pub(crate) mod shell;
pub(crate) mod code;
pub(crate) mod images;
pub(crate) mod server;
pub(crate) mod list_models;
pub(crate) mod chat;

// #[async_trait]
pub(crate) trait Action {
    fn exec(&self, user_prompt: String) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Debug, Clone)]
pub(crate) struct ActionError {
    pub message: String,
}

impl ActionError {
    fn new(msg: &str) -> Self {
        ActionError {
            message: msg.to_string(),
        }
    }
    fn from_string(msg: String) -> Self {
        ActionError { message: msg }
    }
}

impl std::fmt::Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Sample error: {}", self.message)
    }
}

impl std::error::Error for ActionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
