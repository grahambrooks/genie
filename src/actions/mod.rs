pub(crate) mod chat;
pub(crate) mod code;
pub(crate) mod images;
pub(crate) mod list_models;
pub(crate) mod shell;

// #[async_trait]
pub(crate) trait Action {
    fn exec(&self, user_prompt: String) -> Result<(), Box<dyn std::error::Error>>;
}
