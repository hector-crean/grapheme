use citation::reference::reference_style::Reference;
use citation::routes::search::post::SearchResponse;
use citation::Server;
use log::info;
use reqwest;
use tokio;

use std::collections::HashMap;

use citation::repository::hashmap::HashMapRepository;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    // Initialize logger first, after loading .env
    dotenv::dotenv().expect("Failed to load .env file");
    env_logger::init();

    let port: u16 = 7654;
    info!("Starting server on port {}", port);

    let db = HashMapRepository::new();
    info!("Initialized database");

    info!("Starting server...");
    let _ = Server::new(db).run(port).await?;

    Ok(())
}
