extern crate termion;


use std::{env, io};
use std::env::VarError;
use std::error::Error;
use std::io::{Read, stdout, Write};
use std::ops::Deref;
use std::process::Command;

use async_openai::Client;
use async_openai::config::OpenAIConfig;
use async_openai::types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs};
use clap::Parser;
use futures::StreamExt;
use termion::event::Key;
use termion::input::TermRead;

use crate::messages::{CODE_TEMPLATE, DEFAULT_TEMPLATE, SHELL_TEMPLATE};
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
    #[arg(long, help = "use local model")]
    local: bool,
    #[arg(long, help = "the model. e.g. openai::gpt-4, ollama::mistral to use", default_value = "openai::gpt-3.5-turbo")]
    model: String,
    prompt: Vec<String>,
}

fn key() -> Result<String, VarError> {
    env::var("OPENAI_API_KEY")
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let current_model = GPT_3_5_TURBO;

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

//     let model = model::Model::from_string(args.model.as_str());
//     let _ = model.chat_adaptor().prompt(args.prompt.join(" ").to_string()).await;
//
//     let openapi_key = key();
//     match openapi_key {
//         Ok(_) => (),
//         Err(e) => {
//             println!("OPENAPI_KEY error: {}", e);
//             return;
//         }
//     }
//
//     if args.list_models {
//         match list_models(current_model).await {
//             Ok(_) => {}
//             Err(_) => { println!("Error listing available models") }
//         }
//
//         return;
//     }
//
//     if args.server {
//         let _ = server::start().await;
//         return;
//     }
//
//     if args.prompt.is_empty() {
//         println!(r"
// Please provide a prompt on the command line.
// e.g. genie how far away is the sun
// genie --help
// for more information.");
//         return;
//     }
//
//     let connection = Client::new();
//
//     if args.image {
//         match images::generator(connection)
//             .count(images::IMAGE_COUNT)
//             .size(images::IMAGE_SIZE)
//             .path(images::SAVE_PATH)
//             .generate(args.prompt).await {
//             Ok(_) => (),
//             Err(e) => {
//                 println!("Error generating images: {}", e);
//                 return;
//             }
//         }
//         return;
//     }
//
//     if args.command {
//         // if true {
//         //     println!("--command not currently implemented");
//         //     return;
//         // }
//
//         if action() {
//             println!("action");
//         } else {
//             println!("no action");
//         }
//
//         match exec() {
//             Ok(_) => (),
//             Err(e) => {
//                 println!("exec error: {}", e);
//                 return;
//             }
//         }
//
//         command(connection, current_model, args.prompt).await;
//         return;
//     }
//
//     if args.code {
//         code(connection, current_model, args.prompt).await;
//         return;
//     }
//
//     default(connection, current_model, args.prompt).await;
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

    if args.local {
        return Box::new(actions::embedded::EmbeddedChatCommand::new(adaptor));
    }

    Box::new(actions::chat::ChatCommand::new(adaptor))
}


async fn list_models(current_model: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let model_list = client.models().list().await?;


    for model in model_list.data {
        if model.id == current_model {
            println!("* {}", model.id, );
            continue;
        }
        println!("  {}", model.id, );
    }
    Ok(())
}

async fn command(connection: Client<OpenAIConfig>, model: &str, elements: Vec<String>) {
    let mut prompt = elements.join(" ").to_string();
    prompt.push_str(read_stdin().as_str());

    let messages = expand_template(prompt, &SHELL_TEMPLATE);

    let _ = make_request(connection, model, messages).await;
}

async fn code(connection: Client<OpenAIConfig>, model: &str, elements: Vec<String>) {
    let mut prompt = elements.join(" ").to_string();
    prompt.push_str(read_stdin().as_str());

    let messages = expand_template(prompt, &CODE_TEMPLATE);

    let _ = make_request(connection, model, messages).await;
}

async fn default(connection: Client<OpenAIConfig>, model: &str, elements: Vec<String>) {
    let mut prompt = elements.join(" ").to_string();
    prompt.push_str(read_stdin().as_str());

    let messages = expand_template(prompt, &DEFAULT_TEMPLATE);

    let _ = make_request(connection, model, messages).await;
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

async fn make_request(connection: Client<OpenAIConfig>, model: &str, prompt: String) -> Result<(), Box<dyn Error>> {
    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .max_tokens(512u16)
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content(prompt)
            .build()?
            .into()])
        .build()?;

    let mut stream = connection.chat().create_stream(request).await?;

    // From Rust docs on print: https://doc.rust-lang.org/std/macro.print.html
    //
    //  Note that stdout is frequently line-buffered by default so it may be necessary
    //  to use io::stdout().flush() to ensure the output is emitted immediately.
    //
    //  The print! macro will lock the standard output on each call.
    //  If you call print! within a hot loop, this behavior may be the bottleneck of the loop.
    //  To avoid this, lock stdout with io::stdout().lock():

    let mut lock = stdout().lock();
    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                response.choices.iter().for_each(|chat_choice| {
                    if let Some(ref content) = chat_choice.delta.content {
                        write!(lock, "{}", content).unwrap();
                    }
                });
            }
            Err(err) => {
                writeln!(lock, "error: {err}").unwrap();
            }
        }
        stdout().flush()?;
    }

    Ok(())
}


#[allow(dead_code)]
fn exec() -> io::Result<()> {
    let output = Command::new("ls").arg("-l").arg("/").output()?;

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("ls -l returned:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        eprint!("ls -l returned an error:\n{}", s);
    }

    Ok(())
}

#[allow(dead_code)]
#[allow(clippy::never_loop)]
fn action() -> bool {
    let stdin = io::stdin();
    for c in stdin.keys() {
        return matches!(c.unwrap(), Key::Char('e'));
    }
    false
}
