use std::error::Error;
use std::io;
use std::io::Write;
use std::process::Command;

use termion::event::Key;
use termion::input::TermRead;

use crate::actions::Action;
use crate::adapters::Adapter;
use crate::errors::GenieError;
use crate::expand_template;
use crate::messages::SHELL_TEMPLATE;
use crate::model::ShellExecutor;

pub(crate) struct ShellCommand {
    // adapter: &'a dyn ChatTrait,
    adapter: Box<dyn Adapter>,
}

impl ShellCommand {
    pub fn new(adapter: Box<dyn Adapter>) -> Self {
        ShellCommand { adapter }
    }
}

impl Action for ShellCommand {
    fn exec(&self, user_prompt: String) -> Result<(), Box<dyn Error>> {
        if user_prompt.is_empty() {
            return Err(Box::new(GenieError::new("No prompt provided")));
        }
        let messages = expand_template(user_prompt, &SHELL_TEMPLATE);
        let future = async {
            match self.adapter.generate(messages).await {
                Ok(response) => {
                    println!("{}", response);

                    print!("Run these commands? (n/e): ");
                    let _ = io::stdout().flush();
                    if action() {
                        exec(response).await.unwrap();
                    }
                    Ok(())
                }
                Err(e) => Err(Box::new(GenieError::new(&format!("Error executing shell action: {}", e)))),
            }
        };
        let result = futures::executor::block_on(future);

        Ok(result?)
    }
}

struct BashExecutor {}

impl ShellExecutor for BashExecutor {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

async fn exec(script: String) -> io::Result<()> {
    println!("executing: {}", script);
    // remove grave characters from the start and end of the script

    let s = script.trim_start_matches('`');

    let mut child = Command::new("zsh")
        .stdin(std::process::Stdio::piped())
        .spawn()?;


    {
        let stdin = child.stdin.as_mut().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to open stdin"))?;
        stdin.write_all(s.as_bytes())?;
    }

    let output = child.wait_with_output()?;

    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;
    Ok(())
}

#[allow(clippy::never_loop)]
fn action() -> bool {
    let stdin = io::stdin();
    for c in stdin.keys() {
        return matches!(c.unwrap(), Key::Char('e'));
    }
    false
}
