use clap::{Parser, Subcommand};
use octocrab::Octocrab;
use std::error::Error;
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
    let cli = Cli::parse();

    match &cli.command {
        Commands::ListCommits { owner, repo, from, to } => {
            let octocrab = Octocrab::builder().build()?;
            let results = list_commits(&octocrab, owner, repo, from, to).await?;
            for line in results {
                println!("{}", line);
            }
        }
    }

    Ok(())
}
