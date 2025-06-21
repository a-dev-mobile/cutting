use anyhow::Result;
use clap::Parser;

// Используем библиотеку вместо объявления модулей
use cutlist_optimizer_cli::{cli::args::Cli, logging};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging with verbose flag from CLI
    if let Err(e) = logging::init_cli(cli.verbose) {
        eprintln!("Ошибка инициализации логирования: {}", e);
        std::process::exit(1);
    }

    cli.execute().await?;

    Ok(())
}
