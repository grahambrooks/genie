use crate::actions::Action;
use crate::adapters::Adapter;
use anyhow::Result;

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
    fn exec(&self, _user_prompt: String) -> Result<()> {
        println!("models");
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            match self.adapter.list_models().await {
                Ok(_) => Ok(()),
                Err(e) => Err(anyhow::anyhow!("Error generating response: {}", e))
            }
        })
    }
}
