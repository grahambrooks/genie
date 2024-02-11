use crate::actions::Action;
use crate::adapters::Adapter;

pub(crate) struct ChatCommand {
    adapter: Box<dyn Adapter>,
}

impl ChatCommand {
    pub fn new(adapter: Box<dyn Adapter>) -> Self {
        ChatCommand {
            adapter
        }
    }
}


impl Action for ChatCommand {
    fn exec(&self, user_prompt: String) -> Result<(), Box<dyn std::error::Error>> {
        let future = async {
            self.adapter.prompt(user_prompt).await
        };

        futures::executor::block_on(future)
    }
}