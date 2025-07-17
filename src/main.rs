#![allow(clippy::uninlined_format_args)]
use std::ops::Deref as _;

use alchemy::cli::Cli;
use alchemy::types::Response;
use clap::Parser;

fn main() {
    match Cli::parse().run().deref() {
        Ok(response) => match response {
            Response::String(s) => println!("{}", s),
            Response::Classifications(classifications) => {
                println!("{}", serde_json::to_string(&classifications).unwrap())
            }
            Response::Conversions(conversions) => {
                println!("{}", serde_json::to_string(&conversions).unwrap())
            }
            Response::Hash(hash) => {
                println!("{}", serde_json::to_string(&hash).unwrap())
            }
            Response::Json(json) => {
                println!("{}", serde_json::to_string(&json).unwrap())
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

