use clap::Parser;
use alchemy::cli::{Cli, Commands, Response};
use alchemy::commands::SubCommand;

fn main() {
    let cli = Cli::parse();
    let list_mode = cli.list;
    
    let result = match &cli.command {
        Commands::Classify(cmd) => cmd.run(list_mode),
        Commands::Convert(cmd) => cmd.run(list_mode),
        Commands::ClassifyAndConvert(cmd) => cmd.run(list_mode),
        Commands::FlattenArray(cmd) => cmd.run(list_mode),
        Commands::ChunkArray(cmd) => cmd.run(list_mode),
        Commands::ReverseArray(cmd) => cmd.run(list_mode),
        Commands::RotateArray(cmd) => cmd.run(list_mode),
        Commands::Generate(cmd) => cmd.run(list_mode),
        Commands::Random(cmd) => cmd.run(list_mode),
        Commands::PadLeft(cmd) => cmd.run(list_mode),
        Commands::PadRight(cmd) => cmd.run(list_mode),
        Commands::Hash(cmd) => cmd.run(list_mode),
        Commands::ClassifyAndHash(cmd) => cmd.run(list_mode),
    };

    match result {
        Ok(response) => {
            match response {
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
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}