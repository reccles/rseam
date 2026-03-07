mod api_client;
mod cli;
mod commands;
mod error;
mod help_agent;
mod types;

use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Handle --help-agent before anything else
    if cli.help_agent {
        println!("{}", help_agent::generate_agent_context());
        return Ok(());
    }

    cli.execute().await
}
