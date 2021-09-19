use std::env;

mod settings;
use settings::Settings;
use mongodb::{Client, options::ClientOptions, options::Credential, options::ServerAddress, bson::doc};

pub mod we {
    tonic::include_proto!("wavesenterprise");
}

#[allow(unused_imports)] //TODO: remove this later please
use we::blockchain_events_service_client as bc_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let node_endpoint = format!("http://{host}:{port}", host = setting.node.ip, port = setting.node.port);

    let mut node_client = bc_client::BlockchainEventsServiceClient::connect(node_endpoint).await?;

    let start_request = we::SubscribeOnRequest {
        start_from: Some(we::subscribe_on_request::StartFrom::GenesisBlock(we::GenesisBlock {})),
        // start_from: Some(we::subscribe_on_request::StartFrom::CurrentEvent(we::CurrentEvent {})),
        events_filters: Vec::new()
    };

    match node_client.subscribe_on(start_request).await {
        Err(status) => println!("Failed during SubscribeOn: {}", status),
        Ok(response) => {
            let mut events = response.into_inner();
            let mut counter: i8 = 1;

            // Let's just see 5 events for now
            while let Some(event) = events.message().await? {
                println!("[counter {}] Received {:?}", counter, event);
                counter += 1;
                if counter > 5 {
                    break;
                }
            }
            println!("Outside of while let loop");
        }
    }

    println!("So long!");
    Ok(())
}