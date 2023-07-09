use std::{env, io};
use std::env::VarError;
use std::io::Read;

use clap::Parser;

use crate::client::GPT_3_5_TURBO;
use crate::completion::{CompletionMessage, StreamedResponse};
use crate::messages::{CODE_TEMPLATE, DEFAULT_TEMPLATE, SHELL_TEMPLATE};

mod client;
mod completion;
mod context;
mod messages;

#[derive(Parser, Debug)]
#[command(author = "Graham Brooks", version = "0.1", about = "Shell for AI assisted development", long_about = r#"Shell for AI assisted development.

    In default mode dev-shell responds to prompts and exists.

    In command mode dev-shell generates a command line give the prompt and the option to run the command.

    In code mode dev-shell generates source code in response to the prompt.

    dev-shell needs an OPENAI_API_KEY environment variable set to a valid OpenAI API key.
"#, )]
struct Args {
    #[arg(long, help = "generate a command line give the prompt and the option to run the command")]
    command: bool,
    #[arg(long, help = "generate source code in response to the prompt")]
    code: bool,
    prompt: Vec<String>,
}

fn key() -> Result<String, VarError> {
    env::var("OPENAI_API_KEY")
}

#[tokio::main]
async fn main() {
    let openapi_key = key();

    match openapi_key {
        Ok(_) => (),
        Err(e) => {
            println!("OPENAPI_KEY error: {}", e);
            return;
        }
    }

    let args = Args::parse();

    if args.prompt.is_empty() {
        println!("Please provide a prompt. dev-shell --help for more information.");
        return;
    }

    let client = client::Client::new(openapi_key.unwrap());

    if args.command {
        command(client, args.prompt).await;
        return;
    }

    if args.code {
        code(client, args.prompt).await;
        return;
    }

    default(client, args.prompt).await;
}

async fn command(client: client::Client, elements: Vec<String>) {
    let mut prompt = elements.join(" ").to_string();
    prompt.push_str(read_stdin().as_str());

    let messages = SHELL_TEMPLATE.expand(vec![
        ("shell", context::shell().as_str()),
        ("os", context::os().as_str()),
        ("request", prompt.as_str()),
    ]).unwrap();

    make_request(client, messages).await;
}

async fn code(client: client::Client, elements: Vec<String>) {
    let mut prompt = elements.join(" ").to_string();
    prompt.push_str(read_stdin().as_str());

    let messages = CODE_TEMPLATE.expand(vec![
        ("request", prompt.as_str()),
    ]).unwrap();

    make_request(client, messages).await;
}

async fn default(client: client::Client, elements: Vec<String>) {
    let mut prompt = elements.join(" ").to_string();
    prompt.push_str(read_stdin().as_str());

    let messages = DEFAULT_TEMPLATE.expand(vec![
        ("shell", context::shell().as_str()),
        ("os", context::os().as_str()),
        ("request", prompt.as_str()),
    ]).unwrap();

    make_request(client, messages).await;
}


fn read_stdin() -> String {
    let mut result: String = "".to_string();
    if !atty::is(atty::Stream::Stdin) {
        let mut buffer: String = "".to_string();
        match io::stdin().read_to_string(&mut buffer) {
            Ok(_) => {
                result.push('\n');
                result.push_str(buffer.as_str());
            }
            Err(error) => {
                println!("Failed to read from stdin: {}", error);
                return "".to_string();
            }
        }
    }
    result.to_string()
}

async fn make_request(client: client::Client, prompt: String) {
    completion::request([
        &CompletionMessage::from_str("user", prompt.as_str())
    ].to_vec())
        .model(GPT_3_5_TURBO)
        .temperature(0.5)
        .stream()
        .call_streamed_response(client, callback)
        .await;
}


fn callback(response: &StreamedResponse) {
    response.choices.iter().for_each(|c| {
        let content = &c.delta.content;
        match content {
            Some(content) => print!("{}", content),
            None => (),
        }
    });
}
