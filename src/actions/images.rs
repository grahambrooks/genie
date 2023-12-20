use async_openai::Client;

use crate::actions::Action;
use crate::images;
use crate::model::ChatTrait;

pub(crate) struct GenerateImagesCommand {}

impl GenerateImagesCommand {
    pub fn new(_adaptor: Box<dyn ChatTrait>) -> Self {
        GenerateImagesCommand {}
    }
}

impl Action for GenerateImagesCommand {
    fn exec(&self, user_prompt: String) -> Result<(), Box<dyn std::error::Error>> {
        println!("image");
        futures::executor::block_on(async {
            let connection = Client::new();

            match images::generator(connection)
                .count(images::IMAGE_COUNT)
                .size(images::IMAGE_SIZE)
                .path(images::SAVE_PATH)
                .generate(user_prompt).await {
                Ok(_) => (),
                Err(e) => {
                    println!("Error generating images: {}", e);
                }
            }
        });
        Ok(())
    }
}
