
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use color_eyre::eyre;
use log::info;
use rich_text_api::repository::hashmap::HashMapRepository;


#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    dotenv::dotenv().ok();
    env_logger::init();

    let server = rich_text_api::Server::new(HashMapRepository::new());
    server.run(3001).await?;

    Ok(())
}
