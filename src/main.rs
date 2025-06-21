use anyhow::Result;
use clap::Parser;
use tracing_subscriber::{filter::EnvFilter, fmt, prelude::*};

// Используем библиотеку вместо объявления модулей
use cutlist_optimizer_cli::cli::args::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    cli.execute().await?;

    Ok(())
}
