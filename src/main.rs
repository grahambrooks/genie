extern crate termion;


use std::io;
// use std::env::VarError;
use std::io::Read;

use clap::Parser;

use crate::model::Model;

pub const GPT_3_5_TURBO: &str = "gpt-3.5-turbo";
pub const GPT_4_0: &str = "gpt-4-1106-preview";

mod messages;
mod context;
mod server;
mod web_socket;
mod images;
mod model;
mod actions;
mod adaptors;
mod errors;
mod filesystem;

#[derive(Parser, Debug)]
#[command(
color = clap::ColorChoice::Auto,
author = "Graham Brooks",
version,
about = "Shell for AI assisted development",
long_about = r#"Shell for AI assisted development.

    In default mode genie responds to prompts and exists.

    In command mode genie generates a command line give the prompt and the option to run the command.

    In code mode genie generates source code in response to the prompt.

    genie needs an OPENAI_API_KEY environment variable set to a valid OpenAI API key.
"#
)]
struct Args {
    #[arg(
    long,
    help = "generate a command line give the prompt and the option to run the command"
    )]
    command: bool,
    #[arg(long, help = "generate source code in response to the prompt")]
    code: bool,
    #[arg(long, help = "generate images based on the prompt")]
    image: bool,
    #[arg(long, help = "List the available models")]
    list_models: bool,
    #[arg(long, help = "Run as a web server")]
    server: bool,
    #[arg(long, help = "the model. e.g. openai::gpt-4, ollama::mistral to use", default_value = "openai::gpt-3.5-turbo")]
    model: String,
    prompt: Vec<String>,
}

// fn key() -> Result<String, VarError> {
//     env::var("OPENAI_API_KEY")
// }

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut user_prompt = args.prompt.join(" ").to_string();
    user_prompt.push_str(read_stdin().as_str());

    let cmd = parse_command_from_args(args);

    match cmd.exec(user_prompt) {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
}

fn parse_command_from_args(args: Args) -> Box<dyn actions::Action> {
    let adaptor = Model::from_string(args.model.as_str()).chat_adaptor();

    if args.command {
        return Box::new(actions::shell::ShellCommand::new(adaptor));
    }

    if args.code {
        return Box::new(actions::code::GenerateCodeCommand::new(adaptor));
    }

    if args.image {
        return Box::new(actions::images::GenerateImagesCommand::new(adaptor));
    }

    if args.server {
        return Box::new(actions::server::ServerCommand::new(adaptor));
    }

    if args.list_models {
        return Box::new(actions::list_models::ListModelsCommand::new(adaptor));
    }

    Box::new(actions::chat::ChatCommand::new(adaptor))
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
