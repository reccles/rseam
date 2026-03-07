mod api_client;
mod cli;
mod commands;
mod error;
mod types;

use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.execute().await
}
