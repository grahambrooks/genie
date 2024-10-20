use crate::actions::Action;
use crate::adapters::Adapter;
use anyhow::Result;

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
    fn exec(&self, user_prompt: String) -> Result<()> {
        if user_prompt.is_empty() {
            return Err(anyhow::anyhow!("No prompt provided"));
        }

        let image_path  = self.image_path.clone();
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            self.adapter.generate_images(user_prompt, image_path).await
        })
    }
}
