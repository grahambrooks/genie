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

