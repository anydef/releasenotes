use clap::{Parser, Subcommand};
use dotenv::dotenv;
use octocrab::Octocrab;
use std::error::Error;
use std::env;
use releasenotes::list_commits;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List commit messages between two commit IDs or tags
    ListCommits {
        /// The owner of the repository
        #[arg(short, long)]
        owner: String,

        /// The name of the repository
        #[arg(short, long)]
        repo: String,

        /// The starting commit ID or tag
        #[arg(short, long)]
        from: String,

        /// The ending commit ID or tag
        #[arg(short, long)]
        to: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    let cli = Cli::parse();

    match &cli.command {
        Commands::ListCommits { owner, repo, from, to } => {
            // Get GitHub PAT from environment variables
            let token = env::var("GH_PAT").expect("GH_PAT environment variable not set");

            // Configure Octocrab with the GitHub PAT
            let octocrab = Octocrab::builder()
                .personal_token(token)
                .build()?;

            let results = list_commits(&octocrab, owner, repo, from, to).await?;
            for line in results {
                println!("{}", line);
            }
        }
    }

    Ok(())
}
