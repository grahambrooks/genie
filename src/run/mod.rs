use std::error::Error;

use crate::actions::Action;
use crate::errors::GenieError;
use crate::model::Model;
use crate::run::script::SinkTarget;

mod template;
mod files;
mod file_extensions;
mod script;

pub(crate) struct RunCommand {
    script: String,
}


impl RunCommand {
    pub fn new(script: String) -> Self {
        RunCommand { script }
    }
}

impl Action for RunCommand {
    fn exec(&self, _user_prompt: String) -> Result<(), Box<dyn Error>> {
        println!("run {}", self.script);

        let script = script::parse_script_file(&self.script)?;

        let adapter = Model::from_string(script.task.model.as_str()).unwrap().adapter()?;

        let mut count = 0;

        script.task.source.items().iter().for_each(|item| {
            count += 1;
            println!("{}", "-".repeat(80));
            println!("\x1b[34m{}\x1b[0m", item);

            let prompt = template::expand_template(script.task.template.as_ref().unwrap().clone(), item.to_string());

            let future = async {
                match adapter.generate(prompt).await {
                    Ok(response_) => {
                        match script.task.sink.write(item.to_string(), response_) {
                            Ok(_) => Ok(()),
                            Err(e) => Err(Box::new(GenieError::new(&format!("Error writing to sink: {}", e)))),
                        }
                    }
                    Err(e) => Err(Box::new(GenieError::new(&format!("Error writing to sink: {}", e)))),
                }
            };
            let _ = futures::executor::block_on(future);
        });

        println!("processed {} items", count);

        Ok(())
    }
}
