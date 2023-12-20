use std::error::Error;
use crate::actions::Action;
use crate::model::{ChatTrait, ShellExecutor};

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

        futures::executor::block_on(async {
            // let executor = Box::new(BashExecutor {});
            match self.adaptor.shell(user_prompt).await {
                Ok(_) => (),
                Err(e) => {
                    println!("Error generating response: {}", e);
                }
            }
        });
        Ok(())
    }
}

struct BashExecutor {}

impl ShellExecutor for  BashExecutor {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}