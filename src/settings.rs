use serde::Deserialize;
use config::{ConfigError, Config, File, Environment};

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub node: NodeCfg,
    pub mongo: MongoCfg
}

#[derive(Deserialize, Debug)]
pub struct NodeCfg {
    pub ip: String,
    pub port: u16
}

#[derive(Deserialize, Debug)]
pub struct MongoCfg {
    pub ip: String,
    pub port: Option<u16>,
    pub user: String,
    pub password: String
}

impl Settings {
    pub fn new(config_path: &str) -> Result<Self, ConfigError> {
        let mut s = Config::default();

        // Read application's default config
        s.merge(File::with_name(DEFAULT_CONFIG_PATH))?;

        // Read user provided config file
        s.merge(File::with_name(config_path))?;

        // Read settings from the ENV with a "CRUSTY" prefix and an underscore separator
        // E.g.: `CRUSTY_MONGO_PASSWORD=qwerty` should override `mongo.password` setting
        s.merge(Environment::with_prefix("CRUSTY").separator("_"))?;

        // Finally, deserialization into the Settings struct
        s.try_into()
    }
}

const DEFAULT_CONFIG_PATH: &str = "config/default.toml";