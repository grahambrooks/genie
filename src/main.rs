extern crate termion;

use std::{env, io};
use std::env::VarError;
use std::error::Error;
use std::io::Read;
use std::process::Command;

use async_openai::Client;
use clap::Parser;
use termion::event::Key;
use termion::input::TermRead;

use openai::{CompletionMessage, GPT_3_5_TURBO, StreamedResponse};

use crate::messages::{CODE_TEMPLATE, DEFAULT_TEMPLATE, SHELL_TEMPLATE};

mod messages;
mod context;
mod server;
mod web_socket;

#[derive(Parser, Debug)]
#[command(
color = clap::ColorChoice::Auto,
author = "Graham Brooks",
version,
about = "Shell for AI assisted development",
long_about = r#"Shell for AI assisted development.

    In default mode dev-shell responds to prompts and exists.

    In command mode dev-shell generates a command line give the prompt and the option to run the command.

    In code mode dev-shell generates source code in response to the prompt.

    dev-shell needs an OPENAI_API_KEY environment variable set to a valid OpenAI API key.
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
    #[arg(long, help = "List the available models")]
    list_models: bool,
    #[arg(long, help = "Run as a web server")]
    server: bool,
    prompt: Vec<String>,
}

fn key() -> Result<String, VarError> {
    env::var("OPENAI_API_KEY")
}

#[tokio::main]
async fn main() {
    // let current_model = GPT_3_5_TURBO;
    let current_model = GPT_3_5_TURBO;
    let openapi_key = key();

    match openapi_key {
        Ok(_) => (),
        Err(e) => {
            println!("OPENAPI_KEY error: {}", e);
            return;
        }
    }
    let args = Args::parse();

    if args.list_models {
        match list_models(current_model).await {
            Ok(_) => {}
            Err(_) => { println!("Error listing available models") }
        }

        return;
    }

    if args.server {
        let _ = server::start().await;
        return;
    }

    if args.prompt.is_empty() {
        println!("Please provide a prompt. dev-shell --help for more information.");
        return;
    }

    let connection = openai::Connection::new(openapi_key.unwrap());

    if args.command {
        // if true {
        //     println!("--command not currently implemented");
        //     return;
        // }

        if action() {
            println!("action");
        } else {
            println!("no action");
        }

        match exec() {
            Ok(_) => (),
            Err(e) => {
                println!("exec error: {}", e);
                return;
            }
        }

        command(connection, current_model, args.prompt).await;
        return;
    }

    if args.code {
        code(connection, current_model, args.prompt).await;
        return;
    }

    default(connection, current_model, args.prompt).await;
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

async fn command(connection: openai::Connection, model: &str, elements: Vec<String>) {
    let mut prompt = elements.join(" ").to_string();
    prompt.push_str(read_stdin().as_str());

    let messages = expand_template(prompt, &SHELL_TEMPLATE);

    make_request(connection, model, messages).await;
}

async fn code(connection: openai::Connection, model: &str, elements: Vec<String>) {
    let mut prompt = elements.join(" ").to_string();
    prompt.push_str(read_stdin().as_str());

    let messages = expand_template(prompt, &CODE_TEMPLATE);

    make_request(connection, model, messages).await;
}

async fn default(connection: openai::Connection, model: &str, elements: Vec<String>) {
    let mut prompt = elements.join(" ").to_string();
    prompt.push_str(read_stdin().as_str());

    let messages = expand_template(prompt, &DEFAULT_TEMPLATE);

    make_request(connection, model, messages).await;
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

async fn make_request(connection: openai::Connection, model: &str, prompt: String) {
    openai::request([&CompletionMessage::from_str("user", prompt.as_str())].to_vec())
        .model(model)
        .temperature(0.5)
        .stream()
        .call_streamed_response(connection, stream_callback)
        .await;
}

fn stream_callback(response: &StreamedResponse) {
    response.choices.iter().for_each(|c| {
        let content = &c.delta.content;
        match content {
            Some(content) => print!("{}", content),
            None => (),
        }
    });
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
