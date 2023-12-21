use crate::actions::Action;
use crate::adaptors::ChatTrait;

pub(crate) struct GenerateCodeCommand {
    adaptor: Box<dyn ChatTrait>,
}

impl GenerateCodeCommand {
    pub fn new(adaptor: Box<dyn ChatTrait>) -> Self {
        GenerateCodeCommand { adaptor }
    }
}

impl Action for GenerateCodeCommand {
    fn exec(&self, user_prompt: String) -> Result<(), Box<dyn std::error::Error>> {
        println!("code");
        futures::executor::block_on(async {
            self.adaptor.generate_code(user_prompt).await
        })
    }
}
