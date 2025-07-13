use clap::{Parser, Subcommand};
use dotenv::dotenv;
use octocrab::Octocrab;
use std::error::Error;
use std::env;
use std::path::PathBuf;
use releasenotes::{list_commits, generate_release_notes};

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
    /// Generate release notes using LLM based on commit messages and diff
    GenerateReleaseNotes {
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

        /// Optional file to save commit messages and diff
        #[arg(short = 'u', long)]
        output_file: Option<PathBuf>,
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
        },
        Commands::GenerateReleaseNotes { owner, repo, from, to, output_file } => {
            // Get GitHub PAT from environment variables
            let token = env::var("GH_PAT").expect("GH_PAT environment variable not set");

            // Configure Octocrab with the GitHub PAT
            let octocrab = Octocrab::builder()
                .personal_token(token)
                .build()?;

            // Get commit messages and diff (but don't print them)
            let commit_info = list_commits(&octocrab, owner, repo, from, to).await?;

            // Generate release notes
            let release_notes = generate_release_notes(&commit_info, output_file.as_deref()).await?;

            // Print only the release notes
            println!("\n=== RELEASE NOTES ===\n");
            println!("{}", release_notes);
            println!("\n=====================\n");
        }
    }

    Ok(())
}
