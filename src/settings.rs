use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
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