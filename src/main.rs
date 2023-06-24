// use reqwest::{header, Client};
mod prompt;
mod client;

use clap::{Parser};
use std::env;
use std::env::VarError;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(author = "Graham Brooks", version = "0.1", about = "Shell for AI assisted development", long_about = None)]
struct Args {
    #[arg(short, long)]
    summarize: bool,
    prompt: Vec<String>,
}


fn key() -> Result<String, VarError> {
 return env::var("OPENAI_API_KEY");
}

#[tokio::main]
async fn main() {
    let openapi_key = key();

    match openapi_key {
        Ok(_) => (),
        Err(e) => {
            println!("OPENAPI_KEY error: {}", e);
            return;
        },
    }
    
    let args = Args::parse();

    if args.summarize {
        println!("Summarize mode");
    }
    println!("prompt: {}", args.prompt.join(" "));
    let c = client::Client::new(openapi_key.unwrap());

     c.call("".to_string(), |s| println!("callback received {}", s)).await;
}


