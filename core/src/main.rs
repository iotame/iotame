use crate::config::IotameConfig;
use clap::Parser;
use log::*;
use serde::{Deserialize, Serialize};
use server::*;
use std::path::PathBuf;

mod config;
mod logging;
mod server;

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(version, about, name = "iotame")]
pub(crate) struct Cli {
    /// Sets a custom environment path
    #[arg(short, long, value_name = "DIRECTORY", default_value=IotameConfig::default().environment_path.into_os_string())]
    environment_path: Option<PathBuf>,

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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config = IotameConfig::load_or_create(&cli)?;

    logging::setup_logger(&config).unwrap();

    trace!("{config:?}");
    info!("Initializing application in {:?}", config.environment_path);

    match &cli.command {
        Commands::Start(args) => {
            start_server(args).await;
        }
    }

    Ok(())
}
