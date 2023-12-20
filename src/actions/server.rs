use async_trait::async_trait;

use crate::actions::Action;
use crate::model::ChatTrait;
use crate::server;

pub(crate) struct ServerCommand {}

impl ServerCommand {
    pub fn new(adaptor: Box<dyn ChatTrait>) -> Self {
        ServerCommand {}
    }
}

impl Action for ServerCommand {
    fn exec(&self, _user_prompt: String) -> Result<(), Box<dyn std::error::Error>> {
        println!("server");
        futures::executor::block_on(async {
            match server::start().await {
                Ok(_) => (),
                Err(e) => {
                    println!("Error starting server: {}", e);
                }
            }
        });
        Ok(())
    }
}
