use crate::actions::Action;
use crate::adapters::Adapter;
use crate::errors::GenieError;

pub(crate) struct ListModelsCommand {
    adapter: Box<dyn Adapter>,
}

impl ListModelsCommand {
    pub fn new(adapter: Box<dyn Adapter>) -> Self {
        ListModelsCommand {
            adapter
        }
    }
}

impl Action for ListModelsCommand {
    fn exec(&self, _user_prompt: String) -> Result<(), Box<dyn std::error::Error>> {
        println!("models");
        let result = futures::executor::block_on(async {
            match self.adapter.list_models().await {
                Ok(_) => Ok(()),
                Err(e) => Err(Box::new(GenieError::new(&format!("Error generating response: {}", e))))
            }
        });
        Ok(result?)
    }
}