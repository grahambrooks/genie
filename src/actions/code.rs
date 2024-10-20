use crate::actions::Action;
use crate::adapters::Adapter;
use crate::expand_template;
use crate::messages::CODE_TEMPLATE;
use anyhow::Result;

pub(crate) struct GenerateCodeCommand {
    adapter: Box<dyn Adapter>,
}

impl GenerateCodeCommand {
    pub fn new(adapter: Box<dyn Adapter>) -> Self {
        GenerateCodeCommand { adapter }
    }
}

impl Action for GenerateCodeCommand {
    fn exec(&self, user_prompt: String) -> Result<()> {
        if user_prompt.is_empty() {
            return Err(anyhow::anyhow!("No prompt provided"));
        }

        let messages = expand_template(user_prompt, &CODE_TEMPLATE);

        let future = async {
            match self.adapter.generate(messages).await {
                Ok(response) => {
                    println!("{}", response);
                    Ok(())
                }
                Err(e) => Err(anyhow::anyhow!("Error executing shell action: {}", e)),
            }
        };

        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(future)
    }
}
