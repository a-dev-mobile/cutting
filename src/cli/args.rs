use crate::{
    cli::commands::{example_command, optimize_command, validate_command},
    errors::Result,
    constants::ConfigurationDefaults,
};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "cutlist")]
#[command(about = "Optimize material cutting layouts")]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Number of threads to use
    #[arg(short, long, global = true, default_value_t = num_cpus::get())]
    pub threads: usize,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Optimize cutting layout from input file
    Optimize {
        /// Input file (CSV or JSON)
        #[arg(short, long)]
        input: PathBuf,

        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Configuration file
        #[arg(short, long)]
        config: Option<PathBuf>,

        /// Cut thickness (kerf) in mm
        #[arg(long, default_value_t = ConfigurationDefaults::DEFAULT_CUT_THICKNESS)]
        cut_thickness: i32,

        /// Minimum trim dimension in mm
        #[arg(long, default_value_t = ConfigurationDefaults::DEFAULT_MIN_TRIM_DIMENSION)]
        min_trim: i32,

        /// Optimization accuracy (1-10)
        #[arg(long, default_value_t = ConfigurationDefaults::DEFAULT_OPTIMIZATION_FACTOR)]
        accuracy: i32,
    },

    /// Validate input file format
    Validate {
        /// Input file to validate
        #[arg(short, long)]
        input: PathBuf,
    },

    /// Show example input file format
    Example {
        /// Output format (csv, json)
        #[arg(short, long, default_value = "csv")]
        format: String,
    },
}

impl Cli {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            Commands::Optimize {
                input,
                output,
                config,
                cut_thickness,
                min_trim,
                accuracy,
            } => {
                optimize_command(
                    input,
                    output,
                    config,
                    cut_thickness,
                    min_trim,
                    accuracy,
                    self.threads,
                )
                .await
            }
            Commands::Validate { input } => validate_command(input).await,
            Commands::Example { format } => example_command(format).await,
        }
    }
}
