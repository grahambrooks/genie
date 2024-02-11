use std::error::Error;

use crate::actions::Action;
use crate::adapters::Adapter;
use crate::errors::GenieError;
use crate::model::ShellExecutor;

pub(crate) struct ShellCommand {
    // adapter: &'a dyn ChatTrait,
    adapter: Box<dyn Adapter>,
}

impl ShellCommand {
    pub fn new(adapter: Box<dyn Adapter>) -> Self {
        ShellCommand { adapter }
    }
}

impl Action for ShellCommand {
    fn exec(&self, user_prompt: String) -> Result<(), Box<dyn Error>> {
        println!("command");

        let future = async {
            match self.adapter.shell(user_prompt).await {
                Ok(_) => Ok(()),
                Err(e) => Err(Box::new(GenieError::new(&format!("Error executing shell action: {}", e)))),
            }
        };
        let result = futures::executor::block_on(future);

        Ok(result?)
    }
}

struct BashExecutor {}

impl ShellExecutor for BashExecutor {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
