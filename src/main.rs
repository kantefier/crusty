use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Config {
    node: NodeCfg,
    mongo: MongoCfg
}

#[derive(Deserialize)]
struct NodeCfg {
    ip: String,
    port: u16
}

#[derive(Deserialize)]
struct MongoCfg {
    ip: String,
    port: Option<u16>,
    user: String,
    password: String
}


fn main() {
    println!("Hello, world!");
}
