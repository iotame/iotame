use std::env;
use std::path::PathBuf;
use crate::{Cli, StartArgs};
use crate::config::IotameConfig;
use anyhow::Result;
use log::info;
use crate::server::*;

const CONFIG_FILE_NAME: &str = "iotame.yaml";

#[derive(Debug)]
pub struct Runtime {
    pub working_directory: PathBuf,
    pub config: IotameConfig,
}

impl Runtime {
    pub fn new(cli: &Cli) -> Result<Self> {
        let path = cli.working_directory.as_deref().unwrap();
        let config = IotameConfig::figment(path.join(CONFIG_FILE_NAME))
            .extract()?;

        Ok(Self {
            working_directory: PathBuf::from(path),
            config
        })
    }

    pub fn default_working_directory() -> PathBuf {
        env::home_dir().unwrap().join(".iotame")
    }

    fn save_config(&self) -> Result<()> {
        let contents = serde_yaml::to_string(&self.config)?;
        std::fs::write(self.working_directory.join(CONFIG_FILE_NAME), contents)?;

        Ok(())
    }

    pub async fn start(&self, args: &StartArgs) -> Result<()> {
        std::fs::create_dir_all(&self.working_directory)?;
        self.save_config()?;

        info!("Initializing application in {:?}", self.working_directory);

        start_server(args).await;
        Ok(())
    }
}