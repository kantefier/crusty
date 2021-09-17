use std::env;

mod settings;
use settings::Settings;
use mongodb::{Client, options::ClientOptions, options::Credential, options::ServerAddress, bson::doc};

pub mod wavesenterprise {
    tonic::include_proto!("wavesenterprise");
}

#[allow(unused_imports)] //TODO: remove this later please
use wavesenterprise::blockchain_events_service_client;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    println!("Crusty starting...");
    let config_path = env::args()
        .nth(1)
        .expect("Expected an argument with path to the configuration file");

    let setting = Settings::new(config_path.as_str()).unwrap();

    println!("Look what I've got: {:?}", setting);

    let mongo_client_options = ClientOptions::builder()
        .app_name("Crusty".to_string())
        .hosts(vec![ServerAddress::Tcp {
            host: setting.mongo.ip,
            port: setting.mongo.port,
        }])
        .credential(Credential::builder()
            .username(setting.mongo.user)
            .password(setting.mongo.password)
            .build())
        .build();

    let mongo_client = Client::with_options(mongo_client_options)?;

    // Ping the server to see if you can connect to the cluster
    mongo_client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("MongoDB Connected successfully.");

    Ok(())
}