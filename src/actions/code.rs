use crate::actions::Action;
use crate::model::ChatTrait;

pub(crate) struct GenerateCodeCommand {}

impl GenerateCodeCommand {
    pub fn new(adaptor: Box<dyn ChatTrait>) -> Self {
        GenerateCodeCommand {}
    }
}

impl Action for GenerateCodeCommand {
    fn exec(&self, user_prompt: String) -> Result<(), Box<dyn std::error::Error>> {
        println!("code");
        Ok(())
    }
}
