use crate::Cli;
use figment::providers::{Env, Format, Serialized, Yaml};
use figment::value::{Dict, Map};
use figment::{Error, Figment, Metadata, Profile, Provider};
use serde::{Deserialize, Serialize};
use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct IotameConfig {
    // Don't serialize the environment path, as it is essentially only a runtime option and does
    // not need to be stored in the configuration file
    #[serde(skip_serializing)]
    pub environment_path: PathBuf,
    pub log_level: log::LevelFilter,
}

impl IotameConfig {
    /// Creates a default figment for the iotame configuration. This figment is populated with the
    /// default config and overridden by any potentially set `IOTAME_*` environment variables.
    fn figment(cli_args: &Cli) -> Figment {
        Figment::from(IotameConfig::default())
            .merge(Serialized::defaults(cli_args))
            .merge(Env::prefixed("IOTAME_"))
    }

    fn config_path(env_path: &Path) -> PathBuf {
        env_path.join("iotame.yaml")
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::create_dir_all(&self.environment_path)?;

        let contents = serde_yaml::to_string(self)?;
        std::fs::write(Self::config_path(&self.environment_path), contents)?;

        Ok(())
    }

    pub fn load_or_create(cli_args: &Cli) -> Result<Self, Box<dyn std::error::Error>> {
        let figment = Self::figment(cli_args);

        let env_path: PathBuf = figment.extract_inner("environment_path")?;
        let config_path = Self::config_path(&env_path);

        if !config_path.exists() {
            let default_config: IotameConfig = figment.extract()?;
            default_config.save()?;
        }

        figment
            .join(Yaml::file(config_path))
            .extract()
            .map_err(Into::into)
    }
}

impl Default for IotameConfig {
    fn default() -> Self {
        Self {
            environment_path: env::home_dir().unwrap().join(".iotame"),
            log_level: log::LevelFilter::Info,
        }
    }
}

impl Provider for IotameConfig {
    fn metadata(&self) -> Metadata {
        Metadata::named("Iotame config")
    }

    fn data(&self) -> Result<Map<Profile, Dict>, Error> {
        use figment::value::Value;

        // Start with the serialized defaults (which excludes environment_path)
        let mut map = Serialized::defaults(Self::default()).data()?;

        // Manually add the environment_path to the default profile
        if let Some(dict) = map.get_mut(&Profile::Default) {
            dict.insert(
                "environment_path".into(),
                Value::serialize(&self.environment_path)?,
            );
        }

        Ok(map)
    }
}
