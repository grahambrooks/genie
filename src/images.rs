use async_openai::Client;
use async_openai::config::OpenAIConfig;
use async_openai::types::{CreateImageRequestArgs, ImageSize, ResponseFormat};
use log::info;

pub(crate) const IMAGE_COUNT: u8 = 2;
pub(crate) const IMAGE_SIZE: ImageSize = ImageSize::S1024x1024;
pub(crate) const SAVE_PATH: &str = "./data";

pub(crate) struct Generator {
    client: Client<OpenAIConfig>,
    count: u8,
    size: ImageSize,
    path: &'static str,
}

pub(crate) fn generator(client: Client<OpenAIConfig>) -> Generator {
    Generator { client, count: IMAGE_COUNT, size: IMAGE_SIZE, path: SAVE_PATH }
}

impl Generator {
    pub(crate) fn count(&mut self, count: u8) -> &mut Self {
        self.count = count;
        self
    }

    pub(crate) fn size(&mut self, size: ImageSize) -> &mut Self {
        self.size = size;
        self
    }

    pub(crate) fn path(&mut self, path: &'static str) -> &mut Self {
        self.path = path;
        self
    }
    pub(crate) async fn generate(&self, user_prompt: String) -> Result<(), Box<dyn std::error::Error>> {
        info!("image prompt: {}", user_prompt);
        let request = CreateImageRequestArgs::default()
            .prompt(user_prompt)
            .n(self.count)
            .response_format(ResponseFormat::Url)
            .size(self.size)
            .user("async-openai")
            .build()?;

        let response = self.client.images().create(request).await?;

        let paths = response.save(self.path).await?;

        for path in &paths {
            info!("Image file path: {}", path.display());
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    // use async_openai::Client;
    // use async_openai::types::ImageSize;
    use super::*;

    #[test]
    fn test_construction() {
        let connection = Client::new();

        let mut gen = generator(connection);
        let foo = gen.count(100).path("/foo").size(ImageSize::S512x512);
        validate_generator(foo, 100, "/foo", ImageSize::S512x512);
    }

    fn validate_generator(gen: &Generator, count: u8, path: &str, size: ImageSize) {
        assert_eq!(gen.size, size);
        assert_eq!(gen.count, count);
        assert_eq!(gen.path, path);
    }
}