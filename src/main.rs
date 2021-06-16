use std::fs::File;
use std::env;
use std::io::Read;

mod settings;

fn main() {
    println!("Hello, world!");
    let mut args = env::args();
    let mut config_content = String::new();
    if args.len() > 1 {
        // we'll take the first argument as a path to config
        let config_path = args.nth(1).unwrap();
        File::open(&config_path)
            .and_then(|mut f| f.read_to_string(&mut config_content))
            .unwrap();
    } else {
        panic!("Expected an argument, providing path to the config file")
    }
    let config: settings::Config = toml::from_str(&config_content).unwrap();
    println!("Look what I've got: {:?}", config)
}
