use clap::Parser;
use git2okf::cli::args::{Cli, Commands};
use git2okf::cli::commands::handle_analyze;
use std::process;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let filter = if cli.verbose { "git2okf=debug" } else { "git2okf=info" };
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(filter))
        .init();

    match &cli.command {
        Commands::Analyze { url, format, output, depth } => {
            if let Err(e) = handle_analyze(url, format, output.as_deref(), *depth).await {
                tracing::error!("Error analyzing repository: {}", e);
                process::exit(1);
            }
        }
    }
}
