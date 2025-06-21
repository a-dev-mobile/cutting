use anyhow::Result;
use clap::Parser;

use tracing_subscriber::{filter::EnvFilter, fmt, prelude::*};

use crate::cli::args::Cli;

mod cli;
mod engine;
mod error;
// mod io;
mod models;
mod utils;



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
