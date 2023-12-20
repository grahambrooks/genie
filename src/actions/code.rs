use crate::actions::Action;
use crate::model::ChatTrait;

pub(crate) struct GenerateCodeCommand {}

impl GenerateCodeCommand {
    pub fn new(_adaptor: Box<dyn ChatTrait>) -> Self {
        GenerateCodeCommand {}
    }
}

impl Action for GenerateCodeCommand {
    fn exec(&self, _user_prompt: String) -> Result<(), Box<dyn std::error::Error>> {
        println!("code");
        Ok(())
    }
}
