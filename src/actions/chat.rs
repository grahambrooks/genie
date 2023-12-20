use crate::actions::Action;
use crate::model::ChatTrait;

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
        futures::executor::block_on(async {
            match self.adaptor.prompt(user_prompt).await {
                Ok(_) => (),
                Err(e) => {
                    println!("Error generating images: {}", e);
                }
            }
        });
        Ok(())
    }
}

