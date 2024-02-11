use crate::actions::Action;
use crate::adapters::Adapter;

pub(crate) struct GenerateCodeCommand {
    adapter: Box<dyn Adapter>,
}

impl GenerateCodeCommand {
    pub fn new(adapter: Box<dyn Adapter>) -> Self {
        GenerateCodeCommand { adapter }
    }
}

impl Action for GenerateCodeCommand {
    fn exec(&self, user_prompt: String) -> Result<(), Box<dyn std::error::Error>> {
        println!("code");
        futures::executor::block_on(async {
            self.adapter.generate_code(user_prompt).await
        })
    }
}
