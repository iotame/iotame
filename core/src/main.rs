use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::logging::setup_logger;
use crate::runtime::Runtime;
use log::*;

mod config;
mod logging;
mod server;
mod runtime;

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(version, about, name = "iotame")]
pub(crate) struct Cli {
    /// Sets a custom working directory
    #[arg(short, long, value_name = "DIRECTORY", default_value=Runtime::default_working_directory().into_os_string())]
    working_directory: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug, Serialize, Deserialize)]
pub(crate) enum Commands {
    /// Starts iotame
    Start(StartArgs),
}

#[derive(clap::Args, Debug, Serialize, Deserialize)]
pub(crate) struct StartArgs {
    /// Use a separately running frontend dev server
    #[arg(short, long)]
    dev: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let runtime = Runtime::new(&cli)?;

    // Set up logging before doing anything else
    setup_logger(runtime.working_directory.clone(), runtime.config.log_level)?;
    trace!("Configuration initialized: {:?}", runtime.config);

    match &cli.command {
        Commands::Start(args) => {
            runtime.start(args).await?;
        }
    }

    Ok(())
}
