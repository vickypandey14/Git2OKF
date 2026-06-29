use clap::Parser;
use git2okf::cli::args::{Cli, Commands};
use git2okf::cli::commands::handle_analyze;
use std::process;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let cli = Cli::parse();

    match &cli.command {
        Commands::Analyze { url } => {
            if let Err(e) = handle_analyze(url).await {
                tracing::error!("Error analyzing repository: {}", e);
                process::exit(1);
            }
        }
    }
}
