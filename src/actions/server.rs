use crate::actions::Action;
use crate::errors::GenieError;
use crate::adaptors::ChatTrait;
use crate::server;

pub(crate) struct ServerCommand {}

impl ServerCommand {
    pub fn new(_adaptor: Box<dyn ChatTrait>) -> Self {
        ServerCommand {}
    }
}

impl Action for ServerCommand {
    fn exec(&self, _user_prompt: String) -> Result<(), Box<dyn std::error::Error>> {
        println!("server");
        let result = futures::executor::block_on(async {
            match server::start().await {
                Ok(_) => Ok(()),
                Err(e) => Err(Box::new(GenieError::new(&format!("Error stating server: {}", e))))
            }
        });
        
        Ok(result?)
    }
}
