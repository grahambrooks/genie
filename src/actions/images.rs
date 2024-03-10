use crate::actions::Action;
use crate::adapters::Adapter;
use crate::errors::GenieError;

pub(crate) struct GenerateImagesCommand {
    adapter: Box<dyn Adapter>,
    image_path: String,
}

impl GenerateImagesCommand {
    pub fn new(adapter: Box<dyn Adapter>, image_path: String) -> Self {
        GenerateImagesCommand {
            adapter,
            image_path
        }
    }
}

impl Action for GenerateImagesCommand {
    fn exec(&self, user_prompt: String) -> Result<(), Box<dyn std::error::Error>> {
        if user_prompt.is_empty() {
            return Err(Box::new(GenieError::new("No prompt provided")));
        }

        let image_path  = self.image_path.clone();
        futures::executor::block_on(async {
            self.adapter.generate_images(user_prompt, image_path).await
        })
    }
}
