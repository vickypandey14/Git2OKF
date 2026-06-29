use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Analyze a Git repository
    Analyze {
        /// The URL of the repository to analyze
        url: String,

        /// Output format (json or yaml)
        #[arg(long, default_value = "json")]
        format: String,

        /// Output to a specific file
        #[arg(short, long)]
        output: Option<String>,

        /// Shallow clone depth
        #[arg(long)]
        depth: Option<u32>,
    },
}
