use crate::actions::Action;
use crate::adapters::Adapter;
use crate::errors::GenieError;
use crate::expand_template;
use crate::messages::CODE_TEMPLATE;

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
        if user_prompt.is_empty() {
            return Err(Box::new(GenieError::new("No prompt provided")));
        }

        let messages = expand_template(user_prompt, &CODE_TEMPLATE);

        let future = async {
            match self.adapter.generate(messages).await {
                Ok(response) => {
                    println!("{}", response);
                    Ok(())
                }
                Err(e) => Err(Box::new(GenieError::new(&format!("Error executing shell action: {}", e)))),
            }
        };

        Ok(futures::executor::block_on(future)?)
    }
}
