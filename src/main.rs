extern crate termion;

use std::io;
use std::io::Read;
use clap::Parser;
use run::RunCommand;
use crate::model::Model;
use dotenv::var;
use anyhow::Result;

pub const GPT_3_5_TURBO: &str = "gpt-3.5-turbo";
pub const GPT_4_0: &str = "gpt-4-1106-preview";

mod messages;
mod context;
mod model;
mod actions;
mod adapters;
mod errors;
mod filesystem;
mod run;

// Buiild a version string based on the version in Cargo.toml and the git commit hash
static VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), ".", include_str!(concat!(env!("OUT_DIR"), "/version.txt")));

#[derive(Parser, Debug)]
#[command(
color = clap::ColorChoice::Auto,
author = "Graham Brooks",
version = VERSION,
about = "Shell for AI assisted development",
long_about = r#"
CLI for AI assisted software development.

In default mode genie responds to prompts and exists.

In command mode genie generates a command line give the prompt and the option to run the command.

In code mode genie generates source code in response to the prompt.

To use genie needs an OPENAI_API_KEY environment variable set to a valid OpenAI API key.
"#
)]
struct Args {
    #[arg(long, help = "generate a command line give the prompt and the option to run the command")]
    command: bool,
    #[arg(long, help = "generate source code in response to the prompt")]
    code: bool,
    #[arg(long, help = "generate images based on the prompt to the given directory")]
    image: Option<String>,
    #[arg(long, help = "List the available OpenAi models")]
    list_models: bool,
    #[arg(long, help = "the model. e.g. openai::gpt-4, ollama::mistral to use", default_value = "openai::gpt-3.5-turbo")]
    model: String,
    #[arg(long, help = "Run a genie script")]
    run: Option<String>,
    prompt: Vec<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut user_prompt = args.prompt.join(" ").to_string();
    user_prompt.push_str(read_stdin().as_str());

    match parse_command_from_args(args) {
        Ok(cmd) => {
            match cmd.exec(user_prompt) {
                Ok(_) => (),
                Err(e) => {
                    println!("Error: {}\nUsage: genie --help", e);
                }
            };
        }
        Err(e) => {
            println!("Error: {}", e)
        }
    }
}

fn parse_command_from_args(args: Args) -> Result<Box<dyn actions::Action>> {
    if args.run.is_some() {
        return Ok(Box::new(RunCommand::new(args.run.unwrap())));
    }

    if args.command {
        let adapter = Model::from_string(args.model.as_str())?.adapter()?;
        return Ok(Box::new(actions::shell::ShellCommand::new(adapter)));
    }

    if args.code {
        let adapter = Model::from_string(args.model.as_str())?.adapter()?;
        return Ok(Box::new(actions::code::GenerateCodeCommand::new(adapter)));
    }

    if args.image.is_some() {
        let adapter = Model::from_string(args.model.as_str())?.adapter()?;
        return Ok(Box::new(actions::images::GenerateImagesCommand::new(adapter, args.image.unwrap())));
    }

    if args.list_models {
        let adapter = Model::from_string(args.model.as_str())?.adapter()?;
        return Ok(Box::new(actions::list_models::ListModelsCommand::new(adapter)));
    }

    let adapter = Model::from_string(args.model.as_str())?.adapter()?;
    Ok(Box::new(actions::chat::ChatCommand::new(adapter)))
}

fn expand_template(prompt: String, template: &messages::template::Template) -> String {
    template
        .expand(vec![
            ("shell", context::shell().as_str()),
            ("os", context::os_type_and_version().as_str()),
            ("request", prompt.as_str()),
        ])
        .unwrap()
}

fn read_stdin() -> String {
    let mut stdin_content = "".to_string();

    if !atty::is(atty::Stream::Stdin) {
        let mut input_buffer = "".to_string();

        match io::stdin().read_to_string(&mut input_buffer) {
            Ok(_) => {
                stdin_content.push('\n');
                stdin_content.push_str(&input_buffer);
            }
            Err(err) => {
                println!("Failed to read from stdin: {}", err);
                return "".to_string();
            }
        }
    }

    stdin_content
}
