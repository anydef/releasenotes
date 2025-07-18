# releasenotes
LLM Rag to summarize and generate release notes from GitHub issues and pull requests.

*This is a 100% vibe-coded project. Use at your own risk.*

## CI/CD Pipeline
This project uses GitHub Actions for continuous integration and deployment. The pipeline:
- Builds and tests the project for both amd64 and arm64 architectures
- Publishes executables to GitHub releases when a new tag is pushed

Pre-built binaries are available for Linux (amd64 and arm64) in the GitHub releases section.

# How to use:

## Prerequisites
1. Rust and Cargo installed
2. GitHub Personal Access Token (PAT)
3. OpenAI API key

## Setup
1. Clone the repository
2. Create a `.env` file in the project root with the following variables:
   ```
   GH_PAT=your_github_personal_access_token
   OPENAI_API_KEY=your_openai_api_key
   OPENAI_MODEL=gpt-4  # or another OpenAI model
   ```
3. Build the project with `cargo build`

## Commands

### List Commits
Lists all commit messages between two commit IDs or tags:

```bash
cargo run -- list-commits -o OWNER -r REPO -f FROM_REFERENCE -t TO_REFERENCE
```

Where `FROM_REFERENCE` and `TO_REFERENCE` can be either commit IDs or git tags.


### Generate Release Notes
Generates release notes using an LLM based on commit messages and author information:

```bash
cargo run -- generate-release-notes -o OWNER -r REPO -f FROM_REFERENCE -t TO_REFERENCE [-u OUTPUT_FILE]
```

Where `FROM_REFERENCE` and `TO_REFERENCE` can be either commit IDs or git tags.

Parameters:
- `-o, --owner`: GitHub repository owner
- `-r, --repo`: GitHub repository name
- `-f, --from`: Starting reference (commit ID or git tag)
- `-t, --to`: Ending reference (commit ID or git tag)
- `-u, --output-file`: Optional file to save commit messages and diff
