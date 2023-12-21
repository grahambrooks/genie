use crate::actions::Action;
use crate::adaptors::ChatTrait;

pub(crate) struct ChatCommand {
    adaptor: Box<dyn ChatTrait>,
}

impl ChatCommand {
    pub fn new(adaptor: Box<dyn ChatTrait>) -> Self {
        ChatCommand {
            adaptor
        }
    }
}


impl Action for ChatCommand {
    fn exec(&self, user_prompt: String) -> Result<(), Box<dyn std::error::Error>> {
        let future = async {
            self.adaptor.prompt(user_prompt).await
        };

        futures::executor::block_on(future)
    }
}