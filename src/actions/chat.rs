use crate::actions::Action;
use crate::adapters::Adapter;
use crate::expand_template;
use crate::messages::DEFAULT_TEMPLATE;
use anyhow::Result;

pub(crate) struct ChatCommand {
    adapter: Box<dyn Adapter>,
}

impl ChatCommand {
    pub fn new(adapter: Box<dyn Adapter>) -> Self {
        ChatCommand {
            adapter
        }
    }
}


impl Action for ChatCommand {
    fn exec(&self, user_prompt: String) -> Result<()> {
        if user_prompt.is_empty() {
            return Err(anyhow::anyhow!("No prompt provided"));
        }

        let messages = expand_template(user_prompt, &DEFAULT_TEMPLATE);

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
