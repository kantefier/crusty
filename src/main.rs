use std::env;

mod settings;
use settings::Settings;

fn main() {
    println!("Crusty starting...");
    let config_path = env::args()
        .nth(1)
        .expect("Expected an argument with path to the configuration file");

    let setting = Settings::new(config_path.as_str()).unwrap();

    println!("Look what I've got: {:?}", setting)
}