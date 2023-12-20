use crate::actions::Action;
use crate::model::ChatTrait;

pub(crate) struct EmbeddedChatCommand {
    adaptor: Box<dyn ChatTrait>,
}

impl EmbeddedChatCommand {
    pub fn new(adaptor: Box<dyn ChatTrait>) -> Self {
        EmbeddedChatCommand {
            adaptor: adaptor
        }
    }
}

impl Action for EmbeddedChatCommand {
    fn exec(&self, user_prompt: String) -> Result<(), Box<dyn std::error::Error>> {
        println!("embedded_chat_command");
        futures::executor::block_on(async {
            match self.adaptor.prompt(user_prompt).await {
                Ok(_) => (),
                Err(e) => (),
            }
        });
        Ok(())
    }
}

