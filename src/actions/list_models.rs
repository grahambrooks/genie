use crate::actions::Action;
use crate::model::ChatTrait;

pub(crate) struct ListModelsCommand {
    adaptor: Box<dyn ChatTrait>,
}

impl ListModelsCommand {
    pub fn new(adaptor: Box<dyn ChatTrait>) -> Self {
        ListModelsCommand {
            adaptor
        }
    }
}

impl Action for ListModelsCommand {
    fn exec(&self, _user_prompt: String) -> Result<(), Box<dyn std::error::Error>> {
        println!("models");
        futures::executor::block_on(async {
            match self.adaptor.list_models().await {
                Ok(_) => (),
                Err(e) => {
                    println!("Error generating images: {}", e);
                }
            }
        });
        Ok(())
    }
}