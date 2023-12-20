use crate::actions::Action;
use crate::model::ChatTrait;

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
            match self.adaptor.prompt(user_prompt).await {
                Ok(_) => (),
                Err(e) => {
                    println!("Error generating response: {}", e);
                }
            }
        });
        Ok(())
    }
}
