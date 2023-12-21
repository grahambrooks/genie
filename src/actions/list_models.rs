use crate::actions::Action;
use crate::errors::GenieError;
use crate::adaptors::ChatTrait;

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
        let result = futures::executor::block_on(async {
            match self.adaptor.list_models().await {
                Ok(_) => Ok(()),
                Err(e) => Err(Box::new(GenieError::new(&format!("Error generating response: {}", e))))
            }
        });
        Ok(result?)
    }
}