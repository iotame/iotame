use figment::providers::{Env, Format, Serialized, Yaml};
use figment::Figment;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct IotameConfig {
    pub log_level: log::LevelFilter,
}

impl IotameConfig {
    pub fn figment(config_file_path: PathBuf) -> Figment {
        Figment::from(Serialized::defaults(IotameConfig::default()))
            .merge(Yaml::file(config_file_path))
            .merge(Env::prefixed("IOTAME_"))
    }
}

impl Default for IotameConfig {
    fn default() -> Self {
        Self {
            log_level: log::LevelFilter::Info,
        }
    }
}
