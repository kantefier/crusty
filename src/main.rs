use std::env;

mod settings;
use settings::Settings;
use flexi_logger::Logger;

pub mod we {
    tonic::include_proto!("wavesenterprise");
}

use we::blockchain_events_service_client as bc_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::try_with_env_or_str("info")?
        .format(flexi_logger::detailed_format)
        .start()?;

    log::info!("Crusty starting...");

    let config_path = env::args()
        .nth(1)
        .expect("Expected an argument with path to the configuration file");

    let setting = Settings::new(config_path.as_str()).unwrap();

    log::debug!("Settings: {:#?}", setting);

    let node_endpoint = format!("http://{host}:{port}", host = setting.node.ip, port = setting.node.port);
    log::trace!("Node endpoint: '{}'", node_endpoint);
    log::info!("Connecting to node...");

    let mut node_client = bc_client::BlockchainEventsServiceClient::connect(node_endpoint).await?;
    log::debug!("Established a connection to the node");

    let start_request = we::SubscribeOnRequest {
        start_from: Some(we::subscribe_on_request::StartFrom::GenesisBlock(we::GenesisBlock {})),
        // start_from: Some(we::subscribe_on_request::StartFrom::CurrentEvent(we::CurrentEvent {})),
        events_filters: Vec::new()
    };
    log::trace!("Prepared a request to be sent to the node: {:#?}", start_request);

    match node_client.subscribe_on(start_request).await {
        Err(status) => log::error!("Failed during SubscribeOn: {}", status),
        Ok(response) => {
            log::trace!("Got a response from subscribe_on RPC call");
            let mut events = response.into_inner();
            let mut counter: i8 = 1;

            // Let's just see 5 events for now
            while let Some(event) = events.message().await? {
                log::trace!("counter = {}", counter);
                log::debug!("Received event: {:?}", event);
                counter += 1;
                if counter > 5 {
                    log::trace!("counter > 5 condition reached, breaking out of receive loop");
                    break;
                }
            }
        }
    }

    log::info!("Shutting down...");
    Ok(())
}