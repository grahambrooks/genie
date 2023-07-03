use std::{env, io};
use std::env::VarError;
use std::io::Read;

use clap::Parser;

use crate::client::GPT_3_5_TURBO;
use crate::completion::{CompletionMessage, StreamedResponse};

mod client;
mod completion;

#[derive(Parser, Debug)]
#[command(author = "Graham Brooks", version = "0.1", about = "Shell for AI assisted development", long_about = None)]
struct Args {
    #[arg(short, long)]
    summarize: bool,
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

    if args.summarize {
        println!("Summarize mode");
    }

    let client = client::Client::new(openapi_key.unwrap());

    fn callback(response: &StreamedResponse) {
        response.choices.iter().for_each(|c| {
            let content = &c.delta.content;
            match content {
                Some(content) => print!("{}", content),
                None => (),
            }
        });
    }

    let mut prompt = args.prompt.join(" ").to_string();

    if !atty::is(atty::Stream::Stdin) {
        let mut buffer: String = "".to_string();
        match io::stdin().read_to_string(&mut buffer) {
            Ok(_) => {
                prompt.push_str("\n");
                prompt.push_str(buffer.as_str());
            }
            Err(error) => {
                println!("Failed to read from stdin: {}", error);
            }
        }
    }

    completion::request([
        // &CompletionMessage::from_str("system", "you are a helpful assistant"),
        &CompletionMessage::from_str("user", prompt.as_str())
    ].to_vec())
        .model(GPT_3_5_TURBO)
        .temperature(0.5)
        .stream()
        .call_streamed_response(client, callback)
// .call(client, |s| println!("callback received {}", s))
        .await
}


