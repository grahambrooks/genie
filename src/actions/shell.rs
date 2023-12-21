use std::error::Error;

use crate::actions::Action;
use crate::adaptors::ChatTrait;
use crate::errors::GenieError;
use crate::model::ShellExecutor;

pub(crate) struct ShellCommand {
    // adaptor: &'a dyn ChatTrait,
    adaptor: Box<dyn ChatTrait>,
}

impl<'a> ShellCommand {
    // pub fn new(adaptor: &'a dyn ChatTrait) -> Self {
    //     ShellCommand { adaptor }
    // }
    pub fn new(adaptor: Box<dyn ChatTrait>) -> Self {
        ShellCommand { adaptor: adaptor }
    }
}

impl Action for ShellCommand {
    fn exec(&self, user_prompt: String) -> Result<(), Box<dyn std::error::Error>> {
        println!("command");

        let future = async {
            // let executor = Box::new(BashExecutor {});
            match self.adaptor.shell(user_prompt).await {
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