use serde::Deserialize;
use config::{ConfigError, Config, File, Environment};

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub node: NodeCfg,
    pub mongo: MongoCfg,
}

#[derive(Deserialize, Debug)]
pub struct NodeCfg {
    pub ip: String,
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct MongoCfg {
    pub ip: String,
    pub port: Option<u16>,
    pub user: String,
    pub password: String,
}

impl Settings {
    pub fn new(config_path: &str) -> Result<Self, ConfigError> {
        Config::builder()
            // Read application's default config
            .add_source(File::with_name(DEFAULT_CONFIG_PATH))
            // Read user provided config file
            .add_source(File::with_name(config_path))
            // Read settings from the ENV with a "CRUSTY" prefix and an underscore separator
            // E.g.: `CRUSTY_MONGO_PASSWORD=qwerty` should override `mongo.password` setting
            .add_source(Environment::with_prefix("CRUSTY").separator("_"))
            .build()?
            .try_deserialize()
    }
}

const DEFAULT_CONFIG_PATH: &str = "config/default.toml";