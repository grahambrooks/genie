use crate::actions::Action;
use crate::adaptors::ChatTrait;

pub(crate) struct GenerateImagesCommand {
    adaptor: Box<dyn ChatTrait>,
}

impl GenerateImagesCommand {
    pub fn new(adaptor: Box<dyn ChatTrait>) -> Self {
        GenerateImagesCommand {
            adaptor
        }
    }
}

impl Action for GenerateImagesCommand {
    fn exec(&self, user_prompt: String) -> Result<(), Box<dyn std::error::Error>> {
        println!("image");
        futures::executor::block_on(async {
            self.adaptor.generate_images(user_prompt).await
        })
    }
}
