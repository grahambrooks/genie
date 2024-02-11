use crate::actions::Action;
use crate::adapters::Adapter;

pub(crate) struct GenerateImagesCommand {
    adapter: Box<dyn Adapter>,
}

impl GenerateImagesCommand {
    pub fn new(adapter: Box<dyn Adapter>) -> Self {
        GenerateImagesCommand {
            adapter
        }
    }
}

impl Action for GenerateImagesCommand {
    fn exec(&self, user_prompt: String) -> Result<(), Box<dyn std::error::Error>> {
        println!("image");
        futures::executor::block_on(async {
            self.adapter.generate_images(user_prompt).await
        })
    }
}
