mod core;
mod cli;
mod git;
mod detector;
mod parser;
mod analyzer;
mod graph;
mod okf;
mod output;

use clap::Parser;
use cli::args::{Cli, Commands};
use cli::commands::handle_analyze;
use std::process;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Analyze { url } => {
            if let Err(e) = handle_analyze(url).await {
                eprintln!("Error analyzing repository: {}", e);
                process::exit(1);
            }
        }
    }
}
